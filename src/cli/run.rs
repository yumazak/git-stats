//! CLI execution logic

use crate::cli::args::{AddArgs, Args, Command, ListArgs, OutputFormat, RemoveArgs};
use crate::config::{
    Config, Defaults, RepoConfig, default_config_path, default_config_path_for_save, expand_tilde,
    load_config, save_config,
};
use crate::error::{Error, Result};
use crate::git::{CommitInfo, Repository};
use crate::output::{CsvFormatter, Formatter, JsonFormatter};
use crate::stats::{DateRange, Days, collect_activity_stats, collect_stats};
use crate::tui::App;
use indicatif::{ProgressBar, ProgressStyle};
use std::path::{Path, PathBuf};
use std::time::Duration;

/// Repository info for analysis
struct RepoInfo {
    path: PathBuf,
    name: String,
    branch: Option<String>,
}

/// RAII guard for spinner to ensure cleanup on error
struct SpinnerGuard(Option<ProgressBar>);

impl SpinnerGuard {
    fn new(enabled: bool) -> Self {
        let spinner = if enabled {
            let sp = ProgressBar::new_spinner();
            sp.set_style(
                ProgressStyle::default_spinner()
                    .template("{spinner:.cyan} {msg}")
                    .expect("valid template"),
            );
            sp.enable_steady_tick(Duration::from_millis(80));
            sp.set_message("Loading repositories...");
            Some(sp)
        } else {
            None
        };
        Self(spinner)
    }

    fn set_message(&self, msg: impl Into<std::borrow::Cow<'static, str>>) {
        if let Some(sp) = &self.0 {
            sp.set_message(msg);
        }
    }
}

impl Drop for SpinnerGuard {
    fn drop(&mut self) {
        if let Some(sp) = self.0.take() {
            sp.finish_and_clear();
        }
    }
}

/// Execute the CLI with the given arguments
///
/// # Errors
///
/// Returns an error if:
/// - Configuration loading fails
/// - Repository access fails
/// - Output formatting fails
///
/// # Panics
///
/// Panics if the progress bar style template is invalid (should never happen).
// Takes ownership because args.command is consumed by match
#[allow(clippy::needless_pass_by_value)]
pub fn execute(args: Args) -> Result<()> {
    // Handle subcommands
    if let Some(command) = args.command {
        return match command {
            Command::Add(add_args) => execute_add(add_args, args.config),
            Command::Remove(remove_args) => execute_remove(remove_args, args.config),
            Command::List(list_args) => execute_list(list_args, args.config),
        };
    }

    // Default: analyze repositories
    // Create spinner for TUI output (RAII ensures cleanup on error)
    let spinner = SpinnerGuard::new(matches!(args.output, OutputFormat::Tui));

    // Get repositories to analyze
    let repos = get_repositories(&args)?;

    // Calculate date range
    let range = DateRange::last_n_days(Days::new(args.days));
    let exclude_merges = !args.include_merges;

    // Collect commits from all repositories
    let mut all_commits: Vec<CommitInfo> = Vec::new();
    let mut repo_names: Vec<String> = Vec::new();
    let repo_count = repos.len();

    for (i, repo_info) in repos.iter().enumerate() {
        spinner.set_message(format!("Collecting commits ({}/{})...", i + 1, repo_count));
        let repo = Repository::open(&repo_info.path, &repo_info.name)?;
        let branch = args.branch.as_deref().or(repo_info.branch.as_deref());
        let commits = repo.commits_in_range(range.from, range.to, branch, exclude_merges)?;
        all_commits.extend(commits);
        repo_names.push(repo_info.name.clone());
    }

    // Create combined repository name
    let combined_name = repo_names
        .first()
        .filter(|_| repo_names.len() == 1)
        .cloned()
        .unwrap_or_else(|| format!("{} repos", repo_names.len()));

    // Collect statistics
    spinner.set_message("Calculating statistics...");
    let extensions = args.ext.as_deref();
    let activity_stats = collect_activity_stats(&all_commits);
    let result = collect_stats(&combined_name, all_commits, range, args.period, extensions);

    // Spinner is automatically cleared by Drop when going out of scope or on error
    drop(spinner);

    // Format and output
    match args.output {
        OutputFormat::Json => {
            let formatter = JsonFormatter::new();
            let output = formatter.format(&result)?;
            println!("{output}");
        }
        OutputFormat::Csv => {
            let formatter = CsvFormatter::new();
            let output = formatter.format(&result)?;
            print!("{output}");
        }
        OutputFormat::Tui => {
            let mut app = App::new(result, activity_stats, args.single_metric);
            app.run()?;
        }
    }

    Ok(())
}

/// Get all repositories to analyze
fn get_repositories(args: &Args) -> Result<Vec<RepoInfo>> {
    // Priority: --repo flag > config file > current directory

    // 1. --repo flag takes highest priority (single repo)
    if let Some(repo_path) = &args.repo {
        let expanded = expand_tilde(repo_path);
        let name = expanded.file_name().map_or_else(
            || "repository".to_string(),
            |s| s.to_string_lossy().to_string(),
        );
        return Ok(vec![RepoInfo {
            path: expanded,
            name,
            branch: args.branch.clone(),
        }]);
    }

    // 2. Try to load config file
    let config_path = args.config.clone().or_else(default_config_path);

    if let Some(path) = config_path
        && path.exists()
    {
        let config = load_config(&path)?;
        let repos = filter_and_validate_repos(&config.repositories, args.repo_name.as_deref());

        if !repos.is_empty() {
            return Ok(repos);
        }
    }

    // 3. Fall back to current directory
    let current_dir = std::env::current_dir()?;
    let name = current_dir.file_name().map_or_else(
        || "repository".to_string(),
        |s| s.to_string_lossy().to_string(),
    );

    // Check if current directory is a git repo
    if !current_dir.join(".git").exists() {
        return Err(Error::NoRepositories);
    }

    Ok(vec![RepoInfo {
        path: current_dir,
        name,
        branch: args.branch.clone(),
    }])
}

/// Filter repositories by name and validate they exist
fn filter_and_validate_repos(repos: &[RepoConfig], filter: Option<&[String]>) -> Vec<RepoInfo> {
    repos
        .iter()
        .filter(|repo| {
            // Filter by name if specified
            if let Some(names) = filter
                && !names.iter().any(|n| n == &repo.name)
            {
                return false;
            }

            // Validate repository exists
            let expanded = expand_tilde(&repo.path);
            expanded.exists() && (expanded.join(".git").exists() || expanded.join("HEAD").exists())
        })
        .map(|repo| RepoInfo {
            path: expand_tilde(&repo.path),
            name: repo.name.clone(),
            branch: repo.branch.clone(),
        })
        .collect()
}

/// Execute the `add` subcommand
fn execute_add(add_args: AddArgs, config_path: Option<PathBuf>) -> Result<()> {
    // Resolve the path
    let path = expand_tilde(&add_args.path);
    let absolute_path = if path.is_absolute() {
        path
    } else {
        std::env::current_dir()?.join(&path).canonicalize()?
    };

    // Verify it's a git repository
    if !is_git_repo(&absolute_path) {
        return Err(Error::NotGitRepo {
            path: absolute_path,
        });
    }

    // Determine the repository name
    let name = add_args.name.unwrap_or_else(|| {
        absolute_path.file_name().map_or_else(
            || "repository".to_string(),
            |s| s.to_string_lossy().to_string(),
        )
    });

    // Get config path
    let config_file = config_path
        .or_else(default_config_path)
        .or_else(default_config_path_for_save)
        .ok_or_else(|| Error::ConfigInvalid {
            message: "Could not determine config path".to_string(),
        })?;

    // Load existing config or create new one
    let mut config = if config_file.exists() {
        load_config(&config_file)?
    } else {
        Config {
            schema: Some(
                "https://raw.githubusercontent.com/yumazak/kodo/main/schemas/config.schema.json"
                    .to_string(),
            ),
            repositories: Vec::new(),
            defaults: Defaults::default(),
        }
    };

    // Format path for storage (use ~ for home directory)
    let path_for_storage = shorten_home_path(&absolute_path);

    // Check for duplicates
    if config
        .repositories
        .iter()
        .any(|r| expand_tilde(&r.path) == absolute_path)
    {
        println!("Repository already exists in config: {name}");
        return Ok(());
    }

    // Add the repository
    let repo_config = RepoConfig {
        name: name.clone(),
        path: path_for_storage.clone(),
        branch: add_args.branch,
    };
    config.repositories.push(repo_config);

    // Save the config
    save_config(&config, &config_file)?;

    println!("Added repository: {name}");
    println!("  Path: {}", path_for_storage.display());
    println!("  Config: {}", config_file.display());

    Ok(())
}

/// Execute the `remove` subcommand
// Takes ownership because we consume identifier from remove_args
#[allow(clippy::needless_pass_by_value)]
fn execute_remove(remove_args: RemoveArgs, config_path: Option<PathBuf>) -> Result<()> {
    // Get config path
    let config_file =
        config_path
            .or_else(default_config_path)
            .ok_or_else(|| Error::ConfigNotFound {
                path: PathBuf::from("~/.config/kodo/config.json"),
            })?;

    // Config must exist to remove from it
    if !config_file.exists() {
        return Err(Error::ConfigNotFound { path: config_file });
    }

    // Load config
    let mut config = load_config(&config_file)?;

    // Resolve identifier as path
    let identifier = &remove_args.identifier;
    let identifier_path = expand_tilde(Path::new(identifier));
    let absolute_identifier = if identifier_path.is_absolute() {
        identifier_path
    } else {
        std::env::current_dir()?
            .join(&identifier_path)
            .canonicalize()
            .unwrap_or(identifier_path)
    };

    // Find and remove matching repository
    let original_len = config.repositories.len();

    config.repositories.retain(|repo| {
        // Match by name
        if repo.name == *identifier {
            return false;
        }
        // Match by path
        let repo_path = expand_tilde(&repo.path);
        if repo_path == absolute_identifier {
            return false;
        }
        true
    });

    // Check if anything was removed
    if config.repositories.len() == original_len {
        return Err(Error::RepoNotInConfig {
            identifier: identifier.clone(),
        });
    }

    // Save config
    save_config(&config, &config_file)?;

    println!("Removed repository: {identifier}");
    println!("  Config: {}", config_file.display());

    Ok(())
}

/// Execute the `list` subcommand
// Takes ownership for consistency with other execute_* functions
#[allow(clippy::needless_pass_by_value)]
fn execute_list(list_args: ListArgs, config_path: Option<PathBuf>) -> Result<()> {
    // Get config path
    let config_file = config_path.or_else(default_config_path);

    // Check if config exists
    let config_file = match config_file {
        Some(path) if path.exists() => path,
        _ => {
            if list_args.json {
                println!("[]");
            } else {
                println!("No repositories registered.");
                println!("Use 'kodo add <path>' to register a repository.");
            }
            return Ok(());
        }
    };

    // Load config
    let config = load_config(&config_file)?;

    // Check if there are any repositories
    if config.repositories.is_empty() {
        if list_args.json {
            println!("[]");
        } else {
            println!("No repositories registered.");
            println!("Use 'kodo add <path>' to register a repository.");
        }
        return Ok(());
    }

    // Build repository info list
    let repos: Vec<_> = config
        .repositories
        .iter()
        .map(|repo| {
            let expanded_path = expand_tilde(&repo.path);
            let exists = is_git_repo(&expanded_path);
            (repo, exists)
        })
        .collect();

    if list_args.json {
        // JSON output
        let json_repos: Vec<_> = repos
            .iter()
            .map(|(repo, exists)| {
                serde_json::json!({
                    "name": repo.name,
                    "path": repo.path.display().to_string(),
                    "branch": repo.branch,
                    "exists": exists,
                })
            })
            .collect();
        println!("{}", serde_json::to_string_pretty(&json_repos)?);
    } else {
        // Table output
        print_repo_table(&repos);
    }

    Ok(())
}

/// Print repositories in table format
fn print_repo_table(repos: &[(&crate::config::RepoConfig, bool)]) {
    // Calculate column widths
    let name_width = repos
        .iter()
        .map(|(r, _)| r.name.len())
        .max()
        .unwrap_or(4)
        .max(4); // "Name" header

    let path_width = repos
        .iter()
        .map(|(r, _)| r.path.display().to_string().len())
        .max()
        .unwrap_or(4)
        .max(4); // "Path" header

    let branch_width = repos
        .iter()
        .map(|(r, _)| r.branch.as_ref().map_or(1, String::len))
        .max()
        .unwrap_or(6)
        .max(6); // "Branch" header

    // Print header
    println!(
        "{:<name_width$}  {:<path_width$}  {:<branch_width$}  Status",
        "Name", "Path", "Branch"
    );

    // Print rows
    for (repo, exists) in repos {
        let branch = repo.branch.as_deref().unwrap_or("-");
        let status = if *exists { "\u{2713}" } else { "\u{2717}" };
        println!(
            "{:<name_width$}  {:<path_width$}  {:<branch_width$}  {}",
            repo.name,
            repo.path.display(),
            branch,
            status
        );
    }
}

/// Check if a path is a git repository
fn is_git_repo(path: &Path) -> bool {
    path.join(".git").exists() || path.join("HEAD").exists()
}

/// Shorten path by replacing home directory with ~
fn shorten_home_path(path: &Path) -> PathBuf {
    if let Some(home) = dirs::home_dir()
        && let Ok(relative) = path.strip_prefix(&home)
    {
        return PathBuf::from("~").join(relative);
    }
    path.to_path_buf()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::process::Command;
    use tempfile::TempDir;

    fn create_test_repo() -> TempDir {
        let dir = TempDir::new().unwrap();
        let path = dir.path();

        Command::new("git")
            .args(["init"])
            .current_dir(path)
            .output()
            .unwrap();

        Command::new("git")
            .args(["config", "user.email", "test@example.com"])
            .current_dir(path)
            .output()
            .unwrap();

        Command::new("git")
            .args(["config", "user.name", "Test User"])
            .current_dir(path)
            .output()
            .unwrap();

        std::fs::write(path.join("README.md"), "# Test\n").unwrap();

        Command::new("git")
            .args(["add", "."])
            .current_dir(path)
            .output()
            .unwrap();

        Command::new("git")
            .args(["commit", "-m", "Initial commit"])
            .current_dir(path)
            .output()
            .unwrap();

        dir
    }

    #[test]
    fn test_execute_with_repo_arg() {
        let dir = create_test_repo();

        let args = Args {
            command: None,
            config: None,
            repo: Some(dir.path().to_path_buf()),
            days: 7,
            include_merges: false,
            output: OutputFormat::Json,
            period: crate::cli::args::Period::Daily,
            branch: None,
            ext: None,
            single_metric: false,
            repo_name: None,
        };

        let result = execute(args);
        assert!(result.is_ok());
    }

    #[test]
    fn test_get_repositories_with_repo_arg() {
        let args = Args {
            command: None,
            config: None,
            repo: Some(PathBuf::from("/tmp/test-repo")),
            days: 7,
            include_merges: false,
            output: OutputFormat::Json,
            period: crate::cli::args::Period::Daily,
            branch: None,
            ext: None,
            single_metric: false,
            repo_name: None,
        };

        let result = get_repositories(&args);
        assert!(result.is_ok());

        let repos = result.unwrap();
        assert_eq!(repos.len(), 1);
        assert_eq!(repos[0].path, PathBuf::from("/tmp/test-repo"));
        assert_eq!(repos[0].name, "test-repo");
    }

    #[test]
    fn test_filter_and_validate_repos() {
        // Empty list should return empty
        let repos: Vec<RepoConfig> = vec![];
        let result = filter_and_validate_repos(&repos, None);
        assert!(result.is_empty());
    }

    #[test]
    fn test_execute_list_no_config() {
        // Test list with non-existent config file
        let list_args = ListArgs { json: false };
        let result = execute_list(list_args, Some(PathBuf::from("/nonexistent/config.json")));
        assert!(result.is_ok());
    }

    #[test]
    fn test_execute_list_json_no_config() {
        // Test list --json with non-existent config file
        let list_args = ListArgs { json: true };
        let result = execute_list(list_args, Some(PathBuf::from("/nonexistent/config.json")));
        assert!(result.is_ok());
    }

    #[test]
    fn test_execute_list_with_repos() {
        // Create a test repo and config
        let dir = create_test_repo();
        let config_dir = TempDir::new().unwrap();
        let config_path = config_dir.path().join("config.json");

        // Create config with the test repo
        let config = Config {
            schema: None,
            repositories: vec![RepoConfig {
                name: "test-repo".to_string(),
                path: dir.path().to_path_buf(),
                branch: Some("main".to_string()),
            }],
            defaults: Defaults::default(),
        };
        save_config(&config, &config_path).unwrap();

        // Test list
        let list_args = ListArgs { json: false };
        let result = execute_list(list_args, Some(config_path.clone()));
        assert!(result.is_ok());

        // Test list --json
        let list_args = ListArgs { json: true };
        let result = execute_list(list_args, Some(config_path));
        assert!(result.is_ok());
    }

    #[test]
    fn test_is_git_repo() {
        let dir = create_test_repo();
        assert!(is_git_repo(dir.path()));

        let non_git_dir = TempDir::new().unwrap();
        assert!(!is_git_repo(non_git_dir.path()));
    }
}

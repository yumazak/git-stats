//! CLI execution logic

use crate::cli::args::{Args, OutputFormat};
use crate::config::{default_config_path, expand_tilde, load_config, RepoConfig};
use crate::error::{Error, Result};
use crate::git::{CommitInfo, Repository};
use crate::output::{CsvFormatter, Formatter, JsonFormatter};
use crate::stats::{collect_stats, DateRange, Days};
use crate::tui::App;
use std::path::PathBuf;

/// Repository info for analysis
struct RepoInfo {
    path: PathBuf,
    name: String,
    branch: Option<String>,
}

/// Execute the CLI with the given arguments
///
/// # Errors
///
/// Returns an error if:
/// - Configuration loading fails
/// - Repository access fails
/// - Output formatting fails
pub fn execute(args: Args) -> Result<()> {
    // Get repositories to analyze
    let repos = get_repositories(&args)?;

    // Calculate date range
    let range = DateRange::last_n_days(Days::new(args.days));
    let exclude_merges = !args.include_merges;

    // Collect commits from all repositories
    let mut all_commits: Vec<CommitInfo> = Vec::new();
    let mut repo_names: Vec<String> = Vec::new();

    for repo_info in &repos {
        let repo = Repository::open(&repo_info.path, &repo_info.name)?;
        let branch = args.branch.as_deref().or(repo_info.branch.as_deref());
        let commits = repo.commits_in_range(range.from, range.to, branch, exclude_merges)?;
        all_commits.extend(commits);
        repo_names.push(repo_info.name.clone());
    }

    // Create combined repository name
    let combined_name = if repo_names.len() == 1 {
        repo_names[0].clone()
    } else {
        format!("{} repos", repo_names.len())
    };

    // Collect statistics
    let extensions = args.ext.as_deref();
    let result = collect_stats(&combined_name, all_commits, range, args.period, extensions);

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
            let mut app = App::new(result, args.single_metric);
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
        let name = expanded
            .file_name()
            .map_or_else(|| "repository".to_string(), |s| s.to_string_lossy().to_string());
        return Ok(vec![RepoInfo {
            path: expanded,
            name,
            branch: args.branch.clone(),
        }]);
    }

    // 2. Try to load config file
    let config_path = args.config.clone().or_else(default_config_path);

    if let Some(path) = config_path {
        if path.exists() {
            let config = load_config(&path)?;
            let repos = filter_and_validate_repos(&config.repositories, args.repo_name.as_deref());

            if !repos.is_empty() {
                return Ok(repos);
            }
        }
    }

    // 3. Fall back to current directory
    let current_dir = std::env::current_dir()?;
    let name = current_dir
        .file_name()
        .map_or_else(|| "repository".to_string(), |s| s.to_string_lossy().to_string());

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
            if let Some(names) = filter {
                if !names.iter().any(|n| n == &repo.name) {
                    return false;
                }
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
}

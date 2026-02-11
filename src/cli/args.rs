//! Command-line argument definitions

use clap::{Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

/// Analyze Git commit statistics across repositories
#[derive(Parser, Debug)]
#[command(name = "kodo")]
#[command(version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub command: Option<Command>,

    /// Path to config file
    #[arg(short, long, env = "KODO_CONFIG", global = true)]
    pub config: Option<PathBuf>,

    /// Repository path (overrides config)
    #[arg(short, long)]
    pub repo: Option<PathBuf>,

    /// Number of days to analyze
    #[arg(short, long, default_value = "7")]
    pub days: u32,

    /// Include merge commits
    #[arg(long)]
    pub include_merges: bool,

    /// Output format
    #[arg(short, long, value_enum, default_value_t = OutputFormat::Table)]
    pub output: OutputFormat,

    /// Aggregation period
    #[arg(short, long, value_enum, default_value = "daily")]
    pub period: Period,

    /// Branch to analyze
    #[arg(short, long)]
    pub branch: Option<String>,

    /// File extensions to include (comma-separated)
    #[arg(long, value_delimiter = ',')]
    pub ext: Option<Vec<String>>,

    /// Show single metric instead of all metrics (TUI mode)
    #[arg(long)]
    pub single_metric: bool,

    /// Filter repositories by name (comma-separated, from config)
    #[arg(long, value_delimiter = ',')]
    pub repo_name: Option<Vec<String>>,
}

/// Available subcommands
#[derive(Subcommand, Debug)]
pub enum Command {
    /// Add a repository to the configuration
    Add(AddArgs),
    /// Remove a repository from the configuration
    Remove(RemoveArgs),
    /// List registered repositories
    List(ListArgs),
}

/// Arguments for the `add` subcommand
#[derive(Parser, Debug)]
pub struct AddArgs {
    /// Path to the repository to add (use . for current directory)
    pub path: PathBuf,

    /// Display name for the repository (defaults to directory name)
    #[arg(short, long)]
    pub name: Option<String>,

    /// Default branch to analyze
    #[arg(short, long)]
    pub branch: Option<String>,
}

/// Arguments for the `remove` subcommand
#[derive(Parser, Debug)]
pub struct RemoveArgs {
    /// Repository path or name to remove
    pub identifier: String,
}

/// Arguments for the `list` subcommand
#[derive(Parser, Debug)]
pub struct ListArgs {
    /// Output in JSON format
    #[arg(long)]
    pub json: bool,
}

/// Output format options
#[derive(ValueEnum, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum OutputFormat {
    /// Terminal UI with charts
    Tui,
    /// Table output
    #[default]
    Table,
    /// JSON output
    Json,
    /// CSV output
    Csv,
}

impl std::fmt::Display for OutputFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Tui => write!(f, "tui"),
            Self::Table => write!(f, "table"),
            Self::Json => write!(f, "json"),
            Self::Csv => write!(f, "csv"),
        }
    }
}

/// Time period for aggregation
#[derive(ValueEnum, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Period {
    /// Aggregate by day
    #[default]
    Daily,
    /// Aggregate by week
    Weekly,
    /// Aggregate by month
    Monthly,
    /// Aggregate by year
    Yearly,
}

impl std::fmt::Display for Period {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Daily => write!(f, "daily"),
            Self::Weekly => write!(f, "weekly"),
            Self::Monthly => write!(f, "monthly"),
            Self::Yearly => write!(f, "yearly"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::CommandFactory;

    #[test]
    fn test_output_format_display() {
        assert_eq!(OutputFormat::Tui.to_string(), "tui");
        assert_eq!(OutputFormat::Table.to_string(), "table");
        assert_eq!(OutputFormat::Json.to_string(), "json");
        assert_eq!(OutputFormat::Csv.to_string(), "csv");
    }

    #[test]
    fn test_period_display() {
        assert_eq!(Period::Daily.to_string(), "daily");
        assert_eq!(Period::Weekly.to_string(), "weekly");
        assert_eq!(Period::Monthly.to_string(), "monthly");
        assert_eq!(Period::Yearly.to_string(), "yearly");
    }

    #[test]
    fn test_args_defaults() {
        let args = Args::parse_from(["kodo"]);
        assert_eq!(args.days, 7);
        assert!(!args.include_merges);
        assert_eq!(args.output, OutputFormat::Table);
        assert_eq!(args.period, Period::Daily);
        assert!(args.command.is_none());
    }

    #[test]
    fn test_args_with_repo() {
        let args = Args::parse_from(["kodo", "--repo", "/tmp/repo"]);
        assert_eq!(args.repo, Some(PathBuf::from("/tmp/repo")));
    }

    #[test]
    fn test_args_with_days() {
        let args = Args::parse_from(["kodo", "--days", "30"]);
        assert_eq!(args.days, 30);
    }

    #[test]
    fn test_args_output_tui_explicit() {
        let args = Args::parse_from(["kodo", "--output", "tui"]);
        assert_eq!(args.output, OutputFormat::Tui);
    }

    #[test]
    fn test_args_output_tui_short() {
        let args = Args::parse_from(["kodo", "-o", "tui"]);
        assert_eq!(args.output, OutputFormat::Tui);
    }

    #[test]
    fn test_args_output_json_explicit() {
        let args = Args::parse_from(["kodo", "--output", "json"]);
        assert_eq!(args.output, OutputFormat::Json);
    }

    #[test]
    fn test_args_output_csv_explicit() {
        let args = Args::parse_from(["kodo", "--output", "csv"]);
        assert_eq!(args.output, OutputFormat::Csv);
    }

    #[test]
    fn test_args_with_extensions() {
        let args = Args::parse_from(["kodo", "--ext", "rs,ts,js"]);
        assert_eq!(
            args.ext,
            Some(vec!["rs".to_string(), "ts".to_string(), "js".to_string()])
        );
    }

    #[test]
    fn test_add_command() {
        let args = Args::parse_from(["kodo", "add", "."]);
        assert!(matches!(args.command, Some(Command::Add(_))));
        if let Some(Command::Add(add_args)) = args.command {
            assert_eq!(add_args.path, PathBuf::from("."));
            assert!(add_args.name.is_none());
            assert!(add_args.branch.is_none());
        }
    }

    #[test]
    fn test_add_command_with_options() {
        let args = Args::parse_from([
            "kodo",
            "add",
            "/tmp/repo",
            "--name",
            "my-repo",
            "--branch",
            "main",
        ]);
        if let Some(Command::Add(add_args)) = args.command {
            assert_eq!(add_args.path, PathBuf::from("/tmp/repo"));
            assert_eq!(add_args.name, Some("my-repo".to_string()));
            assert_eq!(add_args.branch, Some("main".to_string()));
        }
    }

    #[test]
    fn test_list_command() {
        let args = Args::parse_from(["kodo", "list"]);
        assert!(matches!(args.command, Some(Command::List(_))));
        if let Some(Command::List(list_args)) = args.command {
            assert!(!list_args.json);
        }
    }

    #[test]
    fn test_list_command_with_json() {
        let args = Args::parse_from(["kodo", "list", "--json"]);
        assert!(matches!(args.command, Some(Command::List(_))));
        if let Some(Command::List(list_args)) = args.command {
            assert!(list_args.json);
        }
    }

    #[test]
    fn test_help_includes_output_short() {
        let help = Args::command().render_help().to_string();
        assert!(help.contains("-o, --output <OUTPUT>"));
    }
}

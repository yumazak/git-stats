//! Command-line argument definitions

use clap::{Parser, ValueEnum};
use std::path::PathBuf;

/// Analyze Git commit statistics across repositories
#[derive(Parser, Debug)]
#[command(name = "kodo")]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Path to config file
    #[arg(short, long, env = "KODO_CONFIG")]
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
    #[arg(short, long, value_enum, default_value = "tui")]
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

/// Output format options
#[derive(ValueEnum, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum OutputFormat {
    /// Terminal UI with charts
    #[default]
    Tui,
    /// JSON output
    Json,
    /// CSV output
    Csv,
}

impl std::fmt::Display for OutputFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Tui => write!(f, "tui"),
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

    #[test]
    fn test_output_format_display() {
        assert_eq!(OutputFormat::Tui.to_string(), "tui");
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
        assert_eq!(args.output, OutputFormat::Tui);
        assert_eq!(args.period, Period::Daily);
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
    fn test_args_with_extensions() {
        let args = Args::parse_from(["kodo", "--ext", "rs,ts,js"]);
        assert_eq!(
            args.ext,
            Some(vec!["rs".to_string(), "ts".to_string(), "js".to_string()])
        );
    }
}

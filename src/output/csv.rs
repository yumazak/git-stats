//! CSV output formatter

use crate::error::Result;
use crate::output::Formatter;
use crate::stats::AnalysisResult;
use std::fmt::Write;

/// CSV output formatter
pub struct CsvFormatter {
    /// Whether to include headers
    pub include_headers: bool,
}

impl CsvFormatter {
    /// Create a new CSV formatter with headers enabled
    #[must_use]
    pub fn new() -> Self {
        Self {
            include_headers: true,
        }
    }

    /// Create a CSV formatter without headers
    #[must_use]
    pub fn without_headers() -> Self {
        Self {
            include_headers: false,
        }
    }
}

impl Default for CsvFormatter {
    fn default() -> Self {
        Self::new()
    }
}

impl Formatter for CsvFormatter {
    fn format(&self, result: &AnalysisResult) -> Result<String> {
        let mut output = String::new();

        // Add headers if enabled
        if self.include_headers {
            output.push_str("date,commits,additions,deletions,net_lines,files_changed\n");
        }

        // Add data rows
        for stat in &result.stats {
            let _ = writeln!(
                output,
                "{},{},{},{},{},{}",
                stat.date,
                stat.commits,
                stat.additions,
                stat.deletions,
                stat.net_lines,
                stat.files_changed
            );
        }

        // Add total row
        let total = &result.total;
        let _ = writeln!(
            output,
            "TOTAL,{},{},{},{},{}",
            total.commits, total.additions, total.deletions, total.net_lines, total.files_changed
        );

        Ok(output)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stats::{PeriodStats, TotalStats};
    use chrono::NaiveDate;

    fn make_result() -> AnalysisResult {
        let from = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
        let to = NaiveDate::from_ymd_opt(2024, 1, 7).unwrap();

        let stats = vec![
            PeriodStats {
                label: "2024-01-01".to_string(),
                date: from,
                commits: 5,
                additions: 100,
                deletions: 20,
                net_lines: 80,
                files_changed: 10,
            },
            PeriodStats {
                label: "2024-01-02".to_string(),
                date: NaiveDate::from_ymd_opt(2024, 1, 2).unwrap(),
                commits: 3,
                additions: 50,
                deletions: 10,
                net_lines: 40,
                files_changed: 5,
            },
        ];

        AnalysisResult {
            repository: "test-repo".to_string(),
            period: "daily".to_string(),
            from,
            to,
            stats,
            total: TotalStats {
                commits: 8,
                additions: 150,
                deletions: 30,
                net_lines: 120,
                files_changed: 15,
            },
        }
    }

    #[test]
    fn test_csv_formatter_with_headers() {
        let formatter = CsvFormatter::new();
        let result = make_result();

        let csv = formatter.format(&result).unwrap();

        assert!(csv.starts_with("date,commits,additions,deletions,net_lines,files_changed\n"));
        assert!(csv.contains("2024-01-01,5,100,20,80,10\n"));
        assert!(csv.contains("2024-01-02,3,50,10,40,5\n"));
        assert!(csv.contains("TOTAL,8,150,30,120,15\n"));
    }

    #[test]
    fn test_csv_formatter_without_headers() {
        let formatter = CsvFormatter::without_headers();
        let result = make_result();

        let csv = formatter.format(&result).unwrap();

        assert!(!csv.starts_with("date,"));
        assert!(csv.starts_with("2024-01-01,5,100,20,80,10\n"));
    }

    #[test]
    fn test_csv_formatter_line_count() {
        let formatter = CsvFormatter::new();
        let result = make_result();

        let csv = formatter.format(&result).unwrap();
        let lines: Vec<&str> = csv.lines().collect();

        // Header + 2 data rows + 1 total row = 4 lines
        assert_eq!(lines.len(), 4);
    }

    #[test]
    fn test_csv_negative_values() {
        let from = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
        let result = AnalysisResult {
            repository: "test".to_string(),
            period: "daily".to_string(),
            from,
            to: from,
            stats: vec![PeriodStats {
                label: "2024-01-01".to_string(),
                date: from,
                commits: 1,
                additions: 10,
                deletions: 50,
                net_lines: -40,
                files_changed: 1,
            }],
            total: TotalStats {
                commits: 1,
                additions: 10,
                deletions: 50,
                net_lines: -40,
                files_changed: 1,
            },
        };

        let formatter = CsvFormatter::new();
        let csv = formatter.format(&result).unwrap();

        assert!(csv.contains("-40"));
    }
}

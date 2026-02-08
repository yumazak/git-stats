//! JSON output formatter

use crate::error::Result;
use crate::output::Formatter;
use crate::stats::AnalysisResult;

/// JSON output formatter
pub struct JsonFormatter {
    /// Whether to pretty-print the output
    pub pretty: bool,
}

impl JsonFormatter {
    /// Create a new JSON formatter with pretty printing enabled
    #[must_use]
    pub fn new() -> Self {
        Self { pretty: true }
    }

    /// Create a compact JSON formatter (no pretty printing)
    #[must_use]
    pub fn compact() -> Self {
        Self { pretty: false }
    }
}

impl Default for JsonFormatter {
    fn default() -> Self {
        Self::new()
    }
}

impl Formatter for JsonFormatter {
    fn format(&self, result: &AnalysisResult) -> Result<String> {
        let json = if self.pretty {
            serde_json::to_string_pretty(result)?
        } else {
            serde_json::to_string(result)?
        };
        Ok(json)
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
    fn test_json_formatter_pretty() {
        let formatter = JsonFormatter::new();
        let result = make_result();

        let json = formatter.format(&result).unwrap();

        // Pretty printed JSON should have newlines
        assert!(json.contains('\n'));
        assert!(json.contains("test-repo"));
        assert!(json.contains("\"commits\": 5"));
    }

    #[test]
    fn test_json_formatter_compact() {
        let formatter = JsonFormatter::compact();
        let result = make_result();

        let json = formatter.format(&result).unwrap();

        // Compact JSON should not have newlines (except possibly in strings)
        assert!(!json.contains("\n  "));
        assert!(json.contains("test-repo"));
    }

    #[test]
    fn test_json_formatter_valid_json() {
        let formatter = JsonFormatter::new();
        let result = make_result();

        let json = formatter.format(&result).unwrap();

        // Should be valid JSON that can be parsed back
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed["repository"], "test-repo");
        assert_eq!(parsed["period"], "daily");
        assert_eq!(parsed["stats"].as_array().unwrap().len(), 2);
    }

    #[test]
    fn test_json_date_format() {
        let formatter = JsonFormatter::new();
        let result = make_result();

        let json = formatter.format(&result).unwrap();

        // Dates should be in YYYY-MM-DD format
        assert!(json.contains("\"from\": \"2024-01-01\""));
        assert!(json.contains("\"to\": \"2024-01-07\""));
    }
}

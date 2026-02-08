//! Core statistics types

#![allow(clippy::cast_possible_wrap)]

use chrono::{NaiveDate, Utc};
use serde::Serialize;

/// Days count (non-negative)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Days(pub u32);

impl Days {
    /// Create a new Days value
    #[must_use]
    pub const fn new(days: u32) -> Self {
        Self(days)
    }

    /// Get the inner value
    #[must_use]
    pub const fn value(self) -> u32 {
        self.0
    }
}

impl From<u32> for Days {
    fn from(days: u32) -> Self {
        Self(days)
    }
}

/// Date range for analysis
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DateRange {
    /// Start date (inclusive)
    pub from: NaiveDate,
    /// End date (inclusive)
    pub to: NaiveDate,
}

impl DateRange {
    /// Create a date range for the last N days (including today)
    #[must_use]
    pub fn last_n_days(days: Days) -> Self {
        let to = Utc::now().date_naive();
        let from = to - chrono::Duration::days(i64::from(days.0));
        Self { from, to }
    }

    /// Create a date range from explicit dates
    #[must_use]
    pub const fn new(from: NaiveDate, to: NaiveDate) -> Self {
        Self { from, to }
    }

    /// Check if a date falls within this range
    #[must_use]
    pub fn contains(&self, date: NaiveDate) -> bool {
        date >= self.from && date <= self.to
    }

    /// Iterate over all dates in the range
    pub fn iter_days(&self) -> impl Iterator<Item = NaiveDate> {
        let from = self.from;
        let to = self.to;
        std::iter::successors(Some(from), move |&d| {
            let next = d + chrono::Duration::days(1);
            if next <= to { Some(next) } else { None }
        })
    }
}

/// Statistics for a single time period
#[derive(Debug, Clone, Serialize, Default)]
pub struct PeriodStats {
    /// Period identifier (date, week, month, or year label)
    pub label: String,

    /// Start date of the period
    #[serde(serialize_with = "serialize_date")]
    pub date: NaiveDate,

    /// Number of commits
    pub commits: u32,

    /// Lines added
    pub additions: u64,

    /// Lines deleted
    pub deletions: u64,

    /// Net line change (additions - deletions)
    pub net_lines: i64,

    /// Number of files changed
    pub files_changed: u32,
}

// serde's serialize_with requires `fn(&T, S)` signature
#[allow(clippy::trivially_copy_pass_by_ref)]
fn serialize_date<S>(date: &NaiveDate, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_str(&date.format("%Y-%m-%d").to_string())
}

impl PeriodStats {
    /// Create a new `PeriodStats` for a given date
    #[must_use]
    pub fn new(date: NaiveDate) -> Self {
        Self {
            label: date.format("%Y-%m-%d").to_string(),
            date,
            ..Default::default()
        }
    }

    /// Create with a custom label (for weekly/monthly/yearly)
    #[must_use]
    pub fn with_label(date: NaiveDate, label: String) -> Self {
        Self {
            label,
            date,
            ..Default::default()
        }
    }

    /// Calculate net line change
    #[must_use]
    pub fn calculate_net_lines(&self) -> i64 {
        self.additions as i64 - self.deletions as i64
    }

    /// Merge another period's stats into this one
    pub fn merge(&mut self, other: &Self) {
        self.commits += other.commits;
        self.additions += other.additions;
        self.deletions += other.deletions;
        self.files_changed += other.files_changed;
        self.net_lines = self.calculate_net_lines();
    }

    /// Update `net_lines` based on current additions/deletions
    pub fn update_net_lines(&mut self) {
        self.net_lines = self.calculate_net_lines();
    }
}

/// Complete analysis result
#[derive(Debug, Clone, Serialize)]
pub struct AnalysisResult {
    /// Repository name
    pub repository: String,

    /// Aggregation period type (daily, weekly, monthly, yearly)
    pub period: String,

    /// Start date of analysis
    #[serde(serialize_with = "serialize_date")]
    pub from: NaiveDate,

    /// End date of analysis
    #[serde(serialize_with = "serialize_date")]
    pub to: NaiveDate,

    /// Statistics per period
    pub stats: Vec<PeriodStats>,

    /// Total statistics across all periods
    pub total: TotalStats,
}

impl AnalysisResult {
    /// Create a new analysis result
    #[must_use]
    pub fn new(
        repository: String,
        period: String,
        from: NaiveDate,
        to: NaiveDate,
        stats: Vec<PeriodStats>,
    ) -> Self {
        let total = TotalStats::from_periods(&stats);
        Self {
            repository,
            period,
            from,
            to,
            stats,
            total,
        }
    }
}

/// Activity statistics by weekday and hour
#[derive(Debug, Clone, Default)]
pub struct ActivityStats {
    /// Commits per weekday (0=Mon, 1=Tue, ..., 6=Sun)
    pub weekday: [u32; 7],
    /// Commits per hour (0-23)
    pub hourly: [u32; 24],
}

impl ActivityStats {
    /// Get weekday labels
    #[must_use]
    pub const fn weekday_labels() -> [&'static str; 7] {
        ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"]
    }

    /// Get hour labels (0-23)
    #[must_use]
    pub fn hour_labels() -> [String; 24] {
        std::array::from_fn(|i| i.to_string())
    }
}

/// Aggregated total statistics
#[derive(Debug, Clone, Serialize, Default)]
pub struct TotalStats {
    /// Total commits
    pub commits: u32,

    /// Total lines added
    pub additions: u64,

    /// Total lines deleted
    pub deletions: u64,

    /// Total net line change
    pub net_lines: i64,

    /// Total files changed
    pub files_changed: u32,
}

impl TotalStats {
    /// Calculate totals from period statistics
    #[must_use]
    pub fn from_periods(periods: &[PeriodStats]) -> Self {
        let mut total = Self::default();
        for p in periods {
            total.commits += p.commits;
            total.additions += p.additions;
            total.deletions += p.deletions;
            total.files_changed += p.files_changed;
        }
        total.net_lines = total.additions as i64 - total.deletions as i64;
        total
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_days_creation() {
        let days = Days::new(7);
        assert_eq!(days.value(), 7);

        let days: Days = 30.into();
        assert_eq!(days.value(), 30);
    }

    #[test]
    fn test_date_range_last_n_days() {
        let range = DateRange::last_n_days(Days::new(7));
        let today = Utc::now().date_naive();

        assert_eq!(range.to, today);
        assert!(range.from < range.to);
    }

    #[test]
    fn test_date_range_contains() {
        let from = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
        let to = NaiveDate::from_ymd_opt(2024, 1, 31).unwrap();
        let range = DateRange::new(from, to);

        assert!(range.contains(NaiveDate::from_ymd_opt(2024, 1, 15).unwrap()));
        assert!(range.contains(from));
        assert!(range.contains(to));
        assert!(!range.contains(NaiveDate::from_ymd_opt(2023, 12, 31).unwrap()));
        assert!(!range.contains(NaiveDate::from_ymd_opt(2024, 2, 1).unwrap()));
    }

    #[test]
    fn test_date_range_iter_days() {
        let from = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
        let to = NaiveDate::from_ymd_opt(2024, 1, 3).unwrap();
        let range = DateRange::new(from, to);

        let days: Vec<_> = range.iter_days().collect();
        assert_eq!(days.len(), 3);
        assert_eq!(days[0], from);
        assert_eq!(days[2], to);
    }

    #[test]
    fn test_period_stats_merge() {
        let mut stats1 = PeriodStats {
            commits: 5,
            additions: 100,
            deletions: 20,
            ..Default::default()
        };
        stats1.update_net_lines();

        let stats2 = PeriodStats {
            commits: 3,
            additions: 50,
            deletions: 10,
            ..Default::default()
        };

        stats1.merge(&stats2);

        assert_eq!(stats1.commits, 8);
        assert_eq!(stats1.additions, 150);
        assert_eq!(stats1.deletions, 30);
        assert_eq!(stats1.net_lines, 120);
    }

    #[test]
    fn test_total_stats_from_periods() {
        let periods = vec![
            PeriodStats {
                commits: 5,
                additions: 100,
                deletions: 20,
                files_changed: 10,
                ..Default::default()
            },
            PeriodStats {
                commits: 3,
                additions: 50,
                deletions: 10,
                files_changed: 5,
                ..Default::default()
            },
        ];

        let total = TotalStats::from_periods(&periods);

        assert_eq!(total.commits, 8);
        assert_eq!(total.additions, 150);
        assert_eq!(total.deletions, 30);
        assert_eq!(total.net_lines, 120);
        assert_eq!(total.files_changed, 15);
    }

    #[test]
    fn test_analysis_result_serialization() {
        let from = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
        let to = NaiveDate::from_ymd_opt(2024, 1, 7).unwrap();

        let result = AnalysisResult::new(
            "test-repo".to_string(),
            "daily".to_string(),
            from,
            to,
            vec![],
        );

        let json = serde_json::to_string(&result).unwrap();
        assert!(json.contains("\"repository\":\"test-repo\""));
        assert!(json.contains("\"from\":\"2024-01-01\""));
    }

    #[test]
    fn test_activity_stats_default() {
        let stats = ActivityStats::default();
        assert_eq!(stats.weekday, [0; 7]);
        assert_eq!(stats.hourly, [0; 24]);
    }

    #[test]
    fn test_activity_stats_weekday_labels() {
        let labels = ActivityStats::weekday_labels();
        assert_eq!(labels.len(), 7);
        assert_eq!(labels[0], "Mon");
        assert_eq!(labels[6], "Sun");
    }

    #[test]
    fn test_activity_stats_hour_labels() {
        let labels = ActivityStats::hour_labels();
        assert_eq!(labels.len(), 24);
        assert_eq!(labels[0], "0");
        assert_eq!(labels[23], "23");
    }
}

//! Commit information types

use crate::git::DiffStats;
use chrono::{DateTime, Utc};

/// Extracted commit information
#[derive(Debug, Clone)]
pub struct CommitInfo {
    /// Commit hash (short, 7 characters)
    pub id: String,

    /// Commit timestamp (UTC)
    pub timestamp: DateTime<Utc>,

    /// Is this a merge commit?
    pub is_merge: bool,

    /// Diff statistics for this commit
    pub diff: DiffStats,
}

impl CommitInfo {
    /// Create a new `CommitInfo`
    #[must_use]
    pub fn new(id: String, timestamp: DateTime<Utc>, is_merge: bool, diff: DiffStats) -> Self {
        Self {
            id,
            timestamp,
            is_merge,
            diff,
        }
    }

    /// Get the date portion of the timestamp (UTC)
    #[must_use]
    pub fn date(&self) -> chrono::NaiveDate {
        self.timestamp.date_naive()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn test_commit_info_date() {
        let timestamp = Utc.with_ymd_and_hms(2024, 1, 15, 10, 30, 0).unwrap();
        let commit = CommitInfo::new(
            "abc1234".to_string(),
            timestamp,
            false,
            DiffStats::default(),
        );

        assert_eq!(commit.date().to_string(), "2024-01-15");
    }

    #[test]
    fn test_commit_info_is_merge() {
        let timestamp = Utc::now();
        let commit = CommitInfo::new("abc1234".to_string(), timestamp, true, DiffStats::default());

        assert!(commit.is_merge);
    }
}

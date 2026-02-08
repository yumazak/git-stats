//! Statistics aggregation utilities
//!
//! This module provides additional aggregation functions beyond what's in collector.
//! Currently, most aggregation is done in collector.rs.

use crate::stats::PeriodStats;

/// Merge multiple period stats into one
#[must_use]
pub fn merge_stats(stats: &[PeriodStats]) -> PeriodStats {
    if stats.is_empty() {
        return PeriodStats::default();
    }

    let mut result = stats[0].clone();
    for stat in stats.iter().skip(1) {
        result.merge(stat);
    }
    result
}

/// Filter stats to only include non-zero periods
#[must_use]
pub fn filter_non_zero(stats: Vec<PeriodStats>) -> Vec<PeriodStats> {
    stats
        .into_iter()
        .filter(|s| s.commits > 0 || s.additions > 0 || s.deletions > 0)
        .collect()
}

/// Calculate running totals for each period
#[must_use]
pub fn running_totals(stats: &[PeriodStats]) -> Vec<PeriodStats> {
    let mut result = Vec::with_capacity(stats.len());
    let mut running = PeriodStats::default();

    for stat in stats {
        running.commits += stat.commits;
        running.additions += stat.additions;
        running.deletions += stat.deletions;
        running.files_changed += stat.files_changed;
        running.update_net_lines();

        let mut period = running.clone();
        period.date = stat.date;
        period.label = stat.label.clone();
        result.push(period);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    #[test]
    fn test_merge_stats_empty() {
        let result = merge_stats(&[]);
        assert_eq!(result.commits, 0);
    }

    #[test]
    fn test_merge_stats() {
        let stats = vec![
            PeriodStats {
                commits: 5,
                additions: 100,
                deletions: 10,
                ..Default::default()
            },
            PeriodStats {
                commits: 3,
                additions: 50,
                deletions: 5,
                ..Default::default()
            },
        ];

        let merged = merge_stats(&stats);
        assert_eq!(merged.commits, 8);
        assert_eq!(merged.additions, 150);
    }

    #[test]
    fn test_filter_non_zero() {
        let stats = vec![
            PeriodStats {
                date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
                commits: 5,
                ..Default::default()
            },
            PeriodStats {
                date: NaiveDate::from_ymd_opt(2024, 1, 2).unwrap(),
                commits: 0,
                ..Default::default()
            },
            PeriodStats {
                date: NaiveDate::from_ymd_opt(2024, 1, 3).unwrap(),
                commits: 3,
                ..Default::default()
            },
        ];

        let filtered = filter_non_zero(stats);
        assert_eq!(filtered.len(), 2);
    }

    #[test]
    fn test_running_totals() {
        let stats = vec![
            PeriodStats {
                date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
                commits: 5,
                additions: 100,
                ..Default::default()
            },
            PeriodStats {
                date: NaiveDate::from_ymd_opt(2024, 1, 2).unwrap(),
                commits: 3,
                additions: 50,
                ..Default::default()
            },
            PeriodStats {
                date: NaiveDate::from_ymd_opt(2024, 1, 3).unwrap(),
                commits: 2,
                additions: 30,
                ..Default::default()
            },
        ];

        let running = running_totals(&stats);

        assert_eq!(running[0].commits, 5);
        assert_eq!(running[1].commits, 8); // 5 + 3
        assert_eq!(running[2].commits, 10); // 5 + 3 + 2
        assert_eq!(running[2].additions, 180); // 100 + 50 + 30
    }
}

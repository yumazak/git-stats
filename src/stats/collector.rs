//! Statistics collection from commits

use crate::cli::args::Period;
use crate::git::CommitInfo;
use crate::stats::types::{AnalysisResult, DateRange, PeriodStats};
use chrono::{Datelike, NaiveDate};
use std::collections::HashMap;

/// Collect statistics from a list of commits
///
/// Groups commits by the specified period and calculates aggregate statistics.
/// Days with no commits are included with zero values.
#[must_use]
pub fn collect_stats(
    repo_name: &str,
    commits: Vec<CommitInfo>,
    range: DateRange,
    period: Period,
    extensions: Option<&[String]>,
) -> AnalysisResult {
    // Group commits by date
    let mut daily_stats: HashMap<NaiveDate, PeriodStats> = HashMap::new();

    for commit in commits {
        let date = commit.date();

        // Filter by extensions if specified
        let (additions, deletions, files_changed) = if let Some(exts) = extensions {
            let filtered: Vec<_> = commit
                .diff
                .files
                .iter()
                .filter(|f| f.matches_extensions(exts))
                .collect();

            (
                filtered.iter().map(|f| f.additions).sum(),
                filtered.iter().map(|f| f.deletions).sum(),
                filtered.len() as u32,
            )
        } else {
            (
                commit.diff.additions,
                commit.diff.deletions,
                commit.diff.files_changed,
            )
        };

        let entry = daily_stats.entry(date).or_insert_with(|| PeriodStats::new(date));
        entry.commits += 1;
        entry.additions += additions;
        entry.deletions += deletions;
        entry.files_changed += files_changed;
        entry.update_net_lines();
    }

    // Fill in missing days with zero stats
    for date in range.iter_days() {
        daily_stats.entry(date).or_insert_with(|| PeriodStats::new(date));
    }

    // Convert to sorted vector
    let mut stats: Vec<_> = daily_stats.into_values().collect();
    stats.sort_by_key(|s| s.date);

    // Apply period aggregation if not daily
    let stats = match period {
        Period::Daily => stats,
        Period::Weekly => aggregate_by_week(stats),
        Period::Monthly => aggregate_by_month(stats),
        Period::Yearly => aggregate_by_year(stats),
    };

    AnalysisResult::new(
        repo_name.to_string(),
        period.to_string(),
        range.from,
        range.to,
        stats,
    )
}

/// Aggregate daily stats by ISO week
fn aggregate_by_week(daily_stats: Vec<PeriodStats>) -> Vec<PeriodStats> {
    let mut weekly: HashMap<(i32, u32), PeriodStats> = HashMap::new();

    for stat in daily_stats {
        let week = stat.date.iso_week();
        let key = (week.year(), week.week());

        let entry = weekly.entry(key).or_insert_with(|| {
            PeriodStats::with_label(stat.date, format!("{}-W{:02}", week.year(), week.week()))
        });
        entry.merge(&stat);
    }

    let mut result: Vec<_> = weekly.into_values().collect();
    result.sort_by_key(|s| s.date);
    result
}

/// Aggregate daily stats by month
fn aggregate_by_month(daily_stats: Vec<PeriodStats>) -> Vec<PeriodStats> {
    let mut monthly: HashMap<(i32, u32), PeriodStats> = HashMap::new();

    for stat in daily_stats {
        let key = (stat.date.year(), stat.date.month());

        let entry = monthly.entry(key).or_insert_with(|| {
            PeriodStats::with_label(stat.date, format!("{}-{:02}", stat.date.year(), stat.date.month()))
        });
        entry.merge(&stat);
    }

    let mut result: Vec<_> = monthly.into_values().collect();
    result.sort_by_key(|s| s.date);
    result
}

/// Aggregate daily stats by year
fn aggregate_by_year(daily_stats: Vec<PeriodStats>) -> Vec<PeriodStats> {
    let mut yearly: HashMap<i32, PeriodStats> = HashMap::new();

    for stat in daily_stats {
        let year = stat.date.year();

        let entry = yearly.entry(year).or_insert_with(|| {
            PeriodStats::with_label(stat.date, year.to_string())
        });
        entry.merge(&stat);
    }

    let mut result: Vec<_> = yearly.into_values().collect();
    result.sort_by_key(|s| s.date);
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::git::{DiffStats, FileChange};
    use chrono::{TimeZone, Utc};

    fn make_commit(date: NaiveDate, additions: u64, deletions: u64) -> CommitInfo {
        let timestamp = Utc.from_utc_datetime(&date.and_hms_opt(12, 0, 0).unwrap());
        CommitInfo {
            id: "abc1234".to_string(),
            timestamp,
            is_merge: false,
            diff: DiffStats::new(additions, deletions, 1),
        }
    }

    #[test]
    fn test_collect_stats_empty() {
        let range = DateRange::new(
            NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            NaiveDate::from_ymd_opt(2024, 1, 3).unwrap(),
        );

        let result = collect_stats("test", vec![], range, Period::Daily, None);

        assert_eq!(result.repository, "test");
        assert_eq!(result.stats.len(), 3); // 3 days with zeros
        assert_eq!(result.total.commits, 0);
    }

    #[test]
    fn test_collect_stats_with_commits() {
        let date1 = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
        let date2 = NaiveDate::from_ymd_opt(2024, 1, 2).unwrap();

        let commits = vec![
            make_commit(date1, 100, 10),
            make_commit(date1, 50, 5),
            make_commit(date2, 30, 3),
        ];

        let range = DateRange::new(date1, date2);
        let result = collect_stats("test", commits, range, Period::Daily, None);

        assert_eq!(result.stats.len(), 2);
        assert_eq!(result.total.commits, 3);
        assert_eq!(result.total.additions, 180);
        assert_eq!(result.total.deletions, 18);
    }

    #[test]
    fn test_collect_stats_with_extension_filter() {
        let date = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
        let timestamp = Utc.from_utc_datetime(&date.and_hms_opt(12, 0, 0).unwrap());

        let mut diff = DiffStats::default();
        diff.add_file(FileChange::new("src/main.rs".to_string(), 100, 10));
        diff.add_file(FileChange::new("src/lib.ts".to_string(), 50, 5));
        diff.add_file(FileChange::new("README.md".to_string(), 20, 2));

        let commit = CommitInfo {
            id: "abc1234".to_string(),
            timestamp,
            is_merge: false,
            diff,
        };

        let range = DateRange::new(date, date);
        let extensions = vec!["rs".to_string()];
        let result = collect_stats("test", vec![commit], range, Period::Daily, Some(&extensions));

        // Only .rs file should be counted
        assert_eq!(result.total.additions, 100);
        assert_eq!(result.total.deletions, 10);
        assert_eq!(result.total.files_changed, 1);
    }

    #[test]
    fn test_aggregate_by_week() {
        // Create stats for two weeks
        let week1_day1 = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(); // Monday
        let week1_day2 = NaiveDate::from_ymd_opt(2024, 1, 2).unwrap();
        let week2_day1 = NaiveDate::from_ymd_opt(2024, 1, 8).unwrap(); // Next Monday

        let daily = vec![
            PeriodStats {
                date: week1_day1,
                commits: 2,
                additions: 100,
                deletions: 10,
                ..Default::default()
            },
            PeriodStats {
                date: week1_day2,
                commits: 3,
                additions: 50,
                deletions: 5,
                ..Default::default()
            },
            PeriodStats {
                date: week2_day1,
                commits: 1,
                additions: 20,
                deletions: 2,
                ..Default::default()
            },
        ];

        let weekly = aggregate_by_week(daily);

        assert_eq!(weekly.len(), 2);
        // First week: 2 + 3 commits
        assert_eq!(weekly[0].commits, 5);
        // Second week: 1 commit
        assert_eq!(weekly[1].commits, 1);
    }

    #[test]
    fn test_aggregate_by_month() {
        let jan = NaiveDate::from_ymd_opt(2024, 1, 15).unwrap();
        let feb = NaiveDate::from_ymd_opt(2024, 2, 15).unwrap();

        let daily = vec![
            PeriodStats {
                date: jan,
                commits: 5,
                additions: 100,
                ..Default::default()
            },
            PeriodStats {
                date: feb,
                commits: 3,
                additions: 50,
                ..Default::default()
            },
        ];

        let monthly = aggregate_by_month(daily);

        assert_eq!(monthly.len(), 2);
        assert!(monthly[0].label.contains("2024-01"));
        assert!(monthly[1].label.contains("2024-02"));
    }
}

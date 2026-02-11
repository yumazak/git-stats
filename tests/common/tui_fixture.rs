use chrono::NaiveDate;
use kodo::stats::{ActivityStats, AnalysisResult, PeriodStats};
use kodo::tui::App;

pub fn fixed_analysis_result() -> AnalysisResult {
    let stats = vec![
        period(2024, 1, 1, 3, 120, 30, 8),
        period(2024, 1, 2, 5, 180, 40, 11),
        period(2024, 1, 3, 2, 60, 15, 4),
        period(2024, 1, 4, 4, 150, 45, 9),
        period(2024, 1, 5, 6, 220, 70, 13),
        period(2024, 1, 6, 1, 20, 10, 2),
        period(2024, 1, 7, 3, 90, 25, 6),
    ];

    AnalysisResult::new(
        "kodo".to_string(),
        "daily".to_string(),
        date(2024, 1, 1),
        date(2024, 1, 7),
        stats,
    )
}

pub fn fixed_activity_stats() -> ActivityStats {
    ActivityStats {
        weekday: [3, 5, 2, 4, 6, 1, 3],
        hourly: [
            0, 0, 0, 0, 0, 1, 2, 1, 3, 4, 3, 2, 1, 2, 3, 2, 4, 5, 4, 3, 2, 1, 0, 0,
        ],
    }
}

pub fn make_app(single_metric: bool) -> App {
    App::new(
        fixed_analysis_result(),
        fixed_activity_stats(),
        single_metric,
    )
}

fn period(
    year: i32,
    month: u32,
    day: u32,
    commits: u32,
    additions: u64,
    deletions: u64,
    files_changed: u32,
) -> PeriodStats {
    let date = date(year, month, day);
    let additions_i64 =
        i64::try_from(additions).expect("fixed test additions must fit in i64 range");
    let deletions_i64 =
        i64::try_from(deletions).expect("fixed test deletions must fit in i64 range");
    PeriodStats {
        label: date.format("%Y-%m-%d").to_string(),
        date,
        commits,
        additions,
        deletions,
        net_lines: additions_i64 - deletions_i64,
        files_changed,
    }
}

fn date(year: i32, month: u32, day: u32) -> NaiveDate {
    NaiveDate::from_ymd_opt(year, month, day).expect("fixed test date must be valid")
}

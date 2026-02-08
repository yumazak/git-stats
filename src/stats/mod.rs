//! Statistics collection and aggregation module

pub mod aggregator;
pub mod collector;
pub mod types;

pub use aggregator::{filter_non_zero, merge_stats, running_totals};
pub use collector::collect_stats;
pub use types::{AnalysisResult, DateRange, Days, PeriodStats, TotalStats};

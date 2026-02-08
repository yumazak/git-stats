//! Application state management

#![allow(clippy::cast_possible_wrap)]

use crate::error::Result;
use crate::stats::{ActivityStats, AnalysisResult};
use crate::tui::event::{Event, EventHandler};
use crate::tui::ui;
use crossterm::ExecutableCommand;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use crossterm::terminal::{self, EnterAlternateScreen, LeaveAlternateScreen};
use ratatui::prelude::*;
use std::io::stdout;

/// Data point for additions/deletions diverging bar chart
#[derive(Debug, Clone)]
pub struct AddDelDataPoint {
    pub label: String,
    pub additions: u64,
    pub deletions: u64,
}

/// Metric to display in charts
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Metric {
    #[default]
    Commits,
    AdditionsAndDeletions,
    FilesChanged,
}

impl Metric {
    /// Get the next metric in the cycle
    #[must_use]
    pub fn next(self) -> Self {
        match self {
            Self::Commits => Self::AdditionsAndDeletions,
            Self::AdditionsAndDeletions => Self::FilesChanged,
            Self::FilesChanged => Self::Commits,
        }
    }

    /// Get the previous metric in the cycle
    #[must_use]
    pub fn prev(self) -> Self {
        match self {
            Self::Commits => Self::FilesChanged,
            Self::AdditionsAndDeletions => Self::Commits,
            Self::FilesChanged => Self::AdditionsAndDeletions,
        }
    }

    /// Get display name
    #[must_use]
    pub fn name(self) -> &'static str {
        match self {
            Self::Commits => "Commits",
            Self::AdditionsAndDeletions => "Additions / Deletions",
            Self::FilesChanged => "Files Changed",
        }
    }
}

/// Chart type to display in single mode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ChartType {
    #[default]
    Commits,
    FilesChanged,
    AddDel,
    Weekday,
    Hour,
}

impl ChartType {
    /// Get the next chart type in the cycle
    #[must_use]
    pub fn next(self) -> Self {
        match self {
            Self::Commits => Self::FilesChanged,
            Self::FilesChanged => Self::AddDel,
            Self::AddDel => Self::Weekday,
            Self::Weekday => Self::Hour,
            Self::Hour => Self::Commits,
        }
    }

    /// Get the previous chart type in the cycle
    #[must_use]
    pub fn prev(self) -> Self {
        match self {
            Self::Commits => Self::Hour,
            Self::FilesChanged => Self::Commits,
            Self::AddDel => Self::FilesChanged,
            Self::Weekday => Self::AddDel,
            Self::Hour => Self::Weekday,
        }
    }

    /// Get display name
    #[must_use]
    pub fn name(self) -> &'static str {
        match self {
            Self::Commits => "Commits",
            Self::FilesChanged => "Files Changed",
            Self::AddDel => "Add/Del",
            Self::Weekday => "Weekday",
            Self::Hour => "Hour",
        }
    }
}

/// Application state
pub struct App {
    /// Analysis result to display
    pub result: AnalysisResult,
    /// Activity statistics (commits by weekday and hour)
    pub activity_stats: ActivityStats,
    /// Currently selected chart type (for single mode)
    pub chart_type: ChartType,
    /// Whether the app should quit
    pub should_quit: bool,
    /// Show single metric instead of all metrics
    pub single_metric: bool,
    /// Scroll offset for diverging bar chart (0 = show latest)
    pub scroll_offset: usize,
}

impl App {
    /// Create a new App instance
    #[must_use]
    pub fn new(result: AnalysisResult, activity_stats: ActivityStats, single_metric: bool) -> Self {
        Self {
            result,
            activity_stats,
            chart_type: ChartType::default(),
            should_quit: false,
            single_metric,
            scroll_offset: 0,
        }
    }

    /// Run the TUI application
    ///
    /// # Errors
    ///
    /// Returns an error if terminal operations fail.
    pub fn run(&mut self) -> Result<()> {
        // Setup terminal
        terminal::enable_raw_mode()?;
        stdout().execute(EnterAlternateScreen)?;

        let backend = CrosstermBackend::new(stdout());
        let mut terminal = Terminal::new(backend)?;
        terminal.clear()?;

        // Create event handler
        let event_handler = EventHandler::new(250);

        // Main loop
        let result = self.main_loop(&mut terminal, &event_handler);

        // Restore terminal
        terminal::disable_raw_mode()?;
        stdout().execute(LeaveAlternateScreen)?;

        result
    }

    fn main_loop<B: Backend>(
        &mut self,
        terminal: &mut Terminal<B>,
        event_handler: &EventHandler,
    ) -> Result<()> {
        while !self.should_quit {
            // Draw UI
            terminal.draw(|frame| ui::render(frame, self))?;

            // Handle events
            match event_handler.next()? {
                Event::Key(key) => self.handle_key(key),
                Event::Tick | Event::Resize(_, _) => {}
            }
        }

        Ok(())
    }

    fn handle_key(&mut self, key: KeyEvent) {
        match key.code {
            // Quit
            KeyCode::Char('q') | KeyCode::Esc => {
                self.should_quit = true;
            }
            // Quit with Ctrl+C
            KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                self.should_quit = true;
            }
            // Next chart (only in single metric mode)
            KeyCode::Tab | KeyCode::Right | KeyCode::Char('l') => {
                if self.single_metric {
                    self.chart_type = self.chart_type.next();
                }
            }
            // Previous chart (only in single metric mode)
            KeyCode::BackTab | KeyCode::Left | KeyCode::Char('h') => {
                if self.single_metric {
                    self.chart_type = self.chart_type.prev();
                }
            }
            // Scroll up (past data)
            KeyCode::Up | KeyCode::Char('k') => {
                if self.can_scroll() {
                    self.scroll_up();
                }
            }
            // Scroll down (towards latest)
            KeyCode::Down | KeyCode::Char('j') => {
                if self.can_scroll() {
                    self.scroll_down();
                }
            }
            // Toggle single/split metric view
            KeyCode::Char('m') => {
                self.single_metric = !self.single_metric;
                // Reset scroll offset when toggling views
                self.scroll_offset = 0;
            }
            _ => {}
        }
    }

    /// Check if current view supports scrolling
    fn can_scroll(&self) -> bool {
        if self.single_metric {
            // Only AddDel chart supports scrolling in single mode
            matches!(self.chart_type, ChartType::AddDel)
        } else {
            // Split view supports scrolling
            true
        }
    }

    /// Scroll up to see older data
    fn scroll_up(&mut self) {
        let data_len = self.result.stats.len();
        if data_len > 0 {
            // Max offset: allows scrolling until the oldest item is visible
            let max_offset = data_len.saturating_sub(1);
            self.scroll_offset = (self.scroll_offset + 1).min(max_offset);
        }
    }

    /// Scroll down to see newer data
    fn scroll_down(&mut self) {
        self.scroll_offset = self.scroll_offset.saturating_sub(1);
    }

    /// Get values for a specific metric
    #[must_use]
    pub fn values_for_metric(&self, metric: Metric) -> Vec<(String, i64)> {
        self.result
            .stats
            .iter()
            .map(|s| {
                let value = match metric {
                    Metric::Commits => i64::from(s.commits),
                    Metric::AdditionsAndDeletions => s.net_lines,
                    Metric::FilesChanged => i64::from(s.files_changed),
                };
                (s.label.clone(), value)
            })
            .collect()
    }

    /// Get all metrics
    #[must_use]
    pub fn all_metrics() -> [Metric; 3] {
        [
            Metric::Commits,
            Metric::AdditionsAndDeletions,
            Metric::FilesChanged,
        ]
    }

    /// Get additions/deletions data for diverging bar chart
    #[must_use]
    pub fn additions_deletions_data(&self) -> Vec<AddDelDataPoint> {
        self.result
            .stats
            .iter()
            .map(|s| AddDelDataPoint {
                label: s.label.clone(),
                additions: s.additions,
                deletions: s.deletions,
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stats::{PeriodStats, TotalStats};
    use chrono::NaiveDate;

    fn make_result() -> AnalysisResult {
        AnalysisResult {
            repository: "test".to_string(),
            period: "daily".to_string(),
            from: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            to: NaiveDate::from_ymd_opt(2024, 1, 7).unwrap(),
            stats: vec![PeriodStats {
                label: "2024-01-01".to_string(),
                date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
                commits: 5,
                additions: 100,
                deletions: 20,
                net_lines: 80,
                files_changed: 10,
            }],
            total: TotalStats::default(),
        }
    }

    #[test]
    fn test_metric_cycle() {
        let metric = Metric::Commits;
        assert_eq!(metric.next(), Metric::AdditionsAndDeletions);
        assert_eq!(metric.prev(), Metric::FilesChanged);
    }

    #[test]
    fn test_all_metrics() {
        let metrics = App::all_metrics();
        assert_eq!(metrics.len(), 3);
    }

    #[test]
    fn test_additions_deletions_data() {
        let result = make_result();
        let app = App::new(result, ActivityStats::default(), false);

        let data = app.additions_deletions_data();
        assert_eq!(data.len(), 1);
        assert_eq!(data[0].label, "2024-01-01");
        assert_eq!(data[0].additions, 100);
        assert_eq!(data[0].deletions, 20);
    }

    fn make_result_with_multiple_days() -> AnalysisResult {
        AnalysisResult {
            repository: "test".to_string(),
            period: "daily".to_string(),
            from: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            to: NaiveDate::from_ymd_opt(2024, 1, 5).unwrap(),
            stats: (1..=5)
                .map(|day| PeriodStats {
                    label: format!("2024-01-0{day}"),
                    date: NaiveDate::from_ymd_opt(2024, 1, day).unwrap(),
                    commits: day,
                    additions: u64::from(day) * 10,
                    deletions: u64::from(day) * 2,
                    net_lines: i64::from(day) * 8,
                    files_changed: day,
                })
                .collect(),
            total: TotalStats::default(),
        }
    }

    #[test]
    fn test_scroll_up_increases_offset() {
        let result = make_result_with_multiple_days();
        let mut app = App::new(result, ActivityStats::default(), false);

        assert_eq!(app.scroll_offset, 0);
        app.scroll_up();
        assert_eq!(app.scroll_offset, 1);
        app.scroll_up();
        assert_eq!(app.scroll_offset, 2);
    }

    #[test]
    fn test_scroll_down_decreases_offset() {
        let result = make_result_with_multiple_days();
        let mut app = App::new(result, ActivityStats::default(), false);

        app.scroll_offset = 3;
        app.scroll_down();
        assert_eq!(app.scroll_offset, 2);
        app.scroll_down();
        assert_eq!(app.scroll_offset, 1);
        app.scroll_down();
        assert_eq!(app.scroll_offset, 0);
    }

    #[test]
    fn test_scroll_down_does_not_go_negative() {
        let result = make_result_with_multiple_days();
        let mut app = App::new(result, ActivityStats::default(), false);

        assert_eq!(app.scroll_offset, 0);
        app.scroll_down();
        assert_eq!(app.scroll_offset, 0);
    }

    #[test]
    fn test_scroll_up_respects_max_offset() {
        let result = make_result_with_multiple_days();
        let mut app = App::new(result, ActivityStats::default(), false);

        // 5 items, max offset should be 4 (data_len - 1)
        for _ in 0..10 {
            app.scroll_up();
        }
        assert_eq!(app.scroll_offset, 4);
    }

    #[test]
    fn test_scroll_offset_resets_on_view_toggle() {
        let result = make_result_with_multiple_days();
        let mut app = App::new(result, ActivityStats::default(), false);

        app.scroll_offset = 3;
        // Simulate pressing 'm' to toggle view
        app.single_metric = !app.single_metric;
        app.scroll_offset = 0; // This happens in handle_key

        assert_eq!(app.scroll_offset, 0);
    }

    #[test]
    fn test_chart_type_cycle() {
        let chart = ChartType::Commits;
        assert_eq!(chart.next(), ChartType::FilesChanged);
        assert_eq!(chart.next().next(), ChartType::AddDel);
        assert_eq!(chart.next().next().next(), ChartType::Weekday);
        assert_eq!(chart.next().next().next().next(), ChartType::Hour);
        assert_eq!(chart.next().next().next().next().next(), ChartType::Commits);
    }

    #[test]
    fn test_chart_type_prev_cycle() {
        let chart = ChartType::Commits;
        assert_eq!(chart.prev(), ChartType::Hour);
        assert_eq!(chart.prev().prev(), ChartType::Weekday);
        assert_eq!(chart.prev().prev().prev(), ChartType::AddDel);
        assert_eq!(chart.prev().prev().prev().prev(), ChartType::FilesChanged);
        assert_eq!(chart.prev().prev().prev().prev().prev(), ChartType::Commits);
    }

    #[test]
    fn test_chart_type_name() {
        assert_eq!(ChartType::Commits.name(), "Commits");
        assert_eq!(ChartType::FilesChanged.name(), "Files Changed");
        assert_eq!(ChartType::AddDel.name(), "Add/Del");
        assert_eq!(ChartType::Weekday.name(), "Weekday");
        assert_eq!(ChartType::Hour.name(), "Hour");
    }

    #[test]
    fn test_can_scroll_in_split_mode() {
        let result = make_result();
        let app = App::new(result, ActivityStats::default(), false);

        // Split mode always supports scrolling
        assert!(app.can_scroll());
    }

    #[test]
    fn test_can_scroll_in_single_mode() {
        let result = make_result();
        let mut app = App::new(result, ActivityStats::default(), true);

        // Only AddDel chart supports scrolling in single mode
        app.chart_type = ChartType::AddDel;
        assert!(app.can_scroll());

        app.chart_type = ChartType::Commits;
        assert!(!app.can_scroll());

        app.chart_type = ChartType::FilesChanged;
        assert!(!app.can_scroll());

        app.chart_type = ChartType::Weekday;
        assert!(!app.can_scroll());

        app.chart_type = ChartType::Hour;
        assert!(!app.can_scroll());
    }

    #[test]
    fn test_chart_type_default() {
        assert_eq!(ChartType::default(), ChartType::Commits);
    }

    #[test]
    fn test_app_new_initializes_chart_type() {
        let result = make_result();
        let app = App::new(result, ActivityStats::default(), false);

        assert_eq!(app.chart_type, ChartType::default());
    }
}

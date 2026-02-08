//! Application state management

#![allow(clippy::cast_possible_wrap)]

use crate::error::Result;
use crate::stats::AnalysisResult;
use crate::tui::event::{Event, EventHandler};
use crate::tui::ui;
use crossterm::ExecutableCommand;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use crossterm::terminal::{self, EnterAlternateScreen, LeaveAlternateScreen};
use ratatui::prelude::*;
use std::io::stdout;

/// Metric to display in charts
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Metric {
    #[default]
    Commits,
    Additions,
    Deletions,
    NetLines,
    FilesChanged,
}

impl Metric {
    /// Get the next metric in the cycle
    #[must_use]
    pub fn next(self) -> Self {
        match self {
            Self::Commits => Self::Additions,
            Self::Additions => Self::Deletions,
            Self::Deletions => Self::NetLines,
            Self::NetLines => Self::FilesChanged,
            Self::FilesChanged => Self::Commits,
        }
    }

    /// Get the previous metric in the cycle
    #[must_use]
    pub fn prev(self) -> Self {
        match self {
            Self::Commits => Self::FilesChanged,
            Self::Additions => Self::Commits,
            Self::Deletions => Self::Additions,
            Self::NetLines => Self::Deletions,
            Self::FilesChanged => Self::NetLines,
        }
    }

    /// Get display name
    #[must_use]
    pub fn name(self) -> &'static str {
        match self {
            Self::Commits => "Commits",
            Self::Additions => "Additions",
            Self::Deletions => "Deletions",
            Self::NetLines => "Net Lines",
            Self::FilesChanged => "Files Changed",
        }
    }
}

/// Application state
pub struct App {
    /// Analysis result to display
    pub result: AnalysisResult,
    /// Currently selected metric
    pub metric: Metric,
    /// Whether the app should quit
    pub should_quit: bool,
    /// Scroll offset for data
    pub scroll_offset: usize,
    /// Show single metric instead of all metrics
    pub single_metric: bool,
}

impl App {
    /// Create a new App instance
    #[must_use]
    pub fn new(result: AnalysisResult, single_metric: bool) -> Self {
        Self {
            result,
            metric: Metric::default(),
            should_quit: false,
            scroll_offset: 0,
            single_metric,
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
            // Next metric
            KeyCode::Tab | KeyCode::Right | KeyCode::Char('l') => {
                self.metric = self.metric.next();
            }
            // Previous metric
            KeyCode::BackTab | KeyCode::Left | KeyCode::Char('h') => {
                self.metric = self.metric.prev();
            }
            // Toggle single/split metric view
            KeyCode::Char('m') => {
                self.single_metric = !self.single_metric;
            }
            // Scroll
            KeyCode::Up | KeyCode::Char('k') => {
                self.scroll_offset = self.scroll_offset.saturating_sub(1);
            }
            KeyCode::Down | KeyCode::Char('j') => {
                let max_offset = self.result.stats.len().saturating_sub(1);
                self.scroll_offset = (self.scroll_offset + 1).min(max_offset);
            }
            KeyCode::Home => {
                self.scroll_offset = 0;
            }
            KeyCode::End => {
                self.scroll_offset = self.result.stats.len().saturating_sub(1);
            }
            _ => {}
        }
    }

    /// Get values for the current metric
    #[must_use]
    pub fn metric_values(&self) -> Vec<(String, i64)> {
        self.values_for_metric(self.metric)
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
                    Metric::Additions => s.additions as i64,
                    Metric::Deletions => s.deletions as i64,
                    Metric::NetLines => s.net_lines,
                    Metric::FilesChanged => i64::from(s.files_changed),
                };
                (s.label.clone(), value)
            })
            .collect()
    }

    /// Get all metrics
    #[must_use]
    pub fn all_metrics() -> [Metric; 5] {
        [
            Metric::Commits,
            Metric::Additions,
            Metric::Deletions,
            Metric::NetLines,
            Metric::FilesChanged,
        ]
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
        assert_eq!(metric.next(), Metric::Additions);
        assert_eq!(metric.prev(), Metric::FilesChanged);
    }

    #[test]
    fn test_metric_values() {
        let result = make_result();
        let app = App::new(result, false);

        let values = app.metric_values();
        assert_eq!(values.len(), 1);
        assert_eq!(values[0], ("2024-01-01".to_string(), 5));
    }

    #[test]
    fn test_all_metrics() {
        let metrics = App::all_metrics();
        assert_eq!(metrics.len(), 5);
    }
}

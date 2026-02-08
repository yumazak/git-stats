//! UI rendering

use crate::stats::ActivityStats;
use crate::tui::app::{App, Metric};
use crate::tui::widgets::{
    BarDataPoint, render_diverging_bar_chart, render_horizontal_bar_chart, render_line_chart,
    render_line_chart_for_metric,
};
use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Paragraph};

/// Render the entire UI
pub fn render(frame: &mut Frame, app: &App) {
    let area = frame.area();

    // Create layout: header, main content, footer
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Header
            Constraint::Min(10),   // Main content
            Constraint::Length(3), // Footer
        ])
        .split(area);

    render_header(frame, chunks[0], app);

    if app.single_metric {
        render_single_chart(frame, chunks[1], app);
    } else {
        render_split_charts(frame, chunks[1], app);
    }

    render_footer(frame, chunks[2], app);
}

fn render_header(frame: &mut Frame, area: Rect, app: &App) {
    let title = format!(
        " {} | {} | {} ",
        app.result.repository,
        app.result.period,
        format_date_range(&app.result.from.to_string(), &app.result.to.to_string())
    );

    let header = Paragraph::new(title)
        .style(Style::default().fg(Color::Cyan).bold())
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Cyan)),
        );

    frame.render_widget(header, area);
}

fn render_single_chart(frame: &mut Frame, area: Rect, app: &App) {
    render_line_chart(frame, area, app);
}

fn render_split_charts(frame: &mut Frame, area: Rect, app: &App) {
    // Split into left (2/3) and right (1/3) columns
    let cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Ratio(2, 3), Constraint::Ratio(1, 3)])
        .split(area);

    // Left column: Commits (top) + Files Changed (bottom)
    let left_rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(cols[0]);

    render_line_chart_for_metric(frame, left_rows[0], app, Metric::Commits);
    render_line_chart_for_metric(frame, left_rows[1], app, Metric::FilesChanged);

    // Right column: 3 sections (Additions/Deletions, Weekday, Hourly)
    // Ratio 1:1:3 - Hourly needs more space for 24 rows
    let right_rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Ratio(1, 5),
            Constraint::Ratio(1, 5),
            Constraint::Ratio(3, 5),
        ])
        .split(cols[1]);

    // Top: Diverging bar chart for Additions/Deletions
    render_diverging_bar_chart(frame, right_rows[0], app);

    // Middle: Weekday activity chart
    render_weekday_chart(frame, right_rows[1], &app.activity_stats);

    // Bottom: Hourly activity chart
    render_hourly_chart(frame, right_rows[2], &app.activity_stats);
}

fn render_weekday_chart(frame: &mut Frame, area: Rect, stats: &ActivityStats) {
    let labels = ActivityStats::weekday_labels();
    let data: Vec<BarDataPoint> = labels
        .iter()
        .zip(stats.weekday.iter())
        .map(|(label, &value)| BarDataPoint::new(*label, value))
        .collect();

    render_horizontal_bar_chart(frame, area, "Weekday", &data, Color::Cyan);
}

fn render_hourly_chart(frame: &mut Frame, area: Rect, stats: &ActivityStats) {
    let data: Vec<BarDataPoint> = stats
        .hourly
        .iter()
        .enumerate()
        .map(|(hour, &value)| BarDataPoint::new(hour.to_string(), value))
        .collect();

    render_horizontal_bar_chart(frame, area, "Hour", &data, Color::Magenta);
}

fn render_footer(frame: &mut Frame, area: Rect, app: &App) {
    let mode_indicator = if app.single_metric { "Single" } else { "Split" };

    let help_text = format!(" [m] Mode: {mode_indicator} | [q] Quit ");

    // Summary stats
    let total = &app.result.total;
    let summary = format!(
        "Total: {} commits | +{} -{} | {} files",
        total.commits, total.additions, total.deletions, total.files_changed
    );

    let footer_text = format!("{help_text}\n{summary}");

    let footer = Paragraph::new(footer_text)
        .style(Style::default().fg(Color::DarkGray))
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::DarkGray)),
        );

    frame.render_widget(footer, area);
}

fn format_date_range(from: &str, to: &str) -> String {
    format!("{from} → {to}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_date_range() {
        assert_eq!(
            format_date_range("2024-01-01", "2024-01-07"),
            "2024-01-01 → 2024-01-07"
        );
    }
}

//! UI rendering

use crate::tui::app::App;
use crate::tui::widgets::{render_line_chart, render_line_chart_for_metric};
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
    // Split into 2 rows: top row with 3 charts, bottom row with 2 charts
    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area);

    // Top row: Commits, Additions, Deletions
    let top_cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(33),
            Constraint::Percentage(34),
            Constraint::Percentage(33),
        ])
        .split(rows[0]);

    // Bottom row: Net Lines, Files Changed
    let bottom_cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(rows[1]);

    let metrics = App::all_metrics();

    render_line_chart_for_metric(frame, top_cols[0], app, metrics[0]);
    render_line_chart_for_metric(frame, top_cols[1], app, metrics[1]);
    render_line_chart_for_metric(frame, top_cols[2], app, metrics[2]);
    render_line_chart_for_metric(frame, bottom_cols[0], app, metrics[3]);
    render_line_chart_for_metric(frame, bottom_cols[1], app, metrics[4]);
}

fn render_footer(frame: &mut Frame, area: Rect, app: &App) {
    let mode_indicator = if app.single_metric { "Single" } else { "Split" };

    let help_text = format!(" [m] Mode: {mode_indicator} | [q] Quit ");

    // Summary stats
    let total = &app.result.total;
    let summary = format!(
        "Total: {} commits | +{} -{} (net: {}) | {} files",
        total.commits, total.additions, total.deletions, total.net_lines, total.files_changed
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

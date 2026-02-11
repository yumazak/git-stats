//! UI rendering

use crate::stats::ActivityStats;
use crate::tui::app::{App, Metric};
use crate::tui::chart_type::ChartType;
use crate::tui::widgets::{
    chart_width, render_diverging_bar_chart, render_line_chart_for_metric,
    render_vertical_bar_chart,
};
use ratatui::layout::Flex;
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

    if app.single_metric() {
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
    match app.chart_type() {
        ChartType::Commits => render_line_chart_for_metric(frame, area, app, Metric::Commits),
        ChartType::FilesChanged => {
            render_line_chart_for_metric(frame, area, app, Metric::FilesChanged);
        }
        ChartType::AddDel => render_diverging_bar_chart(frame, area, app),
        ChartType::Weekday => {
            let centered = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Max(chart_width(7))])
                .flex(Flex::Center)
                .split(area)[0];
            render_weekday_chart(frame, centered, &app.activity_stats);
        }
        ChartType::Hour => {
            let centered = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Max(chart_width(24))])
                .flex(Flex::Center)
                .split(area)[0];
            render_hourly_chart(frame, centered, &app.activity_stats);
        }
    }
}

fn render_split_charts(frame: &mut Frame, area: Rect, app: &App) {
    // Split into top and bottom rows (3:1)
    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Ratio(3, 4), Constraint::Ratio(1, 4)])
        .split(area);

    // Top row: Commits + Files Changed (left) | Addition/Deletions (right) - 3:1
    let top_cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Ratio(3, 4), Constraint::Ratio(1, 4)])
        .split(rows[0]);

    // Left side of top row: Commits + Files Changed stacked
    let top_left_rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(top_cols[0]);

    render_line_chart_for_metric(frame, top_left_rows[0], app, Metric::Commits);
    render_line_chart_for_metric(frame, top_left_rows[1], app, Metric::FilesChanged);

    // Right side of top row: Addition/Deletions
    render_diverging_bar_chart(frame, top_cols[1], app);

    // Bottom row: Weekdays (1/5) | Hour (4/5)
    let bottom_cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Ratio(1, 5), Constraint::Ratio(4, 5)])
        .split(rows[1]);

    render_weekday_chart(frame, bottom_cols[0], &app.activity_stats);
    render_hourly_chart(frame, bottom_cols[1], &app.activity_stats);
}

fn render_weekday_chart(frame: &mut Frame, area: Rect, stats: &ActivityStats) {
    let labels = ActivityStats::weekday_labels();
    render_vertical_bar_chart(frame, area, "Weekday", &labels, &stats.weekday, Color::Cyan);
}

fn render_hourly_chart(frame: &mut Frame, area: Rect, stats: &ActivityStats) {
    // Use shorter labels for hours to fit
    let labels: Vec<&str> = (0..24).map(hour_label).collect();
    render_vertical_bar_chart(frame, area, "Hour", &labels, &stats.hourly, Color::Magenta);
}

fn hour_label(hour: usize) -> &'static str {
    match hour {
        0 => "0",
        1 => "1",
        2 => "2",
        3 => "3",
        4 => "4",
        5 => "5",
        6 => "6",
        7 => "7",
        8 => "8",
        9 => "9",
        10 => "10",
        11 => "11",
        12 => "12",
        13 => "13",
        14 => "14",
        15 => "15",
        16 => "16",
        17 => "17",
        18 => "18",
        19 => "19",
        20 => "20",
        21 => "21",
        22 => "22",
        23 => "23",
        _ => "",
    }
}

fn render_footer(frame: &mut Frame, area: Rect, app: &App) {
    let mode_indicator = if app.single_metric() {
        format!("Single: {}", app.chart_type().name())
    } else {
        "Split".to_string()
    };

    let nav_hint = if app.single_metric() {
        "[Tab] Switch | "
    } else {
        ""
    };
    let help_text = format!(" {nav_hint}[m] Mode: {mode_indicator} | [q] Quit ");

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

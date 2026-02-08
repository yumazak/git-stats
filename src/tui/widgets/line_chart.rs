//! Line chart widget

#![allow(clippy::cast_precision_loss, clippy::cast_possible_truncation)]

use crate::tui::app::{App, Metric};
use ratatui::prelude::*;
use ratatui::widgets::{Axis, Block, Borders, Chart, Dataset, GraphType, Paragraph};

/// Render a line chart for a specific metric
pub fn render_line_chart_for_metric(frame: &mut Frame, area: Rect, app: &App, metric: Metric) {
    let values = app.values_for_metric(metric);

    if values.is_empty() {
        let empty = Paragraph::new("No data to display")
            .alignment(Alignment::Center)
            .block(
                Block::default()
                    .title(format!(" {} ", metric.name()))
                    .borders(Borders::ALL),
            );
        frame.render_widget(empty, area);
        return;
    }

    // Convert to chart data points (use absolute values for consistency)
    let data_points: Vec<(f64, f64)> = values
        .iter()
        .enumerate()
        .map(|(i, (_, v))| (i as f64, v.abs() as f64))
        .collect();

    // Calculate bounds
    let max_y = values.iter().map(|(_, v)| v.abs()).max().unwrap_or(1) as f64;
    let y_max = max_y * 1.1;

    // Calculate total for title
    let total: i64 = values.iter().map(|(_, v)| *v).sum();
    let title = format!(" {} (Total: {}) ", metric.name(), format_number(total));

    // Create dataset (no name to avoid legend display)
    let dataset = Dataset::default()
        .marker(symbols::Marker::Braille)
        .graph_type(GraphType::Line)
        .style(Style::default().fg(Color::Cyan))
        .data(&data_points);

    // Simple Y-axis labels
    let y_labels = vec![
        Span::raw("0"),
        Span::raw(format_number((y_max / 2.0) as i64)),
        Span::raw(format_number(y_max as i64)),
    ];

    let chart = Chart::new(vec![dataset])
        .block(
            Block::default()
                .title(title)
                .title_style(Style::default().fg(Color::Yellow).bold())
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::White)),
        )
        .x_axis(
            Axis::default()
                .style(Style::default().fg(Color::DarkGray))
                .bounds([0.0, (values.len() - 1).max(1) as f64]),
        )
        .y_axis(
            Axis::default()
                .style(Style::default().fg(Color::Gray))
                .bounds([0.0, y_max])
                .labels(y_labels),
        );

    frame.render_widget(chart, area);
}

fn format_number(value: i64) -> String {
    if value.abs() >= 1_000_000 {
        format!("{:.1}M", value as f64 / 1_000_000.0)
    } else if value.abs() >= 1_000 {
        format!("{:.1}K", value as f64 / 1_000.0)
    } else {
        value.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_number() {
        assert_eq!(format_number(100), "100");
        assert_eq!(format_number(2500), "2.5K");
        assert_eq!(format_number(2_500_000), "2.5M");
        assert_eq!(format_number(-2500), "-2.5K");
    }
}

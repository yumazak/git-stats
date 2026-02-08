//! Horizontal bar chart widget for activity statistics

#![allow(
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::cast_lossless
)]

use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Paragraph};

/// Minimum width required to render the chart
const MIN_WIDTH: u16 = 15;

/// Data point for horizontal bar chart
#[derive(Debug, Clone)]
pub struct BarDataPoint {
    pub label: String,
    pub value: u32,
}

impl BarDataPoint {
    /// Create a new bar data point
    #[must_use]
    pub fn new(label: impl Into<String>, value: u32) -> Self {
        Self {
            label: label.into(),
            value,
        }
    }
}

/// Render a horizontal bar chart
pub fn render_horizontal_bar_chart(
    frame: &mut Frame,
    area: Rect,
    title: &str,
    data: &[BarDataPoint],
    color: Color,
) {
    // Check minimum width
    if area.width < MIN_WIDTH {
        let msg = Paragraph::new("Too narrow")
            .alignment(Alignment::Center)
            .block(Block::default().title(title).borders(Borders::ALL));
        frame.render_widget(msg, area);
        return;
    }

    if data.is_empty() {
        let empty = Paragraph::new("No data")
            .alignment(Alignment::Center)
            .block(Block::default().title(title).borders(Borders::ALL));
        frame.render_widget(empty, area);
        return;
    }

    // Calculate total for title
    let total: u32 = data.iter().map(|d| d.value).sum();
    let title_with_total = format!(" {title} ({total}) ");

    // Create block
    let block = Block::default()
        .title(title_with_total)
        .title_style(Style::default().fg(color).bold())
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::White));

    let inner = block.inner(area);
    frame.render_widget(block, area);

    if inner.height < 1 || inner.width < 5 {
        return;
    }

    // Determine how many rows we can display
    let available_rows = inner.height as usize;
    let display_data: Vec<_> = data.iter().take(available_rows).collect();

    // Find max value for scaling
    let max_value = display_data
        .iter()
        .map(|d| d.value)
        .max()
        .unwrap_or(1)
        .max(1);

    // Calculate label width
    let label_width = display_data
        .iter()
        .map(|d| d.label.chars().count())
        .max()
        .unwrap_or(3)
        .min(5) as u16;

    // Calculate bar area width (excluding label and space)
    let bar_area_width = inner.width.saturating_sub(label_width + 2); // +2 for " " separator

    // Render each row
    for (i, point) in display_data.iter().enumerate() {
        let y = inner.y + i as u16;
        if y >= inner.y + inner.height {
            break;
        }

        // Render label (right-aligned)
        let label_span = Span::styled(
            format!("{:>width$}", point.label, width = label_width as usize),
            Style::default().fg(Color::DarkGray),
        );
        frame.render_widget(
            Paragraph::new(label_span),
            Rect::new(inner.x, y, label_width, 1),
        );

        // Calculate bar length
        let bar_len = if max_value > 0 {
            ((point.value as f64 / max_value as f64) * bar_area_width as f64) as u16
        } else {
            0
        };

        // Render bar
        if bar_len > 0 {
            let bar_x = inner.x + label_width + 1;
            let bar = Span::styled(
                "\u{2588}".repeat(bar_len as usize),
                Style::default().fg(color),
            );
            frame.render_widget(Paragraph::new(bar), Rect::new(bar_x, y, bar_len, 1));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bar_data_point_new() {
        let point = BarDataPoint::new("Mon", 10);
        assert_eq!(point.label, "Mon");
        assert_eq!(point.value, 10);
    }

    #[test]
    fn test_bar_data_point_with_string() {
        let point = BarDataPoint::new(String::from("Tue"), 5);
        assert_eq!(point.label, "Tue");
        assert_eq!(point.value, 5);
    }
}

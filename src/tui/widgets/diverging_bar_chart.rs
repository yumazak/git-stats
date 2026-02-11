//! Diverging bar chart widget for additions/deletions

#![allow(
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::cast_lossless,
    clippy::too_many_lines
)]

use crate::tui::app::App;
use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Paragraph};

/// Minimum width required to render the chart
const MIN_WIDTH: u16 = 20;

/// Render a diverging bar chart for additions/deletions
pub fn render_diverging_bar_chart(frame: &mut Frame, area: Rect, app: &App) {
    let data = app.additions_deletions_data();

    // Check minimum width
    if area.width < MIN_WIDTH {
        let msg = Paragraph::new("Too narrow")
            .alignment(Alignment::Center)
            .block(
                Block::default()
                    .title(" Additions / Deletions ")
                    .borders(Borders::ALL),
            );
        frame.render_widget(msg, area);
        return;
    }

    if data.is_empty() {
        let empty = Paragraph::new("No data to display")
            .alignment(Alignment::Center)
            .block(
                Block::default()
                    .title(" Additions / Deletions ")
                    .borders(Borders::ALL),
            );
        frame.render_widget(empty, area);
        return;
    }

    // Calculate totals for title
    let total_additions: u64 = data.iter().map(|d| d.additions).sum();
    let total_deletions: u64 = data.iter().map(|d| d.deletions).sum();
    let title = format!(
        " Additions / Deletions (+{} / -{}) ",
        format_number(total_additions),
        format_number(total_deletions)
    );

    // Create block
    let block = Block::default()
        .title(title)
        .title_style(Style::default().fg(Color::Yellow).bold())
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::White));

    let inner = block.inner(area);
    frame.render_widget(block, area);

    if inner.height < 1 || inner.width < 10 {
        return;
    }

    // Determine how many rows we can display
    let available_rows = inner.height as usize;

    // Calculate display range with scroll offset
    // offset=0 means show latest data (end of array)
    // offset>0 means scroll up to see older data
    let total = data.len();
    let scroll_offset = app.scroll_offset().min(total.saturating_sub(1));
    let end = total.saturating_sub(scroll_offset);
    let start = end.saturating_sub(available_rows);
    let display_data: Vec<_> = data[start..end].iter().collect();

    // Find max value for unified scale
    let max_value = display_data
        .iter()
        .map(|d| d.additions.max(d.deletions))
        .max()
        .unwrap_or(1)
        .max(1);

    // Calculate label width (date labels)
    let label_width = display_data
        .iter()
        .map(|d| d.label.chars().count())
        .max()
        .unwrap_or(10)
        .min(12) as u16;

    // Calculate bar area width (excluding labels and center line marker)
    let bar_area_width = inner.width.saturating_sub(label_width + 3); // +3 for " | "
    let half_bar_width = bar_area_width / 2;

    // Render each row
    for (i, point) in display_data.iter().enumerate() {
        let y = inner.y + i as u16;
        if y >= inner.y + inner.height {
            break;
        }

        // Render label (right-aligned, truncated if needed)
        let label = truncate_tail(&point.label, label_width as usize);
        let label_span = Span::styled(
            format!("{:>width$}", label, width = label_width as usize),
            Style::default().fg(Color::DarkGray),
        );
        frame.render_widget(
            Paragraph::new(label_span),
            Rect::new(inner.x, y, label_width, 1),
        );

        // Calculate bar lengths
        let del_bar_len = if max_value > 0 {
            ((point.deletions as f64 / max_value as f64) * half_bar_width as f64) as u16
        } else {
            0
        };
        let add_bar_len = if max_value > 0 {
            ((point.additions as f64 / max_value as f64) * half_bar_width as f64) as u16
        } else {
            0
        };

        // Center position (after label and space)
        let bar_start_x = inner.x + label_width + 1;
        let center_x = bar_start_x + half_bar_width;

        // Render deletion bar (red, going left from center)
        if del_bar_len > 0 {
            let del_start = center_x.saturating_sub(del_bar_len);
            let del_bar = Span::styled(
                "\u{2588}".repeat(del_bar_len as usize),
                Style::default().fg(Color::Red),
            );
            frame.render_widget(
                Paragraph::new(del_bar),
                Rect::new(del_start, y, del_bar_len, 1),
            );
        }

        // Render center line
        let center_span = Span::styled("|", Style::default().fg(Color::DarkGray));
        frame.render_widget(Paragraph::new(center_span), Rect::new(center_x, y, 1, 1));

        // Render addition bar (green, going right from center)
        if add_bar_len > 0 {
            let add_bar = Span::styled(
                "\u{2588}".repeat(add_bar_len as usize),
                Style::default().fg(Color::Green),
            );
            frame.render_widget(
                Paragraph::new(add_bar),
                Rect::new(center_x + 1, y, add_bar_len, 1),
            );
        }
    }
}

/// Truncate a string to the last `max_chars` characters (safe for multi-byte UTF-8)
fn truncate_tail(label: &str, max_chars: usize) -> String {
    let count = label.chars().count();
    if count <= max_chars {
        return label.to_string();
    }
    label
        .chars()
        .rev()
        .take(max_chars)
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .collect()
}

fn format_number(value: u64) -> String {
    if value >= 1_000_000 {
        format!("{:.1}M", value as f64 / 1_000_000.0)
    } else if value >= 1_000 {
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
    }

    #[test]
    fn test_truncate_tail_ascii() {
        // ASCII string that needs truncation
        assert_eq!(truncate_tail("2024-01-15", 8), "24-01-15");
        assert_eq!(truncate_tail("abcdefghij", 5), "fghij");
    }

    #[test]
    fn test_truncate_tail_no_truncation_needed() {
        // String shorter than or equal to max_chars
        assert_eq!(truncate_tail("hello", 10), "hello");
        assert_eq!(truncate_tail("hello", 5), "hello");
        assert_eq!(truncate_tail("", 5), "");
    }

    #[test]
    fn test_truncate_tail_non_ascii() {
        // Japanese characters (multi-byte UTF-8)
        assert_eq!(truncate_tail("こんにちは", 3), "にちは");
        assert_eq!(truncate_tail("日本語テスト", 4), "語テスト");
        // Mixed ASCII and non-ASCII
        assert_eq!(truncate_tail("Hello世界", 4), "lo世界");
    }
}

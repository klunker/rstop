// src/ui/memory.rs
// Memory and Swap overview panel: textual metrics and graphical gauges.
// Adapts responsively to narrow column widths (e.g. 50% split) and full widths.

use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Gauge, Paragraph},
    Frame,
};
use crate::system::SystemMetrics;
use crate::util::format_bytes;
use super::Panel;

pub struct MemoryPanel;

impl Panel for MemoryPanel {
    fn draw(&self, frame: &mut Frame, area: Rect, m: &SystemMetrics) {
        // Divide panel into 3 vertical slots:
        //   [0] — Text summary: Used | Avail | Free | Total
        //   [1] — RAM gauge bar
        //   [2] — Swap gauge bar
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(1), // Metrics text row
                Constraint::Length(1), // RAM gauge
                Constraint::Length(1), // Swap gauge
            ])
            .split(area);

        // --- Textual metrics row ---
        // Responsive formatting: adapts if rendered in a narrow column (e.g. 50% parallel)
        let info_line = if area.width >= 55 {
            Line::from(vec![
                Span::styled("Memory: ", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
                Span::raw("Used: "),
                Span::styled(format_bytes(m.mem_used), Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
                Span::raw("  Avail: "),
                Span::styled(format_bytes(m.mem_available), Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
                Span::raw("  Free: "),
                Span::styled(format_bytes(m.mem_free), Style::default().fg(Color::Gray)),
                Span::raw("  Total: "),
                Span::styled(format_bytes(m.mem_total), Style::default().add_modifier(Modifier::BOLD)),
            ])
        } else {
            Line::from(vec![
                Span::styled("Mem: ", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
                Span::styled(format_bytes(m.mem_used), Style::default().fg(Color::Yellow)),
                Span::raw("/"),
                Span::styled(format_bytes(m.mem_total), Style::default()),
                Span::raw(" (Free: "),
                Span::styled(format_bytes(m.mem_free), Style::default().fg(Color::Gray)),
                Span::raw(")"),
            ])
        };
        frame.render_widget(Paragraph::new(info_line), chunks[0]);

        // --- RAM gauge ---
        let mem_ratio = (m.mem_percent / 100.0).clamp(0.0, 1.0);
        let mem_label = if area.width >= 45 {
            format!(
                "  RAM  {:.1}%  ({}/{})",
                m.mem_percent,
                format_bytes(m.mem_used),
                format_bytes(m.mem_total)
            )
        } else {
            format!("  RAM  {:.1}%", m.mem_percent)
        };
        let mem_gauge = Gauge::default()
            .gauge_style(Style::default().fg(Color::Green).bg(Color::Black))
            .ratio(mem_ratio)
            .label(mem_label);
        frame.render_widget(mem_gauge, chunks[1]);

        // --- Swap gauge ---
        let swap_ratio = (m.swap_percent / 100.0).clamp(0.0, 1.0);
        let swap_label = if area.width >= 45 {
            format!(
                "  Swap {:.1}%  ({}/{})",
                m.swap_percent,
                format_bytes(m.swap_used),
                format_bytes(m.swap_total),
            )
        } else {
            format!("  Swap {:.1}%", m.swap_percent)
        };
        let swap_gauge = Gauge::default()
            .gauge_style(Style::default().fg(Color::Yellow).bg(Color::Black))
            .ratio(swap_ratio)
            .label(swap_label);
        frame.render_widget(swap_gauge, chunks[2]);
    }
}

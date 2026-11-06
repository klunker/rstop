// src/ui/memory.rs
// Memory and Swap overview panel: textual metrics and graphical gauges.
// Additional metrics can be added here without affecting other components.

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
        let info_line = Line::from(vec![
            Span::styled("Memory: ", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            Span::raw("Used: "),
            Span::styled(format_bytes(m.mem_used), Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::raw("  Avail: "),
            Span::styled(format_bytes(m.mem_available), Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
            Span::raw("  Free: "),
            Span::styled(format_bytes(m.mem_free), Style::default().fg(Color::Gray)),
            Span::raw("  Total: "),
            Span::styled(format_bytes(m.mem_total), Style::default().add_modifier(Modifier::BOLD)),
        ]);
        frame.render_widget(Paragraph::new(info_line), chunks[0]);

        // --- RAM gauge ---
        // Clamp to [0.0, 1.0] prevents invalid ratio bounds.
        let mem_ratio = (m.mem_percent / 100.0).clamp(0.0, 1.0);
        let mem_gauge = Gauge::default()
            .gauge_style(Style::default().fg(Color::Green).bg(Color::DarkGray))
            .ratio(mem_ratio)
            .label(format!("  RAM  {:.1}%", m.mem_percent));
        frame.render_widget(mem_gauge, chunks[1]);

        // --- Swap gauge ---
        let swap_ratio = (m.swap_percent / 100.0).clamp(0.0, 1.0);
        let swap_label = format!(
            "  Swap {:.1}%  ({}/{})",
            m.swap_percent,
            format_bytes(m.swap_used),
            format_bytes(m.swap_total),
        );
        let swap_gauge = Gauge::default()
            .gauge_style(Style::default().fg(Color::Red).bg(Color::DarkGray))
            .ratio(swap_ratio)
            .label(swap_label);
        frame.render_widget(swap_gauge, chunks[2]);
    }
}

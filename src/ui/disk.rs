// src/ui/disk.rs
// Storage (Disk / "Винт") and Processor overview panel.
// Designed to pair side-by-side with MemoryPanel in dual-column mode,
// or render as an independent modular panel.

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

#[allow(dead_code)]
pub struct DiskPanel;

impl Panel for DiskPanel {
    fn draw(&self, frame: &mut Frame, area: Rect, m: &SystemMetrics) {
        // Divide panel into 3 vertical slots matching the height of MemoryPanel:
        //   [0] — Disk text summary
        //   [1] — Disk gauge bar
        //   [2] — CPU gauge bar
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(1), // Metrics text row
                Constraint::Length(1), // Disk gauge
                Constraint::Length(1), // CPU gauge
            ])
            .split(area);

        // --- Textual metrics row ---
        // Responsive formatting: adapts when allocated narrow width (e.g. in 50% split)
        let info_line = if area.width >= 50 {
            Line::from(vec![
                Span::styled("Disk: ", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
                Span::raw("Used: "),
                Span::styled(format_bytes(m.disk_used), Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
                Span::raw("  Free: "),
                Span::styled(format_bytes(m.disk_free), Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
                Span::raw("  Total: "),
                Span::styled(format_bytes(m.disk_total), Style::default().add_modifier(Modifier::BOLD)),
            ])
        } else {
            Line::from(vec![
                Span::styled("Disk: ", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
                Span::styled(format_bytes(m.disk_used), Style::default().fg(Color::Yellow)),
                Span::raw("/"),
                Span::styled(format_bytes(m.disk_total), Style::default()),
                Span::raw(" (Free: "),
                Span::styled(format_bytes(m.disk_free), Style::default().fg(Color::Green)),
                Span::raw(")"),
            ])
        };
        frame.render_widget(Paragraph::new(info_line), chunks[0]);

        // --- Disk gauge bar ---
        let disk_ratio = (m.disk_percent / 100.0).clamp(0.0, 1.0);
        let disk_label = if area.width >= 45 {
            format!(
                "  Disk {:.1}%  ({}/{})",
                m.disk_percent,
                format_bytes(m.disk_used),
                format_bytes(m.disk_total)
            )
        } else {
            format!("  Disk {:.1}%", m.disk_percent)
        };
        let disk_gauge = Gauge::default()
            .gauge_style(Style::default().fg(Color::Blue).bg(Color::DarkGray))
            .ratio(disk_ratio)
            .label(disk_label);
        frame.render_widget(disk_gauge, chunks[1]);

        // --- CPU gauge bar ---
        let cpu_ratio = (m.cpu_avg_percent / 100.0).clamp(0.0, 1.0);
        let cpu_label = if area.width >= 45 {
            format!("  CPU  {:.1}%  ({} cores)", m.cpu_avg_percent, m.cpu_count)
        } else {
            format!("  CPU  {:.1}%", m.cpu_avg_percent)
        };
        let cpu_gauge = Gauge::default()
            .gauge_style(Style::default().fg(Color::Magenta).bg(Color::DarkGray))
            .ratio(cpu_ratio)
            .label(cpu_label);
        frame.render_widget(cpu_gauge, chunks[2]);
    }
}


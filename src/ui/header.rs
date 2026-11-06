// src/ui/header.rs
// Top header panel: shows application title and live system metrics on a green background.
// Implements the Panel trait so it can be composed in ui/mod.rs without touching other files.

use ratatui::{
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};
use crate::system::SystemMetrics;
use super::Panel;

pub struct HeaderPanel;

impl Panel for HeaderPanel {
    fn draw(&self, frame: &mut Frame, area: Rect, m: &SystemMetrics) {
        // Construct header string: title and system stats separated by pipe delimiters
        let content = format!(
            " rstop | Host: {} | OS: {} {} | CPUs: {} | Up: {}",
            m.host_name, m.os_name, m.kernel_version, m.cpu_count, m.uptime_str,
        );

        let line = Line::from(vec![
            Span::styled(content, Style::default()
                .fg(Color::Black)
                .bg(Color::Green)),
        ]);

        // Paragraph widget filled with green background across the given area
        let widget = Paragraph::new(line)
            .style(Style::default().bg(Color::Green));

        frame.render_widget(widget, area);
    }
}

// src/ui/footer.rs
// Bottom status bar displaying available keybindings, active layout mode, and app state.
// Uses inverted colors (Style::reversed) in line with standard status bar styling.

use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};
use crate::system::SystemMetrics;
use super::layout::PanelLayoutMode;
use super::Panel;

pub struct FooterPanel {
    pub layout_mode: PanelLayoutMode,
}

impl Panel for FooterPanel {
    fn draw(&self, frame: &mut Frame, area: Rect, _m: &SystemMetrics) {
        let line = Line::from(vec![
            Span::styled(" q", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::raw(": Quit  |  "),
            Span::styled("Tab", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            Span::raw(": Layout ["),
            Span::styled(self.layout_mode.label(), Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
            Span::raw("]"),
        ]);
        let widget = Paragraph::new(line)
            .style(Style::default().add_modifier(Modifier::REVERSED));
        frame.render_widget(widget, area);
    }
}

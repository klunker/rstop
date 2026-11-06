// src/ui/footer.rs
// Bottom status bar displaying available keybindings and application state.
// Uses inverted colors (Style::reversed) in line with standard status bar styling.

use ratatui::{
    layout::Rect,
    style::{Modifier, Style},
    text::Line,
    widgets::Paragraph,
    Frame,
};
use crate::system::SystemMetrics;
use super::Panel;

pub struct FooterPanel;

impl Panel for FooterPanel {
    fn draw(&self, frame: &mut Frame, area: Rect, _m: &SystemMetrics) {
        // _m is currently unused here, but kept in the trait signature for future extensions
        // (such as displaying selected process PID, active filter, or sorting mode).
        let line = Line::from(
            " q/Ctrl+C: Quit",
        );
        let widget = Paragraph::new(line)
            .style(Style::default().add_modifier(Modifier::REVERSED));
        frame.render_widget(widget, area);
    }
}

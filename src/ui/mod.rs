// src/ui/mod.rs
// Top-level UI rendering coordinator: composes all panels into a single frame.
//
// Adding a new panel:
//   1. Create `src/ui/my_panel.rs` implementing `Panel for MyPanel`.
//   2. Declare `pub mod my_panel;` and add it to the Layout constraints + draw() call below.
//   No other files need to be modified.

pub mod footer;
pub mod header;
pub mod memory;

use ratatui::{
    layout::{Constraint, Direction, Layout},
    widgets::{Block, BorderType, Borders},
    Frame,
};
use crate::system::SystemMetrics;

/// Trait required for all modular UI panels.
/// Receives the render frame, its allocated area, and current system metrics.
pub trait Panel {
    fn draw(&self, frame: &mut Frame, area: ratatui::layout::Rect, metrics: &SystemMetrics);
}

/// Main drawing function invoked once per tick from App::run().
/// Calculates layout constraints and delegates rendering to each panel.
pub fn draw(frame: &mut Frame, metrics: &SystemMetrics) {
    let total = frame.area();

    // Top-level vertical screen layout:
    //   [0] Header  — 1 row
    //   [1] Content — remaining expandable area
    //   [2] Footer  — 1 row
    let root = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),   // Header
            Constraint::Min(0),      // Content area
            Constraint::Length(1),   // Footer
        ])
        .split(total);

    // --- Header ---
    header::HeaderPanel.draw(frame, root[0], metrics);

    // --- Content area: memory + future panels ---
    // Outer border acts as top and bottom separators.
    let content_block = Block::default()
        .borders(Borders::TOP | Borders::BOTTOM)
        .border_type(BorderType::Plain);
    let inner = content_block.inner(root[1]);
    frame.render_widget(content_block, root[1]);

    // Inner vertical stack within content area:
    //   [0] Memory panel — 3 rows (text summary + 2 gauge bars)
    //   [1] Remaining expandable area (reserved for process table)
    let content_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // MemoryPanel (1 info line + 2 gauge bars)
            Constraint::Min(0),     // Available space for future panels
        ])
        .split(inner);

    memory::MemoryPanel.draw(frame, content_chunks[0], metrics);

    // --- Footer ---
    footer::FooterPanel.draw(frame, root[2], metrics);
}

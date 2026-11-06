// src/ui/mod.rs
// Top-level UI rendering coordinator: composes all panels into a single frame.
// Supports dynamic width regulation (100%, 70%, 50%) and parallel dual-panel placement.

pub mod cpu;
pub mod disk;
pub mod footer;
pub mod header;
pub mod layout;
pub mod memory;

pub use layout::PanelLayoutMode;

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
/// Calculates layout constraints and delegates rendering to panels according to layout_mode.
pub fn draw(frame: &mut Frame, metrics: &SystemMetrics, layout_mode: PanelLayoutMode) {
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

    // --- Content area: panels + future process table ---
    let content_block = Block::default()
        .borders(Borders::TOP | Borders::BOTTOM)
        .border_type(BorderType::Plain);
    let inner = content_block.inner(root[1]);
    frame.render_widget(content_block, root[1]);

    // Vertical split inside content:
    //   [0] Meter panels row (height 3)
    //   [1] Remaining expandable area (reserved for process table)
    let content_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // Meter panels row (3 rows for text + 2 gauges)
            Constraint::Min(0),     // Available space for future process table
        ])
        .split(inner);

    // Apply horizontal column splitting according to layout_mode (Full, 70%, 50%, DualEqual)
    let columns = layout_mode.split(content_chunks[0]);
    if columns.len() == 2 {
        // Parallel mode: 50% Memory on left, 50% CPU per-core widget on right
        memory::MemoryPanel.draw(frame, columns[0], metrics);
        cpu::CpuPanel.draw(frame, columns[1], metrics);
    } else if columns.len() == 1 {
        // Single panel mode: Memory panel rendered with the regulated width (100%, 70%, or 50%)
        memory::MemoryPanel.draw(frame, columns[0], metrics);
    }

    // --- Footer ---
    footer::FooterPanel { layout_mode }.draw(frame, root[2], metrics);
}

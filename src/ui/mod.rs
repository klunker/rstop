// src/ui/mod.rs
// Top-level UI rendering coordinator: composes all panels into a single frame.
// Layout is explicitly defined in code: Memory and CPU panels are placed side-by-side (50% / 50%)
// with an adaptive vertical fallback for narrow terminals.

pub mod cpu;
pub mod disk;
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
/// Layout structure:
///   [Header]
///   [Content Area: Memory (Left 50%) and CPU (Right 50%)]
///   [Footer]
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

    // --- Content area: panels + future process table ---
    let content_block = Block::default()
        .borders(Borders::TOP | Borders::BOTTOM)
        .border_type(BorderType::Plain);
    let inner = content_block.inner(root[1]);
    frame.render_widget(content_block, root[1]);

    // Check terminal width to determine horizontal (parallel) or vertical (stacked) layout:
    // If width >= 70 cols, render Memory and CPU side-by-side with a 2-char gap.
    // If width < 70 cols, stack Memory above CPU so neither widget is hidden.
    if inner.width >= 70 {
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

        // Horizontal split: Memory (50%) + Gap (2 chars) + CPU (50%)
        let gap = 2u16;
        let half_width = (content_chunks[0].width.saturating_sub(gap)) / 2;
        let left_area = ratatui::layout::Rect {
            x: content_chunks[0].x,
            y: content_chunks[0].y,
            width: half_width,
            height: content_chunks[0].height,
        };
        let right_area = ratatui::layout::Rect {
            x: content_chunks[0].x + half_width + gap,
            y: content_chunks[0].y,
            width: content_chunks[0].width.saturating_sub(half_width + gap),
            height: content_chunks[0].height,
        };

        memory::MemoryPanel.draw(frame, left_area, metrics);
        cpu::CpuPanel.draw(frame, right_area, metrics);
    } else {
        // Narrow terminal: stack vertically so BOTH Memory and CPU remain fully visible
        let content_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),  // Memory panel
                Constraint::Length(1),  // Small separator gap
                Constraint::Length(3),  // CPU panel
                Constraint::Min(0),     // Remaining area for process table
            ])
            .split(inner);

        memory::MemoryPanel.draw(frame, content_chunks[0], metrics);
        cpu::CpuPanel.draw(frame, content_chunks[2], metrics);
    }

    // --- Footer ---
    footer::FooterPanel.draw(frame, root[2], metrics);
}

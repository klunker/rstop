// src/ui/cpu.rs
// Multi-core CPU panel.
// Displays core count, overall CPU load, and splits cores evenly into 2 rows (e.g. 4 cores per row for 8 cores).

use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Gauge, Paragraph},
    Frame,
};
use crate::system::SystemMetrics;
use super::Panel;

pub struct CpuPanel;

impl Panel for CpuPanel {
    fn draw(&self, frame: &mut Frame, area: Rect, m: &SystemMetrics) {
        // Divide panel into 3 vertical slots matching MemoryPanel height:
        //   [0] — Summary: Cores count + average load
        //   [1] — First row of cores (e.g. Core 1..4)
        //   [2] — Second row of cores (e.g. Core 5..8)
        let row_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(1), // Summary text
                Constraint::Length(1), // Cores row 1
                Constraint::Length(1), // Cores row 2
            ])
            .split(area);

        // --- Textual summary row ---
        let summary_line = Line::from(vec![
            Span::styled("CPU: ", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            Span::raw("Cores: "),
            Span::styled(format!("{}", m.cpu_count), Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            Span::raw("  Avg Load: "),
            Span::styled(format!("{:.1}%", m.cpu_avg_percent), Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
        ]);
        frame.render_widget(Paragraph::new(summary_line), row_chunks[0]);

        // If no cores detected, exit early
        let total_cores = m.cpu_cores.len();
        if total_cores == 0 {
            return;
        }

        // Divide cores into 2 rows evenly (e.g. 8 -> 4 and 4; 6 -> 3 and 3; 4 -> 2 and 2)
        let row1_count = total_cores.div_ceil(2);

        // --- Row 1 of cores ---
        let row1_cores = &m.cpu_cores[0..row1_count];
        render_cores_row(frame, row_chunks[1], row1_cores, 1);

        // --- Row 2 of cores ---
        if total_cores > row1_count {
            let row2_cores = &m.cpu_cores[row1_count..total_cores];
            render_cores_row(frame, row_chunks[2], row2_cores, row1_count + 1);
        }
    }
}

/// Renders a horizontal row of core gauges with spacing between them.
fn render_cores_row(frame: &mut Frame, area: Rect, cores: &[f64], start_index: usize) {
    if cores.is_empty() {
        return;
    }

    // Interleave gauges with 1-character gaps (spacers) so indicators don't merge into each other:
    // [Gauge, Spacer, Gauge, Spacer, Gauge...]
    let mut constraints = Vec::with_capacity(cores.len() * 2);
    for i in 0..cores.len() {
        constraints.push(Constraint::Ratio(1, cores.len() as u32));
        if i + 1 < cores.len() {
            constraints.push(Constraint::Length(1));
        }
    }

    let cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(constraints)
        .split(area);

    for (i, &usage) in cores.iter().enumerate() {
        let col_idx = i * 2; // skip spacer slots
        let core_num = start_index + i;
        let ratio = (usage / 100.0).clamp(0.0, 1.0);

        // Choose gauge color based on load severity: Green < 60%, Yellow < 85%, Red >= 85%
        let bar_color = if usage >= 85.0 {
            Color::Red
        } else if usage >= 60.0 {
            Color::Yellow
        } else {
            Color::Green
        };

        // Compact label for narrower columns, e.g. "C1 12%" vs "Core1 12%"
        let label = if cols[col_idx].width >= 12 {
            format!("{}-Core {:.0}%", core_num, usage)
        } else if cols[col_idx].width >= 8 {
            format!("C{} {:.0}%", core_num, usage)
        } else {
            format!("{:.0}%", usage)
        };

        let gauge = Gauge::default()
            .gauge_style(Style::default().fg(bar_color).bg(Color::Black))
            .ratio(ratio)
            .label(label);

        frame.render_widget(gauge, cols[col_idx]);
    }
}

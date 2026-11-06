// src/ui/layout.rs
// Configurable panel layout engine supporting width regulation (100%, 70%, 50%)
// as well as parallel dual-panel placement (50% / 50%) with a separating margin.

use ratatui::layout::{Constraint, Direction, Layout, Rect};

/// Defines horizontal width regulation and parallel column arrangements for panels.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PanelLayoutMode {
    /// Two parallel panels displayed side by side with a gap between them.
    #[default]
    DualEqual,
    /// Single panel spanning the full available width (100%).
    Full,
    /// Single panel spanning 70% of the available width.
    Width70,
    /// Single panel spanning 50% of the available width.
    Half,
}

impl PanelLayoutMode {
    /// Cycles to the next layout mode (useful for Tab key toggle).
    pub fn next(&self) -> Self {
        match self {
            Self::DualEqual => Self::Full,
            Self::Full => Self::Width70,
            Self::Width70 => Self::Half,
            Self::Half => Self::DualEqual,
        }
    }

    /// User-friendly label for status bars and indicators.
    pub fn label(&self) -> &'static str {
        match self {
            Self::DualEqual => "50/50 Dual",
            Self::Full => "100%",
            Self::Width70 => "70%",
            Self::Half => "50%",
        }
    }

    /// Splits a rectangular area into one or two panel viewports.
    /// In dual mode, returns two adjacent columns separated by a gap so panels don't merge.
    /// In single panel mode, returns a single viewport with the specified width percentage.
    pub fn split(&self, area: Rect) -> Vec<Rect> {
        match self {
            Self::Full => vec![area],
            Self::Width70 => {
                let chunks = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints([
                        Constraint::Percentage(70),
                        Constraint::Percentage(30),
                    ])
                    .split(area);
                vec![chunks[0]]
            }
            Self::Half => {
                let chunks = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints([
                        Constraint::Percentage(50),
                        Constraint::Percentage(50),
                    ])
                    .split(area);
                vec![chunks[0]]
            }
            Self::DualEqual => {
                // If terminal is too narrow (< 60 cols), fallback to single column for readability
                if area.width < 60 {
                    vec![area]
                } else {
                    let gap = 2u16.min(area.width.saturating_sub(40));
                    let half_width = (area.width.saturating_sub(gap)) / 2;
                    let left = Rect {
                        x: area.x,
                        y: area.y,
                        width: half_width,
                        height: area.height,
                    };
                    let right = Rect {
                        x: area.x + half_width + gap,
                        y: area.y,
                        width: area.width.saturating_sub(half_width + gap),
                        height: area.height,
                    };
                    vec![left, right]
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_layout_cycle() {
        let mode = PanelLayoutMode::DualEqual;
        assert_eq!(mode.next(), PanelLayoutMode::Full);
        assert_eq!(mode.next().next(), PanelLayoutMode::Width70);
        assert_eq!(mode.next().next().next(), PanelLayoutMode::Half);
        assert_eq!(mode.next().next().next().next(), PanelLayoutMode::DualEqual);
    }

    #[test]
    fn test_split_dual_equal() {
        let area = Rect::new(0, 0, 102, 10);
        let chunks = PanelLayoutMode::DualEqual.split(area);
        assert_eq!(chunks.len(), 2);
        // Both panels get equal space (50 each with a 2-char gap)
        assert_eq!(chunks[0].width, 50);
        assert_eq!(chunks[1].width, 50);
        assert_eq!(chunks[1].x, chunks[0].x + chunks[0].width + 2);
    }

    #[test]
    fn test_split_width70() {
        let area = Rect::new(0, 0, 100, 10);
        let chunks = PanelLayoutMode::Width70.split(area);
        assert_eq!(chunks.len(), 1);
        assert_eq!(chunks[0].width, 70);
    }
}

// src/app.rs
// Central application coordinator. Owns state and drives the main event loop.
//
// Why a dedicated App struct instead of everything inside main()?
//   Keeps main() focused strictly on startup and process exit codes.
//   App encapsulates state and lifecycle logic, making it straightforward to add features
//   such as display modes, interactive panels, or configuration settings.

use std::io;
use std::time::Duration;
use sysinfo::System;
use crate::event::{poll_action, AppAction};
use crate::system::collect_metrics;
use crate::terminal::TerminalGuard;
use crate::ui::{self, PanelLayoutMode};

/// Interval between metric collection ticks and UI redraws.
const TICK_RATE: Duration = Duration::from_millis(1000);

pub struct App {
    /// RAII terminal manager: automatically restores the terminal upon Drop.
    guard: TerminalGuard,
    /// System inspection instance held across the application lifecycle to avoid costly re-initialization.
    sys: System,
    /// Active panel layout mode (50/50 dual parallel, 100%, 70%, 50%).
    layout_mode: PanelLayoutMode,
}

impl App {
    /// Initializes terminal raw mode and system profiler.
    /// Returns an error Result if initialization fails so the caller can report it gracefully.
    pub fn new() -> io::Result<Self> {
        let guard = TerminalGuard::new()?;
        let sys = System::new_all().map_err(|e| {
            io::Error::new(io::ErrorKind::Other, format!("sysinfo init: {}", e))
        })?;
        let layout_mode = PanelLayoutMode::default();
        Ok(Self { guard, sys, layout_mode })
    }

    /// Primary event loop: collect metrics → draw UI → poll user events.
    /// Exits on AppAction::Quit; terminal cleanup is handled automatically by TerminalGuard's Drop.
    pub fn run(&mut self) -> io::Result<()> {
        loop {
            // 1. Collect updated metrics
            let metrics = collect_metrics(&mut self.sys);

            // 2. Render UI in a single atomic pass using ratatui's double-buffering
            self.guard.terminal.draw(|frame| {
                ui::draw(frame, &metrics, self.layout_mode);
            })?;

            // 3. Wait for keyboard event or timeout tick
            match poll_action(TICK_RATE)? {
                AppAction::Quit => break,
                AppAction::CycleLayout => {
                    self.layout_mode = self.layout_mode.next();
                }
                AppAction::None => {}
            }
        }
        Ok(())
    }
}

// src/terminal.rs
// RAII wrapper for the terminal session. Guarantees restoration of terminal state
// on any exit path: normal termination, `?` operator error propagation, or panics.
//
// Why RAII instead of panic hook?
//   A panic hook only triggers on panics. If a function exits early via `return` or
//   the `?` operator, the hook does not run, leaving the terminal in raw mode.
//   Rust's Drop semantics guarantee execution whenever the variable goes out of scope.

use std::io::{self, stdout};
use crossterm::{
    cursor::{Hide, Show},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};

/// Owns the ratatui Terminal handle and restores the terminal state upon Drop.
pub struct TerminalGuard {
    pub terminal: Terminal<CrosstermBackend<std::io::Stdout>>,
}

impl TerminalGuard {
    /// Enables raw mode, enters the alternate screen, hides the cursor,
    /// and instantiates the ratatui terminal backend.
    /// Returns Result rather than panicking to allow the caller to handle failures gracefully.
    pub fn new() -> io::Result<Self> {
        enable_raw_mode()?;
        let mut out = stdout();
        execute!(out, EnterAlternateScreen, Hide)?;
        let backend = CrosstermBackend::new(out);
        let terminal = Terminal::new(backend)?;
        Ok(Self { terminal })
    }
}

/// Drop implementation guarantees that the terminal is restored to normal state.
/// Errors here are silently ignored since the terminal might already be closing,
/// but both cleanup operations are attempted.
impl Drop for TerminalGuard {
    fn drop(&mut self) {
        let _ = disable_raw_mode();
        let _ = execute!(self.terminal.backend_mut(), LeaveAlternateScreen, Show);
    }
}

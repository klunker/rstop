// src/event.rs
// Mapping of crossterm input events to typed application actions.
//
// Why use an AppAction enum instead of checking keycodes directly in app.rs?
//   It centralizes user intent mapping in one place.
//   Adding a keybinding is as simple as adding one line in poll_action() and
//   handling the corresponding match arm in app.rs.

use std::io;
use std::time::Duration;
use crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers};

/// Strongly typed user action.
/// To extend: add a new variant here and handle it in App::run().
#[derive(Debug, PartialEq)]
pub enum AppAction {
    /// Exit the application.
    Quit,
    /// No relevant action in this tick (e.g. timeout or unmapped key).
    None,
}

/// Polls for crossterm events within the given timeout and returns an AppAction.
/// If no events occur within `timeout`, returns `Ok(AppAction::None)`.
pub fn poll_action(timeout: Duration) -> io::Result<AppAction> {
    // poll() unblocks before the timeout if an event arrives, serving as the UI tick timer.
    if !event::poll(timeout)? {
        return Ok(AppAction::None);
    }

    match event::read()? {
        Event::Key(key) if key.kind == KeyEventKind::Press => {
            Ok(match key.code {
                // Ctrl+C: standard unix interrupt keybinding
                KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => AppAction::Quit,
                // 'q': standard single-key quit shortcut
                KeyCode::Char('q') => AppAction::Quit,
                _ => AppAction::None,
            })
        }
        // Resize, mouse, paste events: ignored for now without causing panics
        _ => Ok(AppAction::None),
    }
}

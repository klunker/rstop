// src/main.rs
// Thin entry point: handles application initialization and top-level errors.
// Core logic is organized in app.rs and its submodules.

mod app;
mod event;
mod system;
mod terminal;
mod ui;
mod util;

use std::io;

fn main() -> io::Result<()> {
    // App::new() initializes the terminal (RAII guard) and system data provider.
    // If initialization fails, an error message is printed to stderr before entering alternate screen.
    let mut application = app::App::new().map_err(|e| {
        eprintln!("rstop: failed to initialize: {}", e);
        e
    })?;

    application.run()
}

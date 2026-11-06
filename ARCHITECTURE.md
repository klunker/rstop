# rstop — Architecture

**rstop** is a terminal process viewer written in Rust, inspired by [htop](https://github.com/htop-dev/htop).  
The original htop C sources are available in `./orig/` for reference.

---

## Project Layout

```
rstop/
├── Cargo.toml
├── Cargo.lock
├── ARCHITECTURE.md   ← this file
└── src/
    ├── main.rs       ← entry point: parse args, construct app, call app.run()
    ├── app.rs        ← App struct: owns state, drives the main event loop
    ├── terminal.rs   ← TerminalGuard: RAII wrapper for raw-mode + alt-screen
    ├── event.rs      ← poll_action(): maps crossterm events → AppAction enum
    ├── system.rs     ← SystemMetrics struct + collect_metrics()
    ├── util.rs       ← pure helpers: format_bytes(), format_uptime(), render_bar()
    └── ui/
        ├── mod.rs    ← draw(): composes all panels into one ratatui frame
        ├── header.rs ← green header bar (title + live system info)
        ├── layout.rs ← PanelLayoutMode: 50/50 dual parallel, 100%, 70%, 50% width
        ├── memory.rs ← Memory & Swap panel with responsive gauge bars
        ├── disk.rs   ← Storage (HDD/SSD) & CPU panel paired in dual-column mode
        └── footer.rs ← bottom status / keybinding bar
```

---

## Key Design Decisions

### 1. RAII `TerminalGuard` (`src/terminal.rs`)

**Why:** A panic hook as a terminal-restore mechanism is fragile — it only catches panics,  
not early `return`s or `?`-propagated errors. A RAII guard whose `Drop` impl calls  
`disable_raw_mode` + `LeaveAlternateScreen` is guaranteed to run in all exit paths.

```rust
pub struct TerminalGuard { /* crossterm + ratatui handle */ }
impl Drop for TerminalGuard { fn drop(&mut self) { /* restore terminal */ } }
```

### 2. `AppAction` enum (`src/event.rs`)

**Why:** Scattering `if key == KeyCode::Char('q') { break }` across the loop makes it hard  
to add keybindings. An enum gives a single place to map keys → intent, and `app.rs` just  
pattern-matches on the action. Adding a new hotkey = one line in `poll_action()` + one arm.

```rust
pub enum AppAction { Quit, None }
pub fn poll_action(timeout: Duration) -> io::Result<AppAction>;
```

### 3. `SystemMetrics` struct (`src/system.rs`)

**Why:** UI code should not import `sysinfo`. Collecting all data into a plain struct  
separates concerns — the data layer can be swapped (e.g. read `/proc` directly) without  
touching any UI module.

```rust
pub struct SystemMetrics {
    pub host_name: String,
    pub os_name: String,
    // ...memory, swap, cpu fields...
}
pub fn collect_metrics(sys: &mut System) -> SystemMetrics;
```

### 4. ratatui for rendering (`src/ui/`)

**Why:** Manual ANSI escape codes (`\x1b[32m...`) are error-prone, hard to test, and  
break on resize. ratatui provides type-safe `Style`, `Layout`, and widgets (`Gauge`,  
`Block`, `Paragraph`) that handle clearing and redrawing automatically.

### 5. `Panel` trait for extensibility (`src/ui/mod.rs`)

**Why:** Each UI section implements `Panel`, which defines a `draw` method. Adding a  
new panel (e.g. CPU bars, process table) means creating one new file and registering it  
in `ui/mod.rs` — no changes to `app.rs` or any existing panel.

```rust
pub trait Panel {
    fn draw(&self, frame: &mut Frame, area: Rect, metrics: &SystemMetrics);
}
```

---

## Data Flow (per tick)

```
app.run() loop
  │
  ├─ system::collect_metrics(&mut sys)   → SystemMetrics
  │
  ├─ terminal.draw(|frame| ui::draw(frame, &metrics))
  │       ├─ header::HeaderPanel::draw()
  │       ├─ memory::MemoryPanel::draw()
  │       └─ footer::FooterPanel::draw()
  │
  └─ event::poll_action(1s timeout)      → AppAction
          └─ AppAction::Quit → break (TerminalGuard::drop restores terminal)
```

---

## Adding a New Panel

1. Create `src/ui/my_panel.rs`
2. Define `pub struct MyPanel;` and `impl Panel for MyPanel { fn draw(...) { ... } }`
3. Register it in `src/ui/mod.rs`:
   - Add to layout split
   - Call `MyPanel.draw(frame, area, metrics)`
4. If new data is needed, add fields to `SystemMetrics` and populate in `collect_metrics()`

---

## Dependencies

| Crate | Purpose |
|---|---|
| `ratatui` | TUI framework: layout, widgets, rendering |
| `crossterm` | Terminal raw mode, alternate screen, event polling |
| `sysinfo` | Cross-platform system metrics (CPU, memory, processes) |
| `libc` | Low-level POSIX calls (e.g. `getpriority` for nice/pri values) |

use std::io::{self, stdout, Write};
use std::time::Duration;
use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{self, Event, KeyCode, KeyEventKind, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use sysinfo::System;

fn format_bytes(bytes: u64) -> String {
    const GIB: u64 = 1024 * 1024 * 1024;
    const MIB: u64 = 1024 * 1024;
    if bytes >= GIB {
        format!("{:.2} GiB", bytes as f64 / GIB as f64)
    } else {
        format!("{:.0} MiB", bytes as f64 / MIB as f64)
    }
}

fn render_bar(percent: f64, bar_width: usize, fill_color: &str) -> String {
    let filled = ((percent / 100.0) * (bar_width as f64)).round() as usize;
    let filled = filled.min(bar_width);
    let empty = bar_width.saturating_sub(filled);
    format!("{fill_color}{}\x1b[90m{}\x1b[0m", "|".repeat(filled), "·".repeat(empty))
}

fn main() -> io::Result<()> {
    // Setup panic hook to restore terminal if panic occurs
    let default_panic = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        let _ = disable_raw_mode();
        let _ = execute!(stdout(), LeaveAlternateScreen, Show);
        default_panic(info);
    }));

    // Enter raw mode and alternate screen (fullscreen buffer)
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen, Hide)?;

    let mut sys = System::new_all().unwrap_or_else(|_| System::new().expect("Failed to initialize System"));

    loop {
        sys.refresh_all();
        let (term_width, term_height) = crossterm::terminal::size().unwrap_or((80, 24));
        let width = term_width as usize;
        let height = term_height as usize;

        // System metrics
        let host_name = System::host_name().unwrap_or_else(|_| "localhost".to_string());
        let os_name = System::name().unwrap_or_else(|_| "Linux".to_string());
        let os_version = System::kernel_version().unwrap_or_else(|_| "Unknown".to_string());
        let cpu_count = sys.cpus().len();
        let uptime_sec = System::uptime().unwrap_or(0);
        let hours = uptime_sec / 3600;
        let mins = (uptime_sec % 3600) / 60;

        let total_mem = sys.total_memory();
        let used_mem = sys.used_memory();
        let free_mem = sys.free_memory();
        let available_mem = sys.available_memory();
        let mem_percent = if total_mem > 0 {
            (used_mem as f64 / total_mem as f64) * 100.0
        } else {
            0.0
        };

        let total_swap = sys.total_swap();
        let used_swap = sys.used_swap();
        let swap_percent = if total_swap > 0 {
            (used_swap as f64 / total_swap as f64) * 100.0
        } else {
            0.0
        };

        // Draw header across full terminal width (title + system info)
        execute!(stdout, MoveTo(0, 0))?;
        let header_content = format!(
            " rstop | Host: {} | OS: {} {} | CPUs: {} | Up: {}h {}m ",
            host_name, os_name, os_version, cpu_count, hours, mins
        );
        let header_line = if width > header_content.len() {
            format!("{}{}", header_content, " ".repeat(width - header_content.len()))
        } else {
            header_content.chars().take(width).collect()
        };
        print!("\x1b[30;42m{}\x1b[0m\x1b[K\r\n", header_line);

        // Row 1: Separator line
        execute!(stdout, MoveTo(0, 1))?;
        let sep = "─".repeat(width);
        print!("\x1b[90m{}\x1b[0m\x1b[K\r\n", sep);

        // Row 2: Memory details panel (Used, Available, Free, Total)
        execute!(stdout, MoveTo(0, 2))?;
        print!(
            "\x1b[1;36mMemory:\x1b[0m Used: \x1b[1;33m{}\x1b[0m | Avail: \x1b[1;32m{}\x1b[0m | Free: \x1b[37m{}\x1b[0m | Total: \x1b[1;37m{}\x1b[0m\x1b[K\r\n",
            format_bytes(used_mem),
            format_bytes(available_mem),
            format_bytes(free_mem),
            format_bytes(total_mem)
        );

        // Row 3: Memory progress bar
        execute!(stdout, MoveTo(0, 3))?;
        let bar_width = width.saturating_sub(18).clamp(10, 50);
        let mem_bar = render_bar(mem_percent, bar_width, "\x1b[32m");
        print!("  RAM  [{}] \x1b[1;37m{:>5.1}%\x1b[0m\x1b[K\r\n", mem_bar, mem_percent);

        // Row 4: Swap progress bar
        execute!(stdout, MoveTo(0, 4))?;
        let swap_bar = render_bar(swap_percent, bar_width, "\x1b[31m");
        print!(
            "  Swap [{}] \x1b[1;37m{:>5.1}%\x1b[0m ({}/{})\x1b[K\r\n",
            swap_bar,
            swap_percent,
            format_bytes(used_swap),
            format_bytes(total_swap)
        );

        // Row 5: Lower separator line
        execute!(stdout, MoveTo(0, 5))?;
        print!("\x1b[90m{}\x1b[0m\x1b[K\r\n", sep);

        // Clear remaining empty lines if any
        for y in 6..height.saturating_sub(1) {
            execute!(stdout, MoveTo(0, y as u16))?;
            print!("\x1b[K");
        }

        // Bottom status line / footer at height - 1
        if height > 1 {
            execute!(stdout, MoveTo(0, (height - 1) as u16))?;
            let footer = " Press Ctrl+C or 'q' to quit";
            let footer_line = if width > footer.len() {
                format!("{}{}", footer, " ".repeat(width - footer.len()))
            } else {
                footer.chars().take(width).collect()
            };
            print!("\x1b[7m{}\x1b[0m\x1b[K", footer_line);
        }

        stdout.flush()?;

        // Wait for user input or timeout (refresh every 1 second)
        if event::poll(Duration::from_millis(1000))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    if (key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('c'))
                        || key.code == KeyCode::Char('q')
                    {
                        break;
                    }
                }
            }
        }
    }

    // Restore terminal
    disable_raw_mode()?;
    execute!(stdout, LeaveAlternateScreen, Show)?;

    Ok(())
}

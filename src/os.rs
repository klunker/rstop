// src/os.rs
// Platform-specific OS identification and update tracking logic.
// This abstract interface isolates the rest of the app from platform details.

use chrono::{DateTime, Utc, Local};
use std::fs;

/// Represents OS information and update status
pub struct OsInfo {
    pub name: String,
    pub version: String,
    pub last_updated: Option<DateTime<Utc>>,
}

/// Detects OS and gathers last update information based on platform.
pub fn get_os_info() -> OsInfo {
    let name = sysinfo::System::name().unwrap_or_else(|_| "Unknown OS".to_string());
    let version = sysinfo::System::os_version().unwrap_or_else(|_| "Unknown Version".to_string());
    
    // Try to find a meaningful log file for "last update"
    let last_updated = find_last_update_date();

    OsInfo { name, version, last_updated }
}

fn find_last_update_date() -> Option<DateTime<Utc>> {
    // List of common update log files across Linux distros
    let candidates = [
        "/var/log/dpkg.log",        // Debian/Ubuntu
        "/var/log/apt/history.log", // Ubuntu apt
        "/var/log/pacman.log",      // Arch
        "/var/log/dnf.log",         // Fedora
        "/var/log/yum.log",         // RHEL/CentOS
        "/var/log/apk.log",         // Alpine
        "/var/log/zypp/history"     // OpenSUSE
    ];

    for path_str in candidates {
        if let Ok(metadata) = fs::metadata(path_str) {
            if let Ok(mtime) = metadata.modified() {
                return Some(mtime.into());
            }
        }
    }
    None
}

/// Formats the "last updated" date into human-readable string.
pub fn format_last_update(dt: Option<DateTime<Utc>>) -> String {
    match dt {
        Some(date) => {
            let now = Utc::now();
            let duration = now.signed_duration_since(date);
            let days = duration.num_days();
            
            let date_str = date.with_timezone(&Local).format("%d %b %Y").to_string();
            format!("last updated at {} ({} day ago)", date_str, days)
        },
        None => "never updated".to_string(),
    }
}

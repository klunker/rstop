// src/util.rs
// Pure helper functions without side effects.
// Separated to prevent logic duplication across UI modules and facilitate unit testing.

/// Formats byte counts into human-readable strings (MiB / GiB).
/// Uses u64 to avoid integer overflow on large RAM configurations.
pub fn format_bytes(bytes: u64) -> String {
    const GIB: u64 = 1024 * 1024 * 1024;
    const MIB: u64 = 1024 * 1024;
    if bytes >= GIB {
        format!("{:.2} GiB", bytes as f64 / GIB as f64)
    } else {
        format!("{:.0} MiB", bytes as f64 / MIB as f64)
    }
}

/// Formats uptime in seconds to "Xd Yh Zm" or "Yh Zm".
pub fn format_uptime(secs: u64) -> String {
    let days = secs / 86400;
    let hours = (secs % 86400) / 3600;
    let mins = (secs % 3600) / 60;
    if days > 0 {
        format!("{}d {}h {}m", days, hours, mins)
    } else {
        format!("{}h {}m", hours, mins)
    }
}

/// Computes the percentage of a value relative to total; safe against division by zero.
#[inline]
pub fn percent(value: u64, total: u64) -> f64 {
    if total == 0 {
        0.0
    } else {
        (value as f64 / total as f64) * 100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_bytes_gib() {
        assert_eq!(format_bytes(2 * 1024 * 1024 * 1024), "2.00 GiB");
    }

    #[test]
    fn test_format_bytes_mib() {
        assert_eq!(format_bytes(512 * 1024 * 1024), "512 MiB");
    }

    #[test]
    fn test_percent_zero_total() {
        assert_eq!(percent(100, 0), 0.0);
    }

    #[test]
    fn test_uptime_with_days() {
        assert_eq!(format_uptime(90061), "1d 1h 1m");
    }
}

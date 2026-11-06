// src/system.rs
// System metrics collection layer. UI modules do not import sysinfo directly;
// they only read this decoupled structure. This separation allows:
//  • Swapping data sources (e.g. sysinfo vs direct /proc access) without touching UI code.
//  • Easily providing mock metric structures for automated UI testing.

use sysinfo::{Disks, System};
use crate::util::{format_uptime, percent};

/// System status snapshot captured on a single tick.
/// Contains plain values (String, u64, f64) with no runtime dependency on sysinfo types.
#[derive(Debug, Clone)]
pub struct SystemMetrics {
    // --- Machine identification ---
    pub host_name: String,
    pub os_name: String,
    pub kernel_version: String,

    // --- Processor ---
    pub cpu_count: usize,
    pub cpu_avg_percent: f64,
    /// Per-core CPU load percentages (e.g. [12.5, 4.0, 99.0, ...])
    pub cpu_cores: Vec<f64>,

    // --- Random Access Memory ---
    pub mem_total: u64,
    pub mem_used: u64,
    pub mem_free: u64,
    pub mem_available: u64,
    pub mem_percent: f64,

    // --- Swap space ---
    pub swap_total: u64,
    pub swap_used: u64,
    pub swap_percent: f64,

    // --- Storage / Disk (HDD / SSD) ---
    #[allow(dead_code)]
    pub disk_total: u64,
    #[allow(dead_code)]
    pub disk_used: u64,
    #[allow(dead_code)]
    pub disk_free: u64,
    #[allow(dead_code)]
    pub disk_percent: f64,

    // --- System uptime ---
    pub uptime_str: String,

    // --- Process statistics ---
    #[allow(dead_code)]
    pub process_count: usize,
}

/// Gathers system metrics for a single update tick.
/// Takes `&mut System` because sysinfo requires mutable access to refresh internal buffers.
/// All fallback values are handled here, providing clean data upstream.
pub fn collect_metrics(sys: &mut System) -> SystemMetrics {
    // Refresh only the required subsystems; refresh_all() is unnecessarily expensive per tick.
    sys.refresh_cpu_all();
    sys.refresh_memory();
    sys.refresh_processes(sysinfo::ProcessesToUpdate::All, false);

    let host_name = System::host_name()
        .unwrap_or_else(|_| "localhost".to_string());
    let os_name = System::name()
        .unwrap_or_else(|_| "Linux".to_string());
    let kernel_version = System::kernel_version()
        .unwrap_or_else(|_| "unknown".to_string());

    let cpu_cores: Vec<f64> = sys.cpus().iter().map(|c| c.usage() as f64).collect();
    let cpu_count = cpu_cores.len();
    // Average CPU usage across all active cores
    let cpu_avg_percent = if cpu_count > 0 {
        cpu_cores.iter().sum::<f64>() / cpu_count as f64
    } else {
        0.0
    };

    let mem_total = sys.total_memory();
    let mem_used = sys.used_memory();
    let mem_free = sys.free_memory();
    let mem_available = sys.available_memory();

    let swap_total = sys.total_swap();
    let swap_used = sys.used_swap();

    // Storage / Disks metrics
    let (disk_total, disk_available) = if let Ok(disks) = Disks::new_with_refreshed_list() {
        let mut total = 0u64;
        let mut avail = 0u64;
        for disk in &disks {
            total += disk.total_space();
            avail += disk.available_space();
        }
        (total, avail)
    } else {
        (0, 0)
    };
    let disk_used = disk_total.saturating_sub(disk_available);
    let disk_percent = percent(disk_used, disk_total);

    let uptime_secs = System::uptime().unwrap_or(0);

    SystemMetrics {
        host_name,
        os_name,
        kernel_version,
        cpu_count,
        cpu_avg_percent,
        cpu_cores,
        mem_total,
        mem_used,
        mem_free,
        mem_available,
        mem_percent: percent(mem_used, mem_total),
        swap_total,
        swap_used,
        swap_percent: percent(swap_used, swap_total),
        disk_total,
        disk_used,
        disk_free: disk_available,
        disk_percent,
        uptime_str: format_uptime(uptime_secs),
        process_count: sys.processes().len(),
    }
}

//! SystemResourceMonitor - low-level system resource monitoring using sysinfo.

use sysinfo::{Pid, System};

/// Monitors system-wide and per-process resource usage.
///
/// This struct provides a lightweight wrapper around `sysinfo::System`
/// so higher-level components (e.g. Byggvir's ResourceManager) can
/// query RAM/CPU usage without depending directly on `sysinfo`.
pub struct SystemResourceMonitor {
    system: System,
}

impl SystemResourceMonitor {
    /// Create a new `SystemResourceMonitor` and perform an initial refresh.
    pub fn new() -> Self {
        let mut system = System::new_all();
        system.refresh_all();
        Self { system }
    }

    /// Refresh all system information.
    pub fn refresh(&mut self) {
        self.system.refresh_all();
    }

    /// Total physical memory in bytes.
    pub fn total_memory(&self) -> u64 {
        self.system.total_memory()
    }

    /// Currently used memory in bytes.
    pub fn used_memory(&self) -> u64 {
        self.system.used_memory()
    }

    /// Approximate total CPU usage percentage (0.0–100.0+).
    pub fn total_cpu(&self) -> f32 {
        self.system.global_cpu_info().cpu_usage()
    }

    /// Memory usage in bytes for a given process ID, if known.
    pub fn process_memory(&self, pid: Pid) -> Option<u64> {
        self.system.process(pid).map(|p| p.memory())
    }

    /// CPU usage percentage for a given process ID, if known.
    pub fn process_cpu(&self, pid: Pid) -> Option<f32> {
        self.system.process(pid).map(|p| p.cpu_usage())
    }
}


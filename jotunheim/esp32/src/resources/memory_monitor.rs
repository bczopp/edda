//! MemoryMonitor (Phase 6.1.1, TDD).

use std::sync::atomic::{AtomicU64, Ordering};

/// Monitors RAM usage and triggers warning/exhaustion alerts.
pub struct MemoryMonitor {
    current_kb: AtomicU64,
    warning_threshold_kb: u64,
    exhaustion_threshold_kb: u64,
}

impl MemoryMonitor {
    pub fn new(warning_threshold_kb: u64, exhaustion_threshold_kb: u64) -> Self {
        Self {
            current_kb: AtomicU64::new(0),
            warning_threshold_kb,
            exhaustion_threshold_kb,
        }
    }

    pub fn current_usage_kb(&self) -> u64 {
        self.current_kb.load(Ordering::Relaxed)
    }

    pub fn set_usage_kb(&self, kb: u64) {
        self.current_kb.store(kb, Ordering::Relaxed);
    }

    pub fn is_warning(&self) -> bool {
        self.current_kb.load(Ordering::Relaxed) >= self.warning_threshold_kb
    }

    pub fn is_exhausted(&self) -> bool {
        self.current_kb.load(Ordering::Relaxed) >= self.exhaustion_threshold_kb
    }
}

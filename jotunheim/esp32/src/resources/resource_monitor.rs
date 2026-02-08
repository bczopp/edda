//! ResourceMonitor (Phase 11.2.1, TDD).
//! Aggregates RAM, CPU, and network usage for monitoring.

use std::sync::atomic::{AtomicU32, Ordering};

use super::{BandwidthMonitor, MemoryMonitor};

/// Tracks RAM, CPU, and network usage in one place.
pub struct ResourceMonitor {
    memory: MemoryMonitor,
    cpu_percent: AtomicU32,
    network: BandwidthMonitor,
}

impl ResourceMonitor {
    pub fn new(
        memory_warning_kb: u64,
        memory_exhaustion_kb: u64,
        network_limit_bytes: u64,
    ) -> Self {
        Self {
            memory: MemoryMonitor::new(memory_warning_kb, memory_exhaustion_kb),
            cpu_percent: AtomicU32::new(0),
            network: BandwidthMonitor::new(network_limit_bytes),
        }
    }

    pub fn ram_usage_kb(&self) -> u64 {
        self.memory.current_usage_kb()
    }

    pub fn set_ram_usage_kb(&self, kb: u64) {
        self.memory.set_usage_kb(kb);
    }

    pub fn cpu_usage_percent(&self) -> u32 {
        self.cpu_percent.load(Ordering::Relaxed)
    }

    pub fn set_cpu_usage_percent(&self, percent: u32) {
        self.cpu_percent.store(percent.min(100), Ordering::Relaxed);
    }

    pub fn network_sent_bytes(&self) -> u64 {
        self.network.sent_bytes()
    }

    pub fn network_received_bytes(&self) -> u64 {
        self.network.received_bytes()
    }

    pub fn add_network_sent(&self, n: u64) {
        self.network.add_sent(n);
    }

    pub fn add_network_received(&self, n: u64) {
        self.network.add_received(n);
    }

    pub fn network_tick(&self) {
        self.network.tick();
    }
}

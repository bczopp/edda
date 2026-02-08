//! BandwidthMonitor (Phase 6.3.1, TDD).

use std::sync::atomic::{AtomicU64, Ordering};

/// Tracks network usage and throttles when over limit (per window).
pub struct BandwidthMonitor {
    limit_bytes: u64,
    sent: AtomicU64,
    received: AtomicU64,
}

impl BandwidthMonitor {
    pub fn new(limit_bytes: u64) -> Self {
        Self {
            limit_bytes,
            sent: AtomicU64::new(0),
            received: AtomicU64::new(0),
        }
    }

    pub fn add_sent(&self, n: u64) {
        self.sent.fetch_add(n, Ordering::Relaxed);
    }

    pub fn add_received(&self, n: u64) {
        self.received.fetch_add(n, Ordering::Relaxed);
    }

    pub fn sent_bytes(&self) -> u64 {
        self.sent.load(Ordering::Relaxed)
    }

    pub fn received_bytes(&self) -> u64 {
        self.received.load(Ordering::Relaxed)
    }

    pub fn should_throttle(&self) -> bool {
        self.sent.load(Ordering::Relaxed) + self.received.load(Ordering::Relaxed) >= self.limit_bytes
    }

    /// Reset window (call periodically).
    pub fn tick(&self) {
        self.sent.store(0, Ordering::Relaxed);
        self.received.store(0, Ordering::Relaxed);
    }
}

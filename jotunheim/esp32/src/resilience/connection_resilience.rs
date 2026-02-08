//! ConnectionResilienceManager (Phase 8.1.1, TDD).

use std::sync::atomic::{AtomicU32, Ordering};
use std::time::Duration;

/// Manages reconnection with exponential backoff.
pub struct ConnectionResilienceManager {
    max_retries: u32,
    base_delay_ms: u64,
    retry_count: AtomicU32,
}

impl ConnectionResilienceManager {
    pub fn new(max_retries: u32, base_delay_ms: u64) -> Self {
        Self {
            max_retries,
            base_delay_ms,
            retry_count: AtomicU32::new(0),
        }
    }

    pub fn retry_count(&self) -> u32 {
        self.retry_count.load(Ordering::Relaxed)
    }

    pub fn should_retry(&self) -> bool {
        self.retry_count.load(Ordering::Relaxed) < self.max_retries
    }

    pub fn record_failure(&self) {
        self.retry_count.fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_success(&self) {
        self.retry_count.store(0, Ordering::Relaxed);
    }

    pub fn next_backoff_delay(&self) -> Duration {
        let n = self.retry_count.load(Ordering::Relaxed);
        let ms = self.base_delay_ms.saturating_mul(1 << n.min(10));
        Duration::from_millis(ms.min(60_000))
    }
}

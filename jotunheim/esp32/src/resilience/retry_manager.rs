//! RetryManager (Phase 8.3.1, TDD).

use std::time::Duration;

/// Computes exponential backoff delay and tracks exhaustion.
pub struct RetryManager {
    max_attempts: u32,
    base_delay_ms: u64,
    max_delay_ms: u64,
}

impl RetryManager {
    pub fn new(max_attempts: u32, base_delay_ms: u64) -> Self {
        Self {
            max_attempts,
            base_delay_ms,
            max_delay_ms: 60_000,
        }
    }

    pub fn with_max_delay(mut self, max_delay_ms: u64) -> Self {
        self.max_delay_ms = max_delay_ms;
        self
    }

    pub fn next_delay(&self, attempt: u32) -> Duration {
        let ms = self
            .base_delay_ms
            .saturating_mul(1 << attempt.min(16))
            .min(self.max_delay_ms);
        Duration::from_millis(ms)
    }

    pub fn exhausted(&self, attempt: u32) -> bool {
        attempt >= self.max_attempts
    }
}

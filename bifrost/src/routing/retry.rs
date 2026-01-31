//! Retry Manager (Phase 10.1.1): exponential backoff, max retries.

use std::time::Duration;

/// Retry policy: immediate first retry, then exponential backoff.
pub struct RetryManager {
    max_retries: u32,
    base_delay: Duration,
    attempt_count: u32,
}

impl RetryManager {
    /// Creates a new retry manager.
    /// * `max_retries` – maximum number of retry attempts (total attempts = max_retries + 1).
    /// * `base_delay` – delay after first failure; doubles each attempt.
    pub fn new(max_retries: u32, base_delay: Duration) -> Self {
        Self {
            max_retries,
            base_delay,
            attempt_count: 0,
        }
    }

    /// Returns true if another retry is allowed.
    pub fn should_retry(&self) -> bool {
        self.attempt_count < self.max_retries
    }

    /// Returns the delay before the next attempt. Zero for the first attempt (immediate retry).
    pub fn next_delay(&self) -> Duration {
        if self.attempt_count == 0 {
            return Duration::ZERO;
        }
        let exponent = (self.attempt_count - 1).min(31);
        self.base_delay.saturating_mul(1 << exponent)
    }

    /// Records that an attempt was made (call after a failed try).
    pub fn record_attempt(&mut self) {
        self.attempt_count = self.attempt_count.saturating_add(1);
    }

    /// Resets attempt count for a new operation.
    pub fn reset(&mut self) {
        self.attempt_count = 0;
    }
}

impl Default for RetryManager {
    fn default() -> Self {
        Self::new(5, Duration::from_secs(1))
    }
}

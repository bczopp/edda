//! Reconnection Manager (Phase 7.2.1): immediate first attempt, exponential backoff, max 60s, jitter, continuous attempts.

use std::time::Duration;

/// Configuration for reconnection backoff.
#[derive(Clone, Debug)]
pub struct ReconnectionConfig {
    /// Delay after first failure; doubles each attempt.
    pub base_delay: Duration,
    /// Maximum delay between attempts (cap).
    pub max_delay: Duration,
    /// Jitter ratio in [0.0, 1.0]. 0 = no jitter. Adds random 0..(delay * jitter_ratio) to avoid thundering herd.
    pub jitter_ratio: f64,
}

impl Default for ReconnectionConfig {
    fn default() -> Self {
        Self {
            base_delay: Duration::from_secs(1),
            max_delay: Duration::from_secs(60),
            jitter_ratio: 0.1,
        }
    }
}

/// Manages reconnection timing: immediate first try, then exponential backoff capped at max_delay, with optional jitter.
pub struct ReconnectionManager {
    config: ReconnectionConfig,
    attempt_count: u32,
}

impl ReconnectionManager {
    pub fn new(config: ReconnectionConfig) -> Self {
        Self {
            config,
            attempt_count: 0,
        }
    }

    /// Delay before the next connection attempt. Zero on first call (immediate reconnect).
    pub fn next_delay(&self) -> Duration {
        let raw = if self.attempt_count == 0 {
            Duration::ZERO
        } else {
            let exponent = (self.attempt_count - 1).min(31);
            let d = self.config.base_delay.saturating_mul(1 << exponent);
            std::cmp::min(d, self.config.max_delay)
        };
        if raw.is_zero() || self.config.jitter_ratio <= 0.0 {
            return raw;
        }
        let jitter_nanos = (raw.as_nanos() as f64 * self.config.jitter_ratio) as u64;
        let added = if jitter_nanos > 0 {
            rand::thread_rng().gen_range(0..=jitter_nanos)
        } else {
            0
        };
        raw + Duration::from_nanos(added)
    }

    /// Record that a connection attempt was made (call after a failed try).
    pub fn record_attempt(&mut self) {
        self.attempt_count = self.attempt_count.saturating_add(1);
    }

    /// Reset attempt count (e.g. after successful connection).
    pub fn reset(&mut self) {
        self.attempt_count = 0;
    }
}

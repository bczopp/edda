//! Rate Limiter (Phase 4.3.1). Token-based / sliding-window; rate-limit-exceeded handling.

use std::collections::HashMap;
use std::sync::RwLock;
use std::time::{Duration, Instant};
use thiserror::Error;

#[derive(Error, Debug)]
#[error("rate limit exceeded")]
pub struct RateLimitExceeded {
    retry_after: Duration,
}

impl RateLimitExceeded {
    pub fn retry_after(&self) -> Duration {
        self.retry_after
    }
}

/// Sliding-window rate limiter: at most `max_requests` per `window_duration` per key.
pub struct RateLimiter {
    max_requests: u32,
    window_duration: Duration,
    /// key -> sorted timestamps of requests in current window
    windows: RwLock<HashMap<String, Vec<Instant>>>,
}

impl RateLimiter {
    pub fn new(max_requests: u32, window_duration: Duration) -> Self {
        Self {
            max_requests: max_requests.max(1),
            window_duration,
            windows: RwLock::new(HashMap::new()),
        }
    }

    /// Returns Ok(()) if request is allowed, Err(RateLimitExceeded) with retry_after if over limit.
    pub fn check(&self, key: &str) -> Result<(), RateLimitExceeded> {
        let now = Instant::now();
        let cutoff = now.checked_sub(self.window_duration).unwrap_or(now);
        let mut map = self.windows.write().unwrap();
        let entries = map.entry(key.to_string()).or_insert_with(Vec::new);
        entries.retain(|&t| t > cutoff);
        if entries.len() < self.max_requests as usize {
            entries.push(now);
            Ok(())
        } else {
            let oldest = entries.first().copied().unwrap_or(now);
            let deadline = oldest + self.window_duration;
            let retry_after = deadline.saturating_duration_since(now);
            Err(RateLimitExceeded {
                retry_after: retry_after.max(Duration::from_millis(1)),
            })
        }
    }
}

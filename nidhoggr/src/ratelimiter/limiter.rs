use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RateLimitError {
    #[error("Rate limit exceeded: {0}")]
    RateLimitExceeded(String),
}

struct RateLimitEntry {
    minute_count: u32,
    hour_count: u32,
    minute_window_start: Instant,
    hour_window_start: Instant,
}

pub struct RateLimiter {
    minute_limit: u32,
    hour_limit: u32,
    entries: Arc<RwLock<HashMap<String, RateLimitEntry>>>,
}

impl RateLimiter {
    pub fn new(minute_limit: u32, hour_limit: u32) -> Self {
        Self {
            minute_limit,
            hour_limit,
            entries: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn check_rate_limit(&self, device_id: &str, user_id: &str) -> Result<(), RateLimitError> {
        let key = format!("{}:{}", device_id, user_id);
        let now = Instant::now();
        
        let mut entries = self.entries.write().await;
        let entry = entries.entry(key.clone()).or_insert_with(|| {
            RateLimitEntry {
                minute_count: 0,
                hour_count: 0,
                minute_window_start: now,
                hour_window_start: now,
            }
        });

        // Reset minute window if expired
        if now.duration_since(entry.minute_window_start) >= Duration::from_secs(60) {
            entry.minute_count = 0;
            entry.minute_window_start = now;
        }

        // Reset hour window if expired
        if now.duration_since(entry.hour_window_start) >= Duration::from_secs(3600) {
            entry.hour_count = 0;
            entry.hour_window_start = now;
        }

        // Check limits
        if entry.minute_count >= self.minute_limit {
            return Err(RateLimitError::RateLimitExceeded(
                format!("Minute limit exceeded: {}/{}", entry.minute_count, self.minute_limit)
            ));
        }

        if entry.hour_count >= self.hour_limit {
            return Err(RateLimitError::RateLimitExceeded(
                format!("Hour limit exceeded: {}/{}", entry.hour_count, self.hour_limit)
            ));
        }

        // Increment counters
        entry.minute_count += 1;
        entry.hour_count += 1;

        Ok(())
    }
}

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
    count: u32,
    window_start: Instant,
}

pub struct RateLimiter {
    limit: u32,
    window_seconds: u64,
    entries: Arc<RwLock<HashMap<String, RateLimitEntry>>>,
}

impl RateLimiter {
    pub fn new(limit: u32, window_seconds: u64) -> Self {
        Self {
            limit,
            window_seconds,
            entries: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn check_rate_limit(&self, key: &str) -> Result<(), RateLimitError> {
        let now = Instant::now();
        let mut entries = self.entries.write().await;
        
        let entry = entries.entry(key.to_string()).or_insert_with(|| {
            RateLimitEntry {
                count: 0,
                window_start: now,
            }
        });

        // Reset window if expired
        if now.duration_since(entry.window_start) >= Duration::from_secs(self.window_seconds) {
            entry.count = 0;
            entry.window_start = now;
        }

        // Check limit
        if entry.count >= self.limit {
            return Err(RateLimitError::RateLimitExceeded(
                format!("Limit of {} requests per {} seconds exceeded", self.limit, self.window_seconds)
            ));
        }

        // Increment counter
        entry.count += 1;

        Ok(())
    }
}

//! ConnectionResilienceManager – exponential backoff retry (Phase 10.2.1).

use shared::{LokiError, Result};
use std::time::Duration;

/// Retry operations with exponential backoff.
pub struct ConnectionResilienceManager {
    max_retries: u32,
    base_delay: Duration,
}

impl ConnectionResilienceManager {
    pub fn new(max_retries: u32, base_delay: Duration) -> Self {
        Self {
            max_retries: max_retries.max(1),
            base_delay,
        }
    }

    /// Run async operation with retry; exponential backoff on retriable errors.
    pub async fn run_with_retry<F, Fut, T>(&self, op: F) -> Result<T>
    where
        F: Fn() -> Fut,
        Fut: std::future::Future<Output = Result<T>>,
    {
        let mut last_err = None;
        for attempt in 0..self.max_retries {
            match op().await {
                Ok(t) => return Ok(t),
                Err(e) => {
                    last_err = Some(e);
                    if !Self::is_retriable(&last_err.as_ref().unwrap()) || attempt + 1 >= self.max_retries {
                        break;
                    }
                    let delay = self.base_delay * 2u32.saturating_pow(attempt);
                    tokio::time::sleep(delay).await;
                }
            }
        }
        Err(last_err.unwrap_or_else(|| LokiError::ServiceUnavailable("no attempt".into())))
    }

    fn is_retriable(e: &LokiError) -> bool {
        matches!(
            e,
            LokiError::ServiceUnavailable(_) | LokiError::IoError(_)
        )
    }
}

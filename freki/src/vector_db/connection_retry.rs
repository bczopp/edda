//! Connection-Retry-Manager (Phase 13.3.1): Reconnect mit Exponential-Backoff, max Retries.

use crate::vector_db::VectorDbError;
use std::time::Duration;

/// Reconnect mit sofortigem Versuch, Exponential-Backoff, max Retries.
pub struct ConnectionRetryManager {
    pub max_retries: u32,
    pub initial_delay: Duration,
    pub max_delay: Duration,
}

impl Default for ConnectionRetryManager {
    fn default() -> Self {
        Self {
            max_retries: 5,
            initial_delay: Duration::from_millis(100),
            max_delay: Duration::from_secs(30),
        }
    }
}

impl ConnectionRetryManager {
    pub fn new(max_retries: u32, initial_delay: Duration, max_delay: Duration) -> Self {
        Self {
            max_retries: max_retries.max(1),
            initial_delay,
            max_delay,
        }
    }

    /// Vector-DB-Client mit Retry erstellen (Exponential-Backoff).
    pub async fn connect_vector_db(&self, url: &str) -> Result<crate::vector_db::VectorDbClient, VectorDbError> {
        let url = url.to_string();
        self.connect_with_retry(|| {
            let u = url.clone();
            async move { crate::vector_db::VectorDbClient::new(&u).await }
        })
        .await
    }

    /// Beliebige async Operation mit Retry (Exponential-Backoff).
    pub async fn connect_with_retry<F, Fut, T, E>(&self, mut op: F) -> Result<T, E>
    where
        F: FnMut() -> Fut,
        Fut: std::future::Future<Output = Result<T, E>>,
        E: std::fmt::Display,
    {
        let mut last_err = None;
        let mut delay = self.initial_delay;
        for attempt in 0..=self.max_retries {
            match op().await {
                Ok(t) => return Ok(t),
                Err(e) => {
                    last_err = Some(e);
                    if attempt < self.max_retries {
                        tokio::time::sleep(delay).await;
                        delay = std::cmp::min(
                            Duration::from_millis(delay.as_millis() as u64 * 2),
                            self.max_delay,
                        );
                    } else {
                        break;
                    }
                }
            }
        }
        Err(last_err.expect("at least one attempt"))
    }
}

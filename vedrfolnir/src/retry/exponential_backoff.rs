use std::time::Duration;
use tokio::time::sleep;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RetryError {
    #[error("Max retries exceeded")]
    MaxRetriesExceeded,
}

#[derive(Clone)]
pub struct ExponentialBackoff {
    initial_delay: Duration,
    max_delay: Duration,
    multiplier: f64,
    max_retries: usize,
}

impl ExponentialBackoff {
    pub fn new(initial_delay_ms: u64, max_delay_ms: u64, multiplier: f64, max_retries: usize) -> Self {
        Self {
            initial_delay: Duration::from_millis(initial_delay_ms),
            max_delay: Duration::from_millis(max_delay_ms),
            multiplier,
            max_retries,
        }
    }

    pub async fn execute<F, T, E>(&self, mut operation: F) -> Result<T, RetryError>
    where
        F: FnMut() -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<T, E>> + Send>>,
        E: std::fmt::Display,
    {
        let mut delay = self.initial_delay;
        let mut retries = 0;

        loop {
            match operation().await {
                Ok(result) => return Ok(result),
                Err(e) => {
                    if retries >= self.max_retries {
                        return Err(RetryError::MaxRetriesExceeded);
                    }

                    tracing::warn!("Operation failed (attempt {}/{}): {}, retrying in {:?}", 
                        retries + 1, self.max_retries, e, delay);
                    
                    sleep(delay).await;
                    
                    delay = Duration::from_millis(
                        (delay.as_millis() as f64 * self.multiplier) as u64
                    ).min(self.max_delay);
                    
                    retries += 1;
                }
            }
        }
    }
}

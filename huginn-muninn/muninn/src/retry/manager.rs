//! Retry Manager for Muninn

use std::time::Duration;
use tracing::{debug, warn, error};
use std::sync::Arc;
use tokio::sync::RwLock;

/// Retry Manager with exponential backoff
pub struct RetryManager {
    max_retries: usize,
    initial_delay: Duration,
}

impl RetryManager {
    /// Create a new RetryManager
    pub fn new(max_retries: usize, initial_delay: Duration) -> Self {
        debug!("Creating RetryManager with max_retries={}, initial_delay={:?}", max_retries, initial_delay);
        Self {
            max_retries,
            initial_delay,
        }
    }
    
    /// Get max retries
    pub fn max_retries(&self) -> usize {
        self.max_retries
    }
    
    /// Get initial delay
    pub fn initial_delay(&self) -> Duration {
        self.initial_delay
    }
    
    /// Calculate retry delays (exponential backoff)
    pub fn calculate_delays(&self) -> Vec<Duration> {
        let mut delays = Vec::new();
        for i in 0..self.max_retries {
            let delay = self.initial_delay * (1 << i); // 2^i multiplier
            delays.push(delay);
        }
        delays
    }
    
    /// Execute a function with retries
    pub async fn execute<F, Fut, T, E>(&self, mut f: F) -> Result<T, E>
    where
        F: FnMut() -> Fut,
        Fut: std::future::Future<Output = Result<T, E>>,
    {
        let mut last_error = None;
        
        // Initial attempt
        match f().await {
            Ok(result) => {
                debug!("Operation succeeded on first attempt");
                return Ok(result);
            }
            Err(e) => {
                warn!("Operation failed on first attempt: {:?}", e);
                last_error = Some(e);
            }
        }
        
        // Retry attempts with exponential backoff
        let delays = self.calculate_delays();
        for (attempt, delay) in delays.iter().enumerate() {
            debug!("Retrying operation (attempt {}/{}) after {:?}", attempt + 1, self.max_retries, delay);
            tokio::time::sleep(*delay).await;
            
            match f().await {
                Ok(result) => {
                    debug!("Operation succeeded after {} retries", attempt + 1);
                    return Ok(result);
                }
                Err(e) => {
                    warn!("Operation failed on retry {}: {:?}", attempt + 1, e);
                    last_error = Some(e);
                }
            }
        }
        
        // All retries exhausted
        error!("Operation failed after {} retries", self.max_retries);
        last_error.unwrap()
    }
}

impl Default for RetryManager {
    fn default() -> Self {
        Self::new(3, Duration::from_millis(100))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_retry_manager_default() {
        let retry = RetryManager::default();
        assert_eq!(retry.max_retries(), 3);
        assert_eq!(retry.initial_delay(), Duration::from_millis(100));
    }
}

//! Retrieval-Error-Handler (Phase 13.2.1): Fehler kategorisieren, Retry, Logging.

use std::error::Error;
use std::time::Duration;

/// Kategorie eines Retrieval-Fehlers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RetrievalErrorCategory {
    VectorDb,
    Embedding,
    Timeout,
    Unknown,
}

/// Handler für Retrieval-Fehler: Kategorisierung, Retry-Strategie, Logging.
pub struct RetrievalErrorHandler {
    pub max_retries: u32,
    pub retry_delay: Duration,
}

impl Default for RetrievalErrorHandler {
    fn default() -> Self {
        Self {
            max_retries: 3,
            retry_delay: Duration::from_millis(100),
        }
    }
}

impl RetrievalErrorHandler {
    pub fn new(max_retries: u32, retry_delay: Duration) -> Self {
        Self {
            max_retries: max_retries.max(1),
            retry_delay,
        }
    }

    /// Fehler kategorisieren (anhand Fehlermeldung).
    pub fn categorize(&self, err: &(dyn Error + 'static)) -> RetrievalErrorCategory {
        let msg = err.to_string().to_lowercase();
        if msg.contains("timeout") || msg.contains("timed out") {
            return RetrievalErrorCategory::Timeout;
        }
        if msg.contains("embed") || msg.contains("embedding") || msg.contains("query") && msg.contains("model") {
            return RetrievalErrorCategory::Embedding;
        }
        if msg.contains("qdrant") || msg.contains("vector") || msg.contains("connection") || msg.contains("search") {
            return RetrievalErrorCategory::VectorDb;
        }
        RetrievalErrorCategory::Unknown
    }

    /// Ob die Kategorie retry-fähig ist.
    pub fn is_retriable(&self, category: RetrievalErrorCategory) -> bool {
        matches!(
            category,
            RetrievalErrorCategory::VectorDb | RetrievalErrorCategory::Timeout | RetrievalErrorCategory::Embedding
        )
    }

    /// Operation mit Retry ausführen (bei retriable Fehlern).
    pub async fn execute_with_retry<F, Fut, T, E>(&self, mut op: F) -> Result<T, E>
    where
        F: FnMut() -> Fut,
        Fut: std::future::Future<Output = Result<T, E>>,
        E: Error + 'static,
    {
        let mut last_err = None;
        for attempt in 0..=self.max_retries {
            match op().await {
                Ok(t) => return Ok(t),
                Err(e) => {
                    last_err = Some(e);
                    let cat = self.categorize(last_err.as_ref().unwrap());
                    if attempt < self.max_retries && self.is_retriable(cat) {
                        tokio::time::sleep(self.retry_delay).await;
                        continue;
                    }
                    break;
                }
            }
        }
        Err(last_err.expect("at least one attempt"))
    }
}

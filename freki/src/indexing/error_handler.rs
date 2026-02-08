//! Indexing-Error-Handler (Phase 13.1.1): Fehler kategorisieren, Retry, Logging.

use std::error::Error;
use std::time::Duration;

/// Kategorie eines Indexing-Fehlers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IndexingErrorCategory {
    Parse,
    Embedding,
    VectorDb,
    Unknown,
}

/// Handler für Indexing-Fehler: Kategorisierung, Retry-Strategie, Logging.
pub struct IndexingErrorHandler {
    pub max_retries: u32,
    pub retry_delay: Duration,
}

impl Default for IndexingErrorHandler {
    fn default() -> Self {
        Self {
            max_retries: 3,
            retry_delay: Duration::from_millis(100),
        }
    }
}

impl IndexingErrorHandler {
    pub fn new(max_retries: u32, retry_delay: Duration) -> Self {
        Self {
            max_retries: max_retries.max(1),
            retry_delay,
        }
    }

    /// Fehler kategorisieren (anhand Fehlermeldung).
    pub fn categorize(&self, err: &(dyn Error + 'static)) -> IndexingErrorCategory {
        let msg = err.to_string().to_lowercase();
        if msg.contains("parse") || msg.contains("unsupported") || msg.contains("file type") {
            return IndexingErrorCategory::Parse;
        }
        if msg.contains("embed") || msg.contains("embedding") || msg.contains("model") {
            return IndexingErrorCategory::Embedding;
        }
        if msg.contains("qdrant") || msg.contains("vector") || msg.contains("connection") || msg.contains("collection") {
            return IndexingErrorCategory::VectorDb;
        }
        IndexingErrorCategory::Unknown
    }

    /// Ob die Kategorie retry-fähig ist.
    pub fn is_retriable(&self, category: IndexingErrorCategory) -> bool {
        matches!(category, IndexingErrorCategory::VectorDb | IndexingErrorCategory::Embedding)
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

    /// Benutzerfreundliche Meldung für eine Kategorie (für User-Benachrichtigung).
    pub fn user_message(&self, category: IndexingErrorCategory) -> &'static str {
        match category {
            IndexingErrorCategory::Parse => "Dokumentformat wird nicht unterstützt oder konnte nicht gelesen werden.",
            IndexingErrorCategory::Embedding => "Embedding-Modell nicht verfügbar oder Fehler bei der Verarbeitung.",
            IndexingErrorCategory::VectorDb => "Verbindung zur Vektordatenbank fehlgeschlagen. Bitte später erneut versuchen.",
            IndexingErrorCategory::Unknown => "Ein Fehler ist beim Indizieren aufgetreten.",
        }
    }
}

//! Batch-Indexing (Phase 6.3.2): Multiple documents, batch-size limit, optional progress.

use async_trait::async_trait;
use std::sync::{Arc, Mutex};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum BatchIndexingError {
    #[error("Indexing error at index {0}: {1}")]
    Item(usize, String),
}

/// Result of a batch indexing run.
#[derive(Debug, Clone, Default)]
pub struct BatchIndexingResult {
    pub indexed: u32,
    pub failed: u32,
    /// (item_index, error_message)
    pub errors: Vec<(usize, String)>,
}

/// Backend that indexes a single document (used by BatchIndexingManager).
#[async_trait]
pub trait SingleDocumentIndexer: Send + Sync {
    async fn index_bytes(
        &self,
        bytes: &[u8],
        file_extension: &str,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
}

/// Indiziert mehrere Dokumente parallel mit Batch-Größen-Limit und optionalem Progress-Callback.
///
/// # Beispiel
///
/// ```no_run
/// # use freki::indexing::{BatchIndexingManager, IndexingManager};
/// # use std::sync::Arc;
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// # let backend = Arc::new(IndexingManager::new(/* ... */));
/// let batch_manager = BatchIndexingManager::new(backend, 4)
///     .with_progress(|current, total| {
///         println!("Progress: {}/{}", current, total);
///     });
///
/// let items = vec![
///     (b"doc1".to_vec(), "txt".to_string()),
///     (b"doc2".to_vec(), "txt".to_string()),
/// ];
/// let result = batch_manager.index_batch(items).await?;
/// println!("Indexed: {}, Failed: {}", result.indexed, result.failed);
/// # Ok(())
/// # }
/// ```
pub struct BatchIndexingManager {
    backend: Arc<dyn SingleDocumentIndexer>,
    batch_size: usize,
    /// Optional: (current_count, total_count) after each document.
    progress: Option<Arc<dyn Fn(usize, usize) + Send + Sync>>,
}

impl BatchIndexingManager {
    pub fn new(backend: Arc<dyn SingleDocumentIndexer>, batch_size: usize) -> Self {
        Self {
            backend,
            batch_size: batch_size.max(1),
            progress: None,
        }
    }

    pub fn with_progress<F>(mut self, f: F) -> Self
    where
        F: Fn(usize, usize) + Send + Sync + 'static,
    {
        self.progress = Some(Arc::new(f));
        self
    }

    /// Index all items. Processes up to `batch_size` documents concurrently per batch.
    pub async fn index_batch(
        &self,
        items: Vec<(Vec<u8>, String)>,
    ) -> Result<BatchIndexingResult, Box<dyn std::error::Error + Send + Sync>> {
        let total = items.len();
        let mut result = BatchIndexingResult::default();

        for (batch_start, chunk) in items.chunks(self.batch_size).enumerate() {
            let mut handles = Vec::with_capacity(chunk.len());
            for (i, (bytes, ext)) in chunk.iter().enumerate() {
                let backend = Arc::clone(&self.backend);
                let bytes = bytes.clone();
                let ext = ext.clone();
                let progress = self.progress.clone();
                let global_index = batch_start * self.batch_size + i;
                handles.push(tokio::spawn(async move {
                    let r = backend.index_bytes(&bytes, &ext).await;
                    if let Some(ref p) = progress {
                        p(global_index + 1, total);
                    }
                    (global_index, r)
                }));
            }
            for h in handles {
                let (idx, outcome) = h.await.map_err(|e| -> Box<dyn std::error::Error + Send + Sync> {
                    format!("join error: {}", e).into()
                })?;
                match outcome {
                    Ok(()) => result.indexed += 1,
                    Err(e) => {
                        result.failed += 1;
                        result.errors.push((idx, e.to_string()));
                    }
                }
            }
        }

        Ok(result)
    }
}

#[async_trait]
impl SingleDocumentIndexer for crate::indexing::IndexingManager {
    async fn index_bytes(
        &self,
        bytes: &[u8],
        file_extension: &str,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        crate::indexing::IndexingManager::index_bytes(self, bytes, file_extension)
            .await
            .map_err(|e| e.to_string().into())
    }
}

/// Test helper: records index_bytes calls for assertions.
#[doc(hidden)]
pub struct RecordingIndexer {
    pub calls: Mutex<Vec<(Vec<u8>, String)>>,
    pub fail_at: Mutex<Option<usize>>,
}

impl Default for RecordingIndexer {
    fn default() -> Self {
        Self::new()
    }
}

impl RecordingIndexer {
    pub fn new() -> Self {
        Self {
            calls: Mutex::new(Vec::new()),
            fail_at: Mutex::new(None),
        }
    }
    pub fn set_fail_at(&self, index: Option<usize>) {
        *self.fail_at.lock().unwrap() = index;
    }
    pub fn calls(&self) -> Vec<(Vec<u8>, String)> {
        self.calls.lock().unwrap().clone()
    }
}

#[async_trait]
impl SingleDocumentIndexer for RecordingIndexer {
    async fn index_bytes(
        &self,
        bytes: &[u8],
        file_extension: &str,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let current = self.calls.lock().unwrap().len();
        if self.fail_at.lock().unwrap() == &Some(current) {
            return Err(format!("mock fail at index {}", current).into());
        }
        self.calls
            .lock()
            .unwrap()
            .push((bytes.to_vec(), file_extension.to_string()));
        Ok(())
    }
}

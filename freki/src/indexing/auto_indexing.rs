//! Auto-Indexing (Phase 8.1.2): Neue Dateien indizieren, geänderte re-indizieren, gelöschte entfernen.

use crate::indexing::{Document, DocumentIndexer, DocumentParser, FullReIndexingManager};
use crate::utils::DataDeletionManager;
use std::path::Path;
use std::sync::Arc;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AutoIndexingError {
    #[error("IO error: {0}")]
    Io(String),
    #[error("Parse error: {0}")]
    Parse(String),
    #[error("Indexing error: {0}")]
    Indexing(String),
    #[error("Deletion error: {0}")]
    Deletion(String),
}

/// Verbindet Watch-Events mit Indexing: neue Dateien indizieren, geänderte re-indizieren, gelöschte entfernen.
///
/// # Beispiel
///
/// ```no_run
/// # use freki::indexing::{AutoIndexingManager, DocumentIndexer, TextParser, FullReIndexingManager};
/// # use freki::utils::DataDeletionManager;
/// # use std::sync::Arc;
/// # use std::path::Path;
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// # let parser = Arc::new(TextParser::new());
/// # let indexer = Arc::new(DocumentIndexer::new(/* ... */));
/// # let full_reindex = Arc::new(FullReIndexingManager::new(Arc::clone(&indexer)));
/// # let data_deletion = Arc::new(DataDeletionManager::new(/* ... */));
/// let auto = AutoIndexingManager::new(parser, indexer, full_reindex, data_deletion);
///
/// // Watch-Events verarbeiten
/// auto.handle_created(Path::new("new_file.txt")).await?;
/// auto.handle_modified(Path::new("modified_file.txt")).await?;
/// auto.handle_removed(Path::new("deleted_file.txt")).await?;
/// # Ok(())
/// # }
/// ```
pub struct AutoIndexingManager {
    parser: Arc<dyn DocumentParser>,
    indexer: Arc<DocumentIndexer>,
    full_reindex: Arc<FullReIndexingManager>,
    data_deletion: Arc<DataDeletionManager>,
}

impl AutoIndexingManager {
    pub fn new(
        parser: Arc<dyn DocumentParser>,
        indexer: Arc<DocumentIndexer>,
        full_reindex: Arc<FullReIndexingManager>,
        data_deletion: Arc<DataDeletionManager>,
    ) -> Self {
        Self {
            parser,
            indexer,
            full_reindex,
            data_deletion,
        }
    }

    /// Konvertiert einen Pfad zu einer stabilen document_id (für Indexierung und Löschung).
    pub fn path_to_document_id(path: &Path) -> String {
        path.to_string_lossy().replace('/', "_").replace('\\', "_")
    }

    /// Verarbeitet ein Created-Event: liest Datei, parst und indiziert.
    pub async fn handle_created(&self, path: &Path) -> Result<(), AutoIndexingError> {
        if path.is_dir() {
            return Ok(());
        }
        let bytes = tokio::fs::read(path)
            .await
            .map_err(|e| AutoIndexingError::Io(e.to_string()))?;
        let extension = path
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("txt");
        if !self.parser.supports_file_type(extension) {
            return Ok(());
        }
        let mut document = self
            .parser
            .parse_document(&bytes, extension)
            .map_err(|e| AutoIndexingError::Parse(e.to_string()))?;
        document.id = Self::path_to_document_id(path);
        self.indexer
            .index_document_auto(document)
            .await
            .map_err(|e| AutoIndexingError::Indexing(e.to_string()))?;
        Ok(())
    }

    /// Verarbeitet ein Modified-Event: liest Datei, parst und re-indiziert vollständig.
    pub async fn handle_modified(&self, path: &Path) -> Result<(), AutoIndexingError> {
        if path.is_dir() {
            return Ok(());
        }
        let bytes = tokio::fs::read(path)
            .await
            .map_err(|e| AutoIndexingError::Io(e.to_string()))?;
        let extension = path
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("txt");
        if !self.parser.supports_file_type(extension) {
            return Ok(());
        }
        let mut document = self
            .parser
            .parse_document(&bytes, extension)
            .map_err(|e| AutoIndexingError::Parse(e.to_string()))?;
        document.id = Self::path_to_document_id(path);
        self.full_reindex
            .reindex_full(document)
            .await
            .map_err(|e| AutoIndexingError::Indexing(e.to_string()))?;
        Ok(())
    }

    /// Verarbeitet ein Removed-Event: entfernt Dokument aus Index.
    pub async fn handle_removed(&self, path: &Path) -> Result<(), AutoIndexingError> {
        let document_id = Self::path_to_document_id(path);
        self.data_deletion
            .delete_document(&document_id)
            .await
            .map_err(|e| AutoIndexingError::Deletion(e.to_string()))?;
        Ok(())
    }
}

//! Indexing-Manager (Phase 6.3): Pipeline parse → optional metadata → chunk → embed → index.

use crate::indexing::{Document, DocumentParser, MetadataExtractor, ParserError};
use std::sync::Arc;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum IndexingManagerError {
    #[error("Parse error: {0}")]
    Parse(#[from] ParserError),
    #[error("Indexing error: {0}")]
    Indexing(String),
}

/// Orchestriert die Indexing-Pipeline: Bytes parsen → optional Metadaten anreichern → indizieren.
///
/// # Beispiel
///
/// ```no_run
/// # use freki::indexing::{IndexingManager, DocumentIndexer, TextParser};
/// # use std::sync::Arc;
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// # let indexer = Arc::new(DocumentIndexer::new(/* ... */));
/// let parser = Arc::new(TextParser::new());
/// let manager = IndexingManager::new(parser, indexer);
///
/// let bytes = b"Sample document content";
/// manager.index_bytes(bytes, "txt").await?;
/// # Ok(())
/// # }
/// ```
pub struct IndexingManager {
    parser: Arc<dyn DocumentParser>,
    metadata_extractor: Option<Arc<MetadataExtractor>>,
    indexer: Arc<crate::indexing::DocumentIndexer>,
}

impl IndexingManager {
    pub fn new(
        parser: Arc<dyn DocumentParser>,
        indexer: Arc<crate::indexing::DocumentIndexer>,
    ) -> Self {
        Self {
            parser,
            metadata_extractor: None,
            indexer,
        }
    }

    pub fn with_metadata_extractor(mut self, ext: Arc<MetadataExtractor>) -> Self {
        self.metadata_extractor = Some(ext);
        self
    }

    /// Lädt Rohdaten, parst zu Document, reichert optional Metadaten an und indiziert.
    ///
    /// # Argumente
    ///
    /// * `bytes` - Rohdaten (z. B. Dateiinhalt)
    /// * `file_extension` - Dateiendung (z. B. "txt", "md") für Parser-Auswahl
    ///
    /// # Fehler
    ///
    /// Gibt einen Fehler zurück bei Parse-Fehlern oder Indexing-Fehlern.
    pub async fn index_bytes(
        &self,
        bytes: &[u8],
        file_extension: &str,
    ) -> Result<(), IndexingManagerError> {
        let mut document = self
            .parser
            .parse_document(bytes, file_extension)
            .map_err(IndexingManagerError::Parse)?;
        if let Some(ref ext) = self.metadata_extractor {
            document.metadata = ext.extract(&document).map_err(|e| {
                IndexingManagerError::Indexing(e.to_string())
            })?;
        }
        self.indexer
            .index_document_auto(document)
            .await
            .map_err(|e| IndexingManagerError::Indexing(e.to_string()))?;
        Ok(())
    }
}

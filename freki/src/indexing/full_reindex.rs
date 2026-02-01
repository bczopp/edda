//! Full-Re-Indexing (Phase 7.3.1): Alte Chunks entfernen, Dokument neu chunken und indizieren.

use crate::indexing::{Document, DocumentIndexer};
use std::sync::Arc;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum FullReIndexingError {
    #[error("Delete error: {0}")]
    Delete(String),
    #[error("Indexing error: {0}")]
    Indexing(String),
}

/// Ergebnis eines Full-Re-Indexing-Laufs.
#[derive(Debug, Clone, Default)]
pub struct FullReIndexingResult {
    /// Anzahl neu indizierter Chunks.
    pub chunks_indexed: usize,
}

/// Führt vollständiges Re-Indexing durch: entfernt alle bisherigen Chunks des Dokuments,
/// chunkt neu, erzeugt Embeddings und indiziert.
pub struct FullReIndexingManager {
    indexer: Arc<DocumentIndexer>,
}

impl FullReIndexingManager {
    pub fn new(indexer: Arc<DocumentIndexer>) -> Self {
        Self { indexer }
    }

    /// Re-Indexierung: alte Chunks löschen, Dokument neu chunken, embedden und indizieren.
    pub async fn reindex_full(&self, document: Document) -> Result<FullReIndexingResult, FullReIndexingError> {
        self.indexer
            .delete_document_chunks(&document.id)
            .await
            .map_err(|e| FullReIndexingError::Delete(e.to_string()))?;

        let chunks = if let Some(chunker) = self.indexer.chunker() {
            chunker
                .chunk_document(&document.content)
                .await
                .map_err(|e| FullReIndexingError::Indexing(e.to_string()))?
        } else {
            vec![document.content.clone()]
        };

        let embedding_model = self.indexer.embedding_model().ok_or_else(|| {
            FullReIndexingError::Indexing("Embedding model required".to_string())
        })?;

        let chunk_strings: Vec<String> = chunks.iter().cloned().collect();
        let embeddings = embedding_model
            .embed_batch(&chunk_strings)
            .await
            .map_err(|e| FullReIndexingError::Indexing(e.to_string()))?;

        for (i, (chunk, embedding)) in chunks.iter().zip(embeddings.iter()).enumerate() {
            let chunk_doc = Document {
                id: format!("{}-chunk-{}", document.id, i),
                content: chunk.clone(),
                metadata: document.metadata.clone(),
            };
            self.indexer
                .index_document(chunk_doc, embedding.clone())
                .await
                .map_err(|e| FullReIndexingError::Indexing(e.to_string()))?;
        }

        Ok(FullReIndexingResult {
            chunks_indexed: chunks.len(),
        })
    }
}

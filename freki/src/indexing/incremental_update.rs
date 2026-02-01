//! Incremental-Update (Phase 7.2.1): Nur geänderte Chunks re-indizieren.

use crate::chunking::DocumentChunker;
use crate::embedding::EmbeddingModel;
use crate::indexing::{Document, DocumentChangeDetector, DocumentHash, DocumentIndexer};
use std::sync::Arc;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum IncrementalUpdateError {
    #[error("Chunking error: {0}")]
    Chunking(String),
    #[error("Embedding error: {0}")]
    Embedding(String),
    #[error("Indexing error: {0}")]
    Indexing(String),
}

/// Ergebnis eines inkrementellen Updates (Anzahl aktualisierter Chunks).
#[derive(Debug, Clone, Default)]
pub struct IncrementalUpdateResult {
    pub updated_count: usize,
}

/// Führt inkrementelle Updates durch: identifiziert geänderte Chunks, erzeugt nur dafür
/// Embeddings und aktualisiert die Vector-DB selektiv.
pub struct IncrementalUpdateManager {
    indexer: Arc<DocumentIndexer>,
    chunker: Arc<dyn DocumentChunker>,
    embedding_model: Arc<dyn EmbeddingModel>,
    change_detector: DocumentChangeDetector,
}

impl IncrementalUpdateManager {
    pub fn new(
        indexer: Arc<DocumentIndexer>,
        chunker: Arc<dyn DocumentChunker>,
        embedding_model: Arc<dyn EmbeddingModel>,
        change_detector: DocumentChangeDetector,
    ) -> Self {
        Self {
            indexer,
            chunker,
            embedding_model,
            change_detector,
        }
    }

    /// Führt ein inkrementelles Update durch: chunkt das Dokument, ermittelt geänderte Chunks,
    /// erzeugt nur für diese Embeddings und schreibt sie in die Vector-DB (Upsert).
    pub async fn update_incremental(
        &self,
        document: Document,
        old_chunk_hashes: &[DocumentHash],
    ) -> Result<IncrementalUpdateResult, IncrementalUpdateError> {
        let chunks = self
            .chunker
            .chunk_document(&document.content)
            .await
            .map_err(|e| IncrementalUpdateError::Chunking(e.to_string()))?;

        let changed = self
            .change_detector
            .changed_chunk_indices(old_chunk_hashes, &chunks);

        if changed.is_empty() {
            return Ok(IncrementalUpdateResult { updated_count: 0 });
        }

        let texts: Vec<String> = changed.iter().map(|&i| chunks[i].clone()).collect();
        let embeddings = self
            .embedding_model
            .embed_batch(&texts)
            .await
            .map_err(|e| IncrementalUpdateError::Embedding(e.to_string()))?;

        for (k, &chunk_idx) in changed.iter().enumerate() {
            let chunk_doc = Document {
                id: format!("{}-chunk-{}", document.id, chunk_idx),
                content: chunks[chunk_idx].clone(),
                metadata: document.metadata.clone(),
            };
            self.indexer
                .index_document(chunk_doc, embeddings[k].clone())
                .await
                .map_err(|e| IncrementalUpdateError::Indexing(e.to_string()))?;
        }

        Ok(IncrementalUpdateResult {
            updated_count: changed.len(),
        })
    }
}

//! Query-Embedding-Generator (Phase 9.1.1): Query-Text zu Embedding, gleiches Modell wie für Dokumente.

use crate::embedding::{EmbeddingError, EmbeddingModel};
use std::sync::Arc;

/// Erzeugt Embeddings für Suchanfragen; nutzt dasselbe Modell wie die Dokument-Indizierung.
pub struct QueryEmbeddingGenerator {
    model: Arc<dyn EmbeddingModel>,
}

impl QueryEmbeddingGenerator {
    pub fn new(model: Arc<dyn EmbeddingModel>) -> Self {
        Self { model }
    }

    /// Query-Text zu Embedding konvertieren (gleiches Modell wie für Dokumente).
    pub async fn generate(&self, query: &str) -> Result<Vec<f32>, EmbeddingError> {
        self.model.embed_text(query).await
    }

    pub fn model_name(&self) -> &str {
        self.model.get_model_name()
    }

    pub fn vector_dimension(&self) -> u64 {
        self.model.get_vector_dimension()
    }
}

//! Test-Embedding-Generators (Phase 1.2.2): Mock-Embedding-Model für Unit- und Integrationstests ohne echten Modell-Service.

use freki::embedding::{EmbeddingError, EmbeddingModel};
use async_trait::async_trait;

/// Mock-Embedding-Model für Tests: liefert deterministische Vektoren fester Dimension (Standard 384).
pub struct TestEmbeddingModel {
    dimension: u64,
}

impl TestEmbeddingModel {
    pub fn new(dimension: u64) -> Self {
        Self { dimension }
    }

    /// Standard-Dimension 384 (z. B. all-MiniLM-L6-v2).
    pub fn default_dimension() -> Self {
        Self::new(384)
    }

    /// Erzeugt einen einfachen deterministischen Vektor aus Text (für stabile Tests).
    fn deterministic_vector(text: &str, dimension: u64) -> Vec<f32> {
        let mut v = vec![0.0f32; dimension as usize];
        for (i, b) in text.bytes().cycle().take(dimension as usize).enumerate() {
            v[i] = (b as f32) / 255.0;
        }
        v
    }
}

#[async_trait]
impl EmbeddingModel for TestEmbeddingModel {
    async fn embed_text(&self, text: &str) -> Result<Vec<f32>, EmbeddingError> {
        Ok(Self::deterministic_vector(text, self.dimension))
    }

    async fn embed_batch(&self, texts: &[String]) -> Result<Vec<Vec<f32>>, EmbeddingError> {
        let mut out = Vec::with_capacity(texts.len());
        for t in texts {
            out.push(Self::deterministic_vector(t, self.dimension));
        }
        Ok(out)
    }

    fn get_model_name(&self) -> &str {
        "test-embedding-model"
    }

    fn get_vector_dimension(&self) -> u64 {
        self.dimension
    }
}

use crate::embedding::EmbeddingModel;
use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;
use thiserror::Error;
use tokio::sync::RwLock;

/// Schlüsseltyp für Embedding-Cache-Einträge.
///
/// Format (erste Iteration, ohne explizite Modellversion):
/// `{document_id}_{chunk_id}_{model_name}`
pub type CacheKey = String;

/// Erzeugt einen Cache-Schlüssel für einen Chunk.
pub fn generate_cache_key(document_id: &str, chunk_id: &str, model_name: &str) -> CacheKey {
    format!("{}_{}_{}", document_id, chunk_id, model_name)
}

#[derive(Debug, Error)]
pub enum EmbeddingCacheError {
    #[error("cache error: {0}")]
    CacheError(String),
}

/// Abstrakte Schnittstelle für Embedding-Caches (In-Memory, Redis, …).
#[async_trait]
pub trait EmbeddingCache: Send + Sync {
    /// Liefert das Embedding zu einem Schlüssel oder `None`, falls nicht vorhanden.
    async fn get(&self, key: &CacheKey) -> Result<Option<Vec<f32>>, EmbeddingCacheError>;

    /// Speichert ein Embedding unter einem Schlüssel.
    async fn set(&self, key: &CacheKey, embedding: &[f32]) -> Result<(), EmbeddingCacheError>;

    /// Invalidiert alle Einträge für ein Dokument (document_id-Präfix).
    async fn invalidate_document(&self, document_id: &str) -> Result<(), EmbeddingCacheError>;

    /// Löscht alle Cache-Einträge.
    async fn clear(&self) -> Result<(), EmbeddingCacheError>;
}

/// Einfache In-Memory-Implementierung des Embedding-Caches.
///
/// Hinweis: Kein Eviction/TTL – für Tests und kleine Datensätze gedacht.
pub struct InMemoryEmbeddingCache {
    inner: Arc<RwLock<HashMap<CacheKey, Vec<f32>>>>,
}

impl InMemoryEmbeddingCache {
    /// Erstellt einen leeren In-Memory-Cache.
    pub fn new() -> Self {
        Self {
            inner: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

#[async_trait]
impl EmbeddingCache for InMemoryEmbeddingCache {
    async fn get(&self, key: &CacheKey) -> Result<Option<Vec<f32>>, EmbeddingCacheError> {
        let guard = self.inner.read().await;
        Ok(guard.get(key).cloned())
    }

    async fn set(&self, key: &CacheKey, embedding: &[f32]) -> Result<(), EmbeddingCacheError> {
        let mut guard = self.inner.write().await;
        guard.insert(key.clone(), embedding.to_vec());
        Ok(())
    }

    async fn invalidate_document(&self, document_id: &str) -> Result<(), EmbeddingCacheError> {
        let mut guard = self.inner.write().await;
        let prefix = format!("{}_", document_id);
        guard.retain(|k, _| !k.starts_with(&prefix));
        Ok(())
    }

    async fn clear(&self) -> Result<(), EmbeddingCacheError> {
        let mut guard = self.inner.write().await;
        guard.clear();
        Ok(())
    }
}

/// Hilfsstruktur, die Cache und Embedding-Model kombiniert.
///
/// Nutzt den Cache für vorhandene Embeddings und ruft das Modell nur für
/// fehlende Chunks auf. Erwartet, dass `chunks` in stabiler Reihenfolge
/// übergeben werden.
pub struct EmbeddingCacheHelper {
    cache: Arc<dyn EmbeddingCache>,
    model: Arc<dyn EmbeddingModel>,
}

impl EmbeddingCacheHelper {
    pub fn new(cache: Arc<dyn EmbeddingCache>, model: Arc<dyn EmbeddingModel>) -> Self {
        Self { cache, model }
    }

    /// Liefert Embeddings für alle Chunks, nutzt Cache-Hits und berechnet
    /// fehlende Embeddings über das Modell.
    pub async fn get_or_compute_embeddings(
        &self,
        document_id: &str,
        chunks: &[String],
    ) -> Result<Vec<Vec<f32>>, EmbeddingCacheError> {
        let model_name = self.model.get_model_name().to_string();

        // Zuerst alle Cache-Hits/Misses bestimmen.
        let mut result: Vec<Option<Vec<f32>>> = Vec::with_capacity(chunks.len());
        let mut missing_indices = Vec::new();

        for (idx, _chunk) in chunks.iter().enumerate() {
            let chunk_id = format!("chunk-{}", idx);
            let key = generate_cache_key(document_id, &chunk_id, &model_name);
            match self.cache.get(&key).await? {
                Some(embedding) => result.push(Some(embedding)),
                None => {
                    result.push(None);
                    missing_indices.push(idx);
                }
            }
        }

        if missing_indices.is_empty() {
            // Alle Embeddings waren im Cache.
            return Ok(result.into_iter().map(|e| e.unwrap()).collect());
        }

        // Fehlende Embeddings berechnen.
        let missing_texts: Vec<String> = missing_indices
            .iter()
            .map(|&i| chunks[i].clone())
            .collect();

        let computed = self
            .model
            .embed_batch(&missing_texts)
            .await
            .map_err(|e| EmbeddingCacheError::CacheError(e.to_string()))?;

        // Ergebnisse einsortieren und in den Cache schreiben.
        for (pos, &idx) in missing_indices.iter().enumerate() {
            let chunk_id = format!("chunk-{}", idx);
            let key = generate_cache_key(document_id, &chunk_id, &model_name);
            let embedding = computed[pos].clone();
            self.cache.set(&key, &embedding).await?;
            result[idx] = Some(embedding);
        }

        Ok(result
            .into_iter()
            .map(|e| e.expect("all entries must be filled"))
            .collect())
    }
}


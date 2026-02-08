//! Similarity-Search-Manager (Phase 9.1.2): Vector-Search mit Top-K und Threshold-Filtering.

use crate::retrieval::RetrievedDocument;
use crate::vector_db::VectorDbClient;

/// Vector-Search mit Top-K (limit) und optionalem Score-Threshold.
pub struct SimilaritySearchManager {
    vector_db: VectorDbClient,
    collection_name: String,
    /// Mindest-Score (Cosine-Similarity); Ergebnisse mit score < threshold werden ausgefiltert.
    score_threshold: f32,
}

impl SimilaritySearchManager {
    pub fn new(
        vector_db: VectorDbClient,
        collection_name: String,
        score_threshold: f32,
    ) -> Self {
        Self {
            vector_db,
            collection_name,
            score_threshold,
        }
    }

    /// Vector-Search: Top-K Retrieval, dann Filterung nach score >= threshold.
    pub async fn search(
        &self,
        query_embedding: Vec<f32>,
        limit: u64,
    ) -> Result<Vec<RetrievedDocument>, Box<dyn std::error::Error + Send + Sync>> {
        let results = self
            .vector_db
            .search(&self.collection_name, query_embedding, limit)
            .await
            .map_err(|e| e.to_string())?;

        let documents: Vec<RetrievedDocument> = results
            .into_iter()
            .filter(|p| p.score >= self.score_threshold)
            .map(|point| RetrievedDocument {
                id: point.id.to_string(),
                content: serde_json::from_value(
                    point
                        .payload
                        .get("content")
                        .cloned()
                        .unwrap_or(serde_json::Value::String(String::new())),
                )
                .unwrap_or_default(),
                metadata: point.payload,
                score: point.score,
            })
            .collect();

        Ok(documents)
    }
}

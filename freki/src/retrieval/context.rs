use crate::vector_db::VectorDbClient;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetrievedContext {
    pub documents: Vec<RetrievedDocument>,
    pub relevance_scores: Vec<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetrievedDocument {
    pub id: String,
    pub content: String,
    pub metadata: serde_json::Value,
    pub score: f32,
}

pub struct ContextRetriever {
    vector_db: VectorDbClient,
    collection_name: String,
}

impl ContextRetriever {
    pub fn new(vector_db: VectorDbClient, collection_name: String) -> Self {
        Self { vector_db, collection_name }
    }

    pub async fn retrieve(&self, query_embedding: Vec<f32>, limit: u64) -> Result<RetrievedContext, Box<dyn std::error::Error>> {
        let results = self.vector_db.search(&self.collection_name, query_embedding, limit).await?;
        
        let documents: Vec<RetrievedDocument> = results.into_iter().map(|point| {
            RetrievedDocument {
                id: point.id.to_string(),
                content: serde_json::from_value(point.payload.get("content").cloned().unwrap_or(serde_json::Value::String(String::new()))).unwrap_or_default(),
                metadata: point.payload,
                score: point.score,
            }
        }).collect();
        
        let scores: Vec<f32> = documents.iter().map(|d| d.score).collect();
        
        Ok(RetrievedContext {
            documents,
            relevance_scores: scores,
        })
    }
}

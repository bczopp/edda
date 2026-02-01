use qdrant_client::prelude::*;
use qdrant_client::qdrant::*;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum VectorDbError {
    #[error("Qdrant connection error: {0}")]
    ConnectionError(String),
    #[error("Collection error: {0}")]
    CollectionError(String),
    #[error("Vector operation error: {0}")]
    VectorError(String),
}

/// Client für Vector-Database-Operationen (Qdrant).
///
/// # Beispiel
///
/// ```no_run
/// # use freki::vector_db::VectorDbClient;
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let client = VectorDbClient::new("http://localhost:6333").await?;
/// client.create_collection("my_collection", 384).await?;
/// # Ok(())
/// # }
/// ```
#[derive(Clone)]
pub struct VectorDbClient {
    client: QdrantClient,
}

impl VectorDbClient {
    /// Erstellt einen neuen VectorDbClient für die angegebene Qdrant-URL.
    ///
    /// # Argumente
    ///
    /// * `url` - Qdrant-Server-URL (z. B. "http://localhost:6333")
    pub async fn new(url: &str) -> Result<Self, VectorDbError> {
        let client = QdrantClient::from_url(url)
            .build()
            .map_err(|e| VectorDbError::ConnectionError(format!("{}", e)))?;
        
        Ok(Self { client })
    }

    pub async fn create_collection(&self, collection_name: &str, vector_size: u64) -> Result<(), VectorDbError> {
        self.client
            .create_collection(&CreateCollection {
                collection_name: collection_name.to_string(),
                vectors_config: Some(VectorsConfig {
                    config: Some(Config::Params(VectorParams {
                        size: vector_size,
                        distance: Distance::Cosine as i32,
                        ..Default::default()
                    })),
                }),
                ..Default::default()
            })
            .await
            .map_err(|e| VectorDbError::CollectionError(format!("{}", e)))?;
        
        Ok(())
    }

    pub async fn upsert_points(&self, collection_name: &str, points: Vec<PointStruct>) -> Result<(), VectorDbError> {
        self.client
            .upsert_points(collection_name, None, points, None)
            .await
            .map_err(|e| VectorDbError::VectorError(format!("{}", e)))?;
        
        Ok(())
    }

    pub async fn search(&self, collection_name: &str, query_vector: Vec<f32>, limit: u64) -> Result<Vec<ScoredPoint>, VectorDbError> {
        let results = self.client
            .search_points(&SearchPoints {
                collection_name: collection_name.to_string(),
                vector: query_vector,
                limit,
                with_payload: Some(true.into()),
                ..Default::default()
            })
            .await
            .map_err(|e| VectorDbError::VectorError(format!("{}", e)))?;
        
        Ok(results.result)
    }

    pub async fn list_collections(&self) -> Result<Vec<String>, VectorDbError> {
        let response = self.client
            .list_collections()
            .await
            .map_err(|e| VectorDbError::CollectionError(format!("{}", e)))?;
        Ok(response.collections.into_iter().map(|c| c.name).collect())
    }

    pub async fn delete_collection(&self, collection_name: &str) -> Result<(), VectorDbError> {
        self.client
            .delete_collection(collection_name)
            .await
            .map_err(|e| VectorDbError::CollectionError(format!("{}", e)))
    }

    /// Delete points by their IDs. Uses Qdrant's delete_points API.
    pub async fn delete_points(&self, collection_name: &str, point_ids: &[PointId]) -> Result<(), VectorDbError> {
        self.client
            .delete_points(collection_name, point_ids, None)
            .await
            .map_err(|e| VectorDbError::VectorError(format!("{}", e)))
    }

    /// Liefert Point-IDs aller Punkte mit payload.document_id == document_id (für Löschung).
    pub async fn scroll_points_by_document_id(
        &self,
        collection_name: &str,
        document_id: &str,
    ) -> Result<Vec<PointId>, VectorDbError> {
        use qdrant_client::qdrant::{Condition, Filter, ScrollPoints};
        let mut ids = Vec::new();
        let mut offset = None;
        let limit = 100u32;
        loop {
            let filter = Filter::must([Condition::matches("document_id", document_id.to_string())]);
            let req = ScrollPoints {
                collection_name: collection_name.to_string(),
                filter: Some(filter),
                limit: Some(limit),
                offset,
                with_payload: Some(false.into()),
                with_vectors: Some(false.into()),
                ..Default::default()
            };
            let result = self
                .client
                .scroll(&req)
                .await
                .map_err(|e| VectorDbError::VectorError(format!("{}", e)))?;
            let points = result.result;
            if points.is_empty() {
                break;
            }
            for p in &points {
                if let Some(ref id) = p.id {
                    ids.push(id.clone());
                }
            }
            offset = points.last().and_then(|p| p.id.clone());
            if (points.len() as u32) < limit {
                break;
            }
        }
        Ok(ids)
    }

    /// Scrollt alle Punkte (ohne Filter); liefert id, content, metadata (JSON-String) für Data-Export.
    pub async fn scroll_all(
        &self,
        collection_name: &str,
        max_points: u32,
    ) -> Result<Vec<(String, String, String)>, VectorDbError> {
        use qdrant_client::qdrant::ScrollPoints;
        let mut out = Vec::new();
        let mut offset = None;
        let limit = max_points.min(100).max(1);
        let mut total = 0u32;
        loop {
            let req = ScrollPoints {
                collection_name: collection_name.to_string(),
                filter: None,
                limit: Some(limit),
                offset,
                with_payload: Some(true.into()),
                with_vectors: Some(false.into()),
                ..Default::default()
            };
            let result = self
                .client
                .scroll(&req)
                .await
                .map_err(|e| VectorDbError::VectorError(format!("{}", e)))?;
            let points = result.result;
            if points.is_empty() {
                break;
            }
            for p in &points {
                let id = p.id.as_ref().map(|i| i.to_string()).unwrap_or_default();
                let content = p
                    .payload
                    .get("content")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string();
                let metadata = serde_json::to_string(&p.payload).unwrap_or_else(|_| "{}".to_string());
                out.push((id, content, metadata));
            }
            total += points.len() as u32;
            if total >= max_points || (points.len() as u32) < limit {
                break;
            }
            offset = points.last().and_then(|p| p.id.clone());
        }
        Ok(out)
    }
}

#[cfg(test)]
#[doc(hidden)]
pub fn point_for_test(id: &str, vector: Vec<f32>, content: &str) -> PointStruct {
    use std::collections::HashMap;
    let mut payload = HashMap::new();
    payload.insert("content".to_string(), serde_json::Value::String(content.to_string()));
    payload.insert("document_id".to_string(), serde_json::Value::String(id.to_string()));
    PointStruct::new(
        uuid::Uuid::parse_str(id).unwrap_or_else(|_| uuid::Uuid::new_v4()),
        vector,
        payload.into(),
    )
}

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

pub struct VectorDbClient {
    client: QdrantClient,
}

impl VectorDbClient {
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
}

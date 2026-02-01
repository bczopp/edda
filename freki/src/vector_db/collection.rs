//! Collection management (Phase 3.2): create, list, delete collections.
//! Thin wrapper over VectorDbClient.

use super::{VectorDbClient, VectorDbError};
use std::sync::Arc;

/// Manages vector collections: create, list, delete.
pub struct CollectionManager {
    client: Arc<VectorDbClient>,
}

impl CollectionManager {
    pub fn new(client: Arc<VectorDbClient>) -> Self {
        Self { client }
    }

    pub async fn create_collection(&self, name: &str, vector_size: u64) -> Result<(), VectorDbError> {
        self.client.create_collection(name, vector_size).await
    }

    pub async fn list_collections(&self) -> Result<Vec<String>, VectorDbError> {
        self.client.list_collections().await
    }

    pub async fn delete_collection(&self, name: &str) -> Result<(), VectorDbError> {
        self.client.delete_collection(name).await
    }
}

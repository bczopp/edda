//! Data-Deletion-Manager (Phase 17.1.1): Sichere Datenlöschung, GDPR Right-to-Deletion.

use crate::vector_db::VectorDbClient;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DataDeletionError {
    #[error("Vector DB error: {0}")]
    VectorDb(#[from] crate::vector_db::VectorDbError),
}

/// Entfernt Dokumente aus dem Index (alle Chunks mit document_id im Payload).
pub struct DataDeletionManager {
    vector_db: VectorDbClient,
    collection_name: String,
}

impl DataDeletionManager {
    pub fn new(vector_db: VectorDbClient, collection_name: String) -> Self {
        Self {
            vector_db,
            collection_name,
        }
    }

    /// Dokument aus Index entfernen (alle Chunks mit document_id im Payload).
    /// Nutzt scroll_points_by_document_id, dann delete_points.
    pub async fn delete_document(&self, document_id: &str) -> Result<(), DataDeletionError> {
        let point_ids = self
            .vector_db
            .scroll_points_by_document_id(&self.collection_name, document_id)
            .await?;
        self.delete_document_by_point_ids(&point_ids).await
    }

    /// Dokument anhand der Punkt-IDs entfernen (z. B. wenn IDs bereits bekannt).
    pub async fn delete_document_by_point_ids(
        &self,
        point_ids: &[qdrant_client::qdrant::PointId],
    ) -> Result<(), DataDeletionError> {
        if point_ids.is_empty() {
            return Ok(());
        }
        self.vector_db
            .delete_points(&self.collection_name, point_ids)
            .await?;
        Ok(())
    }
}

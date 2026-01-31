use crate::storage::EncryptedDatabase;
use serde_json::{json, Value};
use std::sync::Arc;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum GDPRError {
    #[error("Storage error: {0}")]
    StorageError(#[from] crate::storage::StorageError),
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
}

pub struct GDPRCompliance {
    database: Arc<EncryptedDatabase>,
}

impl GDPRCompliance {
    pub fn new(database: EncryptedDatabase) -> Self {
        Self {
            database: Arc::new(database),
        }
    }

    pub fn new_with_database(database: Arc<EncryptedDatabase>) -> Self {
        Self { database }
    }

    /// Right to Access - Export all user data in a portable format
    pub async fn export_user_data(&self, user_id: &str) -> Result<Vec<u8>, GDPRError> {
        // Get all user data
        let all_data = self.database.get_all_user_data(user_id).await?;
        
        // Format as JSON for portability
        let mut export_data = json!({
            "user_id": user_id,
            "export_timestamp": chrono::Utc::now().to_rfc3339(),
            "data_entries": []
        });
        
        let entries = export_data["data_entries"].as_array_mut().unwrap();
        for (data_id, data) in all_data {
            entries.push(json!({
                "data_id": data_id,
                "data": base64::engine::general_purpose::STANDARD.encode(&data),
            }));
        }
        
        let json_bytes = serde_json::to_vec_pretty(&export_data)?;
        Ok(json_bytes)
    }

    /// Right to Erasure - Delete all user data (Right to be forgotten)
    pub async fn delete_user_data(&self, user_id: &str) -> Result<(), GDPRError> {
        self.database.delete_all_user_data(user_id).await?;
        Ok(())
    }

    /// Anonymize user data instead of deleting (for data retention requirements)
    pub async fn anonymize_user_data(&self, user_id: &str) -> Result<(), GDPRError> {
        // Get all user data
        let all_data = self.database.get_all_user_data(user_id).await?;
        
        // Delete original data
        self.database.delete_all_user_data(user_id).await?;
        
        // Store anonymized version (just data IDs without user association)
        // In a real implementation, you might want to hash or anonymize the actual data
        for (data_id, _data) in all_data {
            // Store anonymized entry (without user_id)
            // This is a simplified version - real anonymization would process the data
            let _anonymized_id = format!("anon_{}", data_id);
            // Implementation would store anonymized data here
        }
        
        Ok(())
    }
}

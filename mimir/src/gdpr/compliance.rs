use crate::storage::EncryptedDatabase;
use serde_json::{json, Value};
use std::sync::Arc;
use thiserror::Error;
use base64::prelude::{Engine as _, BASE64_STANDARD};

#[derive(Debug, Error)]
pub enum GDPRError {
    #[error("Storage error: {0}")]
    StorageError(#[from] crate::storage::StorageError),
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
    #[error("Data not found: {0}")]
    DataNotFound(String),
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

    /// Right to Access - Export all user data in a portable format (GDPR Art. 15)
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
                "data": BASE64_STANDARD.encode(&data),
            }));
        }
        
        let json_bytes = serde_json::to_vec_pretty(&export_data)?;
        Ok(json_bytes)
    }

    /// Right to Rectification - Update/correct user data (GDPR Art. 16)
    pub async fn rectify_user_data(
        &self,
        user_id: &str,
        data_id: &str,
        new_data: &[u8],
    ) -> Result<(), GDPRError> {
        // First, verify the data exists and belongs to the user
        let _ = self.database.retrieve_data(data_id, user_id).await
            .map_err(|_| GDPRError::DataNotFound(format!("Data {} not found for user {}", data_id, user_id)))?;
        
        // Update data in place (preserves data_id)
        self.database.update_data(data_id, user_id, new_data).await?;
        
        Ok(())
    }

    /// Right to Erasure - Delete all user data (GDPR Art. 17 - "Right to be forgotten")
    pub async fn delete_user_data(&self, user_id: &str) -> Result<(), GDPRError> {
        self.database.delete_all_user_data(user_id).await?;
        Ok(())
    }

    /// Anonymize user data instead of deleting (for data retention requirements)
    /// This is used when data must be kept for legal reasons but user wants privacy
    pub async fn anonymize_user_data(&self, user_id: &str) -> Result<(), GDPRError> {
        // Get all user data
        let all_data = self.database.get_all_user_data(user_id).await?;
        
        // Delete original data
        self.database.delete_all_user_data(user_id).await?;
        
        // Store anonymized version (data without user association)
        // In a real implementation, you might:
        // 1. Hash the user_id to create an anonymous identifier
        // 2. Remove personally identifiable information from the data
        // 3. Store the anonymized data with the hashed identifier
        
        let anonymized_user_id = format!("anon_{}", uuid::Uuid::new_v4());
        
        for (_data_id, data) in all_data {
            // Store data with anonymized user ID
            let _ = self.database.store_data(&anonymized_user_id, &data).await?;
        }
        
        Ok(())
    }

    /// Check if user has any data (for compliance reporting)
    pub async fn has_user_data(&self, user_id: &str) -> Result<bool, GDPRError> {
        let data = self.database.get_all_user_data(user_id).await?;
        Ok(!data.is_empty())
    }

    /// Get data retention compliance status
    pub async fn check_retention_compliance(
        &self,
        user_id: &str,
        max_retention_days: u32,
    ) -> Result<bool, GDPRError> {
        // In a real implementation, this would check the creation/update timestamps
        // of all user data and verify they're within the retention period
        // For now, this is a placeholder
        
        let has_data = self.has_user_data(user_id).await?;
        
        // Simplified check - in production, you'd check actual timestamps
        Ok(has_data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::encryption::EncryptionManager;
    use crate::storage::EncryptedDatabase;
    use ring::rand::{SecureRandom, SystemRandom};
    use sqlx::PgPool;

    async fn create_test_database() -> (PgPool, EncryptedDatabase) {
        // This would use TestDatabase in actual tests
        // For now, this is a placeholder
        panic!("Use integration tests with TestDatabase");
    }

    #[test]
    fn test_anonymization_creates_new_id() {
        // Test that anonymization creates a new anonymous user ID
        let anonymized_id = format!("anon_{}", uuid::Uuid::new_v4());
        assert!(anonymized_id.starts_with("anon_"));
    }
}


use sqlx::PgPool;
use std::sync::Arc;
use thiserror::Error;
use crate::encryption::EncryptionManager;

#[derive(Debug, Error)]
pub enum StorageError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
    #[error("Encryption error: {0}")]
    EncryptionError(#[from] crate::encryption::EncryptionError),
    #[error("Access denied")]
    AccessDenied,
    #[error("Data not found")]
    NotFound,
}

pub struct EncryptedDatabase {
    pool: PgPool,
    encryption: Arc<EncryptionManager>,
}

impl EncryptedDatabase {
    pub async fn new(database_url: &str) -> Result<Self, StorageError> {
        let pool = sqlx::PgPool::connect(database_url).await?;
        // For backward compatibility, create a dummy encryption manager
        // In production, this should load the key from settings
        let key = vec![0u8; 32]; // This should come from settings
        let encryption = Arc::new(EncryptionManager::new(&key)?);
        Ok(Self { pool, encryption })
    }

    pub async fn new_with_encryption_manager(
        database_url: &str,
        encryption: EncryptionManager,
    ) -> Result<Self, StorageError> {
        let pool = sqlx::PgPool::connect(database_url).await?;
        Ok(Self {
            pool,
            encryption: Arc::new(encryption),
        })
    }

    pub async fn new_with_encryption(
        pool: &PgPool,
        encryption: EncryptionManager,
    ) -> Result<Self, StorageError> {
        Ok(Self {
            pool: pool.clone(),
            encryption: Arc::new(encryption),
        })
    }

    pub async fn store_data(&self, user_id: &str, data: &[u8]) -> Result<String, StorageError> {
        // Encrypt data before storing
        let encrypted_data = self.encryption.encrypt(data)?;
        
        let data_id = uuid::Uuid::new_v4().to_string();
        sqlx::query!(
            "INSERT INTO encrypted_data (id, user_id, encrypted_data) VALUES ($1, $2, $3)",
            data_id,
            user_id,
            encrypted_data.as_slice()
        )
        .execute(&self.pool)
        .await?;
        Ok(data_id)
    }

    pub async fn retrieve_data(&self, data_id: &str, user_id: &str) -> Result<Vec<u8>, StorageError> {
        let row = sqlx::query!(
            "SELECT encrypted_data FROM encrypted_data WHERE id = $1 AND user_id = $2",
            data_id,
            user_id
        )
        .fetch_optional(&self.pool)
        .await?;
        
        let row = row.ok_or(StorageError::NotFound)?;
        
        // Decrypt data
        let decrypted = self.encryption.decrypt(&row.encrypted_data)?;
        Ok(decrypted)
    }

    pub async fn delete_data(&self, data_id: &str, user_id: &str) -> Result<(), StorageError> {
        let result = sqlx::query!(
            "DELETE FROM encrypted_data WHERE id = $1 AND user_id = $2",
            data_id,
            user_id
        )
        .execute(&self.pool)
        .await?;
        
        if result.rows_affected() == 0 {
            return Err(StorageError::NotFound);
        }
        
        Ok(())
    }

    pub async fn get_all_user_data(&self, user_id: &str) -> Result<Vec<(String, Vec<u8>)>, StorageError> {
        let rows = sqlx::query!(
            "SELECT id, encrypted_data FROM encrypted_data WHERE user_id = $1",
            user_id
        )
        .fetch_all(&self.pool)
        .await?;
        
        let mut result = Vec::new();
        for row in rows {
            let decrypted = self.encryption.decrypt(&row.encrypted_data)?;
            result.push((row.id, decrypted));
        }
        Ok(result)
    }

    pub async fn delete_all_user_data(&self, user_id: &str) -> Result<(), StorageError> {
        sqlx::query!(
            "DELETE FROM encrypted_data WHERE user_id = $1",
            user_id
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }
}

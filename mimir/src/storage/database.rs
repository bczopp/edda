use sqlx::{PgPool, postgres::PgPoolOptions};
use std::sync::Arc;
use thiserror::Error;
use crate::encryption::EncryptionManager;
use crate::access_control::{AccessControlManager, Action};
use crate::audit::{AuditLogManager, AuditEvent};
use crate::cache::CacheManager;
use crate::monitoring::PerformanceMonitor;

mod pool_stats;
pub use pool_stats::PoolStats;

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
    #[error("Access control error: {0}")]
    AccessControlError(#[from] crate::access_control::AccessControlError),
    #[error("Audit logging error: {0}")]
    AuditError(#[from] crate::audit::AuditLogError),
}

pub struct EncryptedDatabase {
    pool: PgPool,
    encryption: Arc<EncryptionManager>,
    access_control: Option<Arc<AccessControlManager>>,
    audit_logger: Option<Arc<AuditLogManager>>,
    cache: Option<Arc<CacheManager>>,
    performance_monitor: Option<Arc<PerformanceMonitor>>,
}

impl EncryptedDatabase {
    pub async fn new(database_url: &str) -> Result<Self, StorageError> {
        Self::new_with_config(database_url, None, None).await
    }

    pub async fn new_with_config(
        database_url: &str,
        max_connections: Option<u32>,
        min_connections: Option<u32>,
    ) -> Result<Self, StorageError> {
        let pool = if let (Some(max), Some(min)) = (max_connections, min_connections) {
            sqlx::postgres::PgPoolOptions::new()
                .max_connections(max)
                .min_connections(min)
                .connect(database_url)
                .await?
        } else {
            sqlx::PgPool::connect(database_url).await?
        };
        // For backward compatibility, create a dummy encryption manager
        // In production, this should load the key from settings
        let key = vec![0u8; 32]; // This should come from settings
        let encryption = Arc::new(EncryptionManager::new(&key)?);
        Ok(Self { 
            pool, 
            encryption,
            access_control: None,
            audit_logger: None,
            cache: None,
        })
    }

    pub async fn new_with_encryption_manager(
        database_url: &str,
        encryption: EncryptionManager,
    ) -> Result<Self, StorageError> {
        Self::new_with_encryption_manager_and_config(database_url, encryption, None, None).await
    }

    pub async fn new_with_encryption_manager_and_config(
        database_url: &str,
        encryption: EncryptionManager,
        max_connections: Option<u32>,
        min_connections: Option<u32>,
    ) -> Result<Self, StorageError> {
        let pool = if let (Some(max), Some(min)) = (max_connections, min_connections) {
            sqlx::postgres::PgPoolOptions::new()
                .max_connections(max)
                .min_connections(min)
                .connect(database_url)
                .await?
        } else {
            sqlx::PgPool::connect(database_url).await?
        };
        Ok(Self {
            pool,
            encryption: Arc::new(encryption),
            access_control: None,
            audit_logger: None,
            cache: None,
            performance_monitor: None,
        })
    }

    pub async fn new_with_encryption(
        pool: &PgPool,
        encryption: EncryptionManager,
    ) -> Result<Self, StorageError> {
        Ok(Self {
            pool: pool.clone(),
            encryption: Arc::new(encryption),
            access_control: None,
            audit_logger: None,
            cache: None,
        })
    }

    /// Get connection pool statistics
    pub fn get_pool_stats(&self) -> PoolStats {
        let stats = PoolStats {
            size: self.pool.size(),
            idle: self.pool.num_idle(),
            is_closed: self.pool.is_closed(),
        };
        
        // Update performance monitor with pool stats (async operation)
        if let Some(monitor) = &self.performance_monitor {
            let monitor_clone = Arc::clone(monitor);
            let size = stats.size;
            let idle = stats.idle;
            let is_closed = stats.is_closed;
            tokio::spawn(async move {
                monitor_clone.update_pool_stats(size, idle, is_closed).await;
            });
        }
        
        stats
    }

    pub async fn new_with_access_control_and_audit(
        pool: &PgPool,
        encryption: EncryptionManager,
        access_control: Arc<AccessControlManager>,
        audit_logger: Arc<AuditLogManager>,
    ) -> Result<Self, StorageError> {
        Ok(Self {
            pool: pool.clone(),
            encryption: Arc::new(encryption),
            access_control: Some(access_control),
            audit_logger: Some(audit_logger),
            cache: None,
            performance_monitor: None,
        })
    }

    pub async fn new_with_cache(
        pool: &PgPool,
        encryption: EncryptionManager,
        cache: Arc<CacheManager>,
    ) -> Result<Self, StorageError> {
        Ok(Self {
            pool: pool.clone(),
            encryption: Arc::new(encryption),
            access_control: None,
            audit_logger: None,
            cache: Some(cache),
            performance_monitor: None,
        })
    }

    pub async fn new_with_all_features(
        pool: &PgPool,
        encryption: EncryptionManager,
        access_control: Option<Arc<AccessControlManager>>,
        audit_logger: Option<Arc<AuditLogManager>>,
        cache: Option<Arc<CacheManager>>,
        performance_monitor: Option<Arc<PerformanceMonitor>>,
    ) -> Result<Self, StorageError> {
        Ok(Self {
            pool: pool.clone(),
            encryption: Arc::new(encryption),
            access_control,
            audit_logger,
            cache,
            performance_monitor,
        })
    }

    /// Check if access control is enabled
    pub fn has_access_control(&self) -> bool {
        self.access_control.is_some()
    }

    pub async fn store_data(&self, user_id: &str, data: &[u8]) -> Result<String, StorageError> {
        self.store_data_with_purpose(user_id, data, None, None).await
    }

    pub async fn store_data_with_purpose(
        &self,
        user_id: &str,
        data: &[u8],
        purpose: Option<&str>,
        expires_at: Option<chrono::DateTime<chrono::Utc>>,
    ) -> Result<String, StorageError> {
        let start = std::time::Instant::now();
        
        // Encrypt data before storing
        let encrypted_data = self.encryption.encrypt(data)?;
        
        let data_id = uuid::Uuid::new_v4().to_string();
        sqlx::query!(
            "INSERT INTO encrypted_data (id, user_id, encrypted_data, purpose, expires_at) VALUES ($1, $2, $3, $4, $5)",
            data_id,
            user_id,
            encrypted_data.as_slice(),
            purpose,
            expires_at
        )
        .execute(&self.pool)
        .await?;
        
        // Invalidate cache for this user (new data added)
        if let Some(cache) = &self.cache {
            cache.invalidate_user(user_id).await;
        }
        
        // Record write time
        if let Some(monitor) = &self.performance_monitor {
            monitor.record_write_time(start.elapsed()).await;
        }
        
        Ok(data_id)
    }

    pub async fn retrieve_data(&self, data_id: &str, user_id: &str) -> Result<Vec<u8>, StorageError> {
        let start = std::time::Instant::now();
        
        // Check cache first if enabled
        if let Some(cache) = &self.cache {
            let cache_key = format!("{}:{}", user_id, data_id);
            if let Some(cached_data) = cache.get(&cache_key).await {
                // Record query time (cache hit - very fast)
                if let Some(monitor) = &self.performance_monitor {
                    monitor.record_query_time(start.elapsed()).await;
                }
                return Ok(cached_data);
            }
        }
        
        let row = sqlx::query!(
            "SELECT encrypted_data, expires_at FROM encrypted_data WHERE id = $1 AND user_id = $2",
            data_id,
            user_id
        )
        .fetch_optional(&self.pool)
        .await?;
        
        let row = row.ok_or(StorageError::NotFound)?;
        
        // Check if data has expired
        if let Some(expires_at) = row.expires_at {
            if expires_at < chrono::Utc::now() {
                return Err(StorageError::NotFound); // Treat expired data as not found
            }
        }
        
        // Decrypt data
        let decrypted = self.encryption.decrypt(&row.encrypted_data)?;
        
        // Store in cache if enabled
        if let Some(cache) = &self.cache {
            let cache_key = format!("{}:{}", user_id, data_id);
            cache.set(cache_key, decrypted.clone()).await;
        }
        
        // Record query time
        if let Some(monitor) = &self.performance_monitor {
            monitor.record_query_time(start.elapsed()).await;
        }
        
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
        
        // Invalidate all cache entries for this user
        if let Some(cache) = &self.cache {
            cache.invalidate_user(user_id).await;
        }
        
        Ok(())
    }

    /// Update existing data (for Right to Rectification - GDPR Art. 16)
    pub async fn update_data(
        &self,
        data_id: &str,
        user_id: &str,
        new_data: &[u8],
    ) -> Result<(), StorageError> {
        // Encrypt new data before updating
        let encrypted_data = self.encryption.encrypt(new_data)?;
        
        let result = sqlx::query!(
            "UPDATE encrypted_data SET encrypted_data = $1, updated_at = NOW() WHERE id = $2 AND user_id = $3",
            encrypted_data.as_slice(),
            data_id,
            user_id
        )
        .execute(&self.pool)
        .await?;
        
        if result.rows_affected() == 0 {
            return Err(StorageError::NotFound);
        }
        
        // Invalidate cache for this specific entry
        if let Some(cache) = &self.cache {
            let cache_key = format!("{}:{}", user_id, data_id);
            cache.invalidate(&cache_key).await;
        }
        
        Ok(())
    }

    /// Store data with access control and audit logging
    pub async fn store_data_with_access_control(
        &self,
        user_id: &str,
        data: &[u8],
        data_owner_id: &str,
    ) -> Result<String, StorageError> {
        self.store_data_with_access_control_and_purpose(user_id, data, data_owner_id, None, None).await
    }

    /// Store data with access control, audit logging, and purpose/retention tracking
    pub async fn store_data_with_access_control_and_purpose(
        &self,
        user_id: &str,
        data: &[u8],
        data_owner_id: &str,
        purpose: Option<&str>,
        expires_at: Option<chrono::DateTime<chrono::Utc>>,
    ) -> Result<String, StorageError> {
        // Check access control if enabled
        if let Some(ac) = &self.access_control {
            ac.check_user_data_access(user_id, data_owner_id, Action::Create).await?;
        }

        // Store data with purpose and expiration
        let data_id = self.store_data_with_purpose(data_owner_id, data, purpose, expires_at).await?;

        // Log audit event if enabled
        if let Some(audit) = &self.audit_logger {
            audit.log_event(
                AuditEvent::DataStored,
                Some(user_id.to_string()),
                Some(data_id.clone()),
                serde_json::json!({
                    "data_owner_id": data_owner_id,
                    "size": data.len(),
                    "purpose": purpose,
                    "expires_at": expires_at.map(|dt| dt.to_rfc3339()),
                }),
            ).await?;
        }

        Ok(data_id)
    }

    /// Retrieve data with access control and audit logging
    pub async fn retrieve_data_with_access_control(
        &self,
        data_id: &str,
        user_id: &str,
        data_owner_id: &str,
    ) -> Result<Vec<u8>, StorageError> {
        self.retrieve_data_with_access_control_and_purpose(data_id, user_id, data_owner_id, None).await
    }

    /// Retrieve data with access control, audit logging, and purpose validation
    pub async fn retrieve_data_with_access_control_and_purpose(
        &self,
        data_id: &str,
        user_id: &str,
        data_owner_id: &str,
        required_purpose: Option<&str>,
    ) -> Result<Vec<u8>, StorageError> {
        // Check access control if enabled
        if let Some(ac) = &self.access_control {
            ac.check_user_data_access(user_id, data_owner_id, Action::Read).await?;
        }

        // Validate purpose if required
        if let Some(required) = required_purpose {
            let data_purpose = self.get_data_purpose(data_id, data_owner_id).await?;
            if data_purpose.as_ref().map(|p| p.as_str()) != Some(required) {
                return Err(StorageError::AccessDenied);
            }
        }

        // Retrieve data
        let data = self.retrieve_data(data_id, data_owner_id).await?;

        // Log audit event if enabled
        if let Some(audit) = &self.audit_logger {
            audit.log_event(
                AuditEvent::DataRetrieved,
                Some(user_id.to_string()),
                Some(data_id.to_string()),
                serde_json::json!({
                    "data_owner_id": data_owner_id,
                    "required_purpose": required_purpose,
                }),
            ).await?;
        }

        Ok(data)
    }

    /// Delete data with access control and audit logging
    pub async fn delete_data_with_access_control(
        &self,
        data_id: &str,
        user_id: &str,
        data_owner_id: &str,
    ) -> Result<(), StorageError> {
        // Check access control if enabled
        if let Some(ac) = &self.access_control {
            ac.check_user_data_access(user_id, data_owner_id, Action::Delete).await?;
        }

        // Delete data
        self.delete_data(data_id, data_owner_id).await?;

        // Log audit event if enabled
        if let Some(audit) = &self.audit_logger {
            audit.log_event(
                AuditEvent::DataDeleted,
                Some(user_id.to_string()),
                Some(data_id.to_string()),
                serde_json::json!({
                    "data_owner_id": data_owner_id,
                }),
            ).await?;
        }

        Ok(())
    }

    /// Update data with access control and audit logging (for Right to Rectification)
    pub async fn update_data_with_access_control(
        &self,
        data_id: &str,
        user_id: &str,
        data_owner_id: &str,
        new_data: &[u8],
    ) -> Result<(), StorageError> {
        // Check access control if enabled
        if let Some(ac) = &self.access_control {
            ac.check_user_data_access(user_id, data_owner_id, Action::Update).await?;
        }

        // Update data
        self.update_data(data_id, data_owner_id, new_data).await?;

        // Log audit event if enabled
        if let Some(audit) = &self.audit_logger {
            audit.log_event(
                AuditEvent::DataUpdated,
                Some(user_id.to_string()),
                Some(data_id.to_string()),
                serde_json::json!({
                    "data_owner_id": data_owner_id,
                    "size": new_data.len(),
                }),
            ).await?;
        }

        Ok(())
    }
}

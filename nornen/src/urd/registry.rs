use sqlx::PgPool;
use serde_json::Value;
use serde::{Serialize, Deserialize};
use thiserror::Error;
use chrono::Utc;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use crate::mimir_client::MimirClient;
use crate::mimir_client::MimirClientError;
use crate::cache::ProviderCache;
use crate::audit::{AuditLogger, AuditEvent};

#[derive(Debug, Error)]
pub enum ProviderRegistryError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
    #[error("Mimir error: {0}")]
    MimirError(#[from] MimirClientError),
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
    #[error("Provider already exists: {0}")]
    ProviderExists(String),
    #[error("Provider not found: {0}")]
    ProviderNotFound(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Provider {
    pub provider_id: String,
    pub name: String,
    pub capabilities: Vec<String>,
    pub endpoint: String,
    pub status: String,
    pub metadata: Value,
    pub created_at: chrono::DateTime<Utc>,
    pub updated_at: chrono::DateTime<Utc>,
}

const PROVIDER_INDEX_KEY: &str = "nornen:providers:all";
const PROVIDER_INDEX_ID_KEY: &str = "nornen:providers:index_id";
const NORNEN_USER_ID: &str = "nornen";

pub struct ProviderRegistry {
    pool: Option<PgPool>,
    mimir_client: Option<Arc<MimirClient>>,
    use_mimir: bool,
    // Track the current index data_id for Mimir (to enable RectifyUserData updates)
    index_data_id: Arc<RwLock<Option<String>>>,
    // Optional cache for automatic invalidation on updates
    cache: Option<Arc<ProviderCache>>,
    // Optional audit logger for security and compliance
    audit_logger: Option<Arc<dyn AuditLogger>>,
}

impl ProviderRegistry {
    /// Create ProviderRegistry with PostgreSQL (legacy)
    pub async fn new(pool: PgPool) -> Result<Self, ProviderRegistryError> {
        Ok(Self {
            pool: Some(pool),
            mimir_client: None,
            use_mimir: false,
            index_data_id: Arc::new(RwLock::new(None)),
            cache: None,
            audit_logger: None,
        })
    }

    /// Create ProviderRegistry with Mimir
    pub async fn new_with_mimir(mimir_client: Arc<MimirClient>) -> Result<Self, ProviderRegistryError> {
        Ok(Self {
            pool: None,
            mimir_client: Some(mimir_client),
            use_mimir: true,
            index_data_id: Arc::new(RwLock::new(None)),
            cache: None,
            audit_logger: None,
        })
    }

    /// Create ProviderRegistry with PostgreSQL and cache
    pub async fn new_with_cache(pool: PgPool, cache: Arc<ProviderCache>) -> Result<Self, ProviderRegistryError> {
        Ok(Self {
            pool: Some(pool),
            mimir_client: None,
            use_mimir: false,
            index_data_id: Arc::new(RwLock::new(None)),
            cache: Some(cache),
            audit_logger: None,
        })
    }

    /// Create ProviderRegistry with Mimir and cache
    pub async fn new_with_mimir_and_cache(mimir_client: Arc<MimirClient>, cache: Arc<ProviderCache>) -> Result<Self, ProviderRegistryError> {
        Ok(Self {
            pool: None,
            mimir_client: Some(mimir_client),
            use_mimir: true,
            index_data_id: Arc::new(RwLock::new(None)),
            cache: Some(cache),
            audit_logger: None,
        })
    }

    /// Set audit logger for security and compliance
    pub fn set_audit_logger(&mut self, logger: Arc<dyn AuditLogger>) {
        self.audit_logger = Some(logger);
    }

    /// Log audit event (non-blocking, errors are logged but don't fail the operation)
    async fn log_audit_event(&self, event: AuditEvent) {
        if let Some(logger) = &self.audit_logger {
            if let Err(e) = logger.log(event).await {
                tracing::warn!("Failed to log audit event: {}", e);
            }
        }
    }

    /// Invalidate cache after provider updates
    async fn invalidate_cache(&self) {
        if let Some(cache) = &self.cache {
            cache.invalidate_all().await;
        }
    }

    /// Load all providers from Mimir
    /// Uses ExportUserData to get all data, then finds the provider index entry
    async fn load_all_providers_from_mimir(&self) -> Result<Vec<Provider>, ProviderRegistryError> {
        let mimir = self.mimir_client.as_ref()
            .ok_or_else(|| ProviderRegistryError::MimirError(MimirClientError::ConnectionError("Mimir client not available".to_string())))?;
        
        // Export all user data
        let exported_data = mimir.export_user_data(NORNEN_USER_ID).await
            .map_err(|e| ProviderRegistryError::MimirError(e))?;
        
        // Parse exported data - Mimir returns JSON with structure:
        // {"user_id": "...", "export_timestamp": "...", "data_entries": [{"data_id": "...", "data": "base64..."}]}
        let exported_json: serde_json::Value = serde_json::from_slice(&exported_data)
            .map_err(|e| ProviderRegistryError::SerializationError(e))?;
        
        // Get data_entries array
        if let Some(data_entries) = exported_json.get("data_entries").and_then(|e| e.as_array()) {
            // Try to find the provider index entry by attempting to parse each entry as Provider array
            // We'll look for the largest valid Provider array (most likely to be our index)
            let mut best_providers: Option<Vec<Provider>> = None;
            let mut best_data_id: Option<String> = None;
            
            for entry in data_entries {
                if let (Some(data_base64), Some(data_id)) = (
                    entry.get("data").and_then(|d| d.as_str()),
                    entry.get("data_id").and_then(|id| id.as_str())
                ) {
                    // Decode base64
                    use base64::prelude::{Engine as _, BASE64_STANDARD};
                    if let Ok(data_bytes) = BASE64_STANDARD.decode(data_base64) {
                        // Try to parse as provider array
                        if let Ok(providers) = serde_json::from_slice::<Vec<Provider>>(&data_bytes) {
                            // Keep the largest Provider array (most likely to be our index)
                            if best_providers.is_none() || providers.len() > best_providers.as_ref().unwrap().len() {
                                best_providers = Some(providers);
                                best_data_id = Some(data_id.to_string());
                            }
                        }
                    }
                }
            }
            
            if let Some(providers) = best_providers {
                // Store the data_id for future updates using RectifyUserData
                if let Some(data_id) = best_data_id {
                    *self.index_data_id.write().await = Some(data_id);
                }
                return Ok(providers);
            }
        }
        
        // If we can't find the index, return empty list
        Ok(Vec::new())
    }

    /// Save all providers to Mimir
    /// Uses RectifyUserData if we have the data_id, otherwise stores new data
    async fn save_all_providers_to_mimir(&self, providers: &[Provider]) -> Result<(), ProviderRegistryError> {
        let mimir = self.mimir_client.as_ref()
            .ok_or_else(|| ProviderRegistryError::MimirError(MimirClientError::ConnectionError("Mimir client not available".to_string())))?;
        
        // Serialize providers to JSON
        let data = serde_json::to_vec(providers)?;
        
        // Check if we have a tracked data_id
        let index_data_id_guard = self.index_data_id.read().await;
        if let Some(data_id) = index_data_id_guard.as_ref() {
            // Use RectifyUserData to update existing entry
            drop(index_data_id_guard);
            match mimir.rectify_user_data(data_id, NORNEN_USER_ID, &data).await {
                Ok(new_data_id) => {
                    // Update tracked data_id (might be different after rectify)
                    *self.index_data_id.write().await = Some(new_data_id);
                    Ok(())
                }
                Err(MimirClientError::NotFound) => {
                    // Data was deleted or doesn't exist, store new
                    let new_data_id = mimir.store_data(NORNEN_USER_ID, &data).await?;
                    *self.index_data_id.write().await = Some(new_data_id);
                    Ok(())
                }
                Err(e) => Err(ProviderRegistryError::MimirError(e)),
            }
        } else {
            // No tracked data_id, store new data
            drop(index_data_id_guard);
            let new_data_id = mimir.store_data(NORNEN_USER_ID, &data).await?;
            *self.index_data_id.write().await = Some(new_data_id);
            Ok(())
        }
    }

    pub async fn register_provider(
        &self,
        provider_id: &str,
        name: &str,
        capabilities: &[String],
        endpoint: &str,
        metadata: &Value,
    ) -> Result<(), ProviderRegistryError> {
        if self.use_mimir {
            // Use Mimir
            let mut providers = self.load_all_providers_from_mimir().await?;
            
            // Check if provider already exists
            if providers.iter().any(|p| p.provider_id == provider_id) {
                return Err(ProviderRegistryError::ProviderExists(provider_id.to_string()));
            }
            
            // Create new provider
            let now = Utc::now();
            let provider = Provider {
                provider_id: provider_id.to_string(),
                name: name.to_string(),
                capabilities: capabilities.to_vec(),
                endpoint: endpoint.to_string(),
                status: "active".to_string(),
                metadata: metadata.clone(),
                created_at: now,
                updated_at: now,
            };
            
            providers.push(provider.clone());
            self.save_all_providers_to_mimir(&providers).await?;
            
            // Invalidate cache after registration
            self.invalidate_cache().await;
            
            // Log audit event
            self.log_audit_event(AuditEvent::new(
                "provider_registered",
                "provider",
                provider_id,
                serde_json::json!({
                    "name": name,
                    "endpoint": endpoint,
                    "capabilities": capabilities,
                    "metadata": metadata
                }),
            )).await;
            
            Ok(())
        } else {
            // Use PostgreSQL (legacy)
            let pool = self.pool.as_ref()
                .ok_or_else(|| ProviderRegistryError::DatabaseError(sqlx::Error::PoolClosed))?;
            
            // Check if provider already exists
            let exists = sqlx::query_scalar::<_, bool>(
                "SELECT EXISTS(SELECT 1 FROM providers WHERE provider_id = $1)"
            )
            .bind(provider_id)
            .fetch_one(pool)
            .await?;

            if exists {
                return Err(ProviderRegistryError::ProviderExists(provider_id.to_string()));
            }

            // Insert provider
            sqlx::query!(
                "INSERT INTO providers (provider_id, name, endpoint, status, metadata) VALUES ($1, $2, $3, $4, $5)",
                provider_id,
                name,
                endpoint,
                "active",
                metadata
            )
            .execute(pool)
            .await?;

            // Insert capabilities
            for capability in capabilities {
                sqlx::query!(
                    "INSERT INTO provider_capabilities (provider_id, capability) VALUES ($1, $2)",
                    provider_id,
                    capability
                )
                .execute(pool)
                .await?;
            }

            // Log audit event (legacy PostgreSQL audit logging)
            sqlx::query!(
                "INSERT INTO audit_logs (event_type, provider_id, details) VALUES ($1, $2, $3)",
                "provider_registered",
                provider_id,
                serde_json::json!({"name": name, "endpoint": endpoint})
            )
            .execute(pool)
            .await?;

            // Also log via audit logger if available
            self.log_audit_event(AuditEvent::new(
                "provider_registered",
                "provider",
                provider_id,
                serde_json::json!({
                    "name": name,
                    "endpoint": endpoint,
                    "capabilities": capabilities,
                    "metadata": metadata
                }),
            )).await;

            // Invalidate cache after registration
            self.invalidate_cache().await;

            Ok(())
        }
    }

    pub async fn update_provider(
        &self,
        provider_id: &str,
        name: Option<&str>,
        capabilities: Option<&[String]>,
        endpoint: Option<&str>,
        metadata: Option<&Value>,
    ) -> Result<(), ProviderRegistryError> {
        if self.use_mimir {
            // Use Mimir
            let mut providers = self.load_all_providers_from_mimir().await?;
            
            // Find provider
            let provider = providers.iter_mut()
                .find(|p| p.provider_id == provider_id)
                .ok_or_else(|| ProviderRegistryError::ProviderNotFound(provider_id.to_string()))?;
            
            // Update fields
            if let Some(name) = name {
                provider.name = name.to_string();
            }
            if let Some(endpoint) = endpoint {
                provider.endpoint = endpoint.to_string();
            }
            if let Some(metadata) = metadata {
                provider.metadata = metadata.clone();
            }
            if let Some(capabilities) = capabilities {
                provider.capabilities = capabilities.to_vec();
            }
            provider.updated_at = Utc::now();
            
            self.save_all_providers_to_mimir(&providers).await?;
            
            // Invalidate cache after update
            self.invalidate_cache().await;
            
            // Log audit event
            self.log_audit_event(AuditEvent::new(
                "provider_updated",
                "provider",
                provider_id,
                serde_json::json!({
                    "name": name,
                    "capabilities": capabilities,
                    "endpoint": endpoint,
                    "metadata": metadata
                }),
            )).await;
            
            Ok(())
        } else {
            // Use PostgreSQL (legacy)
            let pool = self.pool.as_ref()
                .ok_or_else(|| ProviderRegistryError::DatabaseError(sqlx::Error::PoolClosed))?;
            
            // Check if provider exists
            let exists = sqlx::query_scalar::<_, bool>(
                "SELECT EXISTS(SELECT 1 FROM providers WHERE provider_id = $1)"
            )
            .bind(provider_id)
            .fetch_one(pool)
            .await?;

            if !exists {
                return Err(ProviderRegistryError::ProviderNotFound(provider_id.to_string()));
            }

            // Update provider fields
            if let Some(name) = name {
                sqlx::query!(
                    "UPDATE providers SET name = $1, updated_at = CURRENT_TIMESTAMP WHERE provider_id = $2",
                    name,
                    provider_id
                )
                .execute(pool)
                .await?;
            }

            if let Some(endpoint) = endpoint {
                sqlx::query!(
                    "UPDATE providers SET endpoint = $1, updated_at = CURRENT_TIMESTAMP WHERE provider_id = $2",
                    endpoint,
                    provider_id
                )
                .execute(pool)
                .await?;
            }

            if let Some(metadata) = metadata {
                sqlx::query!(
                    "UPDATE providers SET metadata = $1, updated_at = CURRENT_TIMESTAMP WHERE provider_id = $2",
                    metadata,
                    provider_id
                )
                .execute(pool)
                .await?;
            }

            // Update capabilities if provided
            if let Some(capabilities) = capabilities {
                // Delete existing capabilities
                sqlx::query!(
                    "DELETE FROM provider_capabilities WHERE provider_id = $1",
                    provider_id
                )
                .execute(pool)
                .await?;

                // Insert new capabilities
                for capability in capabilities {
                    sqlx::query!(
                        "INSERT INTO provider_capabilities (provider_id, capability) VALUES ($1, $2)",
                        provider_id,
                        capability
                    )
                    .execute(pool)
                    .await?;
                }
            }

            // Invalidate cache after update
            self.invalidate_cache().await;
            
            // Log audit event
            self.log_audit_event(AuditEvent::new(
                "provider_updated",
                "provider",
                provider_id,
                serde_json::json!({
                    "name": name,
                    "capabilities": capabilities,
                    "endpoint": endpoint,
                    "metadata": metadata
                }),
            )).await;

            Ok(())
        }
    }

    pub async fn update_provider_status(
        &self,
        provider_id: &str,
        status: &str,
    ) -> Result<(), ProviderRegistryError> {
        if self.use_mimir {
            // Use Mimir
            let mut providers = self.load_all_providers_from_mimir().await?;
            
            // Find provider
            let provider = providers.iter_mut()
                .find(|p| p.provider_id == provider_id)
                .ok_or_else(|| ProviderRegistryError::ProviderNotFound(provider_id.to_string()))?;
            
            provider.status = status.to_string();
            provider.updated_at = Utc::now();
            
            self.save_all_providers_to_mimir(&providers).await?;
            
            // Invalidate cache after status update
            self.invalidate_cache().await;
            
            // Log audit event
            self.log_audit_event(AuditEvent::new(
                "provider_status_updated",
                "provider",
                provider_id,
                serde_json::json!({
                    "status": status
                }),
            )).await;
            
            Ok(())
        } else {
            // Use PostgreSQL (legacy)
            let pool = self.pool.as_ref()
                .ok_or_else(|| ProviderRegistryError::DatabaseError(sqlx::Error::PoolClosed))?;
            
            let rows_affected = sqlx::query!(
                "UPDATE providers SET status = $1, updated_at = CURRENT_TIMESTAMP WHERE provider_id = $2",
                status,
                provider_id
            )
            .execute(pool)
            .await?;

            if rows_affected.rows_affected() == 0 {
                return Err(ProviderRegistryError::ProviderNotFound(provider_id.to_string()));
            }

            // Invalidate cache after status update
            self.invalidate_cache().await;
            
            // Log audit event
            self.log_audit_event(AuditEvent::new(
                "provider_status_updated",
                "provider",
                provider_id,
                serde_json::json!({
                    "status": status
                }),
            )).await;

            Ok(())
        }
    }

    pub async fn query_providers(
        &self,
        required_capabilities: &[String],
        status: Option<&str>,
    ) -> Result<Vec<Provider>, ProviderRegistryError> {
        if self.use_mimir {
            // Use Mimir - load all providers and filter in memory
            let mut providers = self.load_all_providers_from_mimir().await?;
            
            // Filter by status if specified
            if let Some(status_filter) = status {
                providers.retain(|p| p.status == status_filter);
            }
            
            // Filter by capabilities if specified
            if !required_capabilities.is_empty() {
                providers.retain(|p| {
                    // Check if provider has all required capabilities
                    required_capabilities.iter().all(|req_cap| {
                        p.capabilities.contains(req_cap)
                    })
                });
            }
            
            Ok(providers)
        } else {
            // Use PostgreSQL (legacy)
            let pool = self.pool.as_ref()
                .ok_or_else(|| ProviderRegistryError::DatabaseError(sqlx::Error::PoolClosed))?;
            
            // First, get provider IDs that have all required capabilities
            let provider_ids: Vec<String> = if !required_capabilities.is_empty() {
                let capability_list: Vec<&str> = required_capabilities.iter().map(|s| s.as_str()).collect();
                sqlx::query_scalar::<_, String>(
                    r#"
                    SELECT provider_id FROM provider_capabilities 
                    WHERE capability = ANY($1::VARCHAR[])
                    GROUP BY provider_id 
                    HAVING COUNT(DISTINCT capability) = $2
                    "#,
                )
                .bind(&capability_list[..] as &[&str])
                .bind(required_capabilities.len() as i64)
                .fetch_all(pool)
                .await?
            } else {
                // Get all provider IDs
                sqlx::query_scalar::<_, String>("SELECT provider_id FROM providers")
                    .fetch_all(pool)
                    .await?
            };

            if provider_ids.is_empty() {
                return Ok(Vec::new());
            }

            // Now get full provider details
            let rows = if let Some(status) = status {
                let provider_id_refs: Vec<&str> = provider_ids.iter().map(|s| s.as_str()).collect();
                sqlx::query!(
                    r#"
                    SELECT provider_id, name, endpoint, status, metadata, created_at, updated_at
                    FROM providers
                    WHERE provider_id = ANY($1::VARCHAR[]) AND status = $2
                    "#,
                    &provider_id_refs[..] as &[&str],
                    status
                )
                .fetch_all(pool)
                .await?
            } else {
                let provider_id_refs: Vec<&str> = provider_ids.iter().map(|s| s.as_str()).collect();
                sqlx::query!(
                    r#"
                    SELECT provider_id, name, endpoint, status, metadata, created_at, updated_at
                    FROM providers
                    WHERE provider_id = ANY($1::VARCHAR[])
                    "#,
                    &provider_id_refs[..] as &[&str]
                )
                .fetch_all(pool)
                .await?
            };

            // Get capabilities for each provider
            let mut providers = Vec::new();
            for row in rows {
                let capabilities = sqlx::query_scalar::<_, String>(
                    "SELECT capability FROM provider_capabilities WHERE provider_id = $1"
                )
                .bind(&row.provider_id)
                .fetch_all(pool)
                .await?;

                providers.push(Provider {
                    provider_id: row.provider_id,
                    name: row.name,
                    capabilities,
                    endpoint: row.endpoint,
                    status: row.status,
                    metadata: row.metadata.unwrap_or(serde_json::json!({})),
                    created_at: row.created_at,
                    updated_at: row.updated_at,
                });
            }

            Ok(providers)
        }
    }

    pub async fn list_providers(
        &self,
        limit: i32,
        offset: i32,
    ) -> Result<ListProvidersResult, ProviderRegistryError> {
        if self.use_mimir {
            // Use Mimir - load all providers and paginate in memory
            let mut providers = self.load_all_providers_from_mimir().await?;
            
            // Sort by created_at DESC
            providers.sort_by(|a, b| b.created_at.cmp(&a.created_at));
            
            let total = providers.len() as i32;
            
            // Apply pagination
            let start = offset as usize;
            let end = (offset + limit) as usize;
            let providers = if start >= providers.len() {
                Vec::new()
            } else {
                providers[start..end.min(providers.len())].to_vec()
            };
            
            Ok(ListProvidersResult {
                providers,
                total,
            })
        } else {
            // Use PostgreSQL (legacy)
            let pool = self.pool.as_ref()
                .ok_or_else(|| ProviderRegistryError::DatabaseError(sqlx::Error::PoolClosed))?;
            
            // Get total count
            let total = sqlx::query_scalar::<_, i64>(
                "SELECT COUNT(*) FROM providers"
            )
            .fetch_one(pool)
            .await?;

            // Get providers with pagination
            let rows = sqlx::query!(
                r#"
                SELECT provider_id, name, endpoint, status, metadata, created_at, updated_at
                FROM providers
                ORDER BY created_at DESC
                LIMIT $1 OFFSET $2
                "#,
                limit,
                offset
            )
            .fetch_all(pool)
            .await?;

            let mut providers = Vec::new();
            for row in rows {
                let capabilities = sqlx::query_scalar::<_, String>(
                    "SELECT capability FROM provider_capabilities WHERE provider_id = $1"
                )
                .bind(&row.provider_id)
                .fetch_all(pool)
                .await?;

                providers.push(Provider {
                    provider_id: row.provider_id,
                    name: row.name,
                    capabilities,
                    endpoint: row.endpoint,
                    status: row.status,
                    metadata: row.metadata.unwrap_or(serde_json::json!({})),
                    created_at: row.created_at,
                    updated_at: row.updated_at,
                });
            }

            Ok(ListProvidersResult {
                providers,
                total: total as i32,
            })
        }
    }
}

pub struct ListProvidersResult {
    pub providers: Vec<Provider>,
    pub total: i32,
}

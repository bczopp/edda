use sqlx::PgPool;
use serde_json::Value;
use thiserror::Error;
use chrono::Utc;
use std::collections::HashMap;

#[derive(Debug, Error)]
pub enum ProviderRegistryError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
    #[error("Provider already exists: {0}")]
    ProviderExists(String),
    #[error("Provider not found: {0}")]
    ProviderNotFound(String),
}

#[derive(Debug, Clone)]
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

pub struct ProviderRegistry {
    pool: PgPool,
}

impl ProviderRegistry {
    pub async fn new(pool: PgPool) -> Result<Self, ProviderRegistryError> {
        Ok(Self { pool })
    }

    pub async fn register_provider(
        &self,
        provider_id: &str,
        name: &str,
        capabilities: &[String],
        endpoint: &str,
        metadata: &Value,
    ) -> Result<(), ProviderRegistryError> {
        // Check if provider already exists
        let exists = sqlx::query_scalar::<_, bool>(
            "SELECT EXISTS(SELECT 1 FROM providers WHERE provider_id = $1)"
        )
        .bind(provider_id)
        .fetch_one(&self.pool)
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
        .execute(&self.pool)
        .await?;

        // Insert capabilities
        for capability in capabilities {
            sqlx::query!(
                "INSERT INTO provider_capabilities (provider_id, capability) VALUES ($1, $2)",
                provider_id,
                capability
            )
            .execute(&self.pool)
            .await?;
        }

        // Log audit event
        sqlx::query!(
            "INSERT INTO audit_logs (event_type, provider_id, details) VALUES ($1, $2, $3)",
            "provider_registered",
            provider_id,
            serde_json::json!({"name": name, "endpoint": endpoint})
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn update_provider(
        &self,
        provider_id: &str,
        name: Option<&str>,
        capabilities: Option<&[String]>,
        endpoint: Option<&str>,
        metadata: Option<&Value>,
    ) -> Result<(), ProviderRegistryError> {
        // Check if provider exists
        let exists = sqlx::query_scalar::<_, bool>(
            "SELECT EXISTS(SELECT 1 FROM providers WHERE provider_id = $1)"
        )
        .bind(provider_id)
        .fetch_one(&self.pool)
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
            .execute(&self.pool)
            .await?;
        }

        if let Some(endpoint) = endpoint {
            sqlx::query!(
                "UPDATE providers SET endpoint = $1, updated_at = CURRENT_TIMESTAMP WHERE provider_id = $2",
                endpoint,
                provider_id
            )
            .execute(&self.pool)
            .await?;
        }

        if let Some(metadata) = metadata {
            sqlx::query!(
                "UPDATE providers SET metadata = $1, updated_at = CURRENT_TIMESTAMP WHERE provider_id = $2",
                metadata,
                provider_id
            )
            .execute(&self.pool)
            .await?;
        }

        // Update capabilities if provided
        if let Some(capabilities) = capabilities {
            // Delete existing capabilities
            sqlx::query!(
                "DELETE FROM provider_capabilities WHERE provider_id = $1",
                provider_id
            )
            .execute(&self.pool)
            .await?;

            // Insert new capabilities
            for capability in capabilities {
                sqlx::query!(
                    "INSERT INTO provider_capabilities (provider_id, capability) VALUES ($1, $2)",
                    provider_id,
                    capability
                )
                .execute(&self.pool)
                .await?;
            }
        }

        Ok(())
    }

    pub async fn update_provider_status(
        &self,
        provider_id: &str,
        status: &str,
    ) -> Result<(), ProviderRegistryError> {
        let rows_affected = sqlx::query!(
            "UPDATE providers SET status = $1, updated_at = CURRENT_TIMESTAMP WHERE provider_id = $2",
            status,
            provider_id
        )
        .execute(&self.pool)
        .await?;

        if rows_affected.rows_affected() == 0 {
            return Err(ProviderRegistryError::ProviderNotFound(provider_id.to_string()));
        }

        Ok(())
    }

    pub async fn query_providers(
        &self,
        required_capabilities: &[String],
        status: Option<&str>,
    ) -> Result<Vec<Provider>, ProviderRegistryError> {
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
            .fetch_all(&self.pool)
            .await?
        } else {
            // Get all provider IDs
            sqlx::query_scalar::<_, String>("SELECT provider_id FROM providers")
                .fetch_all(&self.pool)
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
            .fetch_all(&self.pool)
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
            .fetch_all(&self.pool)
            .await?
        };

        // Get capabilities for each provider
        let mut providers = Vec::new();
        for row in rows {
            let capabilities = sqlx::query_scalar::<_, String>(
                "SELECT capability FROM provider_capabilities WHERE provider_id = $1"
            )
            .bind(&row.provider_id)
            .fetch_all(&self.pool)
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

    pub async fn list_providers(
        &self,
        limit: i32,
        offset: i32,
    ) -> Result<ListProvidersResult, ProviderRegistryError> {
        // Get total count
        let total = sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM providers"
        )
        .fetch_one(&self.pool)
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
        .fetch_all(&self.pool)
        .await?;

        let mut providers = Vec::new();
        for row in rows {
            let capabilities = sqlx::query_scalar::<_, String>(
                "SELECT capability FROM provider_capabilities WHERE provider_id = $1"
            )
            .bind(&row.provider_id)
            .fetch_all(&self.pool)
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

pub struct ListProvidersResult {
    pub providers: Vec<Provider>,
    pub total: i32,
}

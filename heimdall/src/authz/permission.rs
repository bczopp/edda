use crate::utils::DeviceRepository;
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PermissionError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
    #[error("Permission not found")]
    NotFound,
}

pub struct PermissionManager {
    pool: PgPool,
    device_repo: Arc<DeviceRepository>,
    cache: Option<Arc<crate::utils::PermissionCheckCache>>,
}

impl PermissionManager {
    pub fn new(pool: PgPool, device_repo: Arc<DeviceRepository>) -> Self {
        Self { 
            pool, 
            device_repo,
            cache: None,
        }
    }

    pub fn with_cache(
        pool: PgPool, 
        device_repo: Arc<DeviceRepository>,
        cache: Arc<crate::utils::PermissionCheckCache>,
    ) -> Self {
        Self { 
            pool, 
            device_repo,
            cache: Some(cache),
        }
    }

    pub async fn check_permission(
        &self,
        device_id: &str,
        user_id: &Uuid,
        resource_type: &str,
        action: &str,
    ) -> Result<bool, PermissionError> {
        // Check cache first
        if let Some(ref cache) = self.cache {
            let cache_key = format!("{}:{}:{}:{}", device_id, user_id, resource_type, action);
            if let Some(cached_allowed) = cache.get(&cache_key).await {
                return Ok(cached_allowed);
            }
        }

        // Get device
        let device = self.device_repo
            .get_by_device_id(device_id)
            .await
            .map_err(|_| PermissionError::NotFound)?;

        // Check direct permissions
        let has_permission = sqlx::query_scalar::<_, bool>(
            r#"
            SELECT EXISTS(
                SELECT 1 FROM device_permissions dp
                JOIN permissions p ON dp.permission_id = p.id
                WHERE dp.device_id = $1
                AND (p.resource_type = $2 OR p.resource_type IS NULL)
                AND (p.action = $3 OR p.action IS NULL)
            )
            "#,
        )
        .bind(device.id)
        .bind(resource_type)
        .bind(action)
        .fetch_one(&self.pool)
        .await?;

        if has_permission {
            // Cache result
            if let Some(ref cache) = self.cache {
                let cache_key = format!("{}:{}:{}:{}", device_id, user_id, resource_type, action);
                cache.set(cache_key, true).await;
            }
            return Ok(true);
        }

        // Check role-based permissions
        let has_role_permission = sqlx::query_scalar::<_, bool>(
            r#"
            SELECT EXISTS(
                SELECT 1 FROM device_roles dr
                JOIN role_permissions rp ON dr.role_id = rp.role_id
                JOIN permissions p ON rp.permission_id = p.id
                WHERE dr.device_id = $1
                AND (p.resource_type = $2 OR p.resource_type IS NULL)
                AND (p.action = $3 OR p.action IS NULL)
            )
            "#,
        )
        .bind(device.id)
        .bind(resource_type)
        .bind(action)
        .fetch_one(&self.pool)
        .await?;

        // Cache result
        if let Some(ref cache) = self.cache {
            let cache_key = format!("{}:{}:{}:{}", device_id, user_id, resource_type, action);
            cache.set(cache_key, has_role_permission).await;
        }

        Ok(has_role_permission)
    }

    pub async fn check_role(
        &self,
        device_id: &str,
        role_name: &str,
    ) -> Result<bool, PermissionError> {
        let device = self.device_repo
            .get_by_device_id(device_id)
            .await
            .map_err(|_| PermissionError::NotFound)?;

        let has_role = sqlx::query_scalar::<_, bool>(
            r#"
            SELECT EXISTS(
                SELECT 1 FROM device_roles dr
                JOIN roles r ON dr.role_id = r.id
                WHERE dr.device_id = $1
                AND r.role_name = $2
            )
            "#,
        )
        .bind(device.id)
        .bind(role_name)
        .fetch_one(&self.pool)
        .await?;

        Ok(has_role)
    }

    pub async fn get_roles(&self, device_id: &str) -> Result<Vec<String>, PermissionError> {
        let device = self.device_repo
            .get_by_device_id(device_id)
            .await
            .map_err(|_| PermissionError::NotFound)?;

        let roles = sqlx::query_scalar::<_, String>(
            r#"
            SELECT r.role_name FROM device_roles dr
            JOIN roles r ON dr.role_id = r.id
            WHERE dr.device_id = $1
            "#,
        )
        .bind(device.id)
        .fetch_all(&self.pool)
        .await?;

        Ok(roles)
    }
}

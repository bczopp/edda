//! Role management: base roles, custom roles, hierarchy, inheritance.

use crate::utils::DeviceRepository;
use sqlx::{PgPool, Row};
use std::sync::Arc;
use uuid::Uuid;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RoleManagerError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    #[error("Device not found")]
    DeviceNotFound,
    #[error("Role not found: {0}")]
    RoleNotFound(String),
}

/// Manages roles: base (admin, user, guest), custom roles, hierarchy, inheritance.
pub struct RoleManager {
    pool: PgPool,
    device_repo: Arc<DeviceRepository>,
}

impl RoleManager {
    pub fn new(pool: PgPool, device_repo: Arc<DeviceRepository>) -> Self {
        Self { pool, device_repo }
    }

    /// Ensure base roles (admin, user, guest) exist; create with hierarchy guest < user < admin.
    pub async fn ensure_base_roles(&self) -> Result<(), RoleManagerError> {
        sqlx::query(
            r#"
            INSERT INTO roles (role_name, description) VALUES ('admin', 'Full access')
            ON CONFLICT (role_name) DO NOTHING
            "#,
        )
        .execute(&self.pool)
        .await?;

        sqlx::query(
            r#"
            INSERT INTO roles (role_name, description, parent_role_id)
            SELECT 'user', 'Standard user', id FROM roles WHERE role_name = 'admin' LIMIT 1
            ON CONFLICT (role_name) DO NOTHING
            "#,
        )
        .execute(&self.pool)
        .await?;

        sqlx::query(
            r#"
            INSERT INTO roles (role_name, description, parent_role_id)
            SELECT 'guest', 'Limited guest', id FROM roles WHERE role_name = 'user' LIMIT 1
            ON CONFLICT (role_name) DO NOTHING
            "#,
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /// Get role UUID by name.
    pub async fn get_role_id(&self, role_name: &str) -> Result<Option<Uuid>, RoleManagerError> {
        let row = sqlx::query_scalar::<_, Uuid>(
            "SELECT id FROM roles WHERE role_name = $1",
        )
        .bind(role_name)
        .fetch_optional(&self.pool)
        .await?;
        Ok(row)
    }

    /// Create a custom role with optional parent (for hierarchy).
    pub async fn create_role(
        &self,
        role_name: &str,
        parent_role_name: Option<&str>,
        description: Option<&str>,
    ) -> Result<Uuid, RoleManagerError> {
        let parent_id: Option<Uuid> = match parent_role_name {
            Some(name) => Some(self.get_role_id(name).await?.ok_or_else(|| RoleManagerError::RoleNotFound(name.to_string()))?),
            None => None,
        };

        let id = sqlx::query_scalar::<_, Uuid>(
            r#"
            INSERT INTO roles (role_name, description, parent_role_id)
            VALUES ($1, $2, $3)
            ON CONFLICT (role_name) DO UPDATE SET description = EXCLUDED.description, parent_role_id = EXCLUDED.parent_role_id
            RETURNING id
            "#,
        )
        .bind(role_name)
        .bind(description)
        .bind(parent_id)
        .fetch_one(&self.pool)
        .await?;
        Ok(id)
    }

    /// Assign role to device by device_id string.
    pub async fn assign_role_to_device(&self, device_id: &str, role_name: &str) -> Result<(), RoleManagerError> {
        let device = self.device_repo
            .get_by_device_id(device_id)
            .await
            .map_err(|_| RoleManagerError::DeviceNotFound)?;
        let role_id = self.get_role_id(role_name).await?
            .ok_or_else(|| RoleManagerError::RoleNotFound(role_name.to_string()))?;

        sqlx::query(
            r#"
            INSERT INTO device_roles (device_id, role_id)
            VALUES ($1, $2)
            ON CONFLICT (device_id, role_id) DO NOTHING
            "#,
        )
        .bind(device.id)
        .bind(role_id)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    /// Remove role from device.
    pub async fn remove_role_from_device(&self, device_id: &str, role_name: &str) -> Result<(), RoleManagerError> {
        let device = self.device_repo
            .get_by_device_id(device_id)
            .await
            .map_err(|_| RoleManagerError::DeviceNotFound)?;
        let role_id = self.get_role_id(role_name).await?
            .ok_or_else(|| RoleManagerError::RoleNotFound(role_name.to_string()))?;

        sqlx::query(
            "DELETE FROM device_roles WHERE device_id = $1 AND role_id = $2",
        )
        .bind(device.id)
        .bind(role_id)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    /// List role names for device.
    pub async fn get_roles_for_device(&self, device_id: &str) -> Result<Vec<String>, RoleManagerError> {
        let device = self.device_repo
            .get_by_device_id(device_id)
            .await
            .map_err(|_| RoleManagerError::DeviceNotFound)?;

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

    /// Return role_id and all ancestor role IDs (parent chain) for permission inheritance.
    pub async fn get_inherited_role_ids(&self, role_id: Uuid) -> Result<Vec<Uuid>, RoleManagerError> {
        let mut out = vec![role_id];
        let mut current = role_id;
        loop {
            let row = sqlx::query("SELECT parent_role_id FROM roles WHERE id = $1")
                .bind(current)
                .fetch_optional(&self.pool)
                .await?;
            let parent: Option<Uuid> = row.and_then(|r| r.get::<Option<Uuid>, _>("parent_role_id"));
            match parent {
                Some(p) => {
                    out.push(p);
                    current = p;
                }
                None => break,
            }
        }
        Ok(out)
    }
}

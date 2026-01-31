use crate::utils::DeviceRepository;
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;
use thiserror::Error;
use crate::utils::models::MeshDevice;

#[derive(Debug, Error)]
pub enum MeshRegistryError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
    #[error("Device not found")]
    DeviceNotFound,
    #[error("Mesh device already registered")]
    AlreadyRegistered,
    #[error("Not the owner of this device")]
    NotOwner,
    #[error("Invalid role: must be admin, user, or guest")]
    InvalidRole,
}

pub struct MeshDeviceRegistry {
    pub pool: PgPool,
    pub device_repo: Arc<DeviceRepository>,
}

impl MeshDeviceRegistry {
    pub fn new(pool: PgPool, device_repo: Arc<DeviceRepository>) -> Self {
        Self { pool, device_repo }
    }

    pub async fn register_device(
        &self,
        device_id: &str,
        _device_name: &str,
        _device_type: &str,
        mesh_public_key: &str,
        owner_user_id: Uuid,
    ) -> Result<(MeshDevice, bool), MeshRegistryError> {
        // Check if device exists in main registry
        let device = self.device_repo
            .get_by_device_id(device_id)
            .await
            .map_err(|_| MeshRegistryError::DeviceNotFound)?;

        // Check if already registered
        let existing = sqlx::query_as::<_, MeshDevice>(
            "SELECT * FROM mesh_devices WHERE device_id = $1",
        )
        .bind(device.id)
        .fetch_optional(&self.pool)
        .await?;

        if let Some(existing) = existing {
            return Ok((existing, false)); // Already registered
        }

        // Register new device
        let mesh_device = sqlx::query_as::<_, MeshDevice>(
            r#"
            INSERT INTO mesh_devices (device_id, mesh_public_key, role, owner_user_id)
            VALUES ($1, $2, $3, $4)
            RETURNING *
            "#,
        )
        .bind(device.id)
        .bind(mesh_public_key)
        .bind("user") // Default role
        .bind(owner_user_id)
        .fetch_one(&self.pool)
        .await?;

        Ok((mesh_device, true)) // New registration
    }

    pub async fn get_by_device_id(&self, device_id: &str) -> Result<MeshDevice, MeshRegistryError> {
        let device = self.device_repo
            .get_by_device_id(device_id)
            .await
            .map_err(|_| MeshRegistryError::DeviceNotFound)?;

        let mesh_device = sqlx::query_as::<_, MeshDevice>(
            "SELECT * FROM mesh_devices WHERE device_id = $1",
        )
        .bind(device.id)
        .fetch_optional(&self.pool)
        .await?
        .ok_or(MeshRegistryError::DeviceNotFound)?;

        Ok(mesh_device)
    }

    pub async fn update_last_seen(&self, device_id: &str) -> Result<(), MeshRegistryError> {
        let device = self.device_repo
            .get_by_device_id(device_id)
            .await
            .map_err(|_| MeshRegistryError::DeviceNotFound)?;

        sqlx::query(
            "UPDATE mesh_devices SET last_seen = CURRENT_TIMESTAMP WHERE device_id = $1",
        )
        .bind(device.id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    /// Owner autorisiert Device: setzt Role und aktiviert das Device.
    pub async fn update_role_and_approve(
        &self,
        device_id: &str,
        owner_user_id: Uuid,
        role: &str,
    ) -> Result<MeshDevice, MeshRegistryError> {
        let device = self.device_repo
            .get_by_device_id(device_id)
            .await
            .map_err(|_| MeshRegistryError::DeviceNotFound)?;

        let mesh_device = sqlx::query_as::<_, MeshDevice>(
            "SELECT * FROM mesh_devices WHERE device_id = $1",
        )
        .bind(device.id)
        .fetch_optional(&self.pool)
        .await?
        .ok_or(MeshRegistryError::DeviceNotFound)?;

        if mesh_device.owner_user_id != owner_user_id {
            return Err(MeshRegistryError::NotOwner);
        }

        let allowed_roles = ["admin", "user", "guest"];
        if !allowed_roles.contains(&role) {
            return Err(MeshRegistryError::InvalidRole);
        }

        let updated = sqlx::query_as::<_, MeshDevice>(
            "UPDATE mesh_devices SET role = $1, is_active = true WHERE device_id = $2 RETURNING *",
        )
        .bind(role)
        .bind(device.id)
        .fetch_one(&self.pool)
        .await?;

        Ok(updated)
    }

    /// Owner lehnt Device ab: setzt is_active = false.
    pub async fn reject_device(
        &self,
        device_id: &str,
        owner_user_id: Uuid,
    ) -> Result<(), MeshRegistryError> {
        let device = self.device_repo
            .get_by_device_id(device_id)
            .await
            .map_err(|_| MeshRegistryError::DeviceNotFound)?;

        let mesh_device = sqlx::query_as::<_, MeshDevice>(
            "SELECT * FROM mesh_devices WHERE device_id = $1",
        )
        .bind(device.id)
        .fetch_optional(&self.pool)
        .await?
        .ok_or(MeshRegistryError::DeviceNotFound)?;

        if mesh_device.owner_user_id != owner_user_id {
            return Err(MeshRegistryError::NotOwner);
        }

        sqlx::query("UPDATE mesh_devices SET is_active = false WHERE device_id = $1")
            .bind(device.id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}

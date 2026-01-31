use sqlx::PgPool;
use uuid::Uuid;
use crate::utils::models::Device;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DeviceRepositoryError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
    #[error("Device not found")]
    NotFound,
}

pub struct DeviceRepository {
    pool: PgPool,
}

impl DeviceRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(
        &self,
        device_id: &str,
        user_id: Uuid,
        public_key: &str,
        device_name: Option<&str>,
        device_type: Option<&str>,
    ) -> Result<Device, DeviceRepositoryError> {
        let device = sqlx::query_as::<_, Device>(
            r#"
            INSERT INTO devices (device_id, user_id, public_key, device_name, device_type)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING *
            "#,
        )
        .bind(device_id)
        .bind(user_id)
        .bind(public_key)
        .bind(device_name)
        .bind(device_type)
        .fetch_one(&self.pool)
        .await?;

        Ok(device)
    }

    pub async fn get_by_device_id(&self, device_id: &str) -> Result<Device, DeviceRepositoryError> {
        let device = sqlx::query_as::<_, Device>(
            "SELECT * FROM devices WHERE device_id = $1",
        )
        .bind(device_id)
        .fetch_optional(&self.pool)
        .await?
        .ok_or(DeviceRepositoryError::NotFound)?;

        Ok(device)
    }

    pub async fn get_by_id(&self, id: Uuid) -> Result<Device, DeviceRepositoryError> {
        let device = sqlx::query_as::<_, Device>(
            "SELECT * FROM devices WHERE id = $1",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?
        .ok_or(DeviceRepositoryError::NotFound)?;

        Ok(device)
    }

    pub async fn update(
        &self,
        id: Uuid,
        device_name: Option<&str>,
        device_type: Option<&str>,
        is_active: Option<bool>,
    ) -> Result<Device, DeviceRepositoryError> {
        let device = sqlx::query_as::<_, Device>(
            r#"
            UPDATE devices
            SET device_name = COALESCE($1, device_name),
                device_type = COALESCE($2, device_type),
                is_active = COALESCE($3, is_active),
                updated_at = CURRENT_TIMESTAMP
            WHERE id = $4
            RETURNING *
            "#,
        )
        .bind(device_name)
        .bind(device_type)
        .bind(is_active)
        .bind(id)
        .fetch_optional(&self.pool)
        .await?
        .ok_or(DeviceRepositoryError::NotFound)?;

        Ok(device)
    }

    pub async fn delete(&self, id: Uuid) -> Result<(), DeviceRepositoryError> {
        let result = sqlx::query("DELETE FROM devices WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;

        if result.rows_affected() == 0 {
            return Err(DeviceRepositoryError::NotFound);
        }

        Ok(())
    }

    pub async fn list_by_user_id(&self, user_id: Uuid) -> Result<Vec<Device>, DeviceRepositoryError> {
        let devices = sqlx::query_as::<_, Device>(
            "SELECT * FROM devices WHERE user_id = $1 ORDER BY created_at DESC",
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(devices)
    }
}

use crate::utils::DeviceRepository;
use sqlx::{PgPool, Row};
use std::sync::Arc;
use uuid::Uuid;
use chrono::{Utc, Duration};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum GuestNetworkError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
    #[error("Device not found")]
    DeviceNotFound,
    #[error("Network not found")]
    NetworkNotFound,
}

pub struct GuestNetworkManager {
    pool: PgPool,
    device_repo: Arc<DeviceRepository>,
}

impl GuestNetworkManager {
    pub fn new(pool: PgPool, device_repo: Arc<DeviceRepository>) -> Self {
        Self { pool, device_repo }
    }

    pub async fn create_guest_network(&self, owner_user_id: Uuid) -> Result<String, GuestNetworkError> {
        let network_id = uuid::Uuid::new_v4().to_string();
        let network_uuid = uuid::Uuid::new_v4();
        
        sqlx::query(
            r#"
            INSERT INTO guest_networks (id, network_id, owner_user_id, name, is_active, expires_at)
            VALUES ($1, $2, $3, $4, $5, $6)
            "#,
        )
        .bind(network_uuid)
        .bind(&network_id)
        .bind(owner_user_id)
        .bind(format!("Guest Network {}", network_id))
        .bind(true)
        .bind(Utc::now() + Duration::days(30))
        .execute(&self.pool)
        .await?;
        
        Ok(network_id)
    }

    pub async fn add_device_to_network(&self, network_id: &str, device_id: Uuid) -> Result<(), GuestNetworkError> {
        let row = sqlx::query("SELECT id FROM guest_networks WHERE network_id = $1")
            .bind(network_id)
            .fetch_optional(&self.pool)
            .await?
            .ok_or(GuestNetworkError::NetworkNotFound)?;
        let network_id_uuid: Uuid = row.get("id");

        sqlx::query(
            r#"
            INSERT INTO guest_network_devices (network_id, device_id)
            VALUES ($1, $2)
            ON CONFLICT (network_id, device_id) DO NOTHING
            "#,
        )
        .bind(network_id_uuid)
        .bind(device_id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn is_guest_device(&self, device_id: Uuid) -> Result<bool, GuestNetworkError> {
        let row = sqlx::query("SELECT COUNT(*) as count FROM guest_network_devices WHERE device_id = $1")
            .bind(device_id)
            .fetch_one(&self.pool)
            .await?;
        let count: i64 = row.get("count");
        Ok(count > 0)
    }

    pub async fn get_network_id_for_device(&self, device_id: Uuid) -> Result<Option<String>, GuestNetworkError> {
        let row = sqlx::query(
            r#"
            SELECT gn.network_id
            FROM guest_networks gn
            JOIN guest_network_devices gnd ON gn.id = gnd.network_id
            WHERE gnd.device_id = $1
            LIMIT 1
            "#,
        )
        .bind(device_id)
        .fetch_optional(&self.pool)
        .await?;
        Ok(row.map(|r| r.get::<String, _>("network_id")))
    }

    /// Whether two devices may communicate (user isolation and guest rules). Delegates to GuestNetworkIsolator.
    pub async fn can_communicate(&self, source_device_id: Uuid, target_device_id: Uuid) -> Result<bool, GuestNetworkError> {
        let isolator = GuestNetworkIsolator::new(self.pool.clone(), self.device_repo.clone());
        isolator.can_communicate(source_device_id, target_device_id).await
    }
}

pub struct GuestNetworkIsolator {
    pool: PgPool,
    device_repo: Arc<DeviceRepository>,
}

impl GuestNetworkIsolator {
    pub fn new(pool: PgPool, device_repo: Arc<DeviceRepository>) -> Self {
        Self {
            pool,
            device_repo,
        }
    }

    pub async fn can_communicate(&self, source_device_id: Uuid, target_device_id: Uuid) -> Result<bool, GuestNetworkError> {
        let source_device = self.device_repo
            .get_by_id(source_device_id)
            .await
            .map_err(|_| GuestNetworkError::DeviceNotFound)?;

        let target_device = self.device_repo
            .get_by_id(target_device_id)
            .await
            .map_err(|_| GuestNetworkError::DeviceNotFound)?;

        // Same user - always allow communication
        if source_device.user_id == target_device.user_id {
            return Ok(true);
        }

        let network_manager = GuestNetworkManager::new(self.pool.clone(), self.device_repo.clone());
        let source_is_guest = network_manager.is_guest_device(source_device_id).await?;
        let target_is_guest = network_manager.is_guest_device(target_device_id).await?;

        // Both are guests - allow if in same network
        if source_is_guest && target_is_guest {
            let source_network = network_manager.get_network_id_for_device(source_device_id).await?;
            let target_network = network_manager.get_network_id_for_device(target_device_id).await?;
            
            if let (Some(source_net), Some(target_net)) = (source_network, target_network) {
                return Ok(source_net == target_net);
            }
            return Ok(false);
        }

        // Guest cannot communicate with main network devices (different users)
        if source_is_guest || target_is_guest {
            return Ok(false);
        }

        // Both are main network devices - allow
        Ok(true)
    }
}

#[allow(dead_code)]
pub struct DataTransferPermissionManager {
    pool: PgPool,
    device_repo: Arc<DeviceRepository>,
}

impl DataTransferPermissionManager {
    pub fn new(pool: PgPool, device_repo: Arc<DeviceRepository>) -> Self {
        Self { pool, device_repo }
    }

    pub async fn grant_permission(
        &self,
        source_device_id: Uuid,
        target_device_id: Uuid,
        session_id: Option<Uuid>,
    ) -> Result<(), GuestNetworkError> {
        sqlx::query(
            r#"
            INSERT INTO data_transfer_permissions (source_device_id, target_device_id, session_id, granted_at, expires_at)
            VALUES ($1, $2, $3, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP + INTERVAL '24 hours')
            ON CONFLICT (source_device_id, target_device_id, session_id)
            DO UPDATE SET granted_at = CURRENT_TIMESTAMP, expires_at = CURRENT_TIMESTAMP + INTERVAL '24 hours'
            "#,
        )
        .bind(source_device_id)
        .bind(target_device_id)
        .bind(session_id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn has_permission(
        &self,
        source_device_id: Uuid,
        target_device_id: Uuid,
    ) -> Result<bool, GuestNetworkError> {
        let row = sqlx::query(
            r#"
            SELECT COUNT(*) as count
            FROM data_transfer_permissions
            WHERE source_device_id = $1 AND target_device_id = $2 AND expires_at > CURRENT_TIMESTAMP
            "#,
        )
        .bind(source_device_id)
        .bind(target_device_id)
        .fetch_one(&self.pool)
        .await?;
        let count: i64 = row.get("count");
        Ok(count > 0)
    }

    pub async fn revoke_permission(
        &self,
        source_device_id: Uuid,
        target_device_id: Uuid,
    ) -> Result<(), GuestNetworkError> {
        sqlx::query(
            "DELETE FROM data_transfer_permissions WHERE source_device_id = $1 AND target_device_id = $2",
        )
        .bind(source_device_id)
        .bind(target_device_id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}

#[allow(dead_code)]
pub struct ExplicitAccessManager {
    pool: PgPool,
    device_repo: Arc<DeviceRepository>,
}

impl ExplicitAccessManager {
    pub fn new(pool: PgPool, device_repo: Arc<DeviceRepository>) -> Self {
        Self { pool, device_repo }
    }

    pub async fn request_access(
        &self,
        guest_device_id: Uuid,
        main_device_id: Uuid,
    ) -> Result<(), GuestNetworkError> {
        sqlx::query(
            r#"
            INSERT INTO explicit_access_requests (guest_device_id, main_device_id, confirmation_count, requested_at)
            VALUES ($1, $2, 1, CURRENT_TIMESTAMP)
            ON CONFLICT (guest_device_id, main_device_id)
            DO UPDATE SET confirmation_count = explicit_access_requests.confirmation_count + 1, requested_at = CURRENT_TIMESTAMP
            "#,
        )
        .bind(guest_device_id)
        .bind(main_device_id)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn confirm_access(
        &self,
        guest_device_id: Uuid,
        main_device_id: Uuid,
    ) -> Result<(), GuestNetworkError> {
        sqlx::query(
            r#"
            UPDATE explicit_access_requests
            SET confirmation_count = confirmation_count + 1, last_confirmed_at = CURRENT_TIMESTAMP
            WHERE guest_device_id = $1 AND main_device_id = $2
            "#,
        )
        .bind(guest_device_id)
        .bind(main_device_id)
        .execute(&self.pool)
        .await?;

        let row = sqlx::query(
            "SELECT confirmation_count FROM explicit_access_requests WHERE guest_device_id = $1 AND main_device_id = $2",
        )
        .bind(guest_device_id)
        .bind(main_device_id)
        .fetch_optional(&self.pool)
        .await?;

        if let Some(r) = row {
            let confirmation_count: i32 = r.get("confirmation_count");
            if confirmation_count >= 3 {
                sqlx::query(
                    r#"
                    INSERT INTO explicit_access_grants (guest_device_id, main_device_id, granted_at, expires_at)
                    VALUES ($1, $2, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP + INTERVAL '24 hours')
                    ON CONFLICT (guest_device_id, main_device_id)
                    DO UPDATE SET granted_at = CURRENT_TIMESTAMP, expires_at = CURRENT_TIMESTAMP + INTERVAL '24 hours'
                    "#,
                )
                .bind(guest_device_id)
                .bind(main_device_id)
                .execute(&self.pool)
                .await?;
            }
        }

        Ok(())
    }

    pub async fn has_access(
        &self,
        guest_device_id: Uuid,
        main_device_id: Uuid,
    ) -> Result<bool, GuestNetworkError> {
        let row = sqlx::query(
            r#"
            SELECT COUNT(*) as count
            FROM explicit_access_grants
            WHERE guest_device_id = $1 AND main_device_id = $2 AND expires_at > CURRENT_TIMESTAMP
            "#,
        )
        .bind(guest_device_id)
        .bind(main_device_id)
        .fetch_one(&self.pool)
        .await?;
        let count: i64 = row.get("count");
        Ok(count > 0)
    }
}

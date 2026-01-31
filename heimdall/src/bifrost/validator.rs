use crate::utils::DeviceRepository;
use crate::authz::PermissionManager;
use crate::guest::GuestNetworkManager;
use std::sync::Arc;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConnectionValidationError {
    #[error("Device not found")]
    DeviceNotFound,
    #[error("Permission denied: {0}")]
    PermissionDenied(String),
    #[error("User isolation violation: {0}")]
    UserIsolationViolation(String),
    #[error("Invalid signature")]
    InvalidSignature,
}

pub struct ConnectionValidator {
    device_repo: Arc<DeviceRepository>,
    permission_manager: Arc<PermissionManager>,
    guest_network_manager: Option<Arc<GuestNetworkManager>>,
}

impl ConnectionValidator {
    pub fn new(
        device_repo: Arc<DeviceRepository>,
        permission_manager: Arc<PermissionManager>,
    ) -> Self {
        Self {
            device_repo,
            permission_manager,
            guest_network_manager: None,
        }
    }

    pub fn with_guest_network(
        device_repo: Arc<DeviceRepository>,
        permission_manager: Arc<PermissionManager>,
        guest_network_manager: Arc<GuestNetworkManager>,
    ) -> Self {
        Self {
            device_repo,
            permission_manager,
            guest_network_manager: Some(guest_network_manager),
        }
    }

    pub async fn validate_connection(
        &self,
        source_device_id: &str,
        target_device_id: &str,
        connection_type: &str,
    ) -> Result<bool, ConnectionValidationError> {
        // Get both devices
        let source_device = self.device_repo
            .get_by_device_id(source_device_id)
            .await
            .map_err(|_| ConnectionValidationError::DeviceNotFound)?;

        let target_device = self.device_repo
            .get_by_device_id(target_device_id)
            .await
            .map_err(|_| ConnectionValidationError::DeviceNotFound)?;

        // User isolation check
        if source_device.user_id != target_device.user_id {
            // Different users - check guest network rules
            if let Some(ref guest_manager) = self.guest_network_manager {
                let can_communicate = guest_manager
                    .can_communicate(source_device.id, target_device.id)
                    .await
                    .map_err(|_| ConnectionValidationError::DeviceNotFound)?;
                
                if !can_communicate {
                    return Err(ConnectionValidationError::UserIsolationViolation(
                        "Guest network isolation: devices cannot communicate".to_string(),
                    ));
                }
            } else {
                // No guest network manager - must go through Yggdrasil (RELAY)
                if connection_type == "DIRECT" {
                    return Err(ConnectionValidationError::UserIsolationViolation(
                        "Direct connections between different users are not allowed".to_string(),
                    ));
                }
                // RELAY connections are allowed (via Yggdrasil)
            }
        }

        // Permission check
        let has_permission = self.permission_manager
            .check_permission(
                source_device_id,
                &source_device.user_id,
                "connection",
                "establish",
            )
            .await
            .map_err(|_| ConnectionValidationError::PermissionDenied(
                "No permission to establish connection".to_string(),
            ))?;

        if !has_permission {
            return Err(ConnectionValidationError::PermissionDenied(
                "No permission to establish connection".to_string(),
            ));
        }

        Ok(true)
    }
}

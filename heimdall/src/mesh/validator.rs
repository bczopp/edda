//! Mesh-Token-Validator: validiert Mesh-Auth-Token, prüft Device-Status und Role.

use std::sync::Arc;
use thiserror::Error;

use crate::mesh::MeshDeviceRegistry;
use crate::token::TokenPayload;
use crate::token::{TokenValidationError, TokenValidator};

#[derive(Debug, Error)]
pub enum MeshTokenValidationError {
    #[error("Token validation failed: {0}")]
    Token(#[from] TokenValidationError),
    #[error("Device not in mesh registry")]
    DeviceNotInMesh,
    #[error("Mesh device is not active")]
    DeviceInactive,
}

/// Ergebnis einer erfolgreichen Mesh-Token-Validierung.
#[derive(Debug, Clone)]
pub struct MeshTokenValidationResult {
    pub device_id: String,
    pub user_id: String,
    pub role: String,
    pub expires_at: i64,
}

/// Validiert Mesh-Auth-Token und prüft Device-Status sowie Role.
pub struct MeshTokenValidator {
    token_validator: Arc<TokenValidator>,
    mesh_registry: Arc<MeshDeviceRegistry>,
}

impl MeshTokenValidator {
    pub fn new(
        token_validator: Arc<TokenValidator>,
        mesh_registry: Arc<MeshDeviceRegistry>,
    ) -> Self {
        Self {
            token_validator,
            mesh_registry,
        }
    }

    /// Validiert den Mesh-Token (Format, Signatur, Ablauf), prüft ob das Device
    /// in der Mesh-Registry ist und aktiv ist, und gibt Role zurück.
    pub async fn validate_mesh_token(
        &self,
        token: &str,
    ) -> Result<MeshTokenValidationResult, MeshTokenValidationError> {
        let payload: TokenPayload = self
            .token_validator
            .validate_token(token)
            .await
            .map_err(MeshTokenValidationError::Token)?;

        let mesh_device = self
            .mesh_registry
            .get_by_device_id(&payload.device_id)
            .await
            .map_err(|_| MeshTokenValidationError::DeviceNotInMesh)?;

        if !mesh_device.is_active {
            return Err(MeshTokenValidationError::DeviceInactive);
        }

        Ok(MeshTokenValidationResult {
            device_id: payload.device_id,
            user_id: payload.user_id,
            role: mesh_device.role,
            expires_at: payload.expires_at,
        })
    }
}

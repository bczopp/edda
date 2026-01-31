//! Owner-Authorization: Device durch Owner freigeben oder ablehnen.

use std::sync::Arc;
use uuid::Uuid;

use crate::mesh::{MeshDeviceRegistry, MeshRegistryError};
use crate::utils::models::MeshDevice;

/// Ergebnis von „Device-Details für Owner“ (z. B. für E-Mail oder UI).
#[derive(Debug, Clone)]
pub struct DeviceDetailsForOwner {
    pub device_id: String,
    pub device_name: Option<String>,
    pub device_type: Option<String>,
    pub mesh_device_id: Uuid,
    pub role: String,
    pub is_active: bool,
}

/// Manager für Owner-Authorization: Freigabe/Ablehnung von Mesh-Devices durch den Owner.
pub struct OwnerAuthorizationManager {
    mesh_registry: Arc<MeshDeviceRegistry>,
}

impl OwnerAuthorizationManager {
    pub fn new(mesh_registry: Arc<MeshDeviceRegistry>) -> Self {
        Self { mesh_registry }
    }

    /// Device-Details für Owner (z. B. für E-Mail-Benachrichtigung oder Rechte-Auswahl).
    pub async fn get_device_details_for_owner(
        &self,
        device_id: &str,
        owner_user_id: Uuid,
    ) -> Result<DeviceDetailsForOwner, MeshRegistryError> {
        let mesh_device = self.mesh_registry.get_by_device_id(device_id).await?;
        if mesh_device.owner_user_id != owner_user_id {
            return Err(MeshRegistryError::NotOwner);
        }
        let device = self
            .mesh_registry
            .device_repo
            .get_by_device_id(device_id)
            .await
            .map_err(|_| MeshRegistryError::DeviceNotFound)?;
        Ok(DeviceDetailsForOwner {
            device_id: device.device_id,
            device_name: device.device_name,
            device_type: device.device_type,
            mesh_device_id: mesh_device.id,
            role: mesh_device.role,
            is_active: mesh_device.is_active,
        })
    }

    /// Owner autorisiert Device mit gegebener Role (admin, user, guest).
    pub async fn approve_device(
        &self,
        owner_user_id: Uuid,
        device_id: &str,
        role: &str,
    ) -> Result<MeshDevice, MeshRegistryError> {
        self.mesh_registry
            .update_role_and_approve(device_id, owner_user_id, role)
            .await
    }

    /// Owner lehnt Device ab (is_active = false).
    pub async fn reject_device(
        &self,
        owner_user_id: Uuid,
        device_id: &str,
    ) -> Result<(), MeshRegistryError> {
        self.mesh_registry
            .reject_device(device_id, owner_user_id)
            .await
    }
}

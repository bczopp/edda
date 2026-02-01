//! Guest Mesh (Phase 12.1). Gast-Mesh-Isolation: isolierter Mesh-Segment **nur für fremde Devices** (Gäste/Besucher).
//! Eigene Devices eines Users nutzen das Main Mesh; Guest Mesh wird nur erstellt, wenn ein fremdes Device
//! (ohne User-Account auf dieser Instanz) verbindet – z. B. Besucher-Phone. Kein VPN.

pub mod cleanup;
pub mod data_transfer;
pub mod isolation;
pub mod permission;
pub mod user_confirmation;

pub use cleanup::GuestMeshCleanupManager;
pub use data_transfer::{
    DataTransferRequest, DataTransferRequestHandler, DataTransferResult,
    HeimdallConfirmationProvider, HeimdallConfirmationStub,
};
pub use isolation::MeshIsolationEnforcer;
pub use permission::{PermissionTokenManager, TokenInfo, TokenValidationError};
pub use user_confirmation::{
    ConfirmationChoice, ConfirmationError, ConfirmationOutcome, UserConfirmationManager,
    UserConfirmationRequest,
};

use std::collections::HashSet;
use std::sync::RwLock;
use uuid::Uuid;

/// Main mesh identifier (non-guest).
pub const MAIN_MESH_ID: &str = "main";

/// Backward-compat alias.
pub use MAIN_MESH_ID as MAIN_NETWORK_ID;

/// Opaque guest mesh ID for segmentation.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GuestMeshId(String);

impl GuestMeshId {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Backward-compat alias.
pub use GuestMeshId as GuestNetworkId;

/// Creates and tracks guest mesh segments; provides separate mesh IDs for isolation (no VPN).
pub struct GuestMeshManager {
    guest_ids: RwLock<HashSet<GuestMeshId>>,
}

/// Backward-compat alias.
pub use GuestMeshManager as GuestNetworkManager;

impl GuestMeshManager {
    pub fn new() -> Self {
        Self {
            guest_ids: RwLock::new(HashSet::new()),
        }
    }

    /// Creates a new guest mesh segment with a distinct ID and registers it for isolation.
    pub fn create_guest_mesh(&self) -> GuestMeshId {
        let id = GuestMeshId(format!("guest-{}", Uuid::new_v4()));
        self.guest_ids.write().unwrap().insert(id.clone());
        id
    }

    /// Backward-compat: same as create_guest_mesh.
    pub fn create_guest_network(&self) -> GuestMeshId {
        self.create_guest_mesh()
    }

    /// Returns true if the given ID is a known guest mesh segment.
    pub fn is_guest_mesh(&self, id: &str) -> bool {
        if id == MAIN_MESH_ID {
            return false;
        }
        let ids = self.guest_ids.read().unwrap();
        ids.contains(&GuestMeshId(id.to_string()))
    }

    /// Backward-compat: same as is_guest_mesh.
    pub fn is_guest_network(&self, id: &str) -> bool {
        self.is_guest_mesh(id)
    }

    /// Returns all created guest mesh segment IDs.
    pub fn list_guest_meshes(&self) -> Vec<GuestMeshId> {
        let ids = self.guest_ids.read().unwrap();
        ids.iter().cloned().collect()
    }

    /// Removes a guest mesh segment (resource release). No-op for MAIN_MESH_ID or unknown id.
    pub fn remove_guest_mesh(&self, id: &str) {
        if id == MAIN_MESH_ID {
            return;
        }
        self.guest_ids
            .write()
            .unwrap()
            .remove(&GuestMeshId(id.to_string()));
    }

    /// Backward-compat: same as list_guest_meshes.
    pub fn list_guest_networks(&self) -> Vec<GuestMeshId> {
        self.list_guest_meshes()
    }
}

impl Default for GuestMeshManager {
    fn default() -> Self {
        Self::new()
    }
}

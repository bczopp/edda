//! Mesh Isolation Enforcer (Phase 12.1.2). Routing-Regeln für Gast-Mesh: kein Flood in Haupt-Mesh, Blockierung Zugriff auf Haupt-Mesh.

use std::sync::Arc;

use super::{GuestMeshManager, MAIN_MESH_ID};

/// Enforces mesh segment isolation: guest cannot reach main, main cannot reach guest; same-segment allowed.
pub struct MeshIsolationEnforcer {
    guest_manager: Arc<GuestMeshManager>,
}

impl MeshIsolationEnforcer {
    pub fn new(guest_manager: Arc<GuestMeshManager>) -> Self {
        Self { guest_manager }
    }

    /// Returns true if a message from `from_mesh_id` may be delivered to `to_mesh_id`.
    /// Rules: main↔main allowed; guest→main and main→guest blocked; guest↔same guest allowed; guest↔other guest blocked.
    pub fn can_deliver(&self, from_mesh_id: &str, to_mesh_id: &str) -> bool {
        let from_guest = self.guest_manager.is_guest_mesh(from_mesh_id);
        let to_guest = self.guest_manager.is_guest_mesh(to_mesh_id);

        if !from_guest && !to_guest {
            return true;
        }
        if from_guest && to_mesh_id == MAIN_MESH_ID {
            return false;
        }
        if from_mesh_id == MAIN_MESH_ID && to_guest {
            return false;
        }
        if from_guest && to_guest {
            return from_mesh_id == to_mesh_id;
        }
        true
    }
}

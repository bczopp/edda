//! Mesh Connection Enforcer (Phase 11.2.1). Prüft Mesh-Membership bei Connection-Request; blockiert und sendet Fehler-Message wenn kein Mesh.

use std::fmt;

use crate::mesh::membership::MeshMembershipChecker;

/// Error when connection is denied by mesh enforcement.
#[derive(Debug)]
pub enum MeshEnforcerError {
    UserNotInMesh { user_id: String },
    DeviceNotInMesh { device_id: String },
    CheckFailed(Box<dyn std::error::Error + Send + Sync>),
}

impl fmt::Display for MeshEnforcerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MeshEnforcerError::UserNotInMesh { user_id } => write!(f, "user {} not in mesh", user_id),
            MeshEnforcerError::DeviceNotInMesh { device_id } => write!(f, "device {} not in mesh", device_id),
            MeshEnforcerError::CheckFailed(e) => write!(f, "mesh check failed: {}", e),
        }
    }
}

impl std::error::Error for MeshEnforcerError {}

impl MeshEnforcerError {
    /// Returns a short message suitable for sending to the client (e.g. in WebSocket close frame).
    pub fn client_message(&self) -> String {
        match self {
            MeshEnforcerError::UserNotInMesh { .. } | MeshEnforcerError::DeviceNotInMesh { .. } => {
                "MESH_ACCESS_DENIED".to_string()
            }
            MeshEnforcerError::CheckFailed(_) => "MESH_CHECK_ERROR".to_string(),
        }
    }
}

/// Enforces mesh membership on connection requests; blocks and returns error message when not in mesh.
pub struct MeshConnectionEnforcer {
    checker: MeshMembershipChecker,
}

impl MeshConnectionEnforcer {
    pub fn new(checker: MeshMembershipChecker) -> Self {
        Self { checker }
    }

    /// Returns Ok(()) if user and device are in mesh; Err with client_message otherwise.
    pub fn allow_connection(
        &self,
        user_id: &str,
        device_id: &str,
    ) -> Result<(), MeshEnforcerError> {
        if !self.checker.is_user_in_mesh(user_id).map_err(MeshEnforcerError::CheckFailed)? {
            return Err(MeshEnforcerError::UserNotInMesh {
                user_id: user_id.to_string(),
            });
        }
        if !self.checker.is_device_in_mesh(device_id).map_err(MeshEnforcerError::CheckFailed)? {
            return Err(MeshEnforcerError::DeviceNotInMesh {
                device_id: device_id.to_string(),
            });
        }
        Ok(())
    }
}

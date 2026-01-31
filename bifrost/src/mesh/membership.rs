//! Mesh-Membership-Checker (Phase 11.1.1). Prüft Mesh-Membership (Heimdall), Connectivity, User-Mesh-Zugehörigkeit.

use std::error::Error;
use std::io;
use std::sync::{Arc, RwLock};

/// Backend for membership checks (Heimdall gRPC later; stub for now).
pub trait MeshMembershipProvider: Send + Sync {
    fn is_user_in_mesh(&self, user_id: &str) -> Result<bool, Box<dyn Error + Send + Sync>>;
    fn is_device_in_mesh(&self, device_id: &str) -> Result<bool, Box<dyn Error + Send + Sync>>;
    fn is_mesh_connected(&self) -> Result<bool, Box<dyn Error + Send + Sync>>;
}

/// Stub provider for tests and until Heimdall is integrated (Phase 5).
/// Use `set_connected()` to simulate mesh failure/recovery in lifecycle tests.
/// Clone shares the same `connected` state (Arc) so tests can mutate and observe.
#[derive(Debug, Clone)]
pub struct MeshMembershipStub {
    user_allowed: bool,
    device_allowed: bool,
    connected: Arc<RwLock<bool>>,
}

impl MeshMembershipStub {
    pub fn all_allowed() -> Self {
        Self {
            user_allowed: true,
            device_allowed: true,
            connected: Arc::new(RwLock::new(true)),
        }
    }

    pub fn all_denied() -> Self {
        Self {
            user_allowed: false,
            device_allowed: false,
            connected: Arc::new(RwLock::new(false)),
        }
    }

    pub fn custom(user_allowed: bool, device_allowed: bool, connected: bool) -> Self {
        Self {
            user_allowed,
            device_allowed,
            connected: Arc::new(RwLock::new(connected)),
        }
    }

    /// Simulate mesh connectivity change (for lifecycle tests). Shared across clones.
    pub fn set_connected(&self, connected: bool) {
        let _ = self.connected.write().map(|mut g| *g = connected);
    }
}

impl MeshMembershipProvider for MeshMembershipStub {
    fn is_user_in_mesh(&self, _user_id: &str) -> Result<bool, Box<dyn Error + Send + Sync>> {
        Ok(self.user_allowed)
    }

    fn is_device_in_mesh(&self, _device_id: &str) -> Result<bool, Box<dyn Error + Send + Sync>> {
        Ok(self.device_allowed)
    }

    fn is_mesh_connected(&self) -> Result<bool, Box<dyn Error + Send + Sync>> {
        let g = self.connected.read().map_err(|_| io::Error::new(io::ErrorKind::Other, "stub lock"))?;
        Ok(*g)
    }
}

/// Checks mesh membership (user/device) and connectivity. Uses Heimdall when available.
pub struct MeshMembershipChecker {
    provider: Box<dyn MeshMembershipProvider>,
}

impl MeshMembershipChecker {
    pub fn new(provider: Box<dyn MeshMembershipProvider>) -> Self {
        Self { provider }
    }

    pub fn is_user_in_mesh(&self, user_id: &str) -> Result<bool, Box<dyn Error + Send + Sync>> {
        self.provider.is_user_in_mesh(user_id)
    }

    pub fn is_device_in_mesh(&self, device_id: &str) -> Result<bool, Box<dyn Error + Send + Sync>> {
        self.provider.is_device_in_mesh(device_id)
    }

    pub fn is_mesh_connected(&self) -> Result<bool, Box<dyn Error + Send + Sync>> {
        self.provider.is_mesh_connected()
    }
}

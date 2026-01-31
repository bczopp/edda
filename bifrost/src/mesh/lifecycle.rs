//! Mesh-based Connection Lifecycle (Phase 11.2.2). Bei Mesh-Ausfall Connections schließen oder alternative Hops; bei Wiederherstellung Wiederverbindung anregen.

use crate::mesh::status_monitor::{MeshStatusMonitor, MeshStatusSnapshot};

/// Action to take after a lifecycle tick (e.g. close connections on failure, trigger reconnect on recovery).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LifecycleAction {
    /// Mesh connectivity lost; caller may close connections or route over alternative hops.
    MeshFailure,
    /// Mesh connectivity restored; caller may trigger reconnection.
    MeshRecovery,
}

/// Manages connection lifecycle based on mesh status; call `tick()` periodically.
pub struct MeshConnectionLifecycleManager {
    monitor: MeshStatusMonitor,
}

impl MeshConnectionLifecycleManager {
    pub fn new(monitor: MeshStatusMonitor) -> Self {
        Self { monitor }
    }

    /// Performs one check; returns action if mesh failure or recovery was detected.
    pub fn tick(
        &self,
    ) -> Result<Option<LifecycleAction>, Box<dyn std::error::Error + Send + Sync>> {
        let snap: MeshStatusSnapshot = self.monitor.check()?;
        if snap.failure_detected {
            return Ok(Some(LifecycleAction::MeshFailure));
        }
        if snap.recovery_detected {
            return Ok(Some(LifecycleAction::MeshRecovery));
        }
        Ok(None)
    }
}

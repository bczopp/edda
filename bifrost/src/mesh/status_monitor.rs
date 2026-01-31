//! Mesh-Status-Monitor (Phase 11.1.2). Kontinuierliche Mesh-Connectivity-Überwachung, Ausfall/Wiederherstellung erkennen.

use std::cell::RefCell;

use crate::mesh::membership::MeshMembershipChecker;

/// Result of a single connectivity check.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MeshStatusSnapshot {
    pub connected: bool,
    pub failure_detected: bool,
    pub recovery_detected: bool,
}

/// Monitors mesh connectivity; call `check()` periodically to detect failure/recovery.
pub struct MeshStatusMonitor {
    checker: MeshMembershipChecker,
    last_connected: RefCell<Option<bool>>,
}

impl MeshStatusMonitor {
    pub fn new(checker: MeshMembershipChecker) -> Self {
        Self {
            checker,
            last_connected: RefCell::new(None),
        }
    }

    /// Performs one connectivity check; updates internal state and returns snapshot.
    pub fn check(&self) -> Result<MeshStatusSnapshot, Box<dyn std::error::Error + Send + Sync>> {
        let current = self.checker.is_mesh_connected()?;
        let mut last = self.last_connected.borrow_mut();
        let (failure_detected, recovery_detected) = match *last {
            Some(prev) => (prev && !current, !prev && current),
            None => (false, false),
        };
        *last = Some(current);
        Ok(MeshStatusSnapshot {
            connected: current,
            failure_detected,
            recovery_detected,
        })
    }

    /// Returns the last known connected state (None until first `check()`).
    pub fn is_connected(&self) -> bool {
        self.last_connected.borrow().unwrap_or(false)
    }
}

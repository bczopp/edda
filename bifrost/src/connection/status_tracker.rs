//! Connection Status Tracker (Phase 5.4.1). Track ACTIVE, IDLE, SUSPICIOUS, BLOCKED; process Heimdall updates; propagate to clients.

use std::collections::HashMap;
use std::sync::{mpsc, RwLock};

/// Connection status (from Heimdall or local).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectionStatus {
    Active,
    Idle,
    Suspicious,
    Blocked,
}

/// Tracks connection status per connection_id; processes status updates (e.g. from Heimdall); propagates changes to clients via optional channel.
pub struct ConnectionStatusTracker {
    statuses: RwLock<HashMap<String, ConnectionStatus>>,
    propagation_tx: Option<mpsc::Sender<(String, ConnectionStatus)>>,
}

impl ConnectionStatusTracker {
    /// Creates a tracker. If `propagation_tx` is set, status updates are sent to it for client propagation.
    pub fn new(propagation_tx: Option<mpsc::Sender<(String, ConnectionStatus)>>) -> Self {
        Self {
            statuses: RwLock::new(HashMap::new()),
            propagation_tx,
        }
    }

    /// Applies a status update (e.g. from Heimdall). Propagates to clients if channel is set.
    pub fn update_status(&self, connection_id: &str, status: ConnectionStatus) {
        self.statuses
            .write()
            .unwrap()
            .insert(connection_id.to_string(), status);
        if let Some(ref tx) = self.propagation_tx {
            let _ = tx.send((connection_id.to_string(), status));
        }
    }

    /// Returns the current status for the connection, or None if unknown.
    pub fn get_status(&self, connection_id: &str) -> Option<ConnectionStatus> {
        self.statuses.read().unwrap().get(connection_id).copied()
    }

    /// Removes the connection from tracking (e.g. on disconnect).
    pub fn remove(&self, connection_id: &str) {
        self.statuses.write().unwrap().remove(connection_id);
    }
}

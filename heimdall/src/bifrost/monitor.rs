//! Connection monitoring: status tracking (ACTIVE, IDLE, SUSPICIOUS, BLOCKED), heartbeat, message monitoring.

use chrono::Utc;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Connection status for monitoring.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectionStatus {
    Active,
    Idle,
    Suspicious,
    Blocked,
}

struct ConnectionState {
    status: ConnectionStatus,
    last_heartbeat_at: i64,
    last_message_at: i64,
}

fn connection_key(source: &str, target: &str) -> String {
    format!("{}:{}", source, target)
}

/// Tracks connection status, heartbeat validity, and message activity.
pub struct ConnectionMonitor {
    connections: Arc<RwLock<HashMap<String, ConnectionState>>>,
}

impl ConnectionMonitor {
    pub fn new() -> Self {
        Self {
            connections: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Register or ensure connection is tracked; initial status Active.
    pub async fn register_connection(&self, source_device_id: &str, target_device_id: &str) {
        let key = connection_key(source_device_id, target_device_id);
        let now = Utc::now().timestamp();
        let mut conn = self.connections.write().await;
        conn.insert(key, ConnectionState {
            status: ConnectionStatus::Active,
            last_heartbeat_at: now,
            last_message_at: now,
        });
    }

    /// Record heartbeat; updates last_heartbeat_at.
    pub async fn record_heartbeat(&self, source_device_id: &str, target_device_id: &str) {
        let key = connection_key(source_device_id, target_device_id);
        let now = Utc::now().timestamp();
        let mut conn = self.connections.write().await;
        if let Some(s) = conn.get_mut(&key) {
            s.last_heartbeat_at = now;
        }
    }

    /// Record message activity; updates last_message_at.
    pub async fn record_message(&self, source_device_id: &str, target_device_id: &str) {
        let key = connection_key(source_device_id, target_device_id);
        let now = Utc::now().timestamp();
        let mut conn = self.connections.write().await;
        if let Some(s) = conn.get_mut(&key) {
            s.last_message_at = now;
        }
    }

    /// Set connection status (e.g. BLOCKED, SUSPICIOUS).
    pub async fn set_status(
        &self,
        source_device_id: &str,
        target_device_id: &str,
        status: ConnectionStatus,
    ) {
        let key = connection_key(source_device_id, target_device_id);
        let mut conn = self.connections.write().await;
        if let Some(s) = conn.get_mut(&key) {
            s.status = status;
        }
    }

    /// Get current connection status; None if not registered.
    pub async fn get_status(
        &self,
        source_device_id: &str,
        target_device_id: &str,
    ) -> Option<ConnectionStatus> {
        let key = connection_key(source_device_id, target_device_id);
        let conn = self.connections.read().await;
        conn.get(&key).map(|s| s.status)
    }

    /// True if last heartbeat was within max_idle_seconds.
    pub async fn is_heartbeat_valid(
        &self,
        source_device_id: &str,
        target_device_id: &str,
        max_idle_seconds: i64,
    ) -> bool {
        let key = connection_key(source_device_id, target_device_id);
        let conn = self.connections.read().await;
        let now = Utc::now().timestamp();
        conn.get(&key)
            .map(|s| now - s.last_heartbeat_at <= max_idle_seconds)
            .unwrap_or(false)
    }
}

impl Default for ConnectionMonitor {
    fn default() -> Self {
        Self::new()
    }
}

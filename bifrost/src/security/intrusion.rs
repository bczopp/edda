//! Intrusion Detector (Phase 15.2.2). Attack patterns, security alerts, automatic connection blocking.

use std::collections::{HashMap, HashSet};
use std::sync::RwLock;

#[derive(Debug, Clone)]
pub enum IntrusionEvent {
    FailedAuth {
        connection_id: String,
        device_id: String,
    },
    InvalidMessage {
        connection_id: String,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AlertKind {
    RepeatedFailedAuth,
    InvalidMessageFlood,
}

/// Security alert for intrusion detection.
#[derive(Debug, Clone)]
pub struct SecurityAlert {
    pub kind: AlertKind,
    pub message: String,
    pub connection_id: Option<String>,
    pub device_id: Option<String>,
}

/// Detects attack patterns; emits security alerts; maintains blocklist for automatic blocking.
pub struct IntrusionDetector {
    /// Threshold: after this many failed auths from same connection, alert and block.
    failed_auth_threshold: u32,
    /// Threshold: after this many invalid messages from same connection, alert and block.
    invalid_message_threshold: u32,
    failed_auth_count: RwLock<HashMap<String, (u32, String)>>,
    invalid_message_count: RwLock<HashMap<String, u32>>,
    blocked_connections: RwLock<std::collections::HashSet<String>>,
    blocked_devices: RwLock<std::collections::HashSet<String>>,
}

impl IntrusionDetector {
    pub fn new(failed_auth_threshold: u32, invalid_message_threshold: u32) -> Self {
        Self {
            failed_auth_threshold: failed_auth_threshold.max(1),
            invalid_message_threshold: invalid_message_threshold.max(1),
            failed_auth_count: RwLock::new(HashMap::new()),
            invalid_message_count: RwLock::new(HashMap::new()),
            blocked_connections: RwLock::new(HashSet::new()),
            blocked_devices: RwLock::new(HashSet::new()),
        }
    }

    pub fn record(&self, event: IntrusionEvent) {
        match event {
            IntrusionEvent::FailedAuth {
                connection_id,
                device_id,
            } => {
                let mut map = self.failed_auth_count.write().unwrap();
                let entry = map
                    .entry(connection_id.clone())
                    .or_insert((0, device_id.clone()));
                entry.0 += 1;
                if entry.0 >= self.failed_auth_threshold {
                    self.blocked_connections.write().unwrap().insert(connection_id.clone());
                    self.blocked_devices.write().unwrap().insert(entry.1.clone());
                }
            }
            IntrusionEvent::InvalidMessage { connection_id } => {
                let mut map = self.invalid_message_count.write().unwrap();
                *map.entry(connection_id.clone()).or_insert(0) += 1;
                let count = *map.get(&connection_id).unwrap();
                if count >= self.invalid_message_threshold {
                    self.blocked_connections
                        .write()
                        .unwrap()
                        .insert(connection_id);
                }
            }
        }
    }

    pub fn check_alert(&self) -> Option<SecurityAlert> {
        let failed = self.failed_auth_count.read().unwrap();
        for (conn_id, (count, device_id)) in failed.iter() {
            if *count >= self.failed_auth_threshold {
                return Some(SecurityAlert {
                    kind: AlertKind::RepeatedFailedAuth,
                    message: format!(
                        "repeated failed auth: connection {} ({} attempts)",
                        conn_id, count
                    ),
                    connection_id: Some(conn_id.clone()),
                    device_id: Some(device_id.clone()),
                });
            }
        }
        let invalid = self.invalid_message_count.read().unwrap();
        for (conn_id, count) in invalid.iter() {
            if *count >= self.invalid_message_threshold {
                return Some(SecurityAlert {
                    kind: AlertKind::InvalidMessageFlood,
                    message: format!(
                        "invalid message flood: connection {} ({} messages)",
                        conn_id, count
                    ),
                    connection_id: Some(conn_id.clone()),
                    device_id: None,
                });
            }
        }
        None
    }

    pub fn should_block_connection(&self, connection_id: &str) -> bool {
        self.blocked_connections
            .read()
            .unwrap()
            .contains(connection_id)
    }

    pub fn should_block_device(&self, device_id: &str) -> bool {
        self.blocked_devices.read().unwrap().contains(device_id)
    }
}

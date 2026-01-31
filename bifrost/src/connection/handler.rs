//! Connection Handler (Phase 6.2.1). WebSocket connection state; message receive/send lifecycle.

use crate::message::BifrostMessage;
use std::sync::RwLock;
use std::time::Instant;

/// Connection lifecycle state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectionState {
    Connected,
    Disconnecting,
    Closed,
}

/// Manages one WebSocket connection: state, last activity; message receive/send are coordinated by server (MessageRouter).
pub struct ConnectionHandler {
    connection_id: String,
    device_id: String,
    user_id: String,
    state: RwLock<ConnectionState>,
    last_activity: RwLock<Option<Instant>>,
}

impl ConnectionHandler {
    pub fn new(connection_id: &str, device_id: &str, user_id: &str) -> Self {
        Self {
            connection_id: connection_id.to_string(),
            device_id: device_id.to_string(),
            user_id: user_id.to_string(),
            state: RwLock::new(ConnectionState::Connected),
            last_activity: RwLock::new(Some(Instant::now())),
        }
    }

    pub fn connection_id(&self) -> &str {
        &self.connection_id
    }

    pub fn device_id(&self) -> &str {
        &self.device_id
    }

    pub fn user_id(&self) -> &str {
        &self.user_id
    }

    pub fn get_state(&self) -> ConnectionState {
        *self.state.read().unwrap()
    }

    pub fn set_state(&self, state: ConnectionState) {
        *self.state.write().unwrap() = state;
    }

    pub fn is_connected(&self) -> bool {
        self.get_state() == ConnectionState::Connected
    }

    /// Called when a message is received (updates last activity).
    pub fn on_message_received(&self, _msg: &BifrostMessage) {
        *self.last_activity.write().unwrap() = Some(Instant::now());
    }

    /// Called when a message is sent (updates last activity). Server uses MessageRouter for actual send.
    pub fn on_message_sent(&self, _msg: &BifrostMessage) {
        *self.last_activity.write().unwrap() = Some(Instant::now());
    }

    pub fn last_activity(&self) -> Option<Instant> {
        *self.last_activity.read().unwrap()
    }
}

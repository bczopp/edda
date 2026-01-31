//! Tests for Phase 6.2.1: ConnectionHandler (state, receive/send facade).

use bifrost::connection::{ConnectionHandler, ConnectionState};
use bifrost::message::{BifrostMessage, MessageType};

fn sample_message() -> BifrostMessage {
    BifrostMessage {
        message_id: "m1".to_string(),
        message_type: MessageType::Message,
        source_device_id: "src".to_string(),
        target_device_id: "tgt".to_string(),
        payload: serde_json::json!({}),
        timestamp: 0,
        protocol_version: None,
    }
}

#[test]
fn new_handler_is_connected() {
    let handler = ConnectionHandler::new("conn-1", "device-1", "user-1");
    assert_eq!(handler.get_state(), ConnectionState::Connected);
    assert!(handler.is_connected());
}

#[test]
fn set_state_disconnecting() {
    let handler = ConnectionHandler::new("conn-1", "d1", "u1");
    handler.set_state(ConnectionState::Disconnecting);
    assert_eq!(handler.get_state(), ConnectionState::Disconnecting);
    assert!(!handler.is_connected());
}

#[test]
fn set_state_closed() {
    let handler = ConnectionHandler::new("conn-1", "d1", "u1");
    handler.set_state(ConnectionState::Closed);
    assert_eq!(handler.get_state(), ConnectionState::Closed);
    assert!(!handler.is_connected());
}

#[test]
fn connection_id_device_id_user_id() {
    let handler = ConnectionHandler::new("c1", "d1", "u1");
    assert_eq!(handler.connection_id(), "c1");
    assert_eq!(handler.device_id(), "d1");
    assert_eq!(handler.user_id(), "u1");
}

#[test]
fn message_received_updates_last_activity() {
    let handler = ConnectionHandler::new("c1", "d1", "u1");
    let msg = sample_message();
    handler.on_message_received(&msg);
    assert!(handler.last_activity().is_some());
}

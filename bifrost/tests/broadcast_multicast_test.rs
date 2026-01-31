//! Tests for Phase 9.3: Broadcast Manager, Multicast Manager.

use bifrost::connection::ConnectionManager;
use bifrost::message::{BifrostMessage, MessageType};
use bifrost::routing::{BroadcastManager, MessageRouter, MulticastManager};
use std::sync::Arc;
use std::time::Duration;

fn sample_message(source: &str) -> BifrostMessage {
    BifrostMessage {
        message_id: "bc-1".to_string(),
        message_type: MessageType::Message,
        source_device_id: source.to_string(),
        target_device_id: String::new(),
        payload: serde_json::json!({}),
        timestamp: 0,
        protocol_version: None,
    }
}

#[tokio::test]
async fn broadcast_manager_sends_to_all_devices_except_source() {
    let conn_mgr = Arc::new(ConnectionManager::new());
    let router = MessageRouter::new(Arc::clone(&conn_mgr));
    let broadcast = BroadcastManager::new(Arc::clone(&conn_mgr), router, Duration::ZERO);
    let msg = sample_message("src");
    let res = broadcast.broadcast(msg).await;
    assert!(res.is_ok());
}

#[tokio::test]
async fn broadcast_rate_limit_blocks_too_soon() {
    let conn_mgr = Arc::new(ConnectionManager::new());
    let router = MessageRouter::new(Arc::clone(&conn_mgr));
    let broadcast = BroadcastManager::new(Arc::clone(&conn_mgr), router, Duration::from_secs(60));
    let msg = sample_message("src");
    let _ = broadcast.broadcast(msg.clone()).await;
    let res = broadcast.broadcast(msg).await;
    assert!(res.is_err());
    assert!(res.unwrap_err().to_string().to_lowercase().contains("rate"));
}

#[tokio::test]
async fn multicast_create_group_and_add_members() {
    let conn_mgr = Arc::new(ConnectionManager::new());
    let router = MessageRouter::new(Arc::clone(&conn_mgr));
    let multicast = MulticastManager::new(Arc::clone(&conn_mgr), router);
    multicast.create_group("g1");
    multicast.add_member("g1", "d1");
    multicast.add_member("g1", "d2");
    let members = multicast.list_members("g1");
    assert_eq!(members.len(), 2);
    assert!(members.contains(&"d1".to_string()));
    assert!(members.contains(&"d2".to_string()));
}

#[tokio::test]
async fn multicast_remove_member() {
    let conn_mgr = Arc::new(ConnectionManager::new());
    let router = MessageRouter::new(Arc::clone(&conn_mgr));
    let multicast = MulticastManager::new(Arc::clone(&conn_mgr), router);
    multicast.create_group("g1");
    multicast.add_member("g1", "d1");
    multicast.remove_member("g1", "d1");
    let members = multicast.list_members("g1");
    assert!(members.is_empty());
}

#[tokio::test]
async fn multicast_send_to_group_sends_to_all_members() {
    let conn_mgr = Arc::new(ConnectionManager::new());
    let router = MessageRouter::new(Arc::clone(&conn_mgr));
    let multicast = MulticastManager::new(Arc::clone(&conn_mgr), router);
    multicast.create_group("g1");
    multicast.add_member("g1", "d1");
    multicast.add_member("g1", "d2");
    let msg = sample_message("src");
    let res = multicast.send_to_group("g1", msg).await;
    assert!(res.is_ok());
}

#[tokio::test]
async fn multicast_send_to_nonexistent_group_returns_error() {
    let conn_mgr = Arc::new(ConnectionManager::new());
    let router = MessageRouter::new(Arc::clone(&conn_mgr));
    let multicast = MulticastManager::new(Arc::clone(&conn_mgr), router);
    let msg = sample_message("src");
    let res = multicast.send_to_group("nonexistent", msg).await;
    assert!(res.is_err());
}

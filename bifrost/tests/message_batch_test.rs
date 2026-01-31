//! Tests for Phase 18.3.1: MessageBatchManager (batch, size limit, delivery).

use bifrost::connection::ConnectionManager;
use bifrost::message::{BifrostMessage, MessageType};
use bifrost::routing::MessageRouter;
use bifrost::routing::MessageBatchManager;
use std::sync::Arc;

fn sample_message(target: &str, id: &str) -> BifrostMessage {
    BifrostMessage {
        message_id: id.to_string(),
        message_type: MessageType::Message,
        source_device_id: "src".to_string(),
        target_device_id: target.to_string(),
        payload: serde_json::json!({}),
        timestamp: 0,
        protocol_version: None,
    }
}

#[test]
fn add_message_and_batch_len() {
    let mgr = MessageBatchManager::new(10);
    mgr.add(sample_message("d1", "m1"));
    mgr.add(sample_message("d1", "m2"));
    assert_eq!(mgr.pending_len(), 2);
}

#[test]
fn flush_clears_pending() {
    let mgr = MessageBatchManager::new(10);
    mgr.add(sample_message("d1", "m1"));
    let batch = mgr.take_batch();
    assert_eq!(batch.len(), 1);
    assert_eq!(mgr.pending_len(), 0);
}

#[test]
fn take_batch_respects_max_size() {
    let mgr = MessageBatchManager::new(2);
    mgr.add(sample_message("d1", "m1"));
    mgr.add(sample_message("d1", "m2"));
    mgr.add(sample_message("d1", "m3"));
    let batch = mgr.take_batch();
    assert_eq!(batch.len(), 2);
    assert_eq!(mgr.pending_len(), 1);
}

#[tokio::test]
async fn deliver_batch_sends_all() {
    let conn_mgr = Arc::new(ConnectionManager::new());
    let router = MessageRouter::new(Arc::clone(&conn_mgr));
    let mgr = MessageBatchManager::new(10);
    mgr.add(sample_message("d1", "m1"));
    mgr.add(sample_message("d1", "m2"));
    let (ok, failed) = mgr.deliver_batch(&router).await;
    assert_eq!(ok, 0);
    assert_eq!(failed, 2);
    assert_eq!(mgr.pending_len(), 0);
}

#[test]
fn take_batch_fifo_order() {
    let mgr = MessageBatchManager::new(10);
    mgr.add(sample_message("d1", "m1"));
    mgr.add(sample_message("d1", "m2"));
    let batch = mgr.take_batch();
    assert_eq!(batch[0].message_id, "m1");
    assert_eq!(batch[1].message_id, "m2");
}

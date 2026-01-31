//! Tests for Phase 16.1: Message Queue Manager, Queue Delivery Manager.

use bifrost::message::{BifrostMessage, MessageType};
use bifrost::queue::{MessageQueueManager, QueueDeliveryManager, QueueOverflowStrategy};
use bifrost::routing::MessageRouter;
use bifrost::connection::ConnectionManager;
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
fn enqueue_and_len() {
    let mgr = MessageQueueManager::new(10, QueueOverflowStrategy::EvictOldest);
    mgr.enqueue("device-1", sample_message("device-1", "m1")).unwrap();
    mgr.enqueue("device-1", sample_message("device-1", "m2")).unwrap();
    assert_eq!(mgr.queue_len("device-1"), 2);
}

#[test]
fn queue_limit_evicts_oldest() {
    let mgr = MessageQueueManager::new(2, QueueOverflowStrategy::EvictOldest);
    mgr.enqueue("d1", sample_message("d1", "m1")).unwrap();
    mgr.enqueue("d1", sample_message("d1", "m2")).unwrap();
    mgr.enqueue("d1", sample_message("d1", "m3")).unwrap();
    assert_eq!(mgr.queue_len("d1"), 2);
    let drained = mgr.drain("d1");
    assert_eq!(drained.len(), 2);
    assert_eq!(drained[0].message_id, "m2");
    assert_eq!(drained[1].message_id, "m3");
}

#[test]
fn queue_limit_reject_when_full() {
    let mgr = MessageQueueManager::new(2, QueueOverflowStrategy::Reject);
    mgr.enqueue("d1", sample_message("d1", "m1")).unwrap();
    mgr.enqueue("d1", sample_message("d1", "m2")).unwrap();
    let res = mgr.enqueue("d1", sample_message("d1", "m3"));
    assert!(res.is_err());
    assert_eq!(mgr.queue_len("d1"), 2);
}

#[test]
fn drain_fifo_order() {
    let mgr = MessageQueueManager::new(10, QueueOverflowStrategy::EvictOldest);
    mgr.enqueue("d1", sample_message("d1", "m1")).unwrap();
    mgr.enqueue("d1", sample_message("d1", "m2")).unwrap();
    let drained = mgr.drain("d1");
    assert_eq!(drained[0].message_id, "m1");
    assert_eq!(drained[1].message_id, "m2");
}

#[tokio::test]
async fn delivery_manager_drains_and_routes() {
    let conn_mgr = Arc::new(ConnectionManager::new());
    let queue_mgr = Arc::new(MessageQueueManager::new(10, QueueOverflowStrategy::EvictOldest));
    queue_mgr.enqueue("device-1", sample_message("device-1", "m1")).unwrap();
    let router = MessageRouter::new(Arc::clone(&conn_mgr));
    let delivery = QueueDeliveryManager::new(Arc::clone(&queue_mgr), router);
    let (delivered, failed) = delivery.deliver_to("device-1").await;
    assert_eq!(delivered, 0);
    assert_eq!(failed, 1);
    assert_eq!(queue_mgr.queue_len("device-1"), 0);
}

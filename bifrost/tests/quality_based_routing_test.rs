//! Tests for Phase 9.4.2: QualityBasedRouter (routing by quality, failover on degradation).

use bifrost::connection::ConnectionManager;
use bifrost::message::{BifrostMessage, MessageType};
use bifrost::routing::{
    ConnectionListProvider, ConnectionQualityMonitor, MessageRouter, QualityBasedRouter,
    QualityDegradedError, StubConnectionListProvider,
};
use std::sync::Arc;
use std::time::Duration;

fn sample_message(target: &str) -> BifrostMessage {
    BifrostMessage {
        message_id: "qbr-1".to_string(),
        message_type: MessageType::Message,
        source_device_id: "src".to_string(),
        target_device_id: target.to_string(),
        payload: serde_json::json!({}),
        timestamp: 0,
        protocol_version: None,
    }
}

#[tokio::test]
async fn routes_via_direct_when_no_connections_tries_router() {
    let conn_mgr = Arc::new(ConnectionManager::new());
    let monitor = Arc::new(ConnectionQualityMonitor::new(10, Duration::from_secs(1), 50));
    let router = MessageRouter::new(Arc::clone(&conn_mgr));
    let provider: Arc<dyn ConnectionListProvider> = Arc::clone(&conn_mgr) as Arc<dyn ConnectionListProvider>;
    let qbr = QualityBasedRouter::new(provider, router, Arc::clone(&monitor));
    let msg = sample_message("device-1");
    let res = qbr.route_message(msg).await;
    assert!(res.is_err());
}

#[tokio::test]
async fn returns_degraded_error_when_all_connections_degraded() {
    let stub = Arc::new(StubConnectionListProvider::new());
    stub.set_connections("device-1", vec!["conn-1".to_string()]);
    let monitor = Arc::new(ConnectionQualityMonitor::new(10, Duration::from_millis(10), 99));
    monitor.record_latency("conn-1", Duration::from_secs(1));
    monitor.record_failure("conn-1");
    let conn_mgr = Arc::new(ConnectionManager::new());
    let router = MessageRouter::new(conn_mgr);
    let qbr = QualityBasedRouter::new(stub, router, Arc::clone(&monitor));
    let msg = sample_message("device-1");
    let res = qbr.route_message(msg).await;
    assert!(res.is_err());
    assert!(res.unwrap_err().downcast_ref::<QualityDegradedError>().is_some());
}

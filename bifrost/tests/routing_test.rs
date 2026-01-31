//! Direct message routing tests (Phase 9.1, TDD).
//! Run in container: docker compose -f docker-compose.test.yml run --rm --no-deps bifrost-test

use bifrost::connection::ConnectionManager;
use bifrost::message::{BifrostMessage, MessageHandler, MessageType};
use bifrost::routing::{MessageRouter, RetryConfig};
use bifrost::websocket::WebSocketServer;
use futures_util::{SinkExt, StreamExt};
use std::sync::Arc;
use std::time::Duration;
use tokio::net::TcpListener;
use tokio_tungstenite::{connect_async, tungstenite::Message};

#[tokio::test]
async fn direct_routing_sends_message_to_target_device_connection() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let port = listener.local_addr().unwrap().port();
    let server = WebSocketServer::new(port);
    tokio::spawn(async move {
        let _ = server.run_listener(listener).await;
    });
    tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;

    let url = format!("ws://127.0.0.1:{}", port);
    let (mut client_a, _) = connect_async(&url).await.expect("client A connect");
    tokio::time::sleep(tokio::time::Duration::from_millis(80)).await;
    let (mut client_b, _) = connect_async(&url).await.expect("client B connect");
    tokio::time::sleep(tokio::time::Duration::from_millis(80)).await;

    let msg = BifrostMessage {
        message_id: "route-test-1".to_string(),
        message_type: MessageType::Message,
        source_device_id: "sender".to_string(),
        target_device_id: "unknown".to_string(),
        payload: serde_json::json!({"test": true}),
        timestamp: 12345,
        protocol_version: Some(1),
    };
    let json = MessageHandler::serialize_message(&msg).unwrap();
    client_a
        .send(Message::Text(json))
        .await
        .expect("client A send");

    let received = client_b.next().await.expect("client B receive");
    let text = match received.expect("frame") {
        Message::Text(t) => t,
        _ => panic!("expected text frame"),
    };
    let parsed = MessageHandler::parse_message(&text).expect("parse");
    assert_eq!(parsed.message_id, msg.message_id);
    assert_eq!(parsed.target_device_id, "unknown");
    assert_eq!(parsed.payload, msg.payload);

    client_a.close(None).await.ok();
    client_b.close(None).await.ok();
}

/// Phase 10.1.1: Without retry config, routing fails immediately when target is not connected.
#[tokio::test]
async fn route_message_fails_immediately_when_target_not_connected_and_no_retry() {
    let manager = Arc::new(ConnectionManager::new());
    let router = MessageRouter::new(manager);

    let msg = BifrostMessage {
        message_id: "no-retry-1".to_string(),
        message_type: MessageType::Message,
        source_device_id: "src".to_string(),
        target_device_id: "missing".to_string(),
        payload: serde_json::json!({}),
        timestamp: 0,
        protocol_version: None,
    };

    let res = router.route_message(msg).await;
    let err = res.unwrap_err();
    assert!(
        err.to_string().contains("not connected"),
        "expected 'not connected', got: {}",
        err
    );
}

/// Phase 10.1.1: With retry config, routing retries then returns Err after max_retries when target never connects.
#[tokio::test]
async fn route_message_retries_then_fails_after_max_retries_when_target_never_connected() {
    let manager = Arc::new(ConnectionManager::new());
    let config = RetryConfig {
        max_retries: 2,
        base_delay: Duration::from_millis(5),
    };
    let router = MessageRouter::new(manager).with_retry(config);

    let msg = BifrostMessage {
        message_id: "retry-1".to_string(),
        message_type: MessageType::Message,
        source_device_id: "src".to_string(),
        target_device_id: "missing".to_string(),
        payload: serde_json::json!({}),
        timestamp: 0,
        protocol_version: None,
    };

    let res = router.route_message(msg).await;
    let err = res.unwrap_err();
    assert!(
        err.to_string().contains("not connected"),
        "expected 'not connected', got: {}",
        err
    );
}

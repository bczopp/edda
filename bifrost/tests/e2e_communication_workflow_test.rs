//! End-to-End Communication Workflow Tests (Phase 20.1.1).
//! Covers: Device-Discovery → Connection-Establishment → Message-Exchange,
//! Direct-Routing, Relay-Routing (Asgard/Yggdrasil), Cross-Device-Actions (gRPC over Bifrost).
//! Run in container: docker compose -f docker-compose.test.yml run --rm --no-deps bifrost-test

use base64::Engine;
use bifrost::connection::ConnectionManager;
use bifrost::discovery::yggdrasil::{DeviceInfo, YggdrasilDiscoveryClient, YggdrasilDiscoveryStub};
use bifrost::grpc_bridge::GrpcBridge;
use bifrost::message::{BifrostMessage, MessageHandler, MessageType};
use bifrost::routing::{AsgardRelayClient, MessageRouter, RelayManager, YggdrasilRelayClient};
use bifrost::websocket::WebSocketServer;
use futures_util::{SinkExt, StreamExt};
use std::sync::Arc;
use std::time::Duration;
use tokio::net::TcpListener;
use tokio_tungstenite::{connect_async, tungstenite::Message};

// --- E2E: Device-Discovery → Connection-Establishment → Message-Exchange ---

#[tokio::test]
async fn e2e_discovery_connection_message_exchange() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let port = listener.local_addr().unwrap().port();
    let server = WebSocketServer::new(port);
    tokio::spawn(async move {
        let _ = server.run_listener(listener).await;
    });
    tokio::time::sleep(Duration::from_millis(50)).await;

    let ws_url = format!("ws://127.0.0.1:{}", port);
    let stub = YggdrasilDiscoveryStub::with_devices(vec![
        DeviceInfo {
            device_id: "device-a".to_string(),
            user_id: "user-1".to_string(),
            host: "127.0.0.1".to_string(),
            port,
        },
        DeviceInfo {
            device_id: "device-b".to_string(),
            user_id: "user-1".to_string(),
            host: "127.0.0.1".to_string(),
            port,
        },
    ]);
    let client = YggdrasilDiscoveryClient::new(Arc::new(stub));
    let devices = client.list_devices("user-1").await.unwrap();
    assert_eq!(devices.len(), 2);

    let url_a = devices[0].ws_url();
    let url_b = devices[1].ws_url();
    assert_eq!(url_a, ws_url);
    assert_eq!(url_b, ws_url);

    let (mut conn_a, _) = connect_async(&url_a).await.expect("client A connect");
    tokio::time::sleep(Duration::from_millis(80)).await;
    let (mut conn_b, _) = connect_async(&url_b).await.expect("client B connect");
    tokio::time::sleep(Duration::from_millis(80)).await;

    let msg = BifrostMessage {
        message_id: "e2e-discovery-1".to_string(),
        message_type: MessageType::Message,
        source_device_id: "device-a".to_string(),
        target_device_id: "unknown".to_string(),
        payload: serde_json::json!({"e2e": "discovery_connection_message"}),
        timestamp: 12345,
        protocol_version: Some(1),
    };
    let json = MessageHandler::serialize_message(&msg).unwrap();
    conn_a.send(Message::Text(json)).await.expect("A send");

    let received = conn_b.next().await.expect("B receive");
    let text = match received.expect("frame") {
        Message::Text(t) => t,
        _ => panic!("expected text frame"),
    };
    let parsed = MessageHandler::parse_message(&text).expect("parse");
    assert_eq!(parsed.message_id, msg.message_id);
    assert_eq!(parsed.payload, msg.payload);

    conn_a.close(None).await.ok();
    conn_b.close(None).await.ok();
}

// --- E2E: Direct-Routing (full workflow with real server) ---

#[tokio::test]
async fn e2e_direct_routing_workflow() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let port = listener.local_addr().unwrap().port();
    let server = WebSocketServer::new(port);
    tokio::spawn(async move {
        let _ = server.run_listener(listener).await;
    });
    tokio::time::sleep(Duration::from_millis(50)).await;

    let url = format!("ws://127.0.0.1:{}", port);
    let (mut client_a, _) = connect_async(&url).await.expect("client A connect");
    tokio::time::sleep(Duration::from_millis(80)).await;
    let (mut client_b, _) = connect_async(&url).await.expect("client B connect");
    tokio::time::sleep(Duration::from_millis(80)).await;

    let msg = BifrostMessage {
        message_id: "e2e-direct-1".to_string(),
        message_type: MessageType::Message,
        source_device_id: "sender".to_string(),
        target_device_id: "unknown".to_string(),
        payload: serde_json::json!({"e2e": "direct_routing"}),
        timestamp: 12345,
        protocol_version: Some(1),
    };
    let json = MessageHandler::serialize_message(&msg).unwrap();
    client_a.send(Message::Text(json)).await.expect("A send");

    let received = client_b.next().await.expect("B receive");
    let text = match received.expect("frame") {
        Message::Text(t) => t,
        _ => panic!("expected text frame"),
    };
    let parsed = MessageHandler::parse_message(&text).expect("parse");
    assert_eq!(parsed.message_id, msg.message_id);
    assert_eq!(parsed.payload, msg.payload);

    client_a.close(None).await.ok();
    client_b.close(None).await.ok();
}

// --- E2E: Relay-Routing (message sent via Asgard relay) ---

#[tokio::test]
async fn e2e_relay_routing_workflow() {
    let asgard = Arc::new(AsgardRelayClient::mock_success());
    let yggdrasil = Arc::new(YggdrasilRelayClient::unconfigured());
    let relay = RelayManager::new(Some(asgard), Some(yggdrasil));

    let msg = BifrostMessage {
        message_id: "e2e-relay-1".to_string(),
        message_type: MessageType::Message,
        source_device_id: "src".to_string(),
        target_device_id: "remote".to_string(),
        payload: serde_json::json!({"e2e": "relay_routing"}),
        timestamp: 0,
        protocol_version: None,
    };

    let res = relay.route_message(&msg).await;
    assert!(res.is_ok(), "relay should succeed when Asgard mock succeeds");
}

// --- E2E: Cross-Device-Actions (gRPC over Bifrost) ---

#[tokio::test]
async fn e2e_grpc_over_bifrost_workflow() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let port = listener.local_addr().unwrap().port();
    let server = WebSocketServer::new(port);
    tokio::spawn(async move {
        let _ = server.run_listener(listener).await;
    });
    tokio::time::sleep(Duration::from_millis(50)).await;

    let url = format!("ws://127.0.0.1:{}", port);
    let (mut client_a, _) = connect_async(&url).await.expect("client A connect");
    tokio::time::sleep(Duration::from_millis(80)).await;
    let (mut client_b, _) = connect_async(&url).await.expect("client B connect");
    tokio::time::sleep(Duration::from_millis(80)).await;

    let manager = Arc::new(ConnectionManager::new());
    let router = Arc::new(MessageRouter::new(manager));
    let bridge = Arc::new(GrpcBridge::new(router, Duration::from_secs(5)));

    let (request_id, req_msg) = bridge
        .build_request("device-a", "unknown", "thor.Thor", "Execute", b"action-payload")
        .unwrap();

    let rx = bridge.register_pending(&request_id).unwrap();

    let json = MessageHandler::serialize_message(&req_msg).unwrap();
    client_a.send(Message::Text(json)).await.expect("A send GrpcRequest");

    let received = client_b.next().await.expect("B receive GrpcRequest");
    let text = match received.expect("frame") {
        Message::Text(t) => t,
        _ => panic!("expected text frame"),
    };
    let req_parsed = MessageHandler::parse_message(&text).expect("parse GrpcRequest");
    assert_eq!(req_parsed.message_type, MessageType::GrpcRequest);

    let resp_body = b"result-payload";
    // Server routes by target_device_id; both clients register as "unknown" (no handshake device_id).
    // Use target "unknown" so the response is delivered to client_a (and any other connection).
    let resp_msg = BifrostMessage {
        message_id: uuid::Uuid::new_v4().to_string(),
        message_type: MessageType::GrpcResponse,
        source_device_id: "unknown".to_string(),
        target_device_id: "unknown".to_string(),
        payload: serde_json::json!({
            "request_id": request_id,
            "body": base64::engine::general_purpose::STANDARD.encode(resp_body),
            "ok": true
        }),
        timestamp: 0,
        protocol_version: None,
    };
    let resp_json = MessageHandler::serialize_message(&resp_msg).unwrap();
    client_b.send(Message::Text(resp_json)).await.expect("B send GrpcResponse");

    let resp_parsed = loop {
        let resp_received = client_a.next().await.expect("A receive");
        let resp_text = match resp_received.expect("frame") {
            Message::Text(t) => t,
            _ => continue,
        };
        let parsed = MessageHandler::parse_message(&resp_text).expect("parse");
        if parsed.message_type == MessageType::GrpcResponse {
            break parsed;
        }
    };
    let payload = GrpcBridge::parse_grpc_response_payload(&resp_parsed).unwrap();
    bridge.on_grpc_response(&payload.request_id, &payload.body, payload.ok);

    let result = tokio::time::timeout(Duration::from_secs(2), rx)
        .await
        .expect("timeout waiting for response")
        .expect("channel");
    assert!(result.is_ok());
    assert_eq!(result.unwrap().as_slice(), resp_body);

    client_a.close(None).await.ok();
    client_b.close(None).await.ok();
}

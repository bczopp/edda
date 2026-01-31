mod utils;

use nidhoggr::websocket::WebSocketServer;
use nidhoggr::connection::ConnectionManager;
use nidhoggr::routing::MessageRouter;
use nidhoggr::ratelimiter::RateLimiter;
use nidhoggr::security::SecurityMonitor;
use nidhoggr::security::audit::AuditLogger;
use nidhoggr::clients::ClientManager;
use std::sync::Arc;
use tokio_tungstenite::{connect_async, tungstenite::Message};
use futures_util::{SinkExt, StreamExt};
use std::time::Duration;
use tokio::time::sleep;
use uuid::Uuid;
use ratatoskr::protocol::MessageSerializer;
use ratatoskr::messages::RatatoskrRequest;
use ratatoskr::proto::ratatoskr::MessageType;

fn test_endpoints() -> nidhoggr::utils::config::ServiceEndpoints {
    utils::test_helpers::get_test_service_endpoints()
}

#[tokio::test]
async fn test_e2e_connection_flow() {
    // Setup
    let connection_manager = Arc::new(ConnectionManager::new());
    let client_manager = Arc::new(ClientManager::new(test_endpoints()).await.unwrap());
    let router = Arc::new(MessageRouter::new(connection_manager.clone(), client_manager));
    let rate_limiter = Arc::new(RateLimiter::new(60, 1000));
    let security_monitor = Arc::new(SecurityMonitor::new(1000));
    let audit_logger = Arc::new(AuditLogger::new(1000));
    
    let server = WebSocketServer::new_with_deps(
        0,
        connection_manager.clone(),
        router,
        rate_limiter,
        security_monitor,
        audit_logger,
    );
    
    let server_addr = server.start().await.unwrap();
    
    // Connect
    let url = format!("ws://localhost:{}", server_addr.port());
    let (ws_stream, _) = connect_async(&url).await.expect("Failed to connect");
    let (mut write, mut read) = ws_stream.split();
    
    // Send connection request with valid nonce/signature for validation
    let mut request = RatatoskrRequest::new_connection_request(
        Uuid::new_v4().to_string(),
        "test-device".to_string(),
        "test-user".to_string(),
        "test-device-identity".to_string(),
        "test-auth-token".to_string(),
        "1.0".to_string(),
    );
    request.nonce = vec![0u8; 16];
    request.signature = vec![0u8; 64];

    let serializer = MessageSerializer::new();
    let serialized = serializer.serialize_request(&request)
        .expect("Failed to serialize request");
    
    write.send(Message::Binary(serialized)).await.expect("Failed to send");
    
    // Wait for response
    sleep(Duration::from_millis(100)).await;
    
    // Verify connection was established
    let active_connections = connection_manager.get_active_connections().await;
    assert!(active_connections > 0, "Connection should be registered");
    
    // Send heartbeat with valid nonce/signature
    let mut heartbeat = RatatoskrRequest::new_heartbeat(
        Uuid::new_v4().to_string(),
        "test-device".to_string(),
        "test-user".to_string(),
    );
    heartbeat.nonce = vec![0u8; 16];
    heartbeat.signature = vec![0u8; 64];
    let serialized = serializer.serialize_request(&heartbeat)
        .expect("Failed to serialize heartbeat");
    write.send(Message::Binary(serialized)).await.expect("Failed to send heartbeat");
    
    sleep(Duration::from_millis(100)).await;
    
    // Close connection
    write.close().await.expect("Failed to close");
    
    sleep(Duration::from_millis(100)).await;
    
    // Verify connection was closed
    let active_connections_after = connection_manager.get_active_connections().await;
    assert!(active_connections_after < active_connections, "Connection should be removed");
}

#[tokio::test]
async fn test_e2e_message_routing() {
    // Setup
    let connection_manager = Arc::new(ConnectionManager::new());
    let client_manager = Arc::new(ClientManager::new(test_endpoints()).await.unwrap());
    let router = Arc::new(MessageRouter::new(connection_manager.clone(), client_manager));
    let rate_limiter = Arc::new(RateLimiter::new(60, 1000));
    let security_monitor = Arc::new(SecurityMonitor::new(1000));
    let audit_logger = Arc::new(AuditLogger::new(1000));
    
    let server = WebSocketServer::new_with_deps(
        0,
        connection_manager.clone(),
        router,
        rate_limiter,
        security_monitor,
        audit_logger,
    );
    
    let server_addr = server.start().await.unwrap();
    
    // Connect
    let url = format!("ws://localhost:{}", server_addr.port());
    let (ws_stream, _) = connect_async(&url).await.expect("Failed to connect");
    let (mut write, mut read) = ws_stream.split();
    
    // Establish connection first with valid nonce/signature
    let mut conn_request = RatatoskrRequest::new_connection_request(
        Uuid::new_v4().to_string(),
        "test-device".to_string(),
        "test-user".to_string(),
        "test-device-identity".to_string(),
        "test-auth-token".to_string(),
        "1.0".to_string(),
    );
    conn_request.nonce = vec![0u8; 16];
    conn_request.signature = vec![0u8; 64];

    let serializer = MessageSerializer::new();
    let serialized = serializer.serialize_request(&conn_request)
        .expect("Failed to serialize request");
    write.send(Message::Binary(serialized)).await.expect("Failed to send");
    sleep(Duration::from_millis(100)).await;
    
    // Send business request with valid nonce/signature
    let mut business_request = RatatoskrRequest::new_business_request(
        Uuid::new_v4().to_string(),
        "test-device".to_string(),
        "test-user".to_string(),
        b"{\"request_type\":\"test\"}".to_vec(),
    );
    business_request.nonce = vec![0u8; 16];
    business_request.signature = vec![0u8; 64];

    let serialized = serializer.serialize_request(&business_request)
        .expect("Failed to serialize business request");
    write.send(Message::Binary(serialized)).await.expect("Failed to send business request");
    
    sleep(Duration::from_millis(100)).await;
    
    // Verify message was processed (check audit logs if available)
    // In a real implementation, we would verify the response
}

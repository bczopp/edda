mod utils;

use nidhoggr::websocket::WebSocketServer;
use nidhoggr::connection::ConnectionManager;
use nidhoggr::routing::MessageRouter;
use nidhoggr::ratelimiter::RateLimiter;
use nidhoggr::security::SecurityMonitor;
use nidhoggr::security::audit::AuditLogger;
use std::sync::Arc;
use tokio_tungstenite::{connect_async, tungstenite::Message};
use futures_util::{SinkExt, StreamExt};
use std::time::Duration;
use tokio::time::sleep;
use uuid::Uuid;
use ratatoskr::protocol::MessageSerializer;

fn test_endpoints() -> nidhoggr::utils::config::ServiceEndpoints {
    utils::test_helpers::get_test_service_endpoints()
}

#[tokio::test]
async fn test_websocket_server_starts() {
    let connection_manager = Arc::new(ConnectionManager::new());
    let client_manager = Arc::new(nidhoggr::clients::ClientManager::new(test_endpoints())
        .await
        .unwrap());
    let router = Arc::new(MessageRouter::new(connection_manager.clone(), client_manager));
    let rate_limiter = Arc::new(RateLimiter::new(60, 1000));
    let security_monitor = Arc::new(SecurityMonitor::new(1000));
    let audit_logger = Arc::new(AuditLogger::new(1000));
    
    let server = WebSocketServer::new_with_deps(
        0,
        connection_manager,
        router,
        rate_limiter,
        security_monitor,
        audit_logger,
    );
    let addr = server.start().await;
    assert!(addr.is_ok());
}

#[tokio::test]
async fn test_websocket_connection_accepts_ratatoskr_handshake() {
    let connection_manager = Arc::new(ConnectionManager::new());
    let client_manager = Arc::new(nidhoggr::clients::ClientManager::new(test_endpoints())
        .await
        .unwrap());
    let router = Arc::new(MessageRouter::new(connection_manager.clone(), client_manager));
    let rate_limiter = Arc::new(RateLimiter::new(60, 1000));
    let security_monitor = Arc::new(SecurityMonitor::new(1000));
    let audit_logger = Arc::new(AuditLogger::new(1000));
    
    let server = WebSocketServer::new_with_deps(
        0,
        connection_manager,
        router,
        rate_limiter,
        security_monitor,
        audit_logger,
    );
    
    let server_addr = server.start().await.unwrap();
    
    // Try to connect
    let url = format!("ws://localhost:{}", server_addr.port());
    let (ws_stream, _) = connect_async(&url).await.expect("Failed to connect");
    
    // Send connection request using Ratatoskr protocol
    // This will fail until we implement the server
    let (mut write, mut read) = ws_stream.split();
    
    // Create a Ratatoskr connection request with valid nonce and signature for validation
    use ratatoskr::messages::RatatoskrRequest;
    use ratatoskr::proto::ratatoskr::MessageType;
    use ratatoskr::protocol::MessageSerializer;

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
    
    if let Some(msg) = read.next().await {
        match msg {
            Ok(Message::Binary(data)) => {
                let serializer = MessageSerializer::new();
                let response = serializer.deserialize_response(&data)
                    .expect("Failed to deserialize response");
                assert_eq!(response.message_type, MessageType::ConnectionResponse as i32);
            }
            _ => panic!("Expected binary connection response"),
        }
    } else {
        panic!("No response received");
    }
}

#[tokio::test]
async fn test_websocket_rejects_invalid_ratatoskr_message() {
    let connection_manager = Arc::new(ConnectionManager::new());
    let client_manager = Arc::new(nidhoggr::clients::ClientManager::new(test_endpoints())
        .await
        .unwrap());
    let router = Arc::new(MessageRouter::new(connection_manager.clone(), client_manager));
    let rate_limiter = Arc::new(RateLimiter::new(60, 1000));
    let security_monitor = Arc::new(SecurityMonitor::new(1000));
    let audit_logger = Arc::new(AuditLogger::new(1000));
    
    let server = WebSocketServer::new_with_deps(
        0,
        connection_manager,
        router,
        rate_limiter,
        security_monitor,
        audit_logger,
    );
    
    let server_addr = server.start().await.unwrap();
    
    let url = format!("ws://localhost:{}", server_addr.port());
    let (ws_stream, _) = connect_async(&url).await.expect("Failed to connect");
    
    let (mut write, mut read) = ws_stream.split();
    
    // Send invalid message
    write.send(Message::Text("invalid json".to_string())).await.expect("Failed to send");
    
    sleep(Duration::from_millis(100)).await;
    
    // Should receive error response or connection close
    if let Some(msg) = read.next().await {
        match msg {
            Ok(Message::Close(_)) => {
                // Connection closed due to invalid message - expected
            }
            Ok(Message::Binary(data)) => {
                let serializer = MessageSerializer::new();
                let response = serializer.deserialize_response(&data)
                    .expect("Failed to deserialize response");
                assert_eq!(response.message_type, MessageType::Error as i32);
            }
            _ => panic!("Expected error response or close"),
        }
    }
}

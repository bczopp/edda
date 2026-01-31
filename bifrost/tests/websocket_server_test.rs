//! WebSocket server tests (Phase 6.1.1, TDD).
//! Run in container: docker compose -f docker-compose.test.yml run --rm --no-deps bifrost-test

use bifrost::connection::ConnectionManager;
use bifrost::websocket::WebSocketServer;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tokio_tungstenite::connect_async;

#[test]
fn websocket_server_new_creates_with_port() {
    let server = WebSocketServer::new(9999);
    // Port is internal; we only verify construction.
    let _ = server;
}

#[tokio::test]
async fn connection_manager_empty_returns_none_and_empty_list() {
    let manager = ConnectionManager::new();
    assert!(manager.get("any").await.is_none());
    assert!(manager.list_by_device("dev1").await.is_empty());
}

#[tokio::test]
async fn server_accepts_websocket_connection() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr: SocketAddr = listener.local_addr().unwrap();
    let port = addr.port();
    let server = WebSocketServer::new(port);
    tokio::spawn(async move {
        let _ = server.run_listener(listener).await;
    });
    tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
    let url = format!("ws://127.0.0.1:{}", port);
    let (mut ws_stream, _) = connect_async(&url).await.expect("connect");
    ws_stream.close(None).await.expect("close");
}

#[tokio::test]
async fn connection_pool_registers_and_removes_connection() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let port = listener.local_addr().unwrap().port();
    let server = WebSocketServer::new(port);
    let manager = server.connection_manager().clone();
    tokio::spawn(async move {
        let _ = server.run_listener(listener).await;
    });
    tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
    assert!(manager.list_by_device("unknown").await.is_empty());
    let url = format!("ws://127.0.0.1:{}", port);
    let (mut ws_stream, _) = connect_async(&url).await.expect("connect");
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    assert_eq!(manager.list_by_device("unknown").await.len(), 1);
    ws_stream.close(None).await.expect("close");
    tokio::time::sleep(tokio::time::Duration::from_millis(150)).await;
    assert!(manager.list_by_device("unknown").await.is_empty());
}

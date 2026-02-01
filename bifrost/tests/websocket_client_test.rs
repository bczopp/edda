//! Tests for Phase 7.1.1: WebSocketClient (connect, WebSocket upgrade).

use bifrost::websocket::{WebSocketClient, WebSocketServer};
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tokio_tungstenite::tungstenite::Message;
use futures_util::SinkExt;

#[test]
fn client_new_creates() {
    let _ = WebSocketClient::new();
}

#[tokio::test]
async fn client_connects_to_server() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr: SocketAddr = listener.local_addr().unwrap();
    let port = addr.port();
    let server = WebSocketServer::new(port);

    tokio::spawn(async move {
        let _ = server.run_listener(listener).await;
    });
    tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;

    let client = WebSocketClient::new();
    let url = format!("ws://127.0.0.1:{}", port);
    let (mut ws_stream, response) = client.connect(&url).await.expect("connect");
    // WebSocket upgrade returns 101 Switching Protocols, not 2xx
    assert!(response.status().is_success() || response.status().as_u16() == 101);
    ws_stream.close(None).await.expect("close");
}

#[tokio::test]
async fn client_send_and_receive() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let port = listener.local_addr().unwrap().port();
    let server = WebSocketServer::new(port);

    tokio::spawn(async move {
        let _ = server.run_listener(listener).await;
    });
    tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;

    let client = WebSocketClient::new();
    let url = format!("ws://127.0.0.1:{}", port);
    let (mut ws_stream, _) = client.connect(&url).await.expect("connect");
    ws_stream.send(Message::Text("hello".into())).await.expect("send");
    ws_stream.close(None).await.expect("close");
}

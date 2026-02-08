//! Tests for WebSocketHandler (TDD – Phase 7.3.1).

use futures_util::{SinkExt, StreamExt};
use jormungandr::websocket::WebSocketHandler;
use tokio::net::TcpListener;
use tokio_tungstenite::accept_async;

#[tokio::test]
async fn websocket_handler_connect_send_receive() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let port = listener.local_addr().unwrap().port();
    let url = format!("ws://127.0.0.1:{}", port);

    tokio::spawn(async move {
        let (tcp, _) = listener.accept().await.unwrap();
        let mut ws = accept_async(tcp).await.unwrap();
        if let Some(Ok(msg)) = ws.next().await {
            if let Some(text) = msg.to_text().ok() {
                let _ = ws.send(msg).await;
            }
        }
    });

    let mut client = WebSocketHandler::new(&url);
    client.connect().await.unwrap();
    client.send("ping").await.unwrap();
    let got = client.receive().await.unwrap();
    assert_eq!(got, Some("ping".to_string()));
}

#[tokio::test]
async fn websocket_handler_reconnect() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let port = listener.local_addr().unwrap().port();
    let url = format!("ws://127.0.0.1:{}", port);

    tokio::spawn(async move {
        let (tcp, _) = listener.accept().await.unwrap();
        let _ = accept_async(tcp).await;
        let (tcp2, _) = listener.accept().await.unwrap();
        let _ = accept_async(tcp2).await;
    });

    let mut client = WebSocketHandler::new(&url);
    client.connect().await.unwrap();
    client.reconnect().await.unwrap();
}

#[tokio::test]
async fn websocket_handler_connect_invalid_fails() {
    let mut client = WebSocketHandler::new("ws://127.0.0.1:0");
    assert!(client.connect().await.is_err());
}

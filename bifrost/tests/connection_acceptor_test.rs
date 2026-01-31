//! Tests for Phase 6.1.2: ConnectionAcceptor (accept, WebSocket upgrade).

use bifrost::websocket::ConnectionAcceptor;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tokio_tungstenite::connect_async;

#[test]
fn acceptor_new_creates() {
    let _ = ConnectionAcceptor::new();
}

#[tokio::test]
async fn accept_upgrades_websocket() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr: SocketAddr = listener.local_addr().unwrap();
    let acceptor = ConnectionAcceptor::new();

    let server_handle = tokio::spawn(async move {
        let (stream, _) = listener.accept().await.unwrap();
        acceptor.accept(stream).await
    });

    let url = format!("ws://127.0.0.1:{}", addr.port());
    let client_handle = tokio::spawn(async move { connect_async(url).await });

    let (server_result, client_result) = tokio::join!(server_handle, client_handle);
    let ws_stream = server_result.unwrap().expect("server accept");
    let _ = client_result.unwrap().expect("client connect");
    drop(ws_stream);
}

#[tokio::test]
async fn accept_invalid_handshake_fails() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let acceptor = ConnectionAcceptor::new();

    let server_handle = tokio::spawn(async move {
        let (stream, _) = listener.accept().await.unwrap();
        acceptor.accept(stream).await
    });

    let client_handle = tokio::spawn(async move {
        let mut stream = tokio::net::TcpStream::connect(addr).await.unwrap();
        stream.write_all(b"GET / HTTP/1.0\r\n\r\n").await.unwrap();
        stream.shutdown().await.unwrap();
    });

    let (server_result, _) = tokio::join!(server_handle, client_handle);
    let result = server_result.unwrap();
    assert!(result.is_err());
}

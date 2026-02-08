// TCPClient tests (Phase 3.2.1, TDD).

use jotunheim_esp32::network::TCPClient;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

#[tokio::test]
async fn connect_send_receive_roundtrip() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let done = Arc::new(tokio::sync::Notify::new());

    let done_clone = done.clone();
    tokio::spawn(async move {
        let (mut stream, _) = listener.accept().await.unwrap();
        let mut buf = [0u8; 64];
        let n = stream.read(&mut buf).await.unwrap();
        assert!(n > 0);
        stream.write_all(b"pong").await.unwrap();
        done_clone.notify_one();
    });

    let mut client = TCPClient::connect(addr.ip().to_string(), addr.port())
        .await
        .unwrap();
    client.send(b"ping").await.unwrap();
    let recv = client.receive().await.unwrap();
    assert_eq!(recv.as_slice(), b"pong");
    done.notified().await;
}

#[tokio::test]
async fn connect_to_unreachable_fails() {
    let r = TCPClient::connect("127.0.0.1", 1).await;
    assert!(r.is_err());
}

#[tokio::test]
async fn receive_after_disconnect_returns_error_or_empty() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    tokio::spawn(async move {
        let (mut stream, _) = listener.accept().await.unwrap();
        stream.write_all(b"one").await.unwrap();
        drop(stream);
    });

    let mut client = TCPClient::connect(addr.ip().to_string(), addr.port())
        .await
        .unwrap();
    let first = client.receive().await.unwrap();
    assert_eq!(first.as_slice(), b"one");
    // Second receive may get empty (closed) or error
    let second = client.receive().await;
    assert!(second.is_err() || second.unwrap().is_empty());
}

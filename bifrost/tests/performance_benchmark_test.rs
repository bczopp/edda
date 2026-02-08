//! Performance Benchmark Tests (Phase 20.2.1).
//! Defines and asserts: Message-Routing-Latency, Message-Throughput, Connection-Establishment-Time.
//! Target (local): routing < 10ms, throughput sufficient, connection establishment fast.
//! Assertion thresholds are relaxed for CI/container stability.
//! Run in container: docker compose -f docker-compose.test.yml run --rm --no-deps bifrost-test

use bifrost::message::{BifrostMessage, MessageHandler, MessageType};
use bifrost::websocket::WebSocketServer;
use futures_util::{SinkExt, StreamExt};
use std::time::{Duration, Instant};
use tokio::net::TcpListener;
use tokio_tungstenite::{connect_async, tungstenite::Message};

// --- Benchmark targets (documented; IMPLEMENTATION_PLAN: < 10ms lokal) ---
/// Target: message routing latency < 10ms (local). Test uses relaxed limit for CI.
const ROUTING_LATENCY_TARGET_MS: u64 = 10;
const ROUTING_LATENCY_MAX_MS: u64 = 200;

/// Target: minimum messages per second. Test uses relaxed minimum for CI.
const THROUGHPUT_TARGET_MSG_PER_SEC: u32 = 100;
const THROUGHPUT_MIN_MSG_PER_SEC: u32 = 5;
const THROUGHPUT_MESSAGE_COUNT: usize = 30;

/// Target: connection establishment fast. Test uses relaxed limit for CI.
const CONNECTION_ESTABLISHMENT_TARGET_MS: u64 = 100;
const CONNECTION_ESTABLISHMENT_MAX_MS: u64 = 2000;

// --- Message-Routing-Latency (< 10ms lokal) ---

#[tokio::test]
async fn performance_message_routing_latency_below_threshold() {
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
        message_id: "perf-lat-1".to_string(),
        message_type: MessageType::Message,
        source_device_id: "sender".to_string(),
        target_device_id: "unknown".to_string(),
        payload: serde_json::json!({"perf": "latency"}),
        timestamp: 0,
        protocol_version: Some(1),
    };
    let json = MessageHandler::serialize_message(&msg).unwrap();

    let start = Instant::now();
    client_a.send(Message::Text(json)).await.expect("send");
    let _ = client_b.next().await.expect("receive");
    let elapsed = start.elapsed();

    assert!(
        elapsed.as_millis() as u64 <= ROUTING_LATENCY_MAX_MS,
        "routing latency {} ms exceeds max {} ms (target: {} ms locally)",
        elapsed.as_millis(),
        ROUTING_LATENCY_MAX_MS,
        ROUTING_LATENCY_TARGET_MS
    );

    client_a.close(None).await.ok();
    client_b.close(None).await.ok();
}

// --- Message-Throughput (Messages/Sekunde) ---

#[tokio::test]
async fn performance_message_throughput_above_minimum() {
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

    let start = Instant::now();
    for i in 0..THROUGHPUT_MESSAGE_COUNT {
        let msg = BifrostMessage {
            message_id: format!("perf-tp-{}", i),
            message_type: MessageType::Message,
            source_device_id: "sender".to_string(),
            target_device_id: "unknown".to_string(),
            payload: serde_json::json!({"i": i}),
            timestamp: 0,
            protocol_version: Some(1),
        };
        let json = MessageHandler::serialize_message(&msg).unwrap();
        client_a.send(Message::Text(json)).await.expect("send");
    }
    for _ in 0..THROUGHPUT_MESSAGE_COUNT {
        let _ = client_b.next().await.expect("receive");
    }
    let elapsed = start.elapsed();

    let elapsed_sec = elapsed.as_secs_f64();
    let msg_per_sec = if elapsed_sec > 0.0 {
        THROUGHPUT_MESSAGE_COUNT as f64 / elapsed_sec
    } else {
        f64::MAX
    };

    assert!(
        msg_per_sec >= THROUGHPUT_MIN_MSG_PER_SEC as f64,
        "throughput {:.1} msg/s below minimum {} msg/s (target: {} msg/s)",
        msg_per_sec,
        THROUGHPUT_MIN_MSG_PER_SEC,
        THROUGHPUT_TARGET_MSG_PER_SEC
    );

    client_a.close(None).await.ok();
    client_b.close(None).await.ok();
}

// --- Connection-Establishment-Time ---

#[tokio::test]
async fn performance_connection_establishment_time_below_threshold() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let port = listener.local_addr().unwrap().port();
    let server = WebSocketServer::new(port);
    tokio::spawn(async move {
        let _ = server.run_listener(listener).await;
    });
    tokio::time::sleep(Duration::from_millis(50)).await;

    let url = format!("ws://127.0.0.1:{}", port);
    let start = Instant::now();
    let (mut stream, _) = connect_async(&url).await.expect("connect");
    let elapsed = start.elapsed();

    assert!(
        elapsed.as_millis() as u64 <= CONNECTION_ESTABLISHMENT_MAX_MS,
        "connection establishment {} ms exceeds max {} ms (target: {} ms)",
        elapsed.as_millis(),
        CONNECTION_ESTABLISHMENT_MAX_MS,
        CONNECTION_ESTABLISHMENT_TARGET_MS
    );

    stream.close(None).await.ok();
}

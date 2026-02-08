// LokiClient tests (Phase 2.2.2). Unit tests; integration tests require mock-loki.

use jotunheim_esp32::grpc::{LokiClient, LokiClientError};

#[tokio::test]
async fn endpoint_builds_correct_url() {
    let e = LokiClient::endpoint("192.168.1.10", 50057).unwrap();
    // Endpoint from_shared normalizes; we only care it doesn't error
    assert!(e.uri().host().is_some());
}

#[tokio::test]
async fn endpoint_rejects_invalid_host() {
    // Empty or invalid host should yield InvalidEndpoint or parse error
    let r = LokiClient::endpoint("", 50057);
    assert!(r.is_err());
}

#[tokio::test]
async fn connect_to_unreachable_fails() {
    // No server on 127.0.0.1:1
    let r = LokiClient::connect("127.0.0.1", 1, 1).await;
    assert!(r.is_err());
}

// Integration test: get_capabilities/list_scripts against mock-loki (run with docker compose).
// #[tokio::test]
// async fn get_capabilities_returns_from_mock() { ... }

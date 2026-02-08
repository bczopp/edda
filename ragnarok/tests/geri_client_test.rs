//! Integration tests for Geri gRPC client (Phase 2).
//! Tests run without a live Geri service (unreachable URL → Err).

use ragnarok::grpc_client::GeriClient;

#[tokio::test]
async fn test_geri_client_connect_unreachable_returns_err() {
    let result = GeriClient::new("127.0.0.1", 38476).await;
    assert!(result.is_err(), "connect to unreachable Geri must fail");
}

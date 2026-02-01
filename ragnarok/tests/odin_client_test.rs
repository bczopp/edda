//! Integration tests for Odin gRPC client (Phase 2).
//! Tests run without a live Odin service (unreachable URL → Err).

use ragnarok::grpc_client::OdinClient;

#[tokio::test]
async fn test_odin_client_connect_unreachable_returns_err() {
    let result = OdinClient::new("127.0.0.1", 38474).await;
    assert!(result.is_err(), "connect to unreachable Odin must fail");
}

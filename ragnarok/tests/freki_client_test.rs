//! Integration tests for Freki gRPC client (Phase 2).
//! Tests run without a live Freki service (unreachable URL → Err).

use ragnarok::grpc_client::FrekiClient;

#[tokio::test]
async fn test_freki_client_connect_unreachable_returns_err() {
    let result = FrekiClient::new("127.0.0.1", 38477).await;
    assert!(result.is_err(), "connect to unreachable Freki must fail");
}

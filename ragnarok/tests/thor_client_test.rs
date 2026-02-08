//! Integration tests for Thor gRPC client (Phase 2/4).
//! Tests run without a live Thor service (unreachable URL → Err).

use ragnarok::grpc_client::ThorClient;

#[tokio::test]
async fn test_thor_client_connect_unreachable_returns_err() {
    let result = ThorClient::new("127.0.0.1", 38475).await;
    assert!(result.is_err(), "connect to unreachable Thor must fail");
}

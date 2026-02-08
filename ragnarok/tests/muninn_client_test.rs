//! Tests for Muninn gRPC client (Phase 2, optional).
//! No live Muninn service required (unreachable URL → Err).

use ragnarok::grpc_client::MuninnClient;

#[tokio::test]
async fn test_muninn_client_connect_unreachable_returns_err() {
    let result = MuninnClient::new("127.0.0.1", 38478).await;
    assert!(result.is_err(), "connect to unreachable Muninn must fail");
}

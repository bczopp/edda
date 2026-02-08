//! Tests for Huginn gRPC client (Phase 2, optional).
//! No live Huginn service required (unreachable URL → Err).

use ragnarok::grpc_client::HuginnClient;

#[tokio::test]
async fn test_huginn_client_connect_unreachable_returns_err() {
    let result = HuginnClient::new("127.0.0.1", 38477).await;
    assert!(result.is_err(), "connect to unreachable Huginn must fail");
}

//! Service integration tests (Phase 4): Odin-Client-Integration.
//! Tests run without a live Odin service (unreachable URL → Err).

use ragnarok::services::OdinServiceIntegration;

#[tokio::test]
async fn test_odin_integration_new_unreachable_returns_err() {
    let result = OdinServiceIntegration::new("127.0.0.1", 38475).await;
    assert!(result.is_err(), "OdinServiceIntegration::new with unreachable host must fail");
}

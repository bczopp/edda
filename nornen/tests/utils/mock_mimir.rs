// Test helper for Mimir-based ProviderRegistry tests
// Note: These tests require a mock Mimir service or refactoring MimirClient to use traits
// For now, these tests are integration-style tests that would work with a mock Mimir service

use nornen::urd::registry::ProviderRegistry;
use nornen::mimir_client::MimirClient;
use std::sync::Arc;

/// Helper to create a ProviderRegistry with Mimir for testing
/// This requires a Mimir service to be available (real or mock)
pub async fn create_registry_with_mimir(mimir_url: &str) -> Result<ProviderRegistry, Box<dyn std::error::Error>> {
    let mimir_client = Arc::new(MimirClient::new(mimir_url.to_string()));
    mimir_client.connect().await?;
    Ok(ProviderRegistry::new_with_mimir(mimir_client).await?)
}

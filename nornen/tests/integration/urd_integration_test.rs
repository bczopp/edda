#[cfg(test)]
mod tests {
    use nornen::urd::registry::ProviderRegistry;
    use crate::common::TestDatabase;

    #[tokio::test]
    async fn test_provider_registration_flow() {
        crate::common::setup_logging();
        let test_db = TestDatabase::new().await.unwrap();
        let registry = ProviderRegistry::new(test_db.pool.clone()).await.unwrap();
        
        // Register provider
        registry.register_provider(
            "provider1",
            "Test Provider",
            &["capability1".to_string(), "capability2".to_string()],
            "http://localhost:8080",
            &serde_json::json!({"version": "1.0"}),
        ).await.unwrap();
        
        // Query providers
        let providers = registry.query_providers(&["capability1".to_string()], Some("active")).await.unwrap();
        assert_eq!(providers.len(), 1);
        assert_eq!(providers[0].provider_id, "provider1");
    }

    #[tokio::test]
    async fn test_provider_update_flow() {
        crate::common::setup_logging();
        let test_db = TestDatabase::new().await.unwrap();
        let registry = ProviderRegistry::new(test_db.pool.clone()).await.unwrap();
        
        // Register provider
        registry.register_provider(
            "provider1",
            "Test Provider",
            &["capability1".to_string()],
            "http://localhost:8080",
            &serde_json::json!({}),
        ).await.unwrap();
        
        // Update status
        registry.update_provider_status("provider1", "inactive").await.unwrap();
        
        // Query should not return inactive provider
        let providers = registry.query_providers(&["capability1".to_string()], Some("active")).await.unwrap();
        assert_eq!(providers.len(), 0);
    }
}

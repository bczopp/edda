#[cfg(test)]
mod tests {
    use nornen::urd::registry::ProviderRegistry;
    use crate::common::TestDatabase;

    #[tokio::test]
    async fn test_register_provider() {
        let test_db = TestDatabase::new().await.unwrap();
        let registry = ProviderRegistry::new(test_db.pool.clone()).await.unwrap();
        
        let result = registry.register_provider(
            "provider1",
            "Test Provider",
            &["capability1", "capability2"],
            "http://localhost:8080",
            &serde_json::json!({"version": "1.0"}),
        ).await;
        
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_register_provider_duplicate() {
        let test_db = TestDatabase::new().await.unwrap();
        let registry = ProviderRegistry::new(test_db.pool.clone()).await.unwrap();
        
        // Register first time
        registry.register_provider(
            "provider1",
            "Test Provider",
            &["capability1"],
            "http://localhost:8080",
            &serde_json::json!({}),
        ).await.unwrap();
        
        // Try to register again with same ID
        let result = registry.register_provider(
            "provider1",
            "Another Provider",
            &["capability2"],
            "http://localhost:8081",
            &serde_json::json!({}),
        ).await;
        
        // Should fail or update (depending on implementation)
        // For now, we'll expect it to fail
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_query_providers_by_capability() {
        let test_db = TestDatabase::new().await.unwrap();
        let registry = ProviderRegistry::new(test_db.pool.clone()).await.unwrap();
        
        // Register providers
        registry.register_provider(
            "provider1",
            "Provider 1",
            &["capability1", "capability2"],
            "http://localhost:8080",
            &serde_json::json!({}),
        ).await.unwrap();
        
        registry.register_provider(
            "provider2",
            "Provider 2",
            &["capability2", "capability3"],
            "http://localhost:8081",
            &serde_json::json!({}),
        ).await.unwrap();
        
        // Query by capability
        let providers = registry.query_providers(&["capability2"], None).await.unwrap();
        assert_eq!(providers.len(), 2);
    }

    #[tokio::test]
    async fn test_update_provider_status() {
        let test_db = TestDatabase::new().await.unwrap();
        let registry = ProviderRegistry::new(test_db.pool.clone()).await.unwrap();
        
        // Register provider
        registry.register_provider(
            "provider1",
            "Test Provider",
            &["capability1"],
            "http://localhost:8080",
            &serde_json::json!({}),
        ).await.unwrap();
        
        // Update status
        let result = registry.update_provider_status("provider1", "inactive").await;
        assert!(result.is_ok());
        
        // Query should not return inactive provider
        let providers = registry.query_providers(&["capability1"], Some("active")).await.unwrap();
        assert_eq!(providers.len(), 0);
    }

    #[tokio::test]
    async fn test_list_providers() {
        let test_db = TestDatabase::new().await.unwrap();
        let registry = ProviderRegistry::new(test_db.pool.clone()).await.unwrap();
        
        // Register multiple providers
        for i in 1..=5 {
            registry.register_provider(
                &format!("provider{}", i),
                &format!("Provider {}", i),
                &["capability1"],
                &format!("http://localhost:{}", 8080 + i),
                &serde_json::json!({}),
            ).await.unwrap();
        }
        
        // List all providers
        let result = registry.list_providers(10, 0).await.unwrap();
        assert_eq!(result.providers.len(), 5);
        assert_eq!(result.total, 5);
    }
}

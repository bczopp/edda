#[cfg(test)]
mod tests {
    use nornen::urd::registry::ProviderRegistry;
    use nornen::cache::ProviderCache;
    use nornen::verdandi::router::RequestRouter;
    use crate::common::TestDatabase;
    use serde_json::json;
    use std::sync::Arc;

    #[tokio::test]
    async fn test_cache_invalidation_on_provider_update() {
        crate::common::setup_logging();
        let test_db = TestDatabase::new().await.unwrap();
        
        // Create cache
        let cache = Arc::new(ProviderCache::new(100, 60));
        
        // Create registry with cache
        let registry = Arc::new(ProviderRegistry::new_with_cache(test_db.pool.clone(), cache.clone()).await.unwrap());
        
        // Create router with cache
        let router = Arc::new(RequestRouter::new_with_cache(registry.clone(), cache.clone()));
        
        // Register provider
        registry.register_provider(
            "provider1",
            "Test Provider",
            &["llm".to_string(), "text".to_string()],
            "http://localhost:8080",
            &json!({}),
        ).await.unwrap();
        
        // Query providers - should populate cache
        let providers = router.registry
            .query_providers(&["llm".to_string()], Some("active"))
            .await.unwrap();
        assert_eq!(providers.len(), 1);
        
        // Route request - should use cache
        let _provider = router.route_request(
            &["llm".to_string()],
            &std::collections::HashMap::new(),
        ).await.unwrap();
        
        // Verify cache has entry
        let cached = cache.get(&["llm".to_string()], Some("active")).await;
        assert!(cached.is_some());
        
        // Update provider
        registry.update_provider(
            "provider1",
            Some("Updated Provider"),
            None,
            None,
            None,
        ).await.unwrap();
        
        // Cache should be invalidated
        let cached_after_update = cache.get(&["llm".to_string()], Some("active")).await;
        assert!(cached_after_update.is_none(), "Cache should be invalidated after provider update");
        
        // Query again - should repopulate cache with updated data
        let providers_after = router.registry
            .query_providers(&["llm".to_string()], Some("active"))
            .await.unwrap();
        assert_eq!(providers_after.len(), 1);
        assert_eq!(providers_after[0].name, "Updated Provider");
    }

    #[tokio::test]
    async fn test_cache_invalidation_on_provider_status_update() {
        crate::common::setup_logging();
        let test_db = TestDatabase::new().await.unwrap();
        
        // Create cache
        let cache = Arc::new(ProviderCache::new(100, 60));
        
        // Create registry with cache
        let registry = Arc::new(ProviderRegistry::new_with_cache(test_db.pool.clone(), cache.clone()).await.unwrap());
        
        // Create router with cache
        let router = Arc::new(RequestRouter::new_with_cache(registry.clone(), cache.clone()));
        
        // Register provider
        registry.register_provider(
            "provider1",
            "Test Provider",
            &["llm".to_string()],
            "http://localhost:8080",
            &json!({}),
        ).await.unwrap();
        
        // Query providers - should populate cache
        let _providers = router.registry
            .query_providers(&["llm".to_string()], Some("active"))
            .await.unwrap();
        
        // Verify cache has entry
        let cached = cache.get(&["llm".to_string()], Some("active")).await;
        assert!(cached.is_some());
        
        // Update provider status
        registry.update_provider_status("provider1", "inactive").await.unwrap();
        
        // Cache should be invalidated
        let cached_after_update = cache.get(&["llm".to_string()], Some("active")).await;
        assert!(cached_after_update.is_none(), "Cache should be invalidated after status update");
    }

    #[tokio::test]
    async fn test_cache_invalidation_on_provider_registration() {
        crate::common::setup_logging();
        let test_db = TestDatabase::new().await.unwrap();
        
        // Create cache
        let cache = Arc::new(ProviderCache::new(100, 60));
        
        // Create registry with cache
        let registry = Arc::new(ProviderRegistry::new_with_cache(test_db.pool.clone(), cache.clone()).await.unwrap());
        
        // Create router with cache
        let router = Arc::new(RequestRouter::new_with_cache(registry.clone(), cache.clone()));
        
        // Register first provider
        registry.register_provider(
            "provider1",
            "Provider 1",
            &["llm".to_string()],
            "http://localhost:8080",
            &json!({}),
        ).await.unwrap();
        
        // Query providers - should populate cache
        let _providers = router.registry
            .query_providers(&["llm".to_string()], Some("active"))
            .await.unwrap();
        
        // Verify cache has entry
        let cached = cache.get(&["llm".to_string()], Some("active")).await;
        assert!(cached.is_some());
        assert_eq!(cached.unwrap().len(), 1);
        
        // Register second provider with same capability
        registry.register_provider(
            "provider2",
            "Provider 2",
            &["llm".to_string()],
            "http://localhost:8081",
            &json!({}),
        ).await.unwrap();
        
        // Cache should be invalidated
        let cached_after_registration = cache.get(&["llm".to_string()], Some("active")).await;
        assert!(cached_after_registration.is_none(), "Cache should be invalidated after new provider registration");
        
        // Query again - should repopulate cache with both providers
        let providers_after = router.registry
            .query_providers(&["llm".to_string()], Some("active"))
            .await.unwrap();
        assert_eq!(providers_after.len(), 2);
    }
}

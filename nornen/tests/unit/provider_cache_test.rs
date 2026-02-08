#[cfg(test)]
mod tests {
    use nornen::cache::ProviderCache;
    use nornen::urd::registry::Provider;
    use serde_json::json;
    use chrono::Utc;
    use std::time::Duration;
    use tokio::time::sleep;

    fn create_test_provider(id: &str, capabilities: Vec<String>) -> Provider {
        Provider {
            provider_id: id.to_string(),
            name: format!("Provider {}", id),
            capabilities,
            endpoint: format!("http://provider{}:8080", id),
            status: "active".to_string(),
            metadata: json!({}),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    #[tokio::test]
    async fn test_cache_get_and_set() {
        let cache = ProviderCache::new(100, 60);
        
        let capabilities = vec!["llm".to_string(), "text".to_string()];
        let providers = vec![
            create_test_provider("provider1", capabilities.clone()),
        ];
        
        // Cache should be empty initially
        assert!(cache.get(&capabilities, Some("active")).await.is_none());
        
        // Store in cache
        cache.set(&capabilities, Some("active"), providers.clone()).await;
        
        // Should retrieve from cache
        let cached = cache.get(&capabilities, Some("active")).await;
        assert!(cached.is_some());
        assert_eq!(cached.unwrap().len(), 1);
        assert_eq!(cached.unwrap()[0].provider_id, "provider1");
    }

    #[tokio::test]
    async fn test_cache_miss_different_capabilities() {
        let cache = ProviderCache::new(100, 60);
        
        let capabilities1 = vec!["llm".to_string()];
        let capabilities2 = vec!["text".to_string()];
        let providers = vec![create_test_provider("provider1", capabilities1.clone())];
        
        cache.set(&capabilities1, Some("active"), providers).await;
        
        // Different capabilities should result in cache miss
        assert!(cache.get(&capabilities2, Some("active")).await.is_none());
    }

    #[tokio::test]
    async fn test_cache_miss_different_status() {
        let cache = ProviderCache::new(100, 60);
        
        let capabilities = vec!["llm".to_string()];
        let providers = vec![create_test_provider("provider1", capabilities.clone())];
        
        cache.set(&capabilities, Some("active"), providers).await;
        
        // Different status should result in cache miss
        assert!(cache.get(&capabilities, Some("inactive")).await.is_none());
        assert!(cache.get(&capabilities, None).await.is_none());
    }

    #[tokio::test]
    async fn test_cache_expiration() {
        let cache = ProviderCache::new(100, 1); // 1 second TTL
        
        let capabilities = vec!["llm".to_string()];
        let providers = vec![create_test_provider("provider1", capabilities.clone())];
        
        cache.set(&capabilities, Some("active"), providers).await;
        
        // Should be in cache immediately
        assert!(cache.get(&capabilities, Some("active")).await.is_some());
        
        // Wait for expiration
        sleep(Duration::from_secs(2)).await;
        
        // Should be expired now
        assert!(cache.get(&capabilities, Some("active")).await.is_none());
    }

    #[tokio::test]
    async fn test_cache_invalidate() {
        let cache = ProviderCache::new(100, 60);
        
        let capabilities = vec!["llm".to_string()];
        let providers = vec![create_test_provider("provider1", capabilities.clone())];
        
        cache.set(&capabilities, Some("active"), providers).await;
        assert!(cache.get(&capabilities, Some("active")).await.is_some());
        
        // Invalidate
        cache.invalidate(&capabilities, Some("active")).await;
        
        // Should be gone
        assert!(cache.get(&capabilities, Some("active")).await.is_none());
    }

    #[tokio::test]
    async fn test_cache_invalidate_all() {
        let cache = ProviderCache::new(100, 60);
        
        let capabilities1 = vec!["llm".to_string()];
        let capabilities2 = vec!["text".to_string()];
        let providers1 = vec![create_test_provider("provider1", capabilities1.clone())];
        let providers2 = vec![create_test_provider("provider2", capabilities2.clone())];
        
        cache.set(&capabilities1, Some("active"), providers1).await;
        cache.set(&capabilities2, Some("active"), providers2).await;
        
        assert!(cache.get(&capabilities1, Some("active")).await.is_some());
        assert!(cache.get(&capabilities2, Some("active")).await.is_some());
        
        // Invalidate all
        cache.invalidate_all().await;
        
        // All should be gone
        assert!(cache.get(&capabilities1, Some("active")).await.is_none());
        assert!(cache.get(&capabilities2, Some("active")).await.is_none());
    }

    #[tokio::test]
    async fn test_cache_max_size_eviction() {
        let cache = ProviderCache::new(2, 60); // Max size 2
        
        // Add 3 entries - should evict oldest
        for i in 1..=3 {
            let capabilities = vec![format!("cap{}", i)];
            let providers = vec![create_test_provider(&format!("provider{}", i), capabilities.clone())];
            cache.set(&capabilities, Some("active"), providers).await;
        }
        
        // First entry should be evicted
        assert!(cache.get(&vec!["cap1".to_string()], Some("active")).await.is_none());
        
        // Second and third should still be there
        assert!(cache.get(&vec!["cap2".to_string()], Some("active")).await.is_some());
        assert!(cache.get(&vec!["cap3".to_string()], Some("active")).await.is_some());
    }

    #[tokio::test]
    async fn test_cache_clean_expired() {
        let cache = ProviderCache::new(100, 1); // 1 second TTL
        
        let capabilities1 = vec!["llm".to_string()];
        let capabilities2 = vec!["text".to_string()];
        let providers1 = vec![create_test_provider("provider1", capabilities1.clone())];
        let providers2 = vec![create_test_provider("provider2", capabilities2.clone())];
        
        cache.set(&capabilities1, Some("active"), providers1).await;
        
        // Wait for expiration
        sleep(Duration::from_secs(2)).await;
        
        // Add new entry (not expired)
        cache.set(&capabilities2, Some("active"), providers2).await;
        
        // Clean expired entries
        cache.clean_expired().await;
        
        // Expired entry should be gone
        assert!(cache.get(&capabilities1, Some("active")).await.is_none());
        
        // Non-expired entry should still be there
        assert!(cache.get(&capabilities2, Some("active")).await.is_some());
    }

    #[tokio::test]
    async fn test_cache_stats() {
        let cache = ProviderCache::new(100, 1); // 1 second TTL
        
        let capabilities1 = vec!["llm".to_string()];
        let capabilities2 = vec!["text".to_string()];
        let providers1 = vec![create_test_provider("provider1", capabilities1.clone())];
        let providers2 = vec![create_test_provider("provider2", capabilities2.clone())];
        
        cache.set(&capabilities1, Some("active"), providers1).await;
        cache.set(&capabilities2, Some("active"), providers2).await;
        
        let stats = cache.stats().await;
        assert_eq!(stats.size, 2);
        assert_eq!(stats.max_size, 100);
        assert_eq!(stats.expired_entries, 0);
        
        // Wait for expiration
        sleep(Duration::from_secs(2)).await;
        
        let stats = cache.stats().await;
        assert_eq!(stats.size, 2);
        assert_eq!(stats.expired_entries, 2); // Both expired
    }
}

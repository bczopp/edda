#[cfg(test)]
mod tests {
    use nornen::urd::registry::ProviderRegistry;
    use nornen::verdandi::router::RequestRouter;
    use std::sync::Arc;
    use std::collections::HashMap;
    use crate::common::TestDatabase;

    #[tokio::test]
    async fn test_request_routing() {
        crate::common::setup_logging();
        let test_db = TestDatabase::new().await.unwrap();
        let registry = Arc::new(ProviderRegistry::new(test_db.pool.clone()).await.unwrap());
        
        // Register provider
        registry.register_provider(
            "provider1",
            "Test Provider",
            &["capability1".to_string()],
            "http://localhost:8080",
            &serde_json::json!({}),
        ).await.unwrap();
        
        // Create router
        let router = Arc::new(RequestRouter::new(registry.clone()));
        
        // Route request
        let mut preferences = HashMap::new();
        preferences.insert("status".to_string(), "active".to_string());
        
        let provider = router.route_request(&["capability1".to_string()], &preferences).await.unwrap();
        assert_eq!(provider.provider_id, "provider1");
    }
}

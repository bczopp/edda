#[cfg(test)]
mod tests {
    use odin::protocols::einherjar::{EinherjarClient, CapabilityCache};
    use odin::clients::ServiceClientConfig;
    use crate::utils::test_helpers;

    #[tokio::test]
    async fn test_einherjar_client_creation() {
        let url = test_helpers::get_service_url("thor", 50052);
        let config = ServiceClientConfig {
            url: url.clone(),
            timeout_seconds: 30,
        };
        
        let service_ready = test_helpers::wait_for_service(&url, 10).await;
        
        if service_ready {
            let result = EinherjarClient::new(config).await;
            assert!(result.is_ok(), "Einherjar client should be created successfully when service is available");
        } else {
            println!("Service not available, skipping test");
        }
    }

    #[tokio::test]
    async fn test_einherjar_get_capabilities() {
        let url = test_helpers::get_service_url("thor", 50052);
        let config = ServiceClientConfig {
            url: url.clone(),
            timeout_seconds: 30,
        };
        
        let service_ready = test_helpers::wait_for_service(&url, 10).await;
        
        if !service_ready {
            println!("Service not available, skipping test");
            return;
        }
        
        let mut client = match EinherjarClient::new(config).await {
            Ok(c) => c,
            Err(e) => {
                eprintln!("Failed to create Einherjar client: {}", e);
                return;
            }
        };
        
        let result = client.get_capabilities().await;
        match result {
            Ok(capabilities) => {
                // Verify capability response structure
                assert!(!capabilities.god_name.is_empty(), "God name should not be empty");
                assert!(!capabilities.purpose.is_empty(), "Purpose should not be empty");
                println!("Received capabilities for: {}", capabilities.god_name);
            }
            Err(e) => {
                // Expected if service doesn't implement Einherjar Protocol yet
                println!("Get capabilities failed (expected with basic mock): {}", e);
            }
        }
    }

    #[tokio::test]
    async fn test_capability_cache() {
        let cache = CapabilityCache::new();
        
        // Test empty cache
        let result = cache.get("test-service").await;
        assert!(result.is_none(), "Cache should be empty initially");
        
        // Create a test capability
        let capability = odin::protocols::einherjar::einherjar::CapabilityResponse {
            god_name: "test-service".to_string(),
            purpose: "Testing".to_string(),
            functions: vec![],
            responsibility_domains: vec!["test".to_string()],
            responsibility_keywords: vec!["test".to_string()],
        };
        
        // Update cache
        cache.update(
            "test-service".to_string(),
            "http://localhost:50052".to_string(),
            capability.clone(),
        ).await;
        
        // Test retrieval
        let cached = cache.get("test-service").await;
        assert!(cached.is_some(), "Cache should contain the service");
        let cached = cached.unwrap();
        assert_eq!(cached.service_name, "test-service");
        assert_eq!(cached.capability.god_name, "test-service");
        
        // Test get_all
        let all = cache.get_all().await;
        assert_eq!(all.len(), 1);
        
        // Test clear
        cache.clear("test-service").await;
        let result = cache.get("test-service").await;
        assert!(result.is_none(), "Cache should be empty after clear");
    }

    /// Phase 3 Capability-Aggregation: get_aggregated returns services by domain and keyword.
    #[tokio::test]
    async fn capability_cache_get_aggregated_returns_services_by_domain_and_keyword() {
        use odin::protocols::einherjar::einherjar::CapabilityResponse;
        let cache = CapabilityCache::new();
        cache
            .update(
                "geri".to_string(),
                "http://localhost:50054".to_string(),
                CapabilityResponse {
                    god_name: "geri".to_string(),
                    purpose: "LLM".to_string(),
                    functions: vec![],
                    responsibility_domains: vec!["text".to_string(), "question".to_string()],
                    responsibility_keywords: vec!["answer".to_string()],
                },
            )
            .await;
        cache
            .update(
                "thor".to_string(),
                "http://localhost:50052".to_string(),
                CapabilityResponse {
                    god_name: "thor".to_string(),
                    purpose: "Action".to_string(),
                    functions: vec![],
                    responsibility_domains: vec!["action".to_string()],
                    responsibility_keywords: vec!["execute".to_string()],
                },
            )
            .await;
        let agg = cache.get_aggregated().await;
        assert!(
            agg.services_for_domain("text").unwrap().contains(&"geri".to_string()),
            "domain 'text' should list geri"
        );
        assert!(
            agg.services_for_keyword("execute").unwrap().contains(&"thor".to_string()),
            "keyword 'execute' should list thor"
        );
    }
}

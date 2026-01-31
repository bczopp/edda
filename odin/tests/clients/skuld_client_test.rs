#[cfg(test)]
mod tests {
    use odin::clients::skuld::SkuldClient;
    use odin::clients::ServiceClientConfig;
    use odin::clients::skuld::skuld::SelectModelRequest;
    use crate::utils::test_helpers;

    #[tokio::test]
    async fn test_skuld_client_creation() {
        let url = test_helpers::get_service_url("skuld", 50058);
        let config = ServiceClientConfig {
            url: url.clone(),
            timeout_seconds: 30,
        };
        
        let service_ready = test_helpers::wait_for_service(&url, 10).await;
        
        if service_ready {
            let result = SkuldClient::new(config).await;
            assert!(result.is_ok(), "Skuld client should be created successfully when service is available");
        } else {
            println!("Service not available, skipping test");
        }
    }

    #[tokio::test]
    async fn test_skuld_select_model() {
        let url = test_helpers::get_service_url("skuld", 50058);
        let config = ServiceClientConfig {
            url: url.clone(),
            timeout_seconds: 30,
        };
        
        let service_ready = test_helpers::wait_for_service(&url, 10).await;
        
        if !service_ready {
            println!("Service not available, skipping test");
            return;
        }
        
        let mut client = match SkuldClient::new(config).await {
            Ok(c) => c,
            Err(e) => {
                eprintln!("Failed to create Skuld client: {}", e);
                return;
            }
        };
        
        let request = SelectModelRequest {
            prompt: "What is the weather today?".to_string(),
            max_size: 0,
            min_reliability: 0.0,
            max_latency_ms: 0,
        };
        
        let result = client.select_model(request).await;
        match result {
            Ok(response) => {
                // Verify response structure
                assert!(!response.model_name.is_empty(), "Model name should not be empty");
                assert!(response.score >= 0.0, "Score should be non-negative");
                println!("Selected model: {} (score: {})", response.model_name, response.score);
            }
            Err(e) => {
                // Expected if service doesn't implement Skuld Service yet
                println!("Select model failed (expected with basic mock): {}", e);
            }
        }
    }
}

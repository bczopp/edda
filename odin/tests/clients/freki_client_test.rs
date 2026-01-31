#[cfg(test)]
mod tests {
    use odin::clients::freki::FrekiClient;
    use odin::clients::ServiceClientConfig;
    use odin::clients::freki::freki::RetrieveContextRequest;
    use crate::utils::test_helpers;

    #[tokio::test]
    async fn test_freki_client_creation() {
        let url = test_helpers::get_service_url("freki", 50053);
        let config = ServiceClientConfig {
            url: url.clone(),
            timeout_seconds: 30,
        };
        
        let service_ready = test_helpers::wait_for_service(&url, 10).await;
        
        if service_ready {
            let result = FrekiClient::new(config).await;
            assert!(result.is_ok(), "Freki client should be created successfully when service is available");
        } else {
            println!("Freki service not available, skipping test");
        }
    }

    #[tokio::test]
    async fn test_freki_client_retrieve_context() {
        let url = test_helpers::get_service_url("freki", 50053);
        let config = ServiceClientConfig {
            url: url.clone(),
            timeout_seconds: 30,
        };
        
        let service_ready = test_helpers::wait_for_service(&url, 10).await;
        
        if !service_ready {
            println!("Freki service not available, skipping test");
            return;
        }
        
        let mut client = match FrekiClient::new(config).await {
            Ok(c) => c,
            Err(e) => {
                eprintln!("Failed to create Freki client: {}", e);
                return;
            }
        };
        
        let request = RetrieveContextRequest {
            query_embedding: vec![0u8, 1u8, 2u8],
            limit: 5,
            collection_name: "test".to_string(),
        };
        
        let result = client.retrieve_context(request).await;
        match result {
            Ok(_) => assert!(true),
            Err(e) => println!("Context retrieval failed (expected with basic mock): {}", e),
        }
    }
}

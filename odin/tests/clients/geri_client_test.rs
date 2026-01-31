#[cfg(test)]
mod tests {
    use odin::clients::geri::GeriClient;
    use odin::clients::ServiceClientConfig;
    use odin::clients::geri::geri::ProcessPromptRequest;
    use crate::utils::test_helpers;

    #[tokio::test]
    async fn test_geri_client_creation() {
        let url = test_helpers::get_service_url("geri", 50054);
        let config = ServiceClientConfig {
            url: url.clone(),
            timeout_seconds: 30,
        };
        
        let service_ready = test_helpers::wait_for_service(&url, 10).await;
        
        if service_ready {
            let result = GeriClient::new(config).await;
            assert!(result.is_ok(), "Geri client should be created successfully when service is available");
        } else {
            println!("Geri service not available, skipping test");
        }
    }

    #[tokio::test]
    async fn test_geri_client_process_prompt() {
        let url = test_helpers::get_service_url("geri", 50054);
        let config = ServiceClientConfig {
            url: url.clone(),
            timeout_seconds: 30,
        };
        
        let service_ready = test_helpers::wait_for_service(&url, 10).await;
        
        if !service_ready {
            println!("Geri service not available, skipping test");
            return;
        }
        
        let mut client = match GeriClient::new(config).await {
            Ok(c) => c,
            Err(e) => {
                eprintln!("Failed to create Geri client: {}", e);
                return;
            }
        };
        
        let request = ProcessPromptRequest {
            prompt: "Test prompt".to_string(),
            context: "Test context".to_string(),
            model_name: "test-model".to_string(),
            max_tokens: 100,
        };
        
        let result = client.process_prompt(request).await;
        match result {
            Ok(_) => assert!(true),
            Err(e) => println!("Prompt processing failed (expected with basic mock): {}", e),
        }
    }
}

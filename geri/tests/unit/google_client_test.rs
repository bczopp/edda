//! Unit tests for Google Gemini API Client

use geri::llm::google::{GoogleClient, GoogleConfig, GoogleError};

#[test]
fn test_google_client_new() {
    let config = GoogleConfig {
        api_key: "test-api-key".to_string(),
        base_url: "https://generativelanguage.googleapis.com/v1beta".to_string(),
    };
    
    let client = GoogleClient::new(config.clone());
    assert_eq!(client.config().api_key, "test-api-key");
}

#[test]
fn test_google_config_validation() {
    let config = GoogleConfig {
        api_key: String::new(),
        base_url: "https://generativelanguage.googleapis.com/v1beta".to_string(),
    };
    
    let client = GoogleClient::new(config);
    // Empty API key should be caught during request, not construction
    assert!(client.config().api_key.is_empty());
}

#[test]
fn test_build_generate_content_request() {
    let config = GoogleConfig {
        api_key: "test-key".to_string(),
        base_url: "https://generativelanguage.googleapis.com/v1beta".to_string(),
    };
    
    let client = GoogleClient::new(config);
    let request = client.build_generate_content_request(
        "gemini-2.5-flash",
        "What is AI?",
        None,
        Some(100),
    );
    
    assert_eq!(request.contents.len(), 1);
    assert_eq!(request.contents[0].parts.len(), 1);
    assert_eq!(request.contents[0].parts[0].text, "What is AI?");
    assert_eq!(request.generation_config.as_ref().unwrap().max_output_tokens, Some(100));
}

#[test]
fn test_build_generate_content_request_with_context() {
    let config = GoogleConfig {
        api_key: "test-key".to_string(),
        base_url: "https://generativelanguage.googleapis.com/v1beta".to_string(),
    };
    
    let client = GoogleClient::new(config);
    let request = client.build_generate_content_request(
        "gemini-2.5-flash",
        "Summarize this",
        Some("Long context text..."),
        None,
    );
    
    assert_eq!(request.contents.len(), 1);
    // Context should be prepended to the prompt
    assert!(request.contents[0].parts[0].text.contains("Long context text..."));
    assert!(request.contents[0].parts[0].text.contains("Summarize this"));
}

#[test]
fn test_build_vision_request() {
    let config = GoogleConfig {
        api_key: "test-key".to_string(),
        base_url: "https://generativelanguage.googleapis.com/v1beta".to_string(),
    };
    
    let client = GoogleClient::new(config);
    let image_data = vec![0xFF, 0xD8, 0xFF, 0xE0]; // Fake JPEG header
    let request = client.build_vision_request(
        "gemini-2.5-flash",
        &image_data,
        Some("What's in this image?"),
    );
    
    assert_eq!(request.contents.len(), 1);
    assert_eq!(request.contents[0].parts.len(), 2);
    // First part should be text prompt
    assert_eq!(request.contents[0].parts[0].text, "What's in this image?");
    // Second part should be inline data with base64
    assert!(request.contents[0].parts[1].inline_data.is_some());
}

#[tokio::test]
#[ignore] // Requires valid API key and network
async fn test_generate_content_integration() {
    let config = GoogleConfig {
        api_key: std::env::var("GEMINI_API_KEY").unwrap_or_default(),
        base_url: "https://generativelanguage.googleapis.com/v1beta".to_string(),
    };
    
    if config.api_key.is_empty() {
        println!("Skipping integration test - no GEMINI_API_KEY");
        return;
    }
    
    let client = GoogleClient::new(config);
    let request = client.build_generate_content_request(
        "gemini-2.5-flash",
        "Say hello in one word",
        None,
        Some(10),
    );
    
    let response = client.generate_content(request).await;
    assert!(response.is_ok());
}

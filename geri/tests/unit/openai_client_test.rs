//! Unit tests for OpenAI Client

use geri::llm::openai::{OpenAIClient, OpenAIConfig, OpenAIError};

#[tokio::test]
async fn test_openai_client_new() {
    let config = OpenAIConfig {
        api_key: "test-key".to_string(),
        base_url: "https://api.openai.com/v1".to_string(),
        timeout_secs: 30,
    };
    
    let client = OpenAIClient::new(config);
    assert_eq!(client.base_url(), "https://api.openai.com/v1");
}

#[tokio::test]
async fn test_openai_client_missing_api_key() {
    let config = OpenAIConfig {
        api_key: "".to_string(),
        base_url: "https://api.openai.com/v1".to_string(),
        timeout_secs: 30,
    };
    
    let client = OpenAIClient::new(config);
    let result = client.validate().await;
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), OpenAIError::InvalidConfig(_)));
}

#[tokio::test]
async fn test_openai_client_chat_completion_request_structure() {
    let config = OpenAIConfig {
        api_key: "sk-test123".to_string(),
        base_url: "https://api.openai.com/v1".to_string(),
        timeout_secs: 30,
    };
    
    let client = OpenAIClient::new(config);
    
    // Test that we can construct a valid request structure
    let request = client.build_chat_request("gpt-4", "Hello", None, Some(100));
    assert_eq!(request.model, "gpt-4");
    assert_eq!(request.messages.len(), 1);
    assert_eq!(request.messages[0].role, "user");
    assert_eq!(request.messages[0].content, "Hello");
    assert_eq!(request.max_tokens, Some(100));
}

#[tokio::test]
async fn test_openai_client_chat_completion_with_context() {
    let config = OpenAIConfig {
        api_key: "sk-test123".to_string(),
        base_url: "https://api.openai.com/v1".to_string(),
        timeout_secs: 30,
    };
    
    let client = OpenAIClient::new(config);
    
    let request = client.build_chat_request(
        "gpt-4",
        "Summarize this",
        Some("Context: Some document text"),
        Some(50),
    );
    
    assert_eq!(request.messages.len(), 2);
    assert_eq!(request.messages[0].role, "system");
    assert!(request.messages[0].content.contains("Context:"));
    assert_eq!(request.messages[1].role, "user");
    assert_eq!(request.messages[1].content, "Summarize this");
}

#[tokio::test]
async fn test_openai_client_vision_request_structure() {
    let config = OpenAIConfig {
        api_key: "sk-test123".to_string(),
        base_url: "https://api.openai.com/v1".to_string(),
        timeout_secs: 30,
    };
    
    let client = OpenAIClient::new(config);
    let image_data = vec![0xFF, 0xD8, 0xFF, 0xE0]; // JPEG header
    
    let request = client.build_vision_request("gpt-4-vision-preview", &image_data, Some("What is in this image?"));
    
    assert_eq!(request.model, "gpt-4-vision-preview");
    assert!(request.messages.len() > 0);
}

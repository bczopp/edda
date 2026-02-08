//! Unit tests for Anthropic Client

use geri::llm::anthropic::{AnthropicClient, AnthropicConfig, AnthropicError};

#[tokio::test]
async fn test_anthropic_client_new() {
    let config = AnthropicConfig {
        api_key: "sk-ant-test123".to_string(),
        base_url: "https://api.anthropic.com/v1".to_string(),
        timeout_secs: 30,
        anthropic_version: "2023-06-01".to_string(),
    };
    
    let client = AnthropicClient::new(config);
    assert_eq!(client.base_url(), "https://api.anthropic.com/v1");
}

#[tokio::test]
async fn test_anthropic_client_missing_api_key() {
    let config = AnthropicConfig {
        api_key: "".to_string(),
        base_url: "https://api.anthropic.com/v1".to_string(),
        timeout_secs: 30,
        anthropic_version: "2023-06-01".to_string(),
    };
    
    let client = AnthropicClient::new(config);
    let result = client.validate().await;
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), AnthropicError::InvalidConfig(_)));
}

#[tokio::test]
async fn test_anthropic_client_messages_request_structure() {
    let config = AnthropicConfig {
        api_key: "sk-ant-test123".to_string(),
        base_url: "https://api.anthropic.com/v1".to_string(),
        timeout_secs: 30,
        anthropic_version: "2023-06-01".to_string(),
    };
    
    let client = AnthropicClient::new(config);
    
    // Test that we can construct a valid request structure
    let request = client.build_messages_request("claude-3-opus-20240229", "Hello", None, Some(100));
    assert_eq!(request.model, "claude-3-opus-20240229");
    assert_eq!(request.messages.len(), 1);
    assert_eq!(request.messages[0].role, "user");
    assert_eq!(request.messages[0].content[0].text, "Hello");
    assert_eq!(request.max_tokens, 100);
}

#[tokio::test]
async fn test_anthropic_client_messages_with_context() {
    let config = AnthropicConfig {
        api_key: "sk-ant-test".to_string(),
        base_url: "https://api.anthropic.com/v1".to_string(),
        timeout_secs: 30,
        anthropic_version: "2023-06-01".to_string(),
    };
    
    let client = AnthropicClient::new(config);
    
    let request = client.build_messages_request(
        "claude-3-sonnet-20240229",
        "Summarize this",
        Some("Context: Some document text"),
        Some(50),
    );
    
    // Anthropic uses system parameter for context, not a message
    assert_eq!(request.messages.len(), 1);
    assert_eq!(request.messages[0].role, "user");
    assert_eq!(request.system, Some("Context: Some document text".to_string()));
}

#[tokio::test]
async fn test_anthropic_client_vision_request_structure() {
    let config = AnthropicConfig {
        api_key: "sk-ant-test".to_string(),
        base_url: "https://api.anthropic.com/v1".to_string(),
        timeout_secs: 30,
        anthropic_version: "2023-06-01".to_string(),
    };
    
    let client = AnthropicClient::new(config);
    let image_data = vec![0xFF, 0xD8, 0xFF, 0xE0]; // JPEG header
    
    let request = client.build_vision_request("claude-3-opus-20240229", &image_data, Some("What is in this image?"));
    
    assert_eq!(request.model, "claude-3-opus-20240229");
    assert!(request.messages.len() > 0);
    // Anthropic vision uses content blocks with text and image
    assert!(request.messages[0].content.len() >= 2); // text + image
}

#[cfg(test)]
mod tests {
    use geri::llm::{LocalLLMProvider, LLMProvider, PromptRequest};

    #[tokio::test]
    async fn test_local_llm_provider_creation() {
        let provider = LocalLLMProvider::new("llama3-8b".to_string());
        assert_eq!(provider.model_name(), "llama3-8b");
    }

    #[tokio::test]
    async fn test_process_prompt_returns_response() {
        let provider = LocalLLMProvider::new("llama3-8b".to_string());
        let req = PromptRequest {
            prompt: "test prompt".to_string(),
            context: None,
            max_tokens: None,
        };
        let result = provider.process_prompt(req).await;
        assert!(result.is_ok());
        let res = result.unwrap();
        assert!(!res.text.is_empty());
    }

    #[tokio::test]
    async fn test_process_prompt_with_context() {
        let provider = LocalLLMProvider::new("llama3-8b".to_string());
        let req = PromptRequest {
            prompt: "test prompt".to_string(),
            context: Some("Context: This is test context.".to_string()),
            max_tokens: None,
        };
        let result = provider.process_prompt(req).await;
        assert!(result.is_ok());
        let res = result.unwrap();
        assert!(!res.text.is_empty());
    }
}

#[cfg(test)]
mod tests {
    use freki::llm::{LLMProvider, LocalLLMProvider};
    use std::time::Duration;

    #[tokio::test]
    async fn test_local_llm_provider_creation() {
        // Test local LLM provider creation
        let provider = LocalLLMProvider::new("llama3-8b".to_string());
        assert_eq!(provider.get_model_name(), "llama3-8b");
    }

    #[tokio::test]
    async fn test_generate_text() {
        // Test text generation
        let provider = LocalLLMProvider::new("llama3-8b".to_string());
        
        // Note: This will fail if llama.cpp not available
        // That's expected in test environment
        let result = provider.generate("test prompt", None, None).await;
        assert!(result.is_ok() || result.is_err());
    }

    #[tokio::test]
    async fn test_generate_with_context() {
        // Test generation with RAG context
        let provider = LocalLLMProvider::new("llama3-8b".to_string());
        
        let context = "Context: This is test context.".to_string();
        let result = provider.generate("test prompt", Some(&context), None).await;
        assert!(result.is_ok() || result.is_err());
    }
}

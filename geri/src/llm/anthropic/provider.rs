//! Anthropic LLM Provider implementing LLMProvider trait

use async_trait::async_trait;
use super::client::{AnthropicClient, AnthropicConfig};
use crate::llm::{LLMError, LLMProvider, PromptRequest, PromptResponse};

pub struct AnthropicLLMProvider {
    client: AnthropicClient,
    model_name: String,
}

impl AnthropicLLMProvider {
    pub fn new(config: AnthropicConfig, model_name: String) -> Self {
        let client = AnthropicClient::new(config);
        Self { client, model_name }
    }
}

#[async_trait]
impl LLMProvider for AnthropicLLMProvider {
    fn model_name(&self) -> &str {
        &self.model_name
    }

    async fn process_prompt(&self, request: PromptRequest) -> Result<PromptResponse, LLMError> {
        // Build Anthropic messages request
        let messages_request = self.client.build_messages_request(
            &self.model_name,
            &request.prompt,
            request.system_prompt.as_deref(),
            request.context.as_deref(),
            request.max_tokens,
        );

        // Send request to Anthropic API
        let response = self
            .client
            .messages(messages_request)
            .await
            .map_err(|e| LLMError::ProcessingFailed(e.to_string()))?;

        // Extract response text from first content block
        let text = response
            .content
            .first()
            .and_then(|c| c.text.clone())
            .ok_or_else(|| LLMError::ProcessingFailed("No content in response".to_string()))?;

        // Calculate total tokens
        let tokens_used = response.usage.input_tokens + response.usage.output_tokens;

        Ok(PromptResponse {
            text,
            tokens_used,
        })
    }
}

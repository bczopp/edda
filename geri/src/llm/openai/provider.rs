//! OpenAI LLM Provider implementing LLMProvider trait

use async_trait::async_trait;
use super::client::{OpenAIClient, OpenAIConfig};
use crate::llm::{LLMError, LLMProvider, PromptRequest, PromptResponse};

pub struct OpenAILLMProvider {
    client: OpenAIClient,
    model_name: String,
}

impl OpenAILLMProvider {
    pub fn new(config: OpenAIConfig, model_name: String) -> Self {
        let client = OpenAIClient::new(config);
        Self { client, model_name }
    }
}

#[async_trait]
impl LLMProvider for OpenAILLMProvider {
    fn model_name(&self) -> &str {
        &self.model_name
    }

    async fn process_prompt(&self, request: PromptRequest) -> Result<PromptResponse, LLMError> {
        // Build OpenAI chat request
        let chat_request = self.client.build_chat_request(
            &self.model_name,
            &request.prompt,
            request.system_prompt.as_deref(),
            request.context.as_deref(),
            request.max_tokens,
        );

        // Send request to OpenAI API
        let response = self
            .client
            .chat_completion(chat_request)
            .await
            .map_err(|e| LLMError::ProcessingFailed(e.to_string()))?;

        // Extract response text from first choice
        let text = response
            .choices
            .first()
            .map(|choice| choice.message.content.clone())
            .ok_or_else(|| LLMError::ProcessingFailed("No choices in response".to_string()))?;

        Ok(PromptResponse {
            text,
            tokens_used: response.usage.total_tokens,
        })
    }
}

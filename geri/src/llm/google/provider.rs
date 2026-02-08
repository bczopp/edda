//! Google LLM Provider implementation

use crate::llm::{LLMProvider, LLMError, PromptRequest, PromptResponse};
use super::client::{GoogleClient, GoogleConfig};

pub struct GoogleLLMProvider {
    client: GoogleClient,
    model_name: String,
}

impl GoogleLLMProvider {
    pub fn new(config: GoogleConfig, model_name: String) -> Self {
        Self {
            client: GoogleClient::new(config),
            model_name,
        }
    }
}

#[async_trait::async_trait]
impl LLMProvider for GoogleLLMProvider {
    fn model_name(&self) -> &str {
        &self.model_name
    }

    async fn process_prompt(&self, request: PromptRequest) -> Result<PromptResponse, LLMError> {
        let generate_request = self.client.build_generate_content_request(
            &self.model_name,
            &request.prompt,
            request.context.as_deref(),
            request.max_tokens,
        );

        let response = self
            .client
            .generate_content(generate_request)
            .await
            .map_err(|e| LLMError::ProcessingFailed(e.to_string()))?;

        // Extract text from first candidate
        let text = response
            .candidates
            .first()
            .and_then(|candidate| {
                candidate.content.parts.first().map(|part| part.text.clone())
            })
            .ok_or_else(|| LLMError::ProcessingFailed("No response from model".to_string()))?;

        let tokens_used = response
            .usage_metadata
            .map(|usage| usage.total_token_count)
            .unwrap_or(0);

        Ok(PromptResponse { text, tokens_used })
    }
}

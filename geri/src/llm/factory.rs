use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use crate::llm::{LLMProvider, LLMError};
use crate::llm::openai::{OpenAILLMProvider, OpenAIConfig};
use crate::llm::anthropic::{AnthropicLLMProvider, AnthropicConfig};
use crate::llm::llamacpp::{LlamaCppLLMProvider, LlamaCppClient, LlamaCppConfig};
use crate::llm::bitnet::{BitNetLLMProvider, BitNetClient, BitNetConfig};

/// Factory and registry for LLM providers
pub struct ProviderFactory {
    providers: Arc<RwLock<HashMap<String, Arc<dyn LLMProvider>>>>,
}

impl ProviderFactory {
    pub fn new() -> Self {
        Self {
            providers: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Register an existing provider instance
    pub async fn register(&self, model_id: &str, provider: Arc<dyn LLMProvider>) {
        let mut providers = self.providers.write().await;
        providers.insert(model_id.to_string(), provider);
    }

    /// Get a provider by model ID
    pub async fn get(&self, model_id: &str) -> Option<Arc<dyn LLMProvider>> {
        let providers = self.providers.read().await;
        providers.get(model_id).cloned()
    }

    /// Create and register an OpenAI provider
    pub async fn add_openai(&self, model_id: &str, config: OpenAIConfig) {
        let provider = Arc::new(OpenAILLMProvider::new(config, model_id.to_string()));
        self.register(model_id, provider).await;
    }

    /// Create and register an Anthropic provider
    pub async fn add_anthropic(&self, model_id: &str, config: AnthropicConfig) {
        let provider = Arc::new(AnthropicLLMProvider::new(config, model_id.to_string()));
        self.register(model_id, provider).await;
    }

    /// Create and register a llama.cpp provider
    pub async fn add_llamacpp(&self, model_id: &str, config: LlamaCppConfig) -> Result<(), String> {
        let client = LlamaCppClient::new(config).map_err(|e| e.to_string())?;
        let provider = Arc::new(LlamaCppLLMProvider::new(client));
        self.register(model_id, provider).await;
        Ok(())
    }

    /// Create and register a BitNet provider
    pub async fn add_bitnet(&self, model_id: &str, config: BitNetConfig) -> Result<(), String> {
        let client = BitNetClient::new(config).map_err(|e| e.to_string())?;
        let provider = Arc::new(BitNetLLMProvider::new(client));
        self.register(model_id, provider).await;
        Ok(())
    }
}

//! Provider-spezifische Token-Counter (Phase 9.1.1): OpenAI, Anthropic, etc.

use crate::prompt::TokenCounter;

/// Token-Counter für OpenAI-Modelle (GPT-4, GPT-3.5, etc.).
#[derive(Debug, Clone)]
pub struct OpenAITokenCounter {
    base: TokenCounter,
}

impl OpenAITokenCounter {
    /// Erstellt einen OpenAI Token-Counter mit dem angegebenen Basis-Counter.
    pub fn new(base: TokenCounter) -> Self {
        Self { base }
    }

    /// Schätzt die Token-Anzahl für den Text beim angegebenen Model (z. B. gpt-4).
    pub fn count(&self, text: &str, model: &str) -> u32 {
        self.base
            .count_for_model(text, &format!("openai-{}", model))
    }
}

/// Token-Counter für Anthropic-Modelle (Claude, etc.).
#[derive(Debug, Clone)]
pub struct AnthropicTokenCounter {
    base: TokenCounter,
}

impl AnthropicTokenCounter {
    /// Erstellt einen Anthropic Token-Counter mit dem angegebenen Basis-Counter.
    pub fn new(base: TokenCounter) -> Self {
        Self { base }
    }

    /// Schätzt die Token-Anzahl für den Text beim angegebenen Model (z. B. claude-3).
    pub fn count(&self, text: &str, model: &str) -> u32 {
        self.base
            .count_for_model(text, &format!("anthropic-{}", model))
    }
}

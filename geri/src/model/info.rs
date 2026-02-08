//! Model-Info Structure (Phase 6.1.1).

use serde::{Deserialize, Serialize};

/// Modell-Typ (LLM, Vision, etc.).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ModelType {
    Llm,
    Vision,
}

impl std::fmt::Display for ModelType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ModelType::Llm => write!(f, "Llm"),
            ModelType::Vision => write!(f, "Vision"),
        }
    }
}

/// Metadaten zu einem registrierten Model (Provider, Typ, Parameter, Hardware, etc.).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelInfo {
    /// Eindeutige Model-ID (z. B. gpt-4-001).
    pub id: String,
    /// Anzeigename (z. B. GPT-4).
    pub name: String,
    /// Provider (z. B. openai, anthropic).
    pub provider: String,
    /// Typ (LLM, Vision).
    pub model_type: ModelType,
    /// Optionale Parameter-Anzahl.
    pub parameter_count: Option<u64>,
    /// Optionale Hardware-Anforderungen (freitext oder strukturiert).
    pub hardware_requirements: Option<String>,
    /// Optionale Context-Window-Größe (Tokens).
    pub context_window: Option<u32>,
    /// Ob das Model lokal oder in der Cloud läuft.
    pub is_local: bool,
    /// Kosten pro input Token (in Dollar).
    pub cost_per_token_input: Option<f64>,
    /// Kosten pro output Token (in Dollar).
    pub cost_per_token_output: Option<f64>,
}

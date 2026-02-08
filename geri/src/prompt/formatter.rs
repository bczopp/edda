//! Prompt-Formatter (Phase 8.1.1): System-Prompt, User-Prompt, Provider-spezifische Formatierung.

/// Formatiert System-Prompt, optionalen RAG-Context und User-Prompt für LLM-Aufrufe.
#[derive(Debug, Clone, Default)]
pub struct PromptFormatter;

impl PromptFormatter {
    /// Formatiert einen vollständigen Prompt aus System-Prompt, User-Prompt und optionalem Context.
    ///
    /// Reihenfolge: System → Context (falls vorhanden) → User.
    /// Leere System-Prompts werden durch einen minimalen Default ersetzt.
    pub fn format(
        &self,
        system_prompt: &str,
        user_prompt: &str,
        context: Option<&str>,
    ) -> String {
        let system = if system_prompt.trim().is_empty() {
            "You are a helpful assistant."
        } else {
            system_prompt.trim()
        };
        let mut parts = vec![system.to_string()];
        if let Some(ctx) = context {
            let t = ctx.trim();
            if !t.is_empty() {
                parts.push(format!("Context:\n{}", t));
            }
        }
        parts.push(user_prompt.trim().to_string());
        parts.join("\n\n")
    }

    /// Formatiert prompt provider-spezifisch (z. B. Llama [INST], OpenAI Chat).
    pub fn format_for_provider(
        &self,
        provider_or_model: &str,
        system_prompt: &str,
        user_prompt: &str,
        context: Option<&str>,
    ) -> String {
        let base = self.format(system_prompt, user_prompt, context);
        if provider_or_model.to_lowercase().contains("llama")
            || provider_or_model.to_lowercase().contains("llama.cpp")
        {
            return format!("[INST] {} [/INST]", base);
        }
        if provider_or_model.to_lowercase().contains("gpt")
            || provider_or_model.to_lowercase().contains("openai")
        {
            let system = system_prompt.trim();
            let mut s = format!("System: {}\n\n", system);
            if let Some(ctx) = context {
                let t = ctx.trim();
                if !t.is_empty() {
                    s.push_str(&format!("Context:\n{}\n\n", t));
                }
            }
            s.push_str(&format!("User: {}", user_prompt.trim()));
            return s;
        }
        base
    }
}

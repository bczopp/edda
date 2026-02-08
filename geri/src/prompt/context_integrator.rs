//! Context-Integrator (Phase 8.2.2): Context zwischen System-Prompt und User-Prompt einfügen, Prompt-Template anwenden.

use super::{ContextDocument, ContextFormatter, PromptFormatter};

/// Fügt RAG-Context zwischen System-Prompt und User-Prompt ein und wendet das Prompt-Template an.
#[derive(Debug, Clone)]
pub struct ContextIntegrator {
    prompt_formatter: PromptFormatter,
    context_formatter: ContextFormatter,
}

impl ContextIntegrator {
    /// Erstellt einen neuen Context-Integrator mit den angegebenen Formatern.
    pub fn new(
        prompt_formatter: PromptFormatter,
        context_formatter: ContextFormatter,
    ) -> Self {
        Self {
            prompt_formatter,
            context_formatter,
        }
    }

    /// Integriert RAG-Dokumente zwischen System- und User-Prompt (Reihenfolge: System → Context → User).
    pub fn integrate(
        &self,
        system_prompt: &str,
        user_prompt: &str,
        context_documents: &[ContextDocument],
    ) -> String {
        let context_str = if context_documents.is_empty() {
            None
        } else {
            Some(self.context_formatter.format(context_documents))
        };
        self.prompt_formatter
            .format(system_prompt, user_prompt, context_str.as_deref())
    }

    /// Wie `integrate`, begrenzt die Gesamtlänge auf ca. `max_total_chars` durch Truncation des Context-Teils.
    pub fn integrate_with_max_chars(
        &self,
        system_prompt: &str,
        user_prompt: &str,
        context_documents: &[ContextDocument],
        max_total_chars: usize,
    ) -> String {
        if context_documents.is_empty() {
            return self.prompt_formatter.format(system_prompt, user_prompt, None);
        }
        let overhead = 20;
        let system_len = system_prompt.trim().len();
        let user_len = user_prompt.trim().len();
        let context_max = max_total_chars
            .saturating_sub(system_len + user_len + overhead)
            .max(0);
        let context_str = self
            .context_formatter
            .format_with_max_chars(context_documents, context_max);
        let context_opt = if context_str.is_empty() {
            None
        } else {
            Some(context_str.as_str())
        };
        self.prompt_formatter
            .format(system_prompt, user_prompt, context_opt)
    }
}

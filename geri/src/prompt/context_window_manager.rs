//! Context-Window-Manager (Phase 8.3.2): Model-Limit prüfen, Response-Reserve 20%, Truncation, Deduplizierung.

use super::{ContextDocument, TokenCounter};

const RESPONSE_RESERVE_RATIO: f32 = 0.2;

/// Prüft Context-Größe gegen Model-Limit, berechnet Response-Reserve (20%), truncation und Deduplizierung.
#[derive(Debug, Clone)]
pub struct ContextWindowManager {
    token_counter: TokenCounter,
}

impl ContextWindowManager {
    /// Erstellt einen neuen Context-Window-Manager mit dem angegebenen Token-Counter.
    pub fn new(token_counter: TokenCounter) -> Self {
        Self { token_counter }
    }

    /// Response-Reserve in Tokens (20% des Model-Limits).
    pub fn response_reserve_tokens(&self, model_limit: u32) -> u32 {
        ((model_limit as f32) * RESPONSE_RESERVE_RATIO) as u32
    }

    /// Maximal nutzbare Context-Tokens (Model-Limit minus 20% Reserve).
    pub fn max_context_tokens(&self, model_limit: u32) -> u32 {
        model_limit.saturating_sub(self.response_reserve_tokens(model_limit))
    }

    /// Schneidet Dokumente relevanzbasiert auf `max_tokens` (Dokumente bereits nach Score sortiert; nimmt von vorne).
    pub fn truncate_to_fit(
        &self,
        documents: &[ContextDocument],
        max_tokens: u32,
        model_name: &str,
    ) -> Vec<ContextDocument> {
        if max_tokens == 0 || documents.is_empty() {
            return vec![];
        }
        let mut acc: u32 = 0;
        let mut out = Vec::new();
        for d in documents {
            let t = self.token_counter.count_for_model(&d.content, model_name);
            if acc + t > max_tokens {
                break;
            }
            acc += t;
            out.push(d.clone());
        }
        out
    }

    /// Entfernt Duplikate nach Dokument-ID (behält erste Vorkommen).
    pub fn deduplicate_by_id(&self, documents: &[ContextDocument]) -> Vec<ContextDocument> {
        let mut seen = std::collections::HashSet::new();
        documents
            .iter()
            .filter(|d| seen.insert(d.id.clone()))
            .cloned()
            .collect()
    }

    /// Prüft, ob System + User + Context in das Model-Limit passen (inkl. 20% Response-Reserve).
    pub fn fits_in_window(
        &self,
        system_prompt: &str,
        user_prompt: &str,
        context_documents: &[ContextDocument],
        model_limit: u32,
        model_name: &str,
    ) -> bool {
        let system_t = self.token_counter.count_for_model(system_prompt, model_name);
        let user_t = self.token_counter.count_for_model(user_prompt, model_name);
        let context_t: u32 = context_documents
            .iter()
            .map(|d| self.token_counter.count_for_model(&d.content, model_name))
            .sum();
        let reserve = self.response_reserve_tokens(model_limit);
        let total = system_t + user_t + context_t + reserve;
        total <= model_limit
    }
}

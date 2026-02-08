//! Token-Counter (Phase 8.3.1): Token-Anzahl für Text berechnen, Model-spezifisches Token-Counting.

/// Schätzt die Token-Anzahl für Text (ohne externes Tokenizer-Modul).
/// Nutzt eine heuristische Näherung (~4 Zeichen pro Token für Englisch).
#[derive(Debug, Clone, Default)]
pub struct TokenCounter;

impl TokenCounter {
    /// Schätzt die Token-Anzahl für den gegebenen Text.
    /// Leerer oder nur aus Whitespace bestehender Text liefert 0.
    pub fn count(&self, text: &str) -> u32 {
        let t = text.trim();
        if t.is_empty() {
            return 0;
        }
        let chars = t.chars().count();
        ((chars + 3) / 4).min(u32::MAX as usize) as u32
    }

    /// Schätzt die Token-Anzahl model-spezifisch.
    /// Verschiedene Modelle können unterschiedliche Token-Verhältnisse haben;
    /// aktuell wird dieselbe Heuristik verwendet, erweiterbar um tiktoken/Model-Vocab.
    pub fn count_for_model(&self, text: &str, _model_name: &str) -> u32 {
        self.count(text)
    }
}

//! Test-Data-Generators für Prompts (Phase 1.2.2).
//! Stellt wiederverwendbare Beispiel-Prompts für Unit- und Integrationstests bereit.

/// Beispiel-System-Prompt für Tests.
pub fn sample_system_prompt() -> String {
    "You are a helpful assistant.".to_string()
}

/// Beispiel-User-Prompt (kurz) für Tests.
pub fn sample_user_prompt() -> String {
    "What is 2 + 2?".to_string()
}

/// Beispiel-User-Prompt (länger) für Context-Window-Tests.
pub fn sample_long_user_prompt() -> String {
    "Explain in detail how retrieval-augmented generation works, including embedding, \
     similarity search, and context injection for LLMs."
        .to_string()
}

/// Beispiel-RAG-Context für Tests (Format wie von Freki).
pub fn sample_rag_context() -> String {
    "[Document 1: doc-001]\nRAG combines retrieval with generation. \
     [Document 2: doc-002]\nEmbeddings are used for similarity search."
        .to_string()
}

/// Leerer Prompt für Randfälle.
pub fn empty_prompt() -> String {
    String::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_prompts_non_empty() {
        assert!(!sample_system_prompt().is_empty());
        assert!(!sample_user_prompt().is_empty());
        assert!(sample_long_user_prompt().len() > sample_user_prompt().len());
        assert!(!sample_rag_context().is_empty());
    }

    #[test]
    fn empty_prompt_is_empty() {
        assert!(empty_prompt().is_empty());
    }
}

//! Context-Formatter (Phase 8.2.1): RAG-Context formatieren, nach Relevanz sortieren, Metadaten beibehalten.

use serde::{Deserialize, Serialize};

/// Ein Dokument aus dem RAG-Context (z. B. von Freki) mit Relevanz-Score und optionalen Metadaten.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextDocument {
    /// Dokument- oder Chunk-ID.
    pub id: String,
    /// Textinhalt.
    pub content: String,
    /// Relevanz-Score (0.0–1.0, höher = relevanter).
    pub score: f32,
    /// Optionale Metadaten (z. B. source, title).
    #[serde(default)]
    pub metadata: Option<serde_json::Value>,
}

/// Formatiert eine Liste von RAG-Dokumenten zu einem Context-String für den LLM-Prompt.
#[derive(Debug, Clone, Default)]
pub struct ContextFormatter;

impl ContextFormatter {
    /// Formatiert Dokumente nach Relevanz sortiert (höchster Score zuerst).
    /// Ausgabe: `[Document 1: id]\ncontent\n\n[Document 2: id]\ncontent...`
    pub fn format(&self, documents: &[ContextDocument]) -> String {
        if documents.is_empty() {
            return String::new();
        }
        let mut sorted: Vec<_> = documents.iter().collect();
        sorted.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));
        sorted
            .iter()
            .enumerate()
            .map(|(i, d)| format!("[Document {}: {}]\n{}", i + 1, d.id, d.content.trim()))
            .collect::<Vec<_>>()
            .join("\n\n")
    }

    /// Wie `format`, begrenzt die Gesamtlänge auf `max_chars` (char-boundary-sicher).
    pub fn format_with_max_chars(&self, documents: &[ContextDocument], max_chars: usize) -> String {
        let full = self.format(documents);
        if full.len() <= max_chars {
            return full;
        }
        let mut end = max_chars;
        while end > 0 && !full.is_char_boundary(end) {
            end -= 1;
        }
        full[..end].to_string()
    }
}

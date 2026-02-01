//! Context-Formatter (Phase 9.4): Kontext für LLM formatieren, RAGContext mit Traceability.

use crate::retrieval::RetrievedDocument;
use serde::{Deserialize, Serialize};

/// Formatierter RAG-Kontext für das LLM (strukturiert, mit Dokument-IDs für Traceability).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RAGContext {
    /// Formatierter Text: "[Document 1: id] content\n\n[Document 2: id] content..."
    pub formatted_text: String,
    /// Dokument-IDs in Reihenfolge (für Traceability).
    pub document_ids: Vec<String>,
}

/// Formatiert abgerufene Dokumente für das LLM.
pub struct ContextFormatter;

impl ContextFormatter {
    pub fn new() -> Self {
        Self
    }

    /// Kontext im strukturierten Format erstellen: [Document N: document_id] content...
    pub fn format(&self, documents: Vec<RetrievedDocument>) -> RAGContext {
        let document_ids: Vec<String> = documents.iter().map(|d| d.id.clone()).collect();
        let mut parts = Vec::with_capacity(documents.len());
        for (i, doc) in documents.iter().enumerate() {
            parts.push(format!("[Document {}: {}]\n{}", i + 1, doc.id, doc.content));
        }
        let formatted_text = parts.join("\n\n");
        RAGContext {
            formatted_text,
            document_ids,
        }
    }
}

impl Default for ContextFormatter {
    fn default() -> Self {
        Self::new()
    }
}

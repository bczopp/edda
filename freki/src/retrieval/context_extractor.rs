//! Context-Extractor (Phase 9.3): Relevante Passagen extrahieren, kombinieren, optional Länge begrenzen.

use crate::retrieval::RetrievedDocument;

/// Eine extrahierte Passage (Dokument-ID + Inhalt).
#[derive(Debug, Clone)]
pub struct ExtractedPassage {
    pub document_id: String,
    pub content: String,
}

/// Ergebnis der Kontext-Extraktion: Passagen und optional begrenzte Gesamtlänge.
#[derive(Debug, Clone)]
pub struct ExtractedContext {
    pub passages: Vec<ExtractedPassage>,
    /// Gesamttext (alle Passagen kombiniert, ggf. bei max_chars abgeschnitten).
    pub combined: String,
}

/// Extrahiert relevante Text-Passagen aus abgerufenen Dokumenten und kombiniert sie.
pub struct ContextExtractor {
    /// Maximale Gesamtzeichenanzahl (None = unbegrenzt).
    max_total_chars: Option<usize>,
}

impl ContextExtractor {
    pub fn new() -> Self {
        Self {
            max_total_chars: None,
        }
    }

    pub fn with_max_chars(mut self, max_chars: usize) -> Self {
        self.max_total_chars = Some(max_chars);
        self
    }

    /// Relevante Passagen extrahieren, kombinieren, optional auf max_total_chars kürzen.
    pub fn extract(&self, documents: Vec<RetrievedDocument>) -> ExtractedContext {
        let passages: Vec<ExtractedPassage> = documents
            .into_iter()
            .map(|d| ExtractedPassage {
                document_id: d.id,
                content: d.content,
            })
            .collect();

        let mut combined: String = passages.iter().map(|p| p.content.as_str()).collect::<Vec<_>>().join("\n\n");
        if let Some(max) = self.max_total_chars {
            if combined.len() > max {
                combined.truncate(max);
                while !combined.is_empty() && !combined.is_char_boundary(combined.len()) {
                    combined.pop();
                }
            }
        }

        ExtractedContext { passages, combined }
    }
}

impl Default for ContextExtractor {
    fn default() -> Self {
        Self::new()
    }
}

//! Metadata-Extractor (Phase 6.2): Standard- und optionale Metadaten aus Dokumenten.

use crate::indexing::Document;
use serde_json::{Map, Value};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum MetadataError {
    #[error("Metadata extraction error: {0}")]
    Extraction(String),
}

/// Extrahiert oder ergänzt Metadaten für ein Document.
pub struct MetadataExtractor;

impl MetadataExtractor {
    pub fn new() -> Self {
        Self
    }

    /// Ergänzt Standard-Metadaten (z. B. title aus erster Zeile, falls fehlend).
    pub fn extract(&self, document: &Document) -> Result<Value, MetadataError> {
        let mut meta = document.metadata.clone();
        if let Some(obj) = meta.as_object_mut() {
            if !obj.contains_key("title") {
                let title = document
                    .content
                    .lines()
                    .next()
                    .map(|l| l.trim().to_string())
                    .unwrap_or_default();
                if !title.is_empty() {
                    obj.insert("title".to_string(), Value::String(title));
                }
            }
        } else {
            let mut obj = Map::new();
            let title = document
                .content
                .lines()
                .next()
                .map(|l| l.trim().to_string())
                .unwrap_or_default();
            if !title.is_empty() {
                obj.insert("title".to_string(), Value::String(title));
            }
            meta = Value::Object(obj);
        }
        Ok(meta)
    }
}

impl Default for MetadataExtractor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_extract_adds_title_from_first_line() {
        let ext = MetadataExtractor::new();
        let doc = Document {
            id: "1".to_string(),
            content: "My Document Title\nSecond line.".to_string(),
            metadata: json!({}),
        };
        let meta = ext.extract(&doc).unwrap();
        assert_eq!(meta.get("title").and_then(|v| v.as_str()), Some("My Document Title"));
    }

    #[test]
    fn test_extract_preserves_existing_metadata() {
        let ext = MetadataExtractor::new();
        let doc = Document {
            id: "1".to_string(),
            content: "Body".to_string(),
            metadata: json!({ "author": "Test" }),
        };
        let meta = ext.extract(&doc).unwrap();
        assert_eq!(meta.get("author").and_then(|v| v.as_str()), Some("Test"));
    }
}

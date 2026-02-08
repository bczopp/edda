//! Document-Parser (Phase 6.1): Trait und Text-Parser für .txt, .md.

use crate::indexing::Document;
use serde_json::json;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ParserError {
    #[error("Unsupported file type: {0}")]
    UnsupportedType(String),
    #[error("Parse error: {0}")]
    Parse(String),
}

/// Parser für Dokumente; liefert Document mit content und optionalen Metadaten.
pub trait DocumentParser: Send + Sync {
    /// Parst Rohdaten zu einem Document.
    fn parse_document(&self, bytes: &[u8], _file_extension: &str) -> Result<Document, ParserError>;

    /// Gibt true zurück, wenn der Parser diesen Dateityp unterstützt.
    fn supports_file_type(&self, extension: &str) -> bool;
}

/// Einfacher Text-Parser für .txt und .md.
pub struct TextParser;

impl TextParser {
    pub fn new() -> Self {
        Self
    }
}

impl Default for TextParser {
    fn default() -> Self {
        Self::new()
    }
}

impl DocumentParser for TextParser {
    fn parse_document(&self, bytes: &[u8], file_extension: &str) -> Result<Document, ParserError> {
        if !self.supports_file_type(file_extension) {
            return Err(ParserError::UnsupportedType(file_extension.to_string()));
        }
        let content = String::from_utf8(bytes.to_vec())
            .map_err(|e| ParserError::Parse(e.to_string()))?;
        Ok(Document {
            id: uuid::Uuid::new_v4().to_string(),
            content,
            metadata: json!({ "source_extension": file_extension }),
        })
    }

    fn supports_file_type(&self, extension: &str) -> bool {
        let ext = extension.trim_start_matches('.').to_lowercase();
        matches!(ext.as_str(), "txt" | "md" | "markdown")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_supports_txt_and_md() {
        let p = TextParser::new();
        assert!(p.supports_file_type("txt"));
        assert!(p.supports_file_type(".md"));
        assert!(p.supports_file_type("markdown"));
        assert!(!p.supports_file_type("pdf"));
    }

    #[test]
    fn test_parse_document_returns_document() {
        let p = TextParser::new();
        let bytes = b"Hello world.";
        let doc = p.parse_document(bytes, "txt").unwrap();
        assert_eq!(doc.content, "Hello world.");
        assert!(doc.metadata.get("source_extension").is_some());
    }

    #[test]
    fn test_parse_unsupported_type_returns_error() {
        let p = TextParser::new();
        let bytes = b"raw";
        let res = p.parse_document(bytes, "pdf");
        assert!(res.is_err());
    }
}

//! Test-Document-Generators (Phase 1.2.2): Erzeugen von Document-Instanzen für Unit- und Integrationstests.

use freki::indexing::Document;
use serde_json::json;

/// Liefert ein Standard-Testdokument.
pub fn sample_document() -> Document {
    Document {
        id: "test-doc-1".to_string(),
        content: "This is sample content for testing. It has multiple sentences. Use it in indexing or retrieval tests.".to_string(),
        metadata: json!({ "title": "Test Document", "source": "test" }),
    }
}

/// Liefert `n` Testdokumente mit eindeutigen IDs und Inhalten.
pub fn sample_documents(n: usize) -> Vec<Document> {
    (0..n)
        .map(|i| document_with_content(&format!("test-doc-{}", i), &format!("Sample content for document {}.", i)))
        .collect()
}

/// Erstellt ein Dokument mit vorgegebener ID und Inhalt; Metadaten optional.
pub fn document_with_content(id: &str, content: &str) -> Document {
    Document {
        id: id.to_string(),
        content: content.to_string(),
        metadata: json!({}),
    }
}

/// Erstellt ein Dokument mit ID, Inhalt und Metadaten.
pub fn document_with_metadata(id: &str, content: &str, metadata: serde_json::Value) -> Document {
    Document {
        id: id.to_string(),
        content: content.to_string(),
        metadata,
    }
}

#[cfg(test)]
mod tests {
    use freki::retrieval::{ContextFormatter, RAGContext, RetrievedDocument};

    fn doc(id: &str, content: &str) -> RetrievedDocument {
        RetrievedDocument {
            id: id.to_string(),
            content: content.to_string(),
            metadata: serde_json::Value::Object(serde_json::Map::new()),
            score: 0.9,
        }
    }

    #[test]
    fn test_format_produces_structured_output() {
        let formatter = ContextFormatter::new();
        let docs = vec![doc("doc-1", "Content one"), doc("doc-2", "Content two")];
        let rag = formatter.format(docs);
        assert_eq!(rag.document_ids, vec!["doc-1", "doc-2"]);
        assert!(rag.formatted_text.contains("[Document 1: doc-1]"));
        assert!(rag.formatted_text.contains("Content one"));
        assert!(rag.formatted_text.contains("[Document 2: doc-2]"));
        assert!(rag.formatted_text.contains("Content two"));
    }

    #[test]
    fn test_format_empty_documents() {
        let formatter = ContextFormatter::new();
        let rag = formatter.format(vec![]);
        assert!(rag.document_ids.is_empty());
        assert!(rag.formatted_text.is_empty());
    }

    #[test]
    fn test_format_single_document() {
        let formatter = ContextFormatter::new();
        let rag = formatter.format(vec![doc("id-1", "Only content")]);
        assert_eq!(rag.document_ids, vec!["id-1"]);
        assert_eq!(rag.formatted_text, "[Document 1: id-1]\nOnly content");
    }

    #[test]
    fn test_rag_context_traceability() {
        let formatter = ContextFormatter::new();
        let docs = vec![doc("a", "A"), doc("b", "B")];
        let rag = formatter.format(docs);
        assert_eq!(rag.document_ids.len(), 2);
        assert_eq!(rag.document_ids[0], "a");
        assert_eq!(rag.document_ids[1], "b");
    }
}

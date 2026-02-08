#[cfg(test)]
mod tests {
    use freki::retrieval::{ContextExtractor, ExtractedContext, ExtractedPassage, RetrievedDocument};

    fn doc(id: &str, content: &str) -> RetrievedDocument {
        RetrievedDocument {
            id: id.to_string(),
            content: content.to_string(),
            metadata: serde_json::Value::Object(serde_json::Map::new()),
            score: 0.9,
        }
    }

    #[test]
    fn test_extract_combines_multiple_documents() {
        let extractor = ContextExtractor::new();
        let docs = vec![
            doc("a", "Content A"),
            doc("b", "Content B"),
        ];
        let ctx = extractor.extract(docs);
        assert_eq!(ctx.passages.len(), 2);
        assert_eq!(ctx.passages[0].document_id, "a");
        assert_eq!(ctx.passages[0].content, "Content A");
        assert_eq!(ctx.passages[1].document_id, "b");
        assert_eq!(ctx.passages[1].content, "Content B");
        assert!(ctx.combined.contains("Content A"));
        assert!(ctx.combined.contains("Content B"));
    }

    #[test]
    fn test_extract_respects_max_chars() {
        let extractor = ContextExtractor::new().with_max_chars(10);
        let docs = vec![doc("a", "Hello world"), doc("b", "More text")];
        let ctx = extractor.extract(docs);
        assert!(ctx.combined.len() <= 10);
        assert_eq!(ctx.passages.len(), 2);
    }

    #[test]
    fn test_extract_empty_documents() {
        let extractor = ContextExtractor::new();
        let ctx = extractor.extract(vec![]);
        assert!(ctx.passages.is_empty());
        assert!(ctx.combined.is_empty());
    }

    #[test]
    fn test_extract_single_document() {
        let extractor = ContextExtractor::new();
        let ctx = extractor.extract(vec![doc("x", "Single content")]);
        assert_eq!(ctx.passages.len(), 1);
        assert_eq!(ctx.passages[0].content, "Single content");
        assert_eq!(ctx.combined, "Single content");
    }
}

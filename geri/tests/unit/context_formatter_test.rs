//! Tests für Context-Formatter (Phase 8.2.1).

#[cfg(test)]
mod tests {
    use geri::prompt::{ContextDocument, ContextFormatter};

    fn doc(id: &str, content: &str, score: f32) -> ContextDocument {
        ContextDocument {
            id: id.to_string(),
            content: content.to_string(),
            score,
            metadata: None,
        }
    }

    #[test]
    fn format_sorts_by_relevance_descending() {
        let formatter = ContextFormatter::default();
        let docs = vec![
            doc("low", "Low relevance", 0.3),
            doc("high", "High relevance", 0.9),
            doc("mid", "Mid relevance", 0.6),
        ];
        let out = formatter.format(&docs);
        let h = out.find("High relevance").expect("High");
        let m = out.find("Mid relevance").expect("Mid");
        let l = out.find("Low relevance").expect("Low");
        assert!(h < m);
        assert!(m < l);
    }

    #[test]
    fn format_produces_document_sections() {
        let formatter = ContextFormatter::default();
        let docs = vec![
            doc("doc-1", "First content.", 0.8),
            doc("doc-2", "Second content.", 0.7),
        ];
        let out = formatter.format(&docs);
        assert!(out.contains("[Document 1: doc-1]"));
        assert!(out.contains("First content."));
        assert!(out.contains("[Document 2: doc-2]"));
        assert!(out.contains("Second content."));
    }

    #[test]
    fn format_empty_returns_empty_string() {
        let formatter = ContextFormatter::default();
        let out = formatter.format(&[]);
        assert!(out.is_empty());
    }

    #[test]
    fn format_single_document() {
        let formatter = ContextFormatter::default();
        let docs = vec![doc("only", "Only one.", 1.0)];
        let out = formatter.format(&docs);
        assert_eq!(out, "[Document 1: only]\nOnly one.");
    }

    #[test]
    fn format_preserves_metadata_in_struct() {
        let formatter = ContextFormatter::default();
        let mut d = doc("with-meta", "Content.", 0.5);
        d.metadata = Some(serde_json::json!({"source": "wiki"}));
        let out = formatter.format(&[d.clone()]);
        assert!(out.contains("with-meta"));
        assert!(out.contains("Content."));
        assert!(d.metadata.as_ref().unwrap().get("source").unwrap().as_str() == Some("wiki"));
    }

    #[test]
    fn format_with_max_chars_truncates() {
        let formatter = ContextFormatter::default();
        let docs = vec![
            doc("a", "Short.", 0.9),
            doc("b", "Also short.", 0.8),
        ];
        let out = formatter.format_with_max_chars(&docs, 50);
        assert!(out.len() <= 50, "output len {} > 50", out.len());
        assert!(out.contains("Short."));
    }
}

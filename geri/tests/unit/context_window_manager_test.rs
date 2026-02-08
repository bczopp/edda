//! Tests für Context-Window-Manager (Phase 8.3.2).

#[cfg(test)]
mod tests {
    use geri::prompt::{ContextDocument, ContextWindowManager, TokenCounter};

    fn doc(id: &str, content: &str, score: f32) -> ContextDocument {
        ContextDocument {
            id: id.to_string(),
            content: content.to_string(),
            score,
            metadata: None,
        }
    }

    #[test]
    fn response_reserve_is_20_percent() {
        let mgr = ContextWindowManager::new(TokenCounter::default());
        let reserve = mgr.response_reserve_tokens(1000);
        assert_eq!(reserve, 200);
    }

    #[test]
    fn max_context_tokens_leaves_reserve() {
        let mgr = ContextWindowManager::new(TokenCounter::default());
        let max_ctx = mgr.max_context_tokens(1000);
        assert_eq!(max_ctx, 800);
    }

    #[test]
    fn truncate_to_fit_keeps_high_relevance() {
        let mgr = ContextWindowManager::new(TokenCounter::default());
        let docs = vec![
            doc("a", "High relevance content here.", 0.9),
            doc("b", "Lower relevance content.", 0.5),
        ];
        let truncated = mgr.truncate_to_fit(&docs, 15, "gpt-4");
        assert!(!truncated.is_empty());
        assert!(truncated.first().unwrap().score >= truncated.last().unwrap().score);
    }

    #[test]
    fn truncate_to_fit_empty_when_max_zero() {
        let mgr = ContextWindowManager::new(TokenCounter::default());
        let docs = vec![doc("a", "Content.", 0.8)];
        let truncated = mgr.truncate_to_fit(&docs, 0, "gpt-4");
        assert!(truncated.is_empty());
    }

    #[test]
    fn deduplicate_removes_duplicate_ids() {
        let mgr = ContextWindowManager::new(TokenCounter::default());
        let docs = vec![
            doc("same", "First.", 0.9),
            doc("same", "Second.", 0.7),
            doc("other", "Other.", 0.8),
        ];
        let dedup = mgr.deduplicate_by_id(&docs);
        assert_eq!(dedup.len(), 2);
        let ids: Vec<_> = dedup.iter().map(|d| d.id.as_str()).collect();
        assert!(ids.contains(&"same"));
        assert!(ids.contains(&"other"));
    }

    #[test]
    fn fits_in_window_true_when_under_limit() {
        let mgr = ContextWindowManager::new(TokenCounter::default());
        let docs = vec![doc("a", "Short.", 0.8)];
        let fits = mgr.fits_in_window("S", "U", &docs, 1000, "gpt-4");
        assert!(fits);
    }

    #[test]
    fn fits_in_window_false_when_over_limit() {
        let mgr = ContextWindowManager::new(TokenCounter::default());
        let long = "x".repeat(5000);
        let docs = vec![doc("a", &long, 0.8)];
        let fits = mgr.fits_in_window("System", "User", &docs, 100, "gpt-4");
        assert!(!fits);
    }
}

#[cfg(test)]
mod tests {
    use freki::retrieval::{DocumentRanker, RetrievedDocument};

    fn doc(id: &str, score: f32) -> RetrievedDocument {
        RetrievedDocument {
            id: id.to_string(),
            content: String::new(),
            metadata: serde_json::Value::Object(serde_json::Map::new()),
            score,
        }
    }

    #[test]
    fn test_rank_sorts_by_score_descending() {
        let ranker = DocumentRanker::new(0.0, 10);
        let docs = vec![doc("a", 0.3), doc("b", 0.9), doc("c", 0.5)];
        let out = ranker.rank(docs);
        assert_eq!(out.len(), 3);
        assert_eq!(out[0].id, "b");
        assert_eq!(out[0].score, 0.9);
        assert_eq!(out[1].id, "c");
        assert_eq!(out[2].id, "a");
    }

    #[test]
    fn test_rank_applies_threshold() {
        let ranker = DocumentRanker::new(0.5, 10);
        let docs = vec![doc("a", 0.3), doc("b", 0.9), doc("c", 0.5)];
        let out = ranker.rank(docs);
        assert_eq!(out.len(), 2);
        assert_eq!(out[0].id, "b");
        assert_eq!(out[1].id, "c");
    }

    #[test]
    fn test_rank_applies_top_k() {
        let ranker = DocumentRanker::new(0.0, 2);
        let docs = vec![doc("a", 0.3), doc("b", 0.9), doc("c", 0.5)];
        let out = ranker.rank(docs);
        assert_eq!(out.len(), 2);
        assert_eq!(out[0].id, "b");
        assert_eq!(out[1].id, "c");
    }

    #[test]
    fn test_rank_empty() {
        let ranker = DocumentRanker::new(0.0, 5);
        let out = ranker.rank(vec![]);
        assert!(out.is_empty());
    }

    #[test]
    fn test_rank_single_doc() {
        let ranker = DocumentRanker::new(0.0, 5);
        let out = ranker.rank(vec![doc("x", 0.7)]);
        assert_eq!(out.len(), 1);
        assert_eq!(out[0].id, "x");
        assert_eq!(out[0].score, 0.7);
    }
}

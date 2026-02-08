//! Document-Ranker (Phase 9.2): Nach Relevanz-Score ranken, Threshold, Top-K.

use crate::retrieval::RetrievedDocument;

/// Rankt Dokumente nach Relevanz-Score, wendet Threshold an und begrenzt auf Top-K.
pub struct DocumentRanker {
    score_threshold: f32,
    top_k: usize,
}

impl DocumentRanker {
    pub fn new(score_threshold: f32, top_k: usize) -> Self {
        Self {
            score_threshold,
            top_k: top_k.max(1),
        }
    }

    /// Dokumente nach Score (absteigend) sortieren, Threshold anwenden, Top-K zurückgeben.
    pub fn rank(&self, documents: Vec<RetrievedDocument>) -> Vec<RetrievedDocument> {
        let mut out: Vec<RetrievedDocument> = documents
            .into_iter()
            .filter(|d| d.score >= self.score_threshold)
            .collect();
        out.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));
        out.into_iter().take(self.top_k).collect()
    }
}

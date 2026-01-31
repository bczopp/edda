/// Sentence boundary detector
pub struct SentenceBoundaryDetector;

impl SentenceBoundaryDetector {
    pub fn new() -> Self {
        Self
    }

    /// Split text into sentences
    pub fn detect_sentences(&self, text: &str) -> Vec<String> {
        // Simple sentence splitting based on punctuation
        // In a real implementation, would use more sophisticated NLP
        text.split(&['.', '!', '?'][..])
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect()
    }
}

impl Default for SentenceBoundaryDetector {
    fn default() -> Self {
        Self::new()
    }
}

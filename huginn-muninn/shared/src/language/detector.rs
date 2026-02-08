//! Language Detection for Huginn & Muninn

use tracing::{debug, warn};
use std::collections::HashMap;

/// Language Detector for automatic language detection
pub struct LanguageDetector {
    default_language: String,
    min_confidence: f64,
}

impl LanguageDetector {
    /// Create a new Language Detector
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        debug!("Creating LanguageDetector");
        Ok(Self {
            default_language: "en-US".to_string(),
            min_confidence: 0.5,
        })
    }
    
    /// Detect language from text
    pub async fn detect(&self, text: &str) -> Result<(String, f64), Box<dyn std::error::Error>> {
        if text.is_empty() {
            debug!("Empty text, returning default language");
            return Ok((self.default_language.clone(), 1.0));
        }
        
        // Simple heuristic-based detection (can be replaced with proper library later)
        // For now, use basic pattern matching
        let detected = self.detect_simple(text);
        
        if detected.1 >= self.min_confidence {
            debug!("Detected language: {} with confidence: {}", detected.0, detected.1);
            Ok(detected)
        } else {
            warn!("Low confidence detection ({}), using default language", detected.1);
            Ok((self.default_language.clone(), 0.5))
        }
    }
    
    /// Simple heuristic-based language detection
    fn detect_simple(&self, text: &str) -> (String, f64) {
        let text_lower = text.to_lowercase();
        
        // Common German words/patterns
        let german_patterns = vec!["der", "die", "das", "und", "ist", "wie", "geht", "dir", "hallo"];
        let german_count = german_patterns.iter()
            .map(|pattern| text_lower.matches(pattern).count())
            .sum::<usize>();
        
        // Common French words/patterns
        let french_patterns = vec!["le", "la", "les", "et", "est", "comment", "ça", "bonjour"];
        let french_count = french_patterns.iter()
            .map(|pattern| text_lower.matches(pattern).count())
            .sum::<usize>();
        
        // Common Spanish words/patterns
        let spanish_patterns = vec!["el", "la", "y", "es", "cómo", "estás", "hola"];
        let spanish_count = spanish_patterns.iter()
            .map(|pattern| text_lower.matches(pattern).count())
            .sum::<usize>();
        
        // Calculate confidence based on pattern matches
        let total_words = text.split_whitespace().count();
        if total_words == 0 {
            return (self.default_language.clone(), 0.5);
        }
        
        let german_ratio = german_count as f64 / total_words as f64;
        let french_ratio = french_count as f64 / total_words as f64;
        let spanish_ratio = spanish_count as f64 / total_words as f64;
        
        // Determine language based on highest ratio
        if german_ratio > 0.1 && german_ratio > french_ratio && german_ratio > spanish_ratio {
            let confidence = (german_ratio * 2.0).min(0.9);
            ("de-DE".to_string(), confidence)
        } else if french_ratio > 0.1 && french_ratio > spanish_ratio {
            let confidence = (french_ratio * 2.0).min(0.9);
            ("fr-FR".to_string(), confidence)
        } else if spanish_ratio > 0.1 {
            let confidence = (spanish_ratio * 2.0).min(0.9);
            ("es-ES".to_string(), confidence)
        } else {
            // Default to English
            let confidence = 0.7; // Moderate confidence for English
            ("en-US".to_string(), confidence)
        }
    }
    
    /// Set default language
    pub fn set_default_language(&mut self, language: String) {
        self.default_language = language;
    }
    
    /// Get default language
    pub fn default_language(&self) -> &str {
        &self.default_language
    }
}

impl Default for LanguageDetector {
    fn default() -> Self {
        Self::new().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_detect_simple_english() {
        let detector = LanguageDetector::new().unwrap();
        let (lang, confidence) = detector.detect_simple("Hello world, how are you today?");
        assert_eq!(lang, "en-US");
        assert!(confidence > 0.0);
    }
    
    #[tokio::test]
    async fn test_detect_simple_german() {
        let detector = LanguageDetector::new().unwrap();
        let (lang, confidence) = detector.detect_simple("Hallo Welt, wie geht es dir heute?");
        assert_eq!(lang, "de-DE");
        assert!(confidence > 0.0);
    }
}

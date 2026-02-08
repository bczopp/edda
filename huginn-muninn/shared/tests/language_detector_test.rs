//! Tests for Language Detector

use shared::language::LanguageDetector;

#[tokio::test]
async fn test_language_detector_new() {
    let detector = LanguageDetector::new();
    assert!(detector.is_ok());
}

#[tokio::test]
async fn test_language_detector_detect() {
    let detector = LanguageDetector::new().unwrap();
    
    // Test English
    let result = detector.detect("Hello, how are you?").await;
    assert!(result.is_ok());
    let (lang, confidence) = result.unwrap();
    assert_eq!(lang, "en-US");
    assert!(confidence > 0.5);
}

#[tokio::test]
async fn test_language_detector_detect_german() {
    let detector = LanguageDetector::new().unwrap();
    
    // Test German
    let result = detector.detect("Hallo, wie geht es dir?").await;
    assert!(result.is_ok());
    let (lang, confidence) = result.unwrap();
    assert_eq!(lang, "de-DE");
    assert!(confidence > 0.5);
}

#[tokio::test]
async fn test_language_detector_detect_empty() {
    let detector = LanguageDetector::new().unwrap();
    
    // Empty text should return default language
    let result = detector.detect("").await;
    assert!(result.is_ok());
    let (lang, _) = result.unwrap();
    assert_eq!(lang, "en-US"); // Default
}

#[tokio::test]
async fn test_language_detector_detect_short() {
    let detector = LanguageDetector::new().unwrap();
    
    // Very short text should return default language
    let result = detector.detect("Hi").await;
    assert!(result.is_ok());
    let (lang, _) = result.unwrap();
    assert_eq!(lang, "en-US"); // Default for short text
}

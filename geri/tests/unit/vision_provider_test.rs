//! Tests für VisionProvider-Trait (Phase 3.1.2).

use geri::vision::{VisionProcessor, VisionRequest};

#[tokio::test]
async fn vision_provider_processor_implements_trait() {
    let processor = VisionProcessor::new("test-model".to_string());
    assert_eq!(processor.model_name(), "test-model");
}

#[tokio::test]
async fn vision_provider_process_returns_description() {
    let processor = VisionProcessor::new("gpt-4v".to_string());
    let req = VisionRequest {
        image_data: vec![0u8; 100],
        prompt: Some("What is in the image?".to_string()),
    };
    let res = processor.process(req).await.expect("process");
    assert!(res.description.contains("gpt-4v"));
    assert!(res.description.contains("100"));
    assert!(res.analysis.get("has_prompt").and_then(|v| v.as_bool()).unwrap());
}

#[tokio::test]
async fn vision_provider_process_without_prompt() {
    let processor = VisionProcessor::new("vision".to_string());
    let req = VisionRequest {
        image_data: vec![],
        prompt: None,
    };
    let res = processor.process(req).await.expect("process");
    assert!(res.description.contains("Has prompt: false"));
}

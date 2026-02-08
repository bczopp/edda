//! Tests for Text Input Processor

use huginn::text_input::TextInputProcessor;

#[tokio::test]
async fn test_text_input_processor_new() {
    let processor = TextInputProcessor::new();
    // Processor should be created successfully
    assert!(true); // No-op, just verify creation doesn't panic
}

#[tokio::test]
async fn test_text_input_processor_process() {
    let processor = TextInputProcessor::new();
    let result = processor.process("Hello, World!", "user123", "device456").await;
    assert!(result.is_ok());
    
    let raven_message = result.unwrap();
    assert_eq!(raven_message.content, "Hello, World!");
    assert_eq!(raven_message.direction, 0); // Incoming
    assert!(raven_message.metadata.is_some());
    let metadata = raven_message.metadata.unwrap();
    assert_eq!(metadata.user_id, "user123");
    assert_eq!(metadata.device_id, "device456");
    assert_eq!(metadata.confidence, 1.0);
}

#[tokio::test]
async fn test_text_input_processor_empty_text() {
    let processor = TextInputProcessor::new();
    let result = processor.process("", "user123", "device456").await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_text_input_processor_create_raven_message() {
    let processor = TextInputProcessor::new();
    let message = processor.create_raven_message("Test message", "user1", "device1").unwrap();
    
    assert_eq!(message.content, "Test message");
    assert_eq!(message.direction, 0); // Incoming
    assert!(message.metadata.is_some());
    let metadata = message.metadata.unwrap();
    assert_eq!(metadata.user_id, "user1");
    assert_eq!(metadata.device_id, "device1");
    assert_eq!(metadata.confidence, 1.0);
    assert!(!message.message_id.is_empty());
    assert!(message.timestamp > 0);
}

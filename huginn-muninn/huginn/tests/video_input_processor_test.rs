//! Tests for Video Input Processor

use huginn::video_input::VideoInputProcessor;

#[tokio::test]
async fn test_video_input_processor_new() {
    let processor = VideoInputProcessor::new();
    // Processor should be created successfully
    assert!(true); // No-op, just verify creation doesn't panic
}

#[tokio::test]
async fn test_video_input_processor_process() {
    let processor = VideoInputProcessor::new();
    
    // Create minimal valid video data (just a small byte array)
    let video_data = vec![0u8; 1024]; // 1KB test data
    
    let result = processor.process(
        &video_data,
        "mp4",
        5000, // 5 seconds duration
        "user123",
        "device456",
    ).await;
    
    assert!(result.is_ok());
    let video_info = result.unwrap();
    assert_eq!(video_info.format, "mp4");
    assert_eq!(video_info.duration_ms, 5000);
    assert_eq!(video_info.user_id, "user123");
    assert_eq!(video_info.device_id, "device456");
    assert_eq!(video_info.size_bytes, 1024);
}

#[tokio::test]
async fn test_video_input_processor_empty_data() {
    let processor = VideoInputProcessor::new();
    let result = processor.process(
        &[],
        "mp4",
        1000,
        "user123",
        "device456",
    ).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_video_input_processor_validate_format() {
    let processor = VideoInputProcessor::new();
    
    // Valid formats
    assert!(processor.validate_format("mp4"));
    assert!(processor.validate_format("webm"));
    assert!(processor.validate_format("avi"));
    assert!(processor.validate_format("mkv"));
    
    // Invalid formats
    assert!(!processor.validate_format(""));
    assert!(!processor.validate_format("invalid"));
    assert!(!processor.validate_format("mov")); // Not yet supported
}

#[tokio::test]
async fn test_video_input_processor_validate_size() {
    let processor = VideoInputProcessor::new();
    
    // Valid sizes (within default limit of 100MB)
    assert!(processor.validate_size(10 * 1024 * 1024)); // 10MB
    assert!(processor.validate_size(100 * 1024 * 1024)); // 100MB
    
    // Invalid sizes (exceeds default limit)
    assert!(!processor.validate_size(101 * 1024 * 1024)); // 101MB
    assert!(!processor.validate_size(1000 * 1024 * 1024)); // 1GB
}

#[tokio::test]
async fn test_video_input_processor_validate_duration() {
    let processor = VideoInputProcessor::new();
    
    // Valid durations
    assert!(processor.validate_duration(1000)); // 1 second
    assert!(processor.validate_duration(60000)); // 1 minute
    assert!(processor.validate_duration(3600000)); // 1 hour
    
    // Invalid durations
    assert!(!processor.validate_duration(0));
    assert!(!processor.validate_duration(-1000));
}

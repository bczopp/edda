//! Tests for Video Stream Processor

use huginn::video_stream::{VideoStreamProcessor, StreamChunk};

#[tokio::test]
async fn test_video_stream_processor_new() {
    let processor = VideoStreamProcessor::new();
    // Processor should be created successfully
    assert!(true); // No-op, just verify creation doesn't panic
}

#[tokio::test]
async fn test_video_stream_processor_process_chunk() {
    let processor = VideoStreamProcessor::new();
    
    let chunk = StreamChunk {
        chunk_index: 0,
        data: vec![0u8; 1024],
        is_last: false,
        format: "mp4".to_string(),
    };
    
    let result = processor.process_chunk(chunk, "user123", "device456").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_video_stream_processor_validate_format() {
    let processor = VideoStreamProcessor::new();
    
    // Valid formats
    assert!(processor.validate_format("mp4"));
    assert!(processor.validate_format("webm"));
    assert!(processor.validate_format("rtsp"));
    assert!(processor.validate_format("webrtc"));
    
    // Invalid formats
    assert!(!processor.validate_format(""));
    assert!(!processor.validate_format("invalid"));
}

#[tokio::test]
async fn test_video_stream_processor_handle_interruption() {
    let processor = VideoStreamProcessor::new();
    
    // Simulate stream interruption
    let result = processor.handle_interruption("user123", "device456").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_video_stream_processor_get_stream_stats() {
    let processor = VideoStreamProcessor::new();
    
    // Process some chunks
    for i in 0..5 {
        let chunk = StreamChunk {
            chunk_index: i,
            data: vec![0u8; 1024],
            is_last: false,
            format: "mp4".to_string(),
        };
        let _ = processor.process_chunk(chunk, "user123", "device456").await;
    }
    
    let stats = processor.get_stream_stats("user123", "device456").await;
    assert!(stats.is_some());
    let stats = stats.unwrap();
    assert_eq!(stats.chunk_count, 5);
    assert_eq!(stats.total_bytes, 5 * 1024);
}

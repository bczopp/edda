//! Tests for Image Input Processor

use huginn::image_input::ImageInputProcessor;

#[tokio::test]
async fn test_image_input_processor_new() {
    let processor = ImageInputProcessor::new();
    // Processor should be created successfully
    assert!(true); // No-op, just verify creation doesn't panic
}

#[tokio::test]
async fn test_image_input_processor_process() {
    let processor = ImageInputProcessor::new();
    
    // Create a minimal valid image (1x1 PNG)
    let image_data = vec![
        0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, // PNG signature
        0x00, 0x00, 0x00, 0x0D, 0x49, 0x48, 0x44, 0x52, // IHDR chunk
        0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, // 1x1 dimensions
        0x08, 0x02, 0x00, 0x00, 0x00, 0x90, 0x77, 0x53, 0xDE, // IHDR data
        0x00, 0x00, 0x00, 0x0C, 0x49, 0x44, 0x41, 0x54, // IDAT chunk
        0x08, 0x99, 0x01, 0x01, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0x00, 0x00, 0x00, 0x02, 0x00, 0x01, // IDAT data
        0x00, 0x00, 0x00, 0x00, 0x49, 0x45, 0x4E, 0x44, 0xAE, 0x42, 0x60, 0x82, // IEND
    ];
    
    let result = processor.process(
        &image_data,
        "png",
        1,
        1,
        "user123",
        "device456",
    ).await;
    
    assert!(result.is_ok());
    let image_info = result.unwrap();
    assert_eq!(image_info.format, "png");
    assert_eq!(image_info.width, 1);
    assert_eq!(image_info.height, 1);
    assert_eq!(image_info.user_id, "user123");
    assert_eq!(image_info.device_id, "device456");
}

#[tokio::test]
async fn test_image_input_processor_empty_data() {
    let processor = ImageInputProcessor::new();
    let result = processor.process(
        &[],
        "png",
        0,
        0,
        "user123",
        "device456",
    ).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_image_input_processor_validate_format() {
    let processor = ImageInputProcessor::new();
    
    // Valid formats
    assert!(processor.validate_format("jpg"));
    assert!(processor.validate_format("jpeg"));
    assert!(processor.validate_format("png"));
    assert!(processor.validate_format("webp"));
    
    // Invalid formats
    assert!(!processor.validate_format(""));
    assert!(!processor.validate_format("invalid"));
    assert!(!processor.validate_format("gif")); // Not yet supported
}

#[tokio::test]
async fn test_image_input_processor_validate_size() {
    let processor = ImageInputProcessor::new();
    
    // Valid sizes (within default limit of 10MB)
    assert!(processor.validate_size(1024 * 1024)); // 1MB
    assert!(processor.validate_size(10 * 1024 * 1024)); // 10MB
    
    // Invalid sizes (exceeds default limit)
    assert!(!processor.validate_size(11 * 1024 * 1024)); // 11MB
    assert!(!processor.validate_size(100 * 1024 * 1024)); // 100MB
}

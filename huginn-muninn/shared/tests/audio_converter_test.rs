//! Tests for Audio Format Converter

use shared::audio::{AudioFormatConverter, AudioBuffer, AudioFormat};

#[test]
fn test_audio_format_converter_creation() {
    let converter = AudioFormatConverter::new();
    assert!(converter.supported_formats().len() > 0);
}

#[tokio::test]
async fn test_convert_sample_rate_16k_to_48k() {
    let converter = AudioFormatConverter::new();
    
    // Create 16kHz audio buffer
    let samples = vec![100i16; 16000]; // 1 second at 16kHz
    let source = AudioBuffer::new(samples, AudioFormat::Pcm16kHz16BitMono);
    
    // Convert to 48kHz
    let result = converter.convert_sample_rate(source, AudioFormat::Pcm48kHz16BitMono).await;
    assert!(result.is_ok());
    
    let converted = result.unwrap();
    assert_eq!(converted.format, AudioFormat::Pcm48kHz16BitMono);
    assert_eq!(converted.len(), 48000); // 3x original length
}

#[tokio::test]
async fn test_convert_sample_rate_48k_to_16k() {
    let converter = AudioFormatConverter::new();
    
    // Create 48kHz audio buffer
    let samples = vec![100i16; 48000]; // 1 second at 48kHz
    let source = AudioBuffer::new(samples, AudioFormat::Pcm48kHz16BitMono);
    
    // Convert to 16kHz
    let result = converter.convert_sample_rate(source, AudioFormat::Pcm16kHz16BitMono).await;
    assert!(result.is_ok());
    
    let converted = result.unwrap();
    assert_eq!(converted.format, AudioFormat::Pcm16kHz16BitMono);
    assert_eq!(converted.len(), 16000); // 1/3 original length
}

#[tokio::test]
async fn test_convert_mono_to_stereo() {
    let converter = AudioFormatConverter::new();
    
    // Create mono audio buffer
    let samples = vec![100i16; 16000];
    let source = AudioBuffer::new(samples, AudioFormat::Pcm16kHz16BitMono);
    
    // Convert to stereo (44.1kHz)
    let result = converter.convert_channels(source, AudioFormat::Pcm44kHz16BitStereo).await;
    assert!(result.is_ok());
    
    let converted = result.unwrap();
    assert_eq!(converted.format.channels(), 2);
}

#[tokio::test]
async fn test_convert_stereo_to_mono() {
    let converter = AudioFormatConverter::new();
    
    // Create stereo audio buffer
    let samples = vec![100i16; 88200]; // 1 second at 44.1kHz stereo
    let source = AudioBuffer::new(samples, AudioFormat::Pcm44kHz16BitStereo);
    
    // Convert to mono
    let result = converter.convert_channels(source, AudioFormat::Pcm16kHz16BitMono).await;
    assert!(result.is_ok());
    
    let converted = result.unwrap();
    assert_eq!(converted.format.channels(), 1);
}

#[tokio::test]
async fn test_convert_empty_buffer() {
    let converter = AudioFormatConverter::new();
    
    let source = AudioBuffer::new(vec![], AudioFormat::Pcm16kHz16BitMono);
    
    let result = converter.convert_sample_rate(source, AudioFormat::Pcm48kHz16BitMono).await;
    assert!(result.is_err());
}

#[test]
fn test_supported_formats() {
    let converter = AudioFormatConverter::new();
    let formats = converter.supported_formats();
    
    assert!(formats.contains(&AudioFormat::Pcm16kHz16BitMono));
    assert!(formats.contains(&AudioFormat::Pcm44kHz16BitStereo));
    assert!(formats.contains(&AudioFormat::Pcm48kHz16BitMono));
}

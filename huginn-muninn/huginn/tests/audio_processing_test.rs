//! Tests for audio processing

use shared::{AudioBuffer, AudioFormat};

#[test]
fn test_audio_format_conversion() {
    let samples = vec![0i16; 16000];
    let buffer = AudioBuffer::new(samples, AudioFormat::Pcm16kHz16BitMono);
    
    assert_eq!(buffer.format, AudioFormat::Pcm16kHz16BitMono);
    assert_eq!(buffer.duration_ms, 1000);
}

#[test]
fn test_audio_buffer_properties() {
    let samples = vec![100i16; 48000];
    let buffer = AudioBuffer::new(samples, AudioFormat::Pcm48kHz16BitMono);
    
    assert!(!buffer.is_empty());
    assert_eq!(buffer.len(), 48000);
    assert_eq!(buffer.frames(), 48000);
}

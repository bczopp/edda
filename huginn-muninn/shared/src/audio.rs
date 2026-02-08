//! Audio format and buffer utilities

use serde::{Deserialize, Serialize};

/// Audio format specification
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AudioFormat {
    /// 16-bit PCM, 16kHz, mono
    Pcm16kHz16BitMono,
    /// 16-bit PCM, 44.1kHz, stereo
    Pcm44kHz16BitStereo,
    /// 16-bit PCM, 48kHz, mono
    Pcm48kHz16BitMono,
    /// MP3 format
    Mp3,
    /// Opus format
    Opus,
    /// WAV format
    Wav,
}

impl AudioFormat {
    /// Get sample rate in Hz
    pub fn sample_rate(&self) -> u32 {
        match self {
            Self::Pcm16kHz16BitMono => 16000,
            Self::Pcm44kHz16BitStereo => 44100,
            Self::Pcm48kHz16BitMono => 48000,
            Self::Mp3 | Self::Opus | Self::Wav => 44100, // Default
        }
    }
    
    /// Get number of channels
    pub fn channels(&self) -> u16 {
        match self {
            Self::Pcm16kHz16BitMono | Self::Pcm48kHz16BitMono => 1,
            Self::Pcm44kHz16BitStereo => 2,
            Self::Mp3 | Self::Opus | Self::Wav => 2, // Default
        }
    }
    
    /// Get bit depth
    pub fn bit_depth(&self) -> u16 {
        match self {
            Self::Pcm16kHz16BitMono
            | Self::Pcm44kHz16BitStereo
            | Self::Pcm48kHz16BitMono
            | Self::Wav => 16,
            Self::Mp3 | Self::Opus => 16, // Default
        }
    }
}

/// Audio sample (single value)
pub type AudioSample = i16;

/// Audio buffer (collection of samples)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioBuffer {
    pub samples: Vec<AudioSample>,
    pub format: AudioFormat,
    pub duration_ms: u32,
}

impl AudioBuffer {
    /// Create a new audio buffer
    pub fn new(samples: Vec<AudioSample>, format: AudioFormat) -> Self {
        let duration_ms = Self::calculate_duration(&samples, format);
        Self {
            samples,
            format,
            duration_ms,
        }
    }
    
    /// Calculate duration in milliseconds
    fn calculate_duration(samples: &[AudioSample], format: AudioFormat) -> u32 {
        let sample_rate = format.sample_rate();
        let channels = format.channels() as u32;
        let frames = samples.len() as u32 / channels;
        (frames * 1000) / sample_rate
    }
    
    /// Get number of frames
    pub fn frames(&self) -> usize {
        self.samples.len() / self.format.channels() as usize
    }
    
    /// Check if buffer is empty
    pub fn is_empty(&self) -> bool {
        self.samples.is_empty()
    }
    
    /// Get buffer length (number of samples)
    pub fn len(&self) -> usize {
        self.samples.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_audio_format_properties() {
        let format = AudioFormat::Pcm16kHz16BitMono;
        assert_eq!(format.sample_rate(), 16000);
        assert_eq!(format.channels(), 1);
        assert_eq!(format.bit_depth(), 16);
        
        let format = AudioFormat::Pcm44kHz16BitStereo;
        assert_eq!(format.sample_rate(), 44100);
        assert_eq!(format.channels(), 2);
        assert_eq!(format.bit_depth(), 16);
    }
    
    #[test]
    fn test_audio_buffer_creation() {
        let samples = vec![0i16; 16000]; // 1 second at 16kHz mono
        let format = AudioFormat::Pcm16kHz16BitMono;
        let buffer = AudioBuffer::new(samples.clone(), format);
        
        assert_eq!(buffer.samples.len(), 16000);
        assert_eq!(buffer.format, format);
        assert_eq!(buffer.duration_ms, 1000); // 1 second
        assert_eq!(buffer.frames(), 16000);
    }
    
    #[test]
    fn test_audio_buffer_stereo() {
        let samples = vec![0i16; 88200]; // 1 second at 44.1kHz stereo
        let format = AudioFormat::Pcm44kHz16BitStereo;
        let buffer = AudioBuffer::new(samples, format);
        
        assert_eq!(buffer.duration_ms, 1000); // 1 second
        assert_eq!(buffer.frames(), 44100);
    }
    
    #[test]
    fn test_audio_buffer_empty() {
        let buffer = AudioBuffer::new(vec![], AudioFormat::Pcm16kHz16BitMono);
        assert!(buffer.is_empty());
        assert_eq!(buffer.len(), 0);
        assert_eq!(buffer.duration_ms, 0);
    }
}

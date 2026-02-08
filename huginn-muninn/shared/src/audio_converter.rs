//! Audio Format Converter

use crate::audio::{AudioBuffer, AudioFormat, AudioSample};
use crate::error::{AudioError, Result};
use tracing::{info, warn};

pub struct AudioFormatConverter {
    supported_formats: Vec<AudioFormat>,
}

impl AudioFormatConverter {
    pub fn new() -> Self {
        info!("Creating AudioFormatConverter");
        Self {
            supported_formats: vec![
                AudioFormat::Pcm16kHz16BitMono,
                AudioFormat::Pcm44kHz16BitStereo,
                AudioFormat::Pcm48kHz16BitMono,
                AudioFormat::Mp3,
                AudioFormat::Opus,
                AudioFormat::Wav,
            ],
        }
    }
    
    pub fn supported_formats(&self) -> &[AudioFormat] {
        &self.supported_formats
    }
    
    /// Convert sample rate
    pub async fn convert_sample_rate(
        &self,
        source: AudioBuffer,
        target_format: AudioFormat,
    ) -> Result<AudioBuffer> {
        if source.is_empty() {
            return Err(AudioError::InvalidData("Source buffer is empty".to_string()));
        }
        
        let source_rate = source.format.sample_rate();
        let target_rate = target_format.sample_rate();
        
        if source_rate == target_rate {
            // No conversion needed
            return Ok(AudioBuffer::new(source.samples, target_format));
        }
        
        info!("Converting sample rate: {} Hz -> {} Hz", source_rate, target_rate);
        
        // Simple linear interpolation resampling
        let ratio = target_rate as f32 / source_rate as f32;
        let new_len = (source.samples.len() as f32 * ratio) as usize;
        let mut converted = Vec::with_capacity(new_len);
        
        for i in 0..new_len {
            let source_idx = (i as f32 / ratio) as usize;
            if source_idx < source.samples.len() {
                converted.push(source.samples[source_idx]);
            } else {
                converted.push(0);
            }
        }
        
        Ok(AudioBuffer::new(converted, target_format))
    }
    
    /// Convert channels (mono <-> stereo)
    pub async fn convert_channels(
        &self,
        source: AudioBuffer,
        target_format: AudioFormat,
    ) -> Result<AudioBuffer> {
        if source.is_empty() {
            return Err(AudioError::InvalidData("Source buffer is empty".to_string()));
        }
        
        let source_channels = source.format.channels();
        let target_channels = target_format.channels();
        
        if source_channels == target_channels {
            // No conversion needed, but might need sample rate conversion
            return self.convert_sample_rate(source, target_format).await;
        }
        
        info!("Converting channels: {} -> {}", source_channels, target_channels);
        
        let converted = if source_channels == 1 && target_channels == 2 {
            // Mono to stereo: duplicate each sample
            let mut stereo = Vec::with_capacity(source.samples.len() * 2);
            for sample in &source.samples {
                stereo.push(*sample);
                stereo.push(*sample);
            }
            stereo
        } else if source_channels == 2 && target_channels == 1 {
            // Stereo to mono: average each pair
            let mut mono = Vec::with_capacity(source.samples.len() / 2);
            for chunk in source.samples.chunks(2) {
                let avg = if chunk.len() == 2 {
                    ((chunk[0] as i32 + chunk[1] as i32) / 2) as AudioSample
                } else {
                    chunk[0]
                };
                mono.push(avg);
            }
            mono
        } else {
            return Err(AudioError::UnsupportedFormat(
                format!("Unsupported channel conversion: {} -> {}", source_channels, target_channels)
            ));
        };
        
        // Create intermediate buffer with correct channels
        let intermediate_format = if target_channels == 1 {
            AudioFormat::Pcm16kHz16BitMono
        } else {
            AudioFormat::Pcm44kHz16BitStereo
        };
        
        let intermediate = AudioBuffer::new(converted, intermediate_format);
        
        // Now convert sample rate if needed
        self.convert_sample_rate(intermediate, target_format).await
    }
}

impl Default for AudioFormatConverter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_convert_sample_rate_no_change() {
        let converter = AudioFormatConverter::new();
        let samples = vec![100i16; 16000];
        let source = AudioBuffer::new(samples.clone(), AudioFormat::Pcm16kHz16BitMono);
        
        let result = converter.convert_sample_rate(source, AudioFormat::Pcm16kHz16BitMono).await;
        assert!(result.is_ok());
        
        let converted = result.unwrap();
        assert_eq!(converted.samples.len(), 16000);
    }
    
    #[tokio::test]
    async fn test_convert_channels_no_change() {
        let converter = AudioFormatConverter::new();
        let samples = vec![100i16; 16000];
        let source = AudioBuffer::new(samples, AudioFormat::Pcm16kHz16BitMono);
        
        let result = converter.convert_channels(source, AudioFormat::Pcm16kHz16BitMono).await;
        assert!(result.is_ok());
    }
}

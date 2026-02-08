//! Settings schema (JSON-compatible structs) for Huginn & Muninn

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

// --- Shared / common ---

/// Audio device configuration (input for Huginn, output for Muninn)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioDeviceConfig {
    /// Device ID or "default"
    pub device: String,
    /// Sample rate (Hz)
    pub sample_rate: u32,
    /// Channels (1 = mono, 2 = stereo)
    pub channels: u16,
    /// Buffer size in ms (Huginn input only)
    pub buffer_size_ms: Option<u32>,
}

impl Default for AudioDeviceConfig {
    fn default() -> Self {
        Self {
            device: "default".to_string(),
            sample_rate: 16000,
            channels: 1,
            buffer_size_ms: Some(100),
        }
    }
}

/// Language settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LanguageSettings {
    /// Default language code (e.g. en-US)
    pub default: String,
    /// Supported language codes
    pub supported: Vec<String>,
}

impl Default for LanguageSettings {
    fn default() -> Self {
        Self {
            default: "en-US".to_string(),
            supported: vec![
                "en-US".into(),
                "de-DE".into(),
                "fr-FR".into(),
                "es-ES".into(),
            ],
        }
    }
}

/// STT quality settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SttQualitySettings {
    /// Model size: small, medium, large
    pub model_size: String,
    /// Language code
    pub language: String,
    /// Confidence threshold 0.0–1.0
    pub confidence_threshold: f64,
}

impl Default for SttQualitySettings {
    fn default() -> Self {
        Self {
            model_size: "medium".to_string(),
            language: "en-US".to_string(),
            confidence_threshold: 0.7,
        }
    }
}

/// TTS quality settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TtsQualitySettings {
    /// Voice quality: standard, high, ultra
    pub voice_quality: String,
    /// Sample rate (Hz)
    pub sample_rate: u32,
    /// Bitrate for compressed formats
    pub bitrate: u32,
}

impl Default for TtsQualitySettings {
    fn default() -> Self {
        Self {
            voice_quality: "high".to_string(),
            sample_rate: 44100,
            bitrate: 192,
        }
    }
}

/// Voice settings (TTS)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoiceSettings {
    /// Voice type: male, female, neutral
    pub voice_type: String,
    /// Language code
    pub language: String,
    /// Speed 0.5–2.0
    pub speed: f64,
    /// Pitch in semitones -12..+12
    pub pitch: i8,
    /// Volume 0.0–1.0
    pub volume: f64,
}

impl Default for VoiceSettings {
    fn default() -> Self {
        Self {
            voice_type: "neutral".to_string(),
            language: "en-US".to_string(),
            speed: 1.0,
            pitch: 0,
            volume: 1.0,
        }
    }
}

/// Wake word settings (optional)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WakeWordSettings {
    /// Enable wake word detection
    pub enabled: bool,
    /// Wake word model path (optional)
    pub model_path: Option<PathBuf>,
    /// Sensitivity 0.0–1.0
    pub sensitivity: f64,
}

impl Default for WakeWordSettings {
    fn default() -> Self {
        Self {
            enabled: false,
            model_path: None,
            sensitivity: 0.5,
        }
    }
}

// --- Huginn ---

/// Full Huginn service settings (JSON-serializable)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HuginnSettings {
    /// gRPC server port
    pub grpc_port: u16,
    /// Audio input device configuration
    pub audio_device_configuration: AudioDeviceConfig,
    /// STT quality settings
    pub quality_settings: SttQualitySettings,
    /// Language settings
    pub language_settings: LanguageSettings,
    /// Wake word settings (optional)
    pub wake_word_settings: Option<WakeWordSettings>,
}

impl Default for HuginnSettings {
    fn default() -> Self {
        Self {
            grpc_port: 50057,
            audio_device_configuration: AudioDeviceConfig::default(),
            quality_settings: SttQualitySettings::default(),
            language_settings: LanguageSettings::default(),
            wake_word_settings: Some(WakeWordSettings::default()),
        }
    }
}

// --- Muninn ---

/// TTS cache settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TtsCacheSettings {
    /// Max cache entries
    pub max_size: usize,
    /// TTL in seconds
    pub ttl_seconds: u64,
    /// Use persistent cache
    pub persistent: bool,
}

impl Default for TtsCacheSettings {
    fn default() -> Self {
        Self {
            max_size: 100,
            ttl_seconds: 3600,
            persistent: false,
        }
    }
}

/// Full Muninn service settings (JSON-serializable)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MuninnSettings {
    /// gRPC server port
    pub grpc_port: u16,
    /// Audio output device configuration
    pub audio_device_configuration: AudioDeviceConfig,
    /// TTS quality settings
    pub quality_settings: TtsQualitySettings,
    /// Voice settings
    pub voice_settings: VoiceSettings,
    /// Language settings
    pub language_settings: LanguageSettings,
    /// TTS cache settings
    pub cache_settings: Option<TtsCacheSettings>,
}

impl Default for MuninnSettings {
    fn default() -> Self {
        Self {
            grpc_port: 50058,
            audio_device_configuration: AudioDeviceConfig::default(),
            quality_settings: TtsQualitySettings::default(),
            voice_settings: VoiceSettings::default(),
            language_settings: LanguageSettings::default(),
            cache_settings: Some(TtsCacheSettings::default()),
        }
    }
}

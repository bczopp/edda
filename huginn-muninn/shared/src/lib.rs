//! Shared audio utilities for Huginn & Muninn

pub mod audio;
pub mod audio_device;
pub mod audio_converter;
pub mod language;
pub mod logging;
pub mod metrics;
pub mod error;
pub mod settings;

pub use audio::{AudioFormat, AudioSample, AudioBuffer};
pub use audio_device::{AudioDevice, AudioDeviceManager, AudioDeviceType};
pub use audio_converter::AudioFormatConverter;
pub use language::{LanguageDetector, LanguageManager};
pub use logging::{LoggingConfig, setup_logging, setup_default_logging};
pub use metrics::{MetricsCollector, MetricsSnapshot, SttMetrics, TtsMetrics, AudioProcessingMetrics, VideoProcessingMetrics};
pub use error::{AudioError, Result};
pub use settings::{
    HuginnSettings, MuninnSettings, SettingsError, SettingsValidator,
    load_huginn_settings, load_muninn_settings,
    HotReloadHandle, start_huginn_hot_reload, start_muninn_hot_reload,
};

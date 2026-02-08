//! Settings validator for Huginn & Muninn

use crate::settings::error::SettingsError;
use crate::settings::schema::{
    HuginnSettings, MuninnSettings, AudioDeviceConfig, SttQualitySettings,
    TtsQualitySettings, VoiceSettings,
};

/// Validates Huginn and Muninn settings.
pub struct SettingsValidator;

impl SettingsValidator {
    /// Validate Huginn settings.
    pub fn validate_huginn(s: &HuginnSettings) -> Result<(), SettingsError> {
        if s.grpc_port == 0 {
            return Err(SettingsError::Validation(
                "grpc_port must be in range 1..65535".into(),
            ));
        }
        Self::validate_audio_device(&s.audio_device_configuration)?;
        Self::validate_stt_quality(&s.quality_settings)?;
        if s.quality_settings.confidence_threshold < 0.0
            || s.quality_settings.confidence_threshold > 1.0
        {
            return Err(SettingsError::Validation(
                "confidence_threshold must be in range 0.0..1.0".into(),
            ));
        }
        Ok(())
    }

    /// Validate Muninn settings.
    pub fn validate_muninn(s: &MuninnSettings) -> Result<(), SettingsError> {
        if s.grpc_port == 0 {
            return Err(SettingsError::Validation(
                "grpc_port must be in range 1..65535".into(),
            ));
        }
        Self::validate_audio_device(&s.audio_device_configuration)?;
        Self::validate_tts_quality(&s.quality_settings)?;
        Self::validate_voice(&s.voice_settings)?;
        if let Some(ref cache) = s.cache_settings {
            if cache.max_size == 0 {
                return Err(SettingsError::Validation(
                    "cache max_size must be > 0".into(),
                ));
            }
        }
        Ok(())
    }

    fn validate_audio_device(a: &AudioDeviceConfig) -> Result<(), SettingsError> {
        if a.sample_rate == 0 {
            return Err(SettingsError::Validation(
                "sample_rate must be > 0".into(),
            ));
        }
        if a.channels == 0 || a.channels > 2 {
            return Err(SettingsError::Validation(
                "channels must be 1 or 2".into(),
            ));
        }
        Ok(())
    }

    fn validate_stt_quality(_s: &SttQualitySettings) -> Result<(), SettingsError> {
        Ok(())
    }

    fn validate_tts_quality(_s: &TtsQualitySettings) -> Result<(), SettingsError> {
        Ok(())
    }

    fn validate_voice(v: &VoiceSettings) -> Result<(), SettingsError> {
        if v.volume < 0.0 || v.volume > 1.0 {
            return Err(SettingsError::Validation(
                "volume must be in range 0.0..1.0".into(),
            ));
        }
        if v.pitch < -12 || v.pitch > 12 {
            return Err(SettingsError::Validation(
                "pitch must be in range -12..12".into(),
            ));
        }
        if v.speed < 0.5 || v.speed > 2.0 {
            return Err(SettingsError::Validation(
                "speed must be in range 0.5..2.0".into(),
            ));
        }
        Ok(())
    }
}

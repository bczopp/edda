//! Tests for SettingsValidator (TDD)

use shared::settings::{HuginnSettings, MuninnSettings, SettingsValidator};

#[test]
fn validate_huginn_settings_default_passes() {
    let s = HuginnSettings::default();
    assert!(SettingsValidator::validate_huginn(&s).is_ok());
}

#[test]
fn validate_muninn_settings_default_passes() {
    let s = MuninnSettings::default();
    assert!(SettingsValidator::validate_muninn(&s).is_ok());
}

#[test]
fn validate_huginn_rejects_grpc_port_zero() {
    let mut s = HuginnSettings::default();
    s.grpc_port = 0;
    assert!(SettingsValidator::validate_huginn(&s).is_err());
}

#[test]
fn validate_muninn_rejects_grpc_port_zero() {
    let mut s = MuninnSettings::default();
    s.grpc_port = 0;
    assert!(SettingsValidator::validate_muninn(&s).is_err());
}

#[test]
fn validate_huginn_rejects_invalid_sample_rate() {
    let mut s = HuginnSettings::default();
    s.audio_device_configuration.sample_rate = 0;
    assert!(SettingsValidator::validate_huginn(&s).is_err());
}

#[test]
fn validate_huginn_rejects_confidence_out_of_range() {
    let mut s = HuginnSettings::default();
    s.quality_settings.confidence_threshold = 1.5;
    assert!(SettingsValidator::validate_huginn(&s).is_err());
}

#[test]
fn validate_muninn_rejects_volume_out_of_range() {
    let mut s = MuninnSettings::default();
    s.voice_settings.volume = 2.0;
    assert!(SettingsValidator::validate_muninn(&s).is_err());
}

#[test]
fn validate_muninn_rejects_pitch_out_of_range() {
    let mut s = MuninnSettings::default();
    s.voice_settings.pitch = 20;
    assert!(SettingsValidator::validate_muninn(&s).is_err());
}

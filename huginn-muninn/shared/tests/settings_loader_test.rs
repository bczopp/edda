//! Tests for SettingsLoader (TDD)

use std::io::Write;
use tempfile::NamedTempFile;

use shared::settings::{load_huginn_settings, load_muninn_settings};

#[test]
fn load_huginn_settings_from_valid_json() {
    let mut f = NamedTempFile::new().unwrap();
    let json = r#"{"grpc_port":50057,"audio_device_configuration":{"device":"default","sample_rate":16000,"channels":1,"buffer_size_ms":100},"quality_settings":{"model_size":"medium","language":"en-US","confidence_threshold":0.7},"language_settings":{"default":"en-US","supported":["en-US","de-DE"]},"wake_word_settings":null}"#;
    f.write_all(json.as_bytes()).unwrap();
    f.flush().unwrap();
    let path = f.path();
    let s = load_huginn_settings(path).unwrap();
    assert_eq!(s.grpc_port, 50057);
    assert_eq!(s.audio_device_configuration.sample_rate, 16000);
}

#[test]
fn load_muninn_settings_from_valid_json() {
    let mut f = NamedTempFile::new().unwrap();
    let json = r#"{"grpc_port":50058,"audio_device_configuration":{"device":"default","sample_rate":44100,"channels":2},"quality_settings":{"voice_quality":"high","sample_rate":44100,"bitrate":192},"voice_settings":{"voice_type":"neutral","language":"en-US","speed":1.0,"pitch":0,"volume":1.0},"language_settings":{"default":"en-US","supported":["en-US"]},"cache_settings":null}"#;
    f.write_all(json.as_bytes()).unwrap();
    f.flush().unwrap();
    let path = f.path();
    let s = load_muninn_settings(path).unwrap();
    assert_eq!(s.grpc_port, 50058);
    assert_eq!(s.voice_settings.volume, 1.0);
}

#[test]
fn load_huginn_settings_invalid_json_fails() {
    let mut f = NamedTempFile::new().unwrap();
    f.write_all(b"{ invalid }").unwrap();
    f.flush().unwrap();
    let path = f.path();
    assert!(load_huginn_settings(path).is_err());
}

#[test]
fn load_huginn_settings_invalid_validation_fails() {
    let mut f = NamedTempFile::new().unwrap();
    let json = r#"{"grpc_port":0,"audio_device_configuration":{"device":"default","sample_rate":16000,"channels":1},"quality_settings":{"model_size":"medium","language":"en-US","confidence_threshold":0.7},"language_settings":{"default":"en-US","supported":[]},"wake_word_settings":null}"#;
    f.write_all(json.as_bytes()).unwrap();
    f.flush().unwrap();
    let path = f.path();
    assert!(load_huginn_settings(path).is_err());
}

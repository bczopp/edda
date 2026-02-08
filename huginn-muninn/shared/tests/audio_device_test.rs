//! Tests for Audio Device Manager

use shared::audio::{AudioDeviceManager, AudioDevice, AudioDeviceType};

#[test]
fn test_audio_device_manager_creation() {
    let manager = AudioDeviceManager::new();
    assert_eq!(manager.device_count(), 0);
}

#[tokio::test]
async fn test_list_devices() {
    let mut manager = AudioDeviceManager::new();
    let devices = manager.list_devices().await;
    
    // Should return at least default devices or empty list
    assert!(devices.is_ok());
}

#[tokio::test]
async fn test_get_default_input_device() {
    let mut manager = AudioDeviceManager::new();
    let device = manager.get_default_device(AudioDeviceType::Input).await;
    
    // May return None if no input device available (e.g., in CI)
    assert!(device.is_ok());
}

#[tokio::test]
async fn test_get_default_output_device() {
    let mut manager = AudioDeviceManager::new();
    let device = manager.get_default_device(AudioDeviceType::Output).await;
    
    // May return None if no output device available (e.g., in CI)
    assert!(device.is_ok());
}

#[test]
fn test_audio_device_properties() {
    let device = AudioDevice {
        id: "test-device-1".to_string(),
        name: "Test Microphone".to_string(),
        device_type: AudioDeviceType::Input,
        sample_rate: 44100,
        channels: 2,
    };
    
    assert_eq!(device.id, "test-device-1");
    assert_eq!(device.name, "Test Microphone");
    assert_eq!(device.device_type, AudioDeviceType::Input);
    assert_eq!(device.sample_rate, 44100);
    assert_eq!(device.channels, 2);
}

#[test]
fn test_audio_device_type_variants() {
    let input = AudioDeviceType::Input;
    let output = AudioDeviceType::Output;
    
    assert_ne!(input, output);
}

#[tokio::test]
async fn test_select_device_by_id() {
    let mut manager = AudioDeviceManager::new();
    
    // Try to select a device by ID
    let result = manager.select_device("nonexistent-device").await;
    
    // Should fail for nonexistent device
    assert!(result.is_err());
}

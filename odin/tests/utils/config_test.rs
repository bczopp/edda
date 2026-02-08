use odin::utils::config::{OdinSettings, SettingsManager, UserPreferences, ProviderSelection};
use std::path::PathBuf;
use tempfile::TempDir;

#[tokio::test]
async fn test_settings_default() {
    let settings = OdinSettings::default();
    
    assert_eq!(settings.grpc_port, 50050);
    assert_eq!(settings.user_preferences.quality_level, "medium");
    assert!(settings.provider_selection.auto_select);
    assert!(settings.plugins.valkyries.enabled);
    assert!(!settings.plugins.frigg.enabled);
    assert!(!settings.scheduler.enabled);
    assert!(settings.scheduler.capability_refresh_enabled);
}

#[tokio::test]
async fn test_settings_load_from_file() {
    let temp_dir = TempDir::new().unwrap();
    let config_path = temp_dir.path().join("settings.json");
    
    let settings_manager = SettingsManager::new(config_path.clone());
    
    // First load should create default config
    settings_manager.load().await.unwrap();
    
    let settings = settings_manager.get().await;
    assert_eq!(settings.grpc_port, 50050);
    
    // Verify file was created
    assert!(config_path.exists());
}

#[tokio::test]
async fn test_settings_validation() {
    let temp_dir = TempDir::new().unwrap();
    let config_path = temp_dir.path().join("settings.json");
    
    // Create invalid settings file
    let invalid_json = r#"{
        "grpc_port": 0,
        "user_preferences": {
            "quality_level": "invalid"
        }
    }"#;
    std::fs::write(&config_path, invalid_json).unwrap();
    
    let settings_manager = SettingsManager::new(config_path.clone());
    
    // Should fail validation
    let result = settings_manager.load().await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_settings_reload() {
    let temp_dir = TempDir::new().unwrap();
    let config_path = temp_dir.path().join("settings.json");
    
    let settings_manager = SettingsManager::new(config_path.clone());
    settings_manager.load().await.unwrap();
    
    // Modify settings file
    let mut settings = settings_manager.get().await;
    settings.grpc_port = 50051;
    let json = serde_json::to_string_pretty(&settings).unwrap();
    std::fs::write(&config_path, json).unwrap();
    
    // Reload
    settings_manager.reload().await.unwrap();
    
    let reloaded_settings = settings_manager.get().await;
    assert_eq!(reloaded_settings.grpc_port, 50051);
}

#[tokio::test]
async fn test_settings_validation_quality_level() {
    let temp_dir = TempDir::new().unwrap();
    let config_path = temp_dir.path().join("settings.json");
    
    let settings_manager = SettingsManager::new(config_path.clone());
    settings_manager.load().await.unwrap();
    
    let mut settings = settings_manager.get().await;
    
    // Test valid quality levels
    for quality in &["low", "medium", "high", "custom"] {
        settings.user_preferences.quality_level = quality.to_string();
        let json = serde_json::to_string_pretty(&settings).unwrap();
        std::fs::write(&config_path, json).unwrap();
        assert!(settings_manager.reload().await.is_ok());
    }
    
    // Test invalid quality level
    settings.user_preferences.quality_level = "invalid".to_string();
    let json = serde_json::to_string_pretty(&settings).unwrap();
    std::fs::write(&config_path, json).unwrap();
    assert!(settings_manager.reload().await.is_err());
}

#[tokio::test]
async fn test_settings_validation_max_cost() {
    let temp_dir = TempDir::new().unwrap();
    let config_path = temp_dir.path().join("settings.json");
    
    let settings_manager = SettingsManager::new(config_path.clone());
    settings_manager.load().await.unwrap();
    
    let mut settings = settings_manager.get().await;
    
    // Valid max_cost
    settings.user_preferences.max_cost = Some(0.10);
    let json = serde_json::to_string_pretty(&settings).unwrap();
    std::fs::write(&config_path, json).unwrap();
    assert!(settings_manager.reload().await.is_ok());
    
    // Invalid max_cost (negative)
    settings.user_preferences.max_cost = Some(-0.10);
    let json = serde_json::to_string_pretty(&settings).unwrap();
    std::fs::write(&config_path, json).unwrap();
    assert!(settings_manager.reload().await.is_err());
}

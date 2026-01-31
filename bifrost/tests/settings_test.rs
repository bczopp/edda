//! Settings validation and loader tests (TDD).
//! Run in container: docker compose -f docker-compose.test.yml run --rm --no-deps bifrost-test

use bifrost::utils::config::{BifrostSettings, SettingsError, SettingsManager};
use tempfile::TempDir;

#[tokio::test]
async fn default_settings_are_valid() {
    let settings = BifrostSettings::default();
    assert!(settings.validate().is_ok());
}

#[tokio::test]
async fn websocket_port_zero_is_invalid() {
    let mut settings = BifrostSettings::default();
    settings.websocket_port = 0;
    let err = settings.validate().unwrap_err();
    assert!(matches!(err, SettingsError::InvalidPort(_)));
}

#[tokio::test]
async fn max_connections_zero_is_invalid() {
    let mut settings = BifrostSettings::default();
    settings.max_connections = 0;
    let err = settings.validate().unwrap_err();
    assert!(matches!(err, SettingsError::InvalidMaxConnections(_)));
}

#[tokio::test]
async fn message_timeout_zero_is_invalid() {
    let mut settings = BifrostSettings::default();
    settings.message_timeout_seconds = 0;
    let err = settings.validate().unwrap_err();
    assert!(matches!(err, SettingsError::InvalidMessageTimeout(_)));
}

#[tokio::test]
async fn empty_heimdall_url_is_invalid() {
    let mut settings = BifrostSettings::default();
    settings.heimdall_url = String::new();
    let err = settings.validate().unwrap_err();
    assert!(matches!(err, SettingsError::EmptyHeimdallUrl));
}

#[tokio::test]
async fn valid_settings_pass_validation() {
    let settings = BifrostSettings {
        websocket_port: 8080,
        heimdall_url: "http://localhost:50051".to_string(),
        max_connections: 100,
        message_timeout_seconds: 60,
    };
    assert!(settings.validate().is_ok());
}

#[tokio::test]
async fn load_nonexistent_file_creates_default_and_succeeds() {
    let tmp = TempDir::new().unwrap();
    let config_path = tmp.path().join("nonexistent.json");
    let manager = SettingsManager::new(config_path.clone());
    manager.load().await.unwrap();
    let settings = manager.get().await;
    assert_eq!(settings.websocket_port, 8080);
    assert!(config_path.exists());
}

#[tokio::test]
async fn load_valid_file_returns_settings() {
    let tmp = TempDir::new().unwrap();
    let config_path = tmp.path().join("bifrost.json");
    let content = r#"{"websocket_port":9000,"heimdall_url":"http://heimdall:50051","max_connections":500,"message_timeout_seconds":45}"#;
    tokio::fs::write(&config_path, content).await.unwrap();
    let manager = SettingsManager::new(config_path);
    manager.load().await.unwrap();
    let settings = manager.get().await;
    assert_eq!(settings.websocket_port, 9000);
    assert_eq!(settings.heimdall_url, "http://heimdall:50051");
    assert_eq!(settings.max_connections, 500);
    assert_eq!(settings.message_timeout_seconds, 45);
}

#[tokio::test]
async fn load_invalid_json_fails() {
    let tmp = TempDir::new().unwrap();
    let config_path = tmp.path().join("bad.json");
    tokio::fs::write(&config_path, "not json").await.unwrap();
    let manager = SettingsManager::new(config_path);
    let result = manager.load().await;
    assert!(result.is_err());
}

#[tokio::test]
async fn load_valid_json_but_invalid_values_fails() {
    let tmp = TempDir::new().unwrap();
    let config_path = tmp.path().join("invalid_values.json");
    let content = r#"{"websocket_port":0,"heimdall_url":"http://x","max_connections":1,"message_timeout_seconds":1}"#;
    tokio::fs::write(&config_path, content).await.unwrap();
    let manager = SettingsManager::new(config_path);
    let result = manager.load().await;
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.to_string().contains("port") || err.to_string().contains("Port"));
}

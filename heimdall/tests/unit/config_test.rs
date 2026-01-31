#[cfg(test)]
mod tests {
    use heimdall::utils::config::*;
    use std::fs;
    use tempfile::TempDir;

    fn valid_settings() -> HeimdallSettings {
        HeimdallSettings::default()
    }

    #[tokio::test]
    async fn test_load_default_settings() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("config.json");
        let manager = SettingsManager::new(config_path.clone());
        
        // Load should create default config
        manager.load().await.unwrap();
        
        let settings = manager.get().await;
        assert_eq!(settings.token_configuration.heimdall_token_expiration_hours, 24);
        assert_eq!(settings.token_configuration.session_token_expiration_hours, 1);
        assert_eq!(settings.token_configuration.refresh_token_expiration_days, 30);
    }

    #[tokio::test]
    async fn test_load_custom_settings() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("config.json");
        
        // Create custom config
        let custom_config = r#"
        {
            "security_policy": {
                "fail_safe": true,
                "default_deny": true
            },
            "token_configuration": {
                "heimdall_token_expiration_hours": 12,
                "session_token_expiration_hours": 30,
                "refresh_token_expiration_days": 14,
                "proactive_renewal_minutes": 10
            },
            "permission_system": {
                "enable_rbac": true,
                "enable_conditional_permissions": false
            },
            "session_management": {
                "session_timeout_hours": 2,
                "enable_hijacking_detection": true
            },
            "grpc_port": 50052,
            "database_url": "postgres://localhost/test"
        }
        "#;
        fs::write(&config_path, custom_config).unwrap();
        
        let manager = SettingsManager::new(config_path);
        manager.load().await.unwrap();
        
        let settings = manager.get().await;
        assert_eq!(settings.token_configuration.heimdall_token_expiration_hours, 12);
        assert_eq!(settings.token_configuration.session_token_expiration_hours, 30);
        assert_eq!(settings.grpc_port, 50052);
    }

    #[tokio::test]
    async fn test_validate_settings() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("config.json");
        
        // Invalid config: zero expiration
        let invalid_config = r#"
        {
            "security_policy": {
                "fail_safe": true,
                "default_deny": true
            },
            "token_configuration": {
                "heimdall_token_expiration_hours": 0,
                "session_token_expiration_hours": 1,
                "refresh_token_expiration_days": 30,
                "proactive_renewal_minutes": 5
            },
            "permission_system": {
                "enable_rbac": true,
                "enable_conditional_permissions": true
            },
            "session_management": {
                "session_timeout_hours": 1,
                "enable_hijacking_detection": true
            },
            "grpc_port": 50051,
            "database_url": "postgres://localhost/heimdall"
        }
        "#;
        fs::write(&config_path, invalid_config).unwrap();
        
        let manager = SettingsManager::new(config_path);
        let result = manager.load().await;
        assert!(result.is_err());
    }

    #[test]
    fn validate_settings_accepts_valid_defaults() {
        let s = valid_settings();
        assert!(validate_settings(&s).is_ok());
    }

    #[test]
    fn validate_settings_rejects_zero_heimdall_token_hours() {
        let mut s = valid_settings();
        s.token_configuration.heimdall_token_expiration_hours = 0;
        assert!(validate_settings(&s).is_err());
    }

    #[test]
    fn validate_settings_rejects_zero_grpc_port() {
        let mut s = valid_settings();
        s.grpc_port = 0;
        assert!(validate_settings(&s).is_err());
    }

    #[test]
    fn validate_settings_rejects_empty_database_url() {
        let mut s = valid_settings();
        s.database_url = String::new();
        assert!(validate_settings(&s).is_err());
    }

    #[tokio::test]
    async fn test_load_rejects_invalid_grpc_port() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("config.json");
        let invalid = r#"{"security_policy":{"fail_safe":true,"default_deny":true},"token_configuration":{"heimdall_token_expiration_hours":24,"session_token_expiration_hours":1,"refresh_token_expiration_days":30,"proactive_renewal_minutes":5},"permission_system":{"enable_rbac":true,"enable_conditional_permissions":true},"session_management":{"session_timeout_hours":1,"enable_hijacking_detection":true},"grpc_port":0,"database_url":"postgres://localhost/heimdall"}"#;
        fs::write(&config_path, invalid).unwrap();
        let manager = SettingsManager::new(config_path);
        assert!(manager.load().await.is_err());
    }

    #[tokio::test]
    async fn test_load_rejects_empty_database_url() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("config.json");
        let invalid = r#"{"security_policy":{"fail_safe":true,"default_deny":true},"token_configuration":{"heimdall_token_expiration_hours":24,"session_token_expiration_hours":1,"refresh_token_expiration_days":30,"proactive_renewal_minutes":5},"permission_system":{"enable_rbac":true,"enable_conditional_permissions":true},"session_management":{"session_timeout_hours":1,"enable_hijacking_detection":true},"grpc_port":50051,"database_url":""}"#;
        fs::write(&config_path, invalid).unwrap();
        let manager = SettingsManager::new(config_path);
        assert!(manager.load().await.is_err());
    }
}

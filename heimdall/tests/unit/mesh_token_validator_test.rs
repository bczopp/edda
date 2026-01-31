#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use uuid::Uuid;

    use heimdall::keys::KeyGenerator;
    use heimdall::mesh::{MeshDeviceRegistry, MeshTokenValidator};
    use heimdall::token::TokenGenerator;
    use heimdall::utils::config::HeimdallSettings;
    use heimdall::utils::device_repository::DeviceRepository;
    use heimdall::token::TokenValidator;
    use heimdall::keys::SecureKeyStorage;
    use crate::common::TestDatabase;
    use tempfile::TempDir;

    async fn setup_validator_and_token(
        test_db: &TestDatabase,
        keys_dir: &std::path::Path,
        device_id: &str,
    ) -> (MeshTokenValidator, String) {
        let device_repo = Arc::new(DeviceRepository::new(test_db.pool.clone()));
        let mesh_registry = Arc::new(MeshDeviceRegistry::new(
            test_db.pool.clone(),
            device_repo.clone(),
        ));
        let (keypair, pkcs8) = KeyGenerator::new()
            .generate_ed25519_keypair()
            .unwrap();
        SecureKeyStorage::new(keys_dir.to_path_buf())
            .store_keypair("heimdall", &pkcs8)
            .unwrap();
        let token_validator = Arc::new(TokenValidator::new(keys_dir.to_path_buf()));
        let config = HeimdallSettings::default().token_configuration;
        let token_generator = TokenGenerator::new(std::sync::Arc::new(keypair), config);
        let mesh_token_validator = MeshTokenValidator::new(
            token_validator,
            mesh_registry.clone(),
        );
        let device = device_repo
            .get_by_device_id(device_id)
            .await
            .unwrap();
        let (mesh_token, _, _) = token_generator
            .generate_session_token(&device.device_id, &device.user_id.to_string())
            .unwrap();
        (mesh_token_validator, mesh_token)
    }

    #[tokio::test]
    async fn validate_mesh_token_valid_token_and_active_device_returns_role() {
        let test_db = TestDatabase::new().await.unwrap();
        let device_repo = DeviceRepository::new(test_db.pool.clone());
        let owner_id = Uuid::new_v4();
        let dev_id = Uuid::new_v4().to_string();
        device_repo
            .create(
                &dev_id,
                owner_id,
                "device-pub-key",
                Some("Mesh Tok Device"),
                Some("desktop"),
            )
            .await
            .unwrap();
        let mesh_registry = Arc::new(MeshDeviceRegistry::new(
            test_db.pool.clone(),
            Arc::new(device_repo),
        ));
        mesh_registry
            .register_device(
                &dev_id,
                "Mesh Tok Device",
                "desktop",
                "mesh-pub-key",
                owner_id,
            )
            .await
            .unwrap();

        let temp_dir = TempDir::new().unwrap();
        let (validator, mesh_token) =
            setup_validator_and_token(&test_db, temp_dir.path(), &dev_id).await;

        let result = validator.validate_mesh_token(&mesh_token).await.unwrap();
        assert_eq!(result.device_id, dev_id);
        assert_eq!(result.user_id, owner_id.to_string());
        assert_eq!(result.role, "user");
        assert!(result.expires_at > 0);
    }

    #[tokio::test]
    async fn validate_mesh_token_device_not_in_mesh_returns_error() {
        let test_db = TestDatabase::new().await.unwrap();
        let device_repo = DeviceRepository::new(test_db.pool.clone());
        let owner_id = Uuid::new_v4();
        let dev_id = Uuid::new_v4().to_string();
        device_repo
            .create(&dev_id, owner_id, "key", None, None)
            .await
            .unwrap();
        // Device in devices table but NOT registered in mesh_devices
        let mesh_registry = Arc::new(MeshDeviceRegistry::new(
            test_db.pool.clone(),
            Arc::new(device_repo),
        ));
        let (keypair, pkcs8) = KeyGenerator::new().generate_ed25519_keypair().unwrap();
        let temp_dir = TempDir::new().unwrap();
        SecureKeyStorage::new(temp_dir.path().to_path_buf())
            .store_keypair("heimdall", &pkcs8)
            .unwrap();
        let token_validator = Arc::new(TokenValidator::new(temp_dir.path().to_path_buf()));
        let config = HeimdallSettings::default().token_configuration;
        let token_generator = TokenGenerator::new(std::sync::Arc::new(keypair), config);
        let device = DeviceRepository::new(test_db.pool.clone())
            .get_by_device_id(&dev_id)
            .await
            .unwrap();
        let (mesh_token, _, _) = token_generator
            .generate_session_token(&device.device_id, &device.user_id.to_string())
            .unwrap();
        let validator = MeshTokenValidator::new(token_validator, mesh_registry);

        let err = validator.validate_mesh_token(&mesh_token).await.unwrap_err();
        assert!(matches!(
            err,
            heimdall::mesh::MeshTokenValidationError::DeviceNotInMesh
        ));
    }

    #[tokio::test]
    async fn validate_mesh_token_invalid_token_returns_error() {
        let test_db = TestDatabase::new().await.unwrap();
        let device_repo = Arc::new(DeviceRepository::new(test_db.pool.clone()));
        let mesh_registry = Arc::new(MeshDeviceRegistry::new(
            test_db.pool.clone(),
            device_repo,
        ));
        let temp_dir = TempDir::new().unwrap();
        let (_keypair, pkcs8) = KeyGenerator::new().generate_ed25519_keypair().unwrap();
        SecureKeyStorage::new(temp_dir.path().to_path_buf())
            .store_keypair("heimdall", &pkcs8)
            .unwrap();
        let token_validator = Arc::new(TokenValidator::new(temp_dir.path().to_path_buf()));
        let validator = MeshTokenValidator::new(token_validator, mesh_registry);

        let err = validator
            .validate_mesh_token("invalid.token.here")
            .await
            .unwrap_err();
        assert!(matches!(
            err,
            heimdall::mesh::MeshTokenValidationError::Token(_)
        ));
    }
}

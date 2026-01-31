// Tests for Authentication gRPC Service.

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use std::sync::Arc;
    use tonic::Request;

    use heimdall::grpc::authentication;
    use heimdall::grpc::authentication::authentication_service_server::AuthenticationService;
    use heimdall::grpc::AuthenticationServiceImpl;
    use heimdall::auth::AuthenticationManager;
    use heimdall::auth::ChallengeGenerator;
    use heimdall::keys::KeyGenerator;
    use heimdall::token::TokenGenerator;
    use heimdall::utils::config::HeimdallSettings;
    use heimdall::utils::device_repository::DeviceRepository;
    use heimdall::utils::token_repository::TokenRepository;
    use crate::common::TestDatabase;

    fn setup_authentication_service(test_db: &TestDatabase) -> AuthenticationServiceImpl {
        let pool = test_db.pool.clone();
        let device_repo = Arc::new(DeviceRepository::new(pool.clone()));
        let token_repo = Arc::new(TokenRepository::new(pool.clone()));
        let keys_dir = std::env::temp_dir().join("heimdall-auth-test-keys");
        let _ = std::fs::create_dir_all(&keys_dir);
        let challenge_generator = Arc::new(ChallengeGenerator::new(
            PathBuf::from(keys_dir),
            device_repo.clone(),
        ));
        let (keypair_for_token, _) = KeyGenerator::new().generate_ed25519_keypair().unwrap();
        let (keypair_for_signing, _) = KeyGenerator::new().generate_ed25519_keypair().unwrap();
        let signing_keypair = Arc::new(keypair_for_signing);
        let config = HeimdallSettings::default().token_configuration;
        let token_generator = Arc::new(TokenGenerator::new(std::sync::Arc::new(keypair_for_token), config));
        let auth_manager = Arc::new(AuthenticationManager::new(
            challenge_generator,
            token_generator,
            device_repo,
            token_repo,
            pool,
        ));
        AuthenticationServiceImpl::new(auth_manager, signing_keypair)
    }

    #[tokio::test]
    async fn generate_token_returns_unimplemented() {
        let test_db = TestDatabase::new().await.unwrap();
        let service = setup_authentication_service(&test_db);
        let req = Request::new(authentication::TokenGenerationRequest {
            device_id: "dev-1".to_string(),
            user_id: "user-1".to_string(),
            permissions: vec![],
        });
        let res = service.generate_token(req).await;
        assert!(res.is_err());
        let err = res.unwrap_err();
        assert_eq!(tonic::Code::Unimplemented, err.code());
    }
}

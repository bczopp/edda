// Tests for Token gRPC Service.

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use std::path::PathBuf;
    use tonic::Request;

    use heimdall::grpc::token;
    use heimdall::grpc::token::token_service_server::TokenService;
    use heimdall::grpc::TokenServiceImpl;
    use heimdall::keys::KeyGenerator;
    use heimdall::token::{TokenGenerator, TokenValidator};
    use heimdall::utils::config::HeimdallSettings;
    use heimdall::utils::token_repository::TokenRepository;
    use crate::common::TestDatabase;

    fn setup_token_service(test_db: &TestDatabase) -> TokenServiceImpl {
        let pool = test_db.pool.clone();
        let token_repo = Arc::new(TokenRepository::new(pool));
        let (keypair, _) = KeyGenerator::new().generate_ed25519_keypair().unwrap();
        let config = HeimdallSettings::default().token_configuration;
        let token_generator = Arc::new(TokenGenerator::new(std::sync::Arc::new(keypair), config));
        let keys_dir = std::env::temp_dir().join("heimdall-token-test-keys");
        let _ = std::fs::create_dir_all(&keys_dir);
        let token_validator = Arc::new(
            TokenValidator::with_cache(PathBuf::from(keys_dir), Arc::new(heimdall::utils::TokenValidationCache::new(300))),
        );
        TokenServiceImpl::new(token_validator, token_repo, token_generator)
    }

    #[tokio::test]
    async fn validate_token_with_invalid_token_returns_error() {
        let test_db = TestDatabase::new().await.unwrap();
        let service = setup_token_service(&test_db);
        let req = Request::new(token::ValidateTokenRequest {
            token: "invalid-token".to_string(),
            device_id: String::new(),
        });
        let res = service.validate_token(req).await;
        assert!(res.is_err());
        let err = res.unwrap_err();
        assert_eq!(tonic::Code::Unauthenticated, err.code());
    }
}

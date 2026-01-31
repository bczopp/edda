//! Security-Test-Skelett: Auth-Bypass verhindern (18.2.1).
//! Ungültiges Token wird abgelehnt; unbekanntes Device wird abgelehnt.

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
        let token_generator = Arc::new(TokenGenerator::new(Arc::new(keypair), config));
        let keys_dir = std::env::temp_dir().join("heimdall-security-test-keys");
        let _ = std::fs::create_dir_all(&keys_dir);
        let token_validator = Arc::new(TokenValidator::with_cache(
            PathBuf::from(keys_dir),
            Arc::new(heimdall::utils::TokenValidationCache::new(300)),
        ));
        TokenServiceImpl::new(token_validator, token_repo, token_generator)
    }

    #[tokio::test]
    async fn invalid_token_is_rejected() {
        let test_db = TestDatabase::new().await.expect("DATABASE_URL set");
        let service = setup_token_service(&test_db);
        let req = Request::new(token::ValidateTokenRequest {
            token: "invalid.token.here".to_string(),
            device_id: String::new(),
        });
        let res = service.validate_token(req).await;
        assert!(res.is_err());
        assert_eq!(tonic::Code::Unauthenticated, res.unwrap_err().code());
    }

    #[tokio::test]
    async fn empty_token_is_rejected() {
        let test_db = TestDatabase::new().await.expect("DATABASE_URL set");
        let service = setup_token_service(&test_db);
        let req = Request::new(token::ValidateTokenRequest {
            token: String::new(),
            device_id: String::new(),
        });
        let res = service.validate_token(req).await;
        assert!(res.is_err());
    }
}

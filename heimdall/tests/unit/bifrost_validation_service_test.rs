// Tests for Bifrost Validation gRPC Service.

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use tonic::Request;

    use heimdall::bifrost::ConnectionValidator;
    use heimdall::grpc::bifrost_validation;
    use heimdall::grpc::bifrost_validation::bifrost_validation_service_server::BifrostValidationService;
    use heimdall::grpc::BifrostValidationServiceImpl;
    use heimdall::keys::KeyGenerator;
    use heimdall::token::TokenGenerator;
    use heimdall::authz::PermissionManager;
    use heimdall::utils::config::HeimdallSettings;
    use heimdall::utils::device_repository::DeviceRepository;
    use crate::common::TestDatabase;

    fn setup_bifrost_validation_service(test_db: &TestDatabase) -> BifrostValidationServiceImpl {
        let pool = test_db.pool.clone();
        let device_repo = Arc::new(DeviceRepository::new(pool.clone()));
        let permission_manager = Arc::new(PermissionManager::new(pool, device_repo.clone()));
        let connection_validator = Arc::new(ConnectionValidator::new(
            device_repo,
            permission_manager,
        ));
        let (keypair, _) = KeyGenerator::new().generate_ed25519_keypair().unwrap();
        let signing_keypair = Arc::new(keypair);
        let config = HeimdallSettings::default().token_configuration;
        let token_generator = Arc::new(TokenGenerator::new(signing_keypair.clone(), config));
        BifrostValidationServiceImpl::new(
            connection_validator,
            signing_keypair,
            token_generator,
        )
    }

    #[tokio::test]
    async fn validate_connection_denies_when_source_device_not_found() {
        let test_db = TestDatabase::new().await.unwrap();
        let service = setup_bifrost_validation_service(&test_db);
        let req = Request::new(bifrost_validation::ConnectionValidationRequest {
            source_device_id: "nonexistent-source".to_string(),
            target_device_id: "nonexistent-target".to_string(),
            source_user_id: "user-1".to_string(),
            target_user_id: "user-2".to_string(),
            connection_type: "DIRECT".to_string(),
            network_id: String::new(),
            timestamp: 0,
            signature: vec![],
        });
        let res = service.validate_connection(req).await;
        assert!(res.is_err());
        let err = res.unwrap_err();
        assert_eq!(tonic::Code::PermissionDenied, err.code());
    }

    #[tokio::test]
    async fn validate_message_with_invalid_token_returns_error() {
        let test_db = TestDatabase::new().await.unwrap();
        let service = setup_bifrost_validation_service(&test_db);
        let req = Request::new(bifrost_validation::MessageValidationRequest {
            connection_token: "invalid-token".to_string(),
            message: vec![],
            signature: vec![],
            timestamp: 0,
        });
        let res = service.validate_message(req).await;
        assert!(res.is_err());
        let err = res.unwrap_err();
        assert_eq!(tonic::Code::Unauthenticated, err.code());
    }
}

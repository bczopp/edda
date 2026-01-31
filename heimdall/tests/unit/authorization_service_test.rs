// Tests for Authorization gRPC Service.

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use tonic::Request;
    use uuid::Uuid;

    use heimdall::grpc::authorization;
    use heimdall::grpc::authorization::authorization_service_server::AuthorizationService;
    use heimdall::grpc::AuthorizationServiceImpl;
    use heimdall::authz::PermissionManager;
    use heimdall::utils::device_repository::DeviceRepository;
    use crate::common::TestDatabase;

    fn setup_authorization_service(test_db: &TestDatabase) -> AuthorizationServiceImpl {
        let pool = test_db.pool.clone();
        let device_repo = Arc::new(DeviceRepository::new(pool.clone()));
        let permission_manager = Arc::new(PermissionManager::new(pool, device_repo));
        AuthorizationServiceImpl::new(permission_manager)
    }

    #[tokio::test]
    async fn check_permission_with_invalid_user_id_returns_error() {
        let test_db = TestDatabase::new().await.unwrap();
        let service = setup_authorization_service(&test_db);
        let req = Request::new(authorization::PermissionCheckRequest {
            device_id: "dev-1".to_string(),
            user_id: "not-a-uuid".to_string(),
            resource_type: "resource".to_string(),
            action: "read".to_string(),
            resource_id: String::new(),
            context: std::collections::HashMap::new(),
        });
        let res = service.check_permission(req).await;
        assert!(res.is_err());
        let err = res.unwrap_err();
        assert_eq!(tonic::Code::InvalidArgument, err.code());
    }

    #[tokio::test]
    async fn check_permission_denies_when_device_not_found() {
        let test_db = TestDatabase::new().await.unwrap();
        let service = setup_authorization_service(&test_db);
        let user_id = Uuid::new_v4();
        let req = Request::new(authorization::PermissionCheckRequest {
            device_id: "nonexistent-device".to_string(),
            user_id: user_id.to_string(),
            resource_type: "resource".to_string(),
            action: "read".to_string(),
            resource_id: String::new(),
            context: std::collections::HashMap::new(),
        });
        let res = service.check_permission(req).await;
        assert!(res.is_err());
    }
}

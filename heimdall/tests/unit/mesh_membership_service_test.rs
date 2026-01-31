#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use tonic::Request;
    use uuid::Uuid;

    use heimdall::grpc::mesh_membership;
    use heimdall::grpc::mesh_membership::mesh_membership_service_server::MeshMembershipService;
    use heimdall::grpc::MeshMembershipServiceImpl;
    use heimdall::keys::KeyGenerator;
    use heimdall::mesh::MeshDeviceRegistry;
    use heimdall::token::TokenGenerator;
    use heimdall::utils::config::HeimdallSettings;
    use heimdall::utils::device_repository::DeviceRepository;
    use crate::common::TestDatabase;

    fn setup_mesh_membership_service(
        test_db: &TestDatabase,
    ) -> MeshMembershipServiceImpl {
        let device_repo = Arc::new(DeviceRepository::new(test_db.pool.clone()));
        let mesh_registry = Arc::new(MeshDeviceRegistry::new(
            test_db.pool.clone(),
            device_repo.clone(),
        ));
        let (keypair, _) = KeyGenerator::new().generate_ed25519_keypair().unwrap();
        let signing_keypair = Arc::new(keypair);
        let config = HeimdallSettings::default().token_configuration;
        let token_generator = Arc::new(TokenGenerator::new(signing_keypair.clone(), config));
        MeshMembershipServiceImpl::new(
            mesh_registry,
            token_generator,
            signing_keypair,
        )
    }

    #[tokio::test]
    async fn register_device_returns_registered_and_mesh_device_id() {
        let test_db = TestDatabase::new().await.unwrap();
        let device_repo = DeviceRepository::new(test_db.pool.clone());
        let owner_id = Uuid::new_v4();
        let dev_id = Uuid::new_v4().to_string();
        device_repo
            .create(
                &dev_id,
                owner_id,
                "device-pub-key",
                Some("Mesh SVC Device"),
                Some("desktop"),
            )
            .await
            .unwrap();

        let service = setup_mesh_membership_service(&test_db);
        let req = Request::new(mesh_membership::MeshMembershipRequest {
            device_id: dev_id.clone(),
            device_name: "Mesh SVC Device".to_string(),
            device_type: "desktop".to_string(),
            mesh_public_key: b"mesh-public-key-bytes".to_vec(),
            owner_user_id: owner_id.to_string(),
            timestamp: 0,
            signature: vec![],
        });

        let res = service.register_device(req).await.unwrap();
        let inner = res.into_inner();
        assert!(inner.registered);
        assert!(inner.requires_approval);
        assert!(!inner.mesh_device_id.is_empty());
        assert!(inner.message.contains("approval") || inner.message.contains("registered"));
    }

    #[tokio::test]
    async fn register_device_twice_second_returns_already_registered() {
        let test_db = TestDatabase::new().await.unwrap();
        let device_repo = DeviceRepository::new(test_db.pool.clone());
        let owner_id = Uuid::new_v4();
        let dev_id = Uuid::new_v4().to_string();
        device_repo
            .create(&dev_id, owner_id, "key", None, None)
            .await
            .unwrap();

        let service = setup_mesh_membership_service(&test_db);
        let req1 = Request::new(mesh_membership::MeshMembershipRequest {
            device_id: dev_id.clone(),
            device_name: "D2".to_string(),
            device_type: "mobile".to_string(),
            mesh_public_key: b"key2".to_vec(),
            owner_user_id: owner_id.to_string(),
            timestamp: 0,
            signature: vec![],
        });
        let req2 = Request::new(mesh_membership::MeshMembershipRequest {
            device_id: dev_id,
            device_name: "D2".to_string(),
            device_type: "mobile".to_string(),
            mesh_public_key: b"key2".to_vec(),
            owner_user_id: owner_id.to_string(),
            timestamp: 0,
            signature: vec![],
        });

        let res1 = service.register_device(req1).await.unwrap().into_inner();
        let res2 = service.register_device(req2).await.unwrap().into_inner();
        assert!(res1.registered && res1.requires_approval);
        assert!(res2.registered && !res2.requires_approval);
        assert!(res2.message.contains("already registered"));
    }

    #[tokio::test]
    async fn register_device_unknown_device_returns_error() {
        let test_db = TestDatabase::new().await.unwrap();
        let service = setup_mesh_membership_service(&test_db);
        let owner_id = Uuid::new_v4();
        let req = Request::new(mesh_membership::MeshMembershipRequest {
            device_id: "nonexistent-device".to_string(),
            device_name: "No Device".to_string(),
            device_type: "desktop".to_string(),
            mesh_public_key: vec![],
            owner_user_id: owner_id.to_string(),
            timestamp: 0,
            signature: vec![],
        });

        let err = service.register_device(req).await.unwrap_err();
        assert!(err.code() == tonic::Code::Internal || err.code() == tonic::Code::InvalidArgument);
    }

    #[tokio::test]
    async fn generate_mesh_auth_token_returns_token_and_role() {
        let test_db = TestDatabase::new().await.unwrap();
        let device_repo = DeviceRepository::new(test_db.pool.clone());
        let owner_id = Uuid::new_v4();
        let dev_id = Uuid::new_v4().to_string();
        device_repo
            .create(&dev_id, owner_id, "key", None, None)
            .await
            .unwrap();

        let service = setup_mesh_membership_service(&test_db);
        let reg_req = Request::new(mesh_membership::MeshMembershipRequest {
            device_id: dev_id.clone(),
            device_name: "D3".to_string(),
            device_type: "mobile".to_string(),
            mesh_public_key: b"mesh-key".to_vec(),
            owner_user_id: owner_id.to_string(),
            timestamp: 0,
            signature: vec![],
        });
        let reg_res = service.register_device(reg_req).await.unwrap().into_inner();
        let mesh_device_id = reg_res.mesh_device_id;

        let token_req = Request::new(mesh_membership::MeshAuthTokenRequest {
            device_id: dev_id,
            mesh_device_id: mesh_device_id.clone(),
            timestamp: 0,
            signature: vec![],
        });
        let token_res = service.generate_mesh_auth_token(token_req).await.unwrap().into_inner();

        assert!(!token_res.mesh_token.is_empty());
        assert_eq!(token_res.role, "user");
        assert!(token_res.expires_at > 0);
        assert!(!token_res.signature.is_empty());
    }

    #[tokio::test]
    async fn generate_mesh_auth_token_unknown_device_returns_not_found() {
        let test_db = TestDatabase::new().await.unwrap();
        let service = setup_mesh_membership_service(&test_db);
        let req = Request::new(mesh_membership::MeshAuthTokenRequest {
            device_id: "no-such-device".to_string(),
            mesh_device_id: Uuid::new_v4().to_string(),
            timestamp: 0,
            signature: vec![],
        });

        let err = service.generate_mesh_auth_token(req).await.unwrap_err();
        assert_eq!(err.code(), tonic::Code::NotFound);
    }
}

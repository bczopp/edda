//! Dedizierte Tests für Mesh-Token-Generierung: Token-Format, Role, Ablauf (10.3.1).

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use tonic::Request;
    use uuid::Uuid;

    use heimdall::grpc::mesh_membership;
    use heimdall::grpc::mesh_membership::mesh_membership_service_server::MeshMembershipService;
    use heimdall::grpc::MeshMembershipServiceImpl;
    use heimdall::keys::KeyGenerator;
    use heimdall::mesh::{MeshDeviceRegistry, OwnerAuthorizationManager};
    use heimdall::token::TokenGenerator;
    use heimdall::utils::config::HeimdallSettings;
    use heimdall::utils::device_repository::DeviceRepository;
    use crate::common::TestDatabase;

    fn setup_service(test_db: &TestDatabase) -> (MeshMembershipServiceImpl, Arc<MeshDeviceRegistry>) {
        let device_repo = Arc::new(DeviceRepository::new(test_db.pool.clone()));
        let mesh_registry = Arc::new(MeshDeviceRegistry::new(
            test_db.pool.clone(),
            device_repo.clone(),
        ));
        let (keypair, _) = KeyGenerator::new().generate_ed25519_keypair().unwrap();
        let signing_keypair = Arc::new(keypair);
        let config = HeimdallSettings::default().token_configuration;
        let token_generator = Arc::new(TokenGenerator::new(signing_keypair.clone(), config));
        let service = MeshMembershipServiceImpl::new(
            mesh_registry.clone(),
            token_generator,
            signing_keypair,
        );
        (service, mesh_registry)
    }

    #[tokio::test]
    async fn mesh_token_has_expected_format() {
        let test_db = TestDatabase::new().await.unwrap();
        let device_repo = DeviceRepository::new(test_db.pool.clone());
        let owner_id = Uuid::new_v4();
        let dev_id = Uuid::new_v4().to_string();
        device_repo
            .create(&dev_id, owner_id, "key", None, None)
            .await
            .unwrap();

        let (service, _) = setup_service(&test_db);
        let reg_req = Request::new(mesh_membership::MeshMembershipRequest {
            device_id: dev_id.clone(),
            device_name: "D1".to_string(),
            device_type: "desktop".to_string(),
            mesh_public_key: b"pubkey".to_vec(),
            owner_user_id: owner_id.to_string(),
            timestamp: 0,
            signature: vec![],
        });
        let reg_res = service.register_device(reg_req).await.unwrap().into_inner();

        let token_req = Request::new(mesh_membership::MeshAuthTokenRequest {
            device_id: dev_id,
            mesh_device_id: reg_res.mesh_device_id,
            timestamp: 0,
            signature: vec![],
        });
        let res = service.generate_mesh_auth_token(token_req).await.unwrap().into_inner();

        assert!(!res.mesh_token.is_empty(), "mesh_token must be non-empty");
        assert!(!res.signature.is_empty(), "signature must be present");
        assert!(res.expires_at > 0, "expires_at must be positive");
        assert!(!res.role.is_empty(), "role must be present");
    }

    #[tokio::test]
    async fn mesh_token_role_reflects_mesh_device_role() {
        let test_db = TestDatabase::new().await.unwrap();
        let device_repo = DeviceRepository::new(test_db.pool.clone());
        let owner_id = Uuid::new_v4();
        let dev_id = Uuid::new_v4().to_string();
        device_repo
            .create(&dev_id, owner_id, "key", None, None)
            .await
            .unwrap();

        let (service, mesh_registry) = setup_service(&test_db);
        let reg_req = Request::new(mesh_membership::MeshMembershipRequest {
            device_id: dev_id.clone(),
            device_name: "D2".to_string(),
            device_type: "mobile".to_string(),
            mesh_public_key: b"pubkey".to_vec(),
            owner_user_id: owner_id.to_string(),
            timestamp: 0,
            signature: vec![],
        });
        let reg_res = service.register_device(reg_req).await.unwrap().into_inner();

        let auth_manager = OwnerAuthorizationManager::new(mesh_registry);
        auth_manager
            .approve_device(owner_id, &dev_id, "admin")
            .await
            .unwrap();

        let token_req = Request::new(mesh_membership::MeshAuthTokenRequest {
            device_id: dev_id,
            mesh_device_id: reg_res.mesh_device_id,
            timestamp: 0,
            signature: vec![],
        });
        let res = service.generate_mesh_auth_token(token_req).await.unwrap().into_inner();
        assert_eq!(res.role, "admin");
    }

    #[tokio::test]
    async fn mesh_token_expires_at_in_future() {
        let test_db = TestDatabase::new().await.unwrap();
        let device_repo = DeviceRepository::new(test_db.pool.clone());
        let owner_id = Uuid::new_v4();
        let dev_id = Uuid::new_v4().to_string();
        device_repo
            .create(&dev_id, owner_id, "key", None, None)
            .await
            .unwrap();

        let (service, _) = setup_service(&test_db);
        let reg_req = Request::new(mesh_membership::MeshMembershipRequest {
            device_id: dev_id.clone(),
            device_name: "D3".to_string(),
            device_type: "desktop".to_string(),
            mesh_public_key: b"pubkey".to_vec(),
            owner_user_id: owner_id.to_string(),
            timestamp: 0,
            signature: vec![],
        });
        let reg_res = service.register_device(reg_req).await.unwrap().into_inner();

        let now_secs = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;

        let token_req = Request::new(mesh_membership::MeshAuthTokenRequest {
            device_id: dev_id,
            mesh_device_id: reg_res.mesh_device_id,
            timestamp: 0,
            signature: vec![],
        });
        let res = service.generate_mesh_auth_token(token_req).await.unwrap().into_inner();

        assert!(
            res.expires_at > now_secs,
            "expires_at {} should be in future (now ~{})",
            res.expires_at,
            now_secs
        );
    }
}

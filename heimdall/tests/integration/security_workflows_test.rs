//! E2E-Skelett: Security-Workflow Challenge-Response → Token → Validation (18.1.1).
//! Nutzt TestDatabase; kann um Connection-Validation und Mesh-Membership erweitert werden.

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use uuid::Uuid;

    use heimdall::utils::DeviceRepository;
    use heimdall::mesh::MeshDeviceRegistry;
    use crate::common::TestDatabase;

    #[tokio::test]
    async fn e2e_skeleton_mesh_register_and_token_workflow() {
        let test_db = TestDatabase::new().await.expect("DATABASE_URL set");
        let device_repo = Arc::new(DeviceRepository::new(test_db.pool.clone()));
        let mesh_registry = MeshDeviceRegistry::new(test_db.pool.clone(), device_repo.clone());

        let owner_id = Uuid::new_v4();
        let dev_id = Uuid::new_v4().to_string();
        device_repo
            .create(&dev_id, owner_id, "pubkey", Some("E2E Device"), Some("desktop"))
            .await
            .expect("device created");

        let (mesh_device, is_new) = mesh_registry
            .register_device(&dev_id, "E2E Device", "desktop", "mesh-key", owner_id)
            .await
            .expect("mesh register");

        assert!(is_new);
        assert!(!mesh_device.id.is_nil());
    }
}

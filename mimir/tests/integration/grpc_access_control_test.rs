#[cfg(test)]
mod tests {
    use mimir::grpc::server::{MimirServiceImpl, GrpcServerDependencies};
    use mimir::storage::database::EncryptedDatabase;
    use mimir::encryption::EncryptionManager;
    use mimir::access_control::{AccessControlManager, Role};
    use mimir::audit::{AuditLogManager, AuditEvent};
    use mimir::gdpr::GDPRCompliance;
    use mimir::grpc::mimir::{StoreDataRequest, RetrieveDataRequest, DeleteDataRequest};
    use tests::common::TestDatabase;
    use ring::rand::{SecureRandom, SystemRandom};
    use std::sync::Arc;
    use tonic::Request;

    fn generate_key() -> Vec<u8> {
        let mut key = vec![0u8; 32];
        let rng = SystemRandom::new();
        rng.fill(&mut key).unwrap();
        key
    }

    async fn setup_service() -> (MimirServiceImpl, Arc<AuditLogManager>) {
        let test_db = TestDatabase::new().await.unwrap();
        let key = generate_key();
        let encryption_manager = EncryptionManager::new(&key).unwrap();
        
        let access_control = Arc::new(AccessControlManager::new());
        let audit_logger = Arc::new(AuditLogManager::new(test_db.pool.clone()));
        
        // Register users
        access_control.register_user("user1".to_string(), Role::User).await;
        access_control.register_user("user2".to_string(), Role::User).await;
        access_control.register_user("admin".to_string(), Role::Admin).await;
        
        let database = Arc::new(
            EncryptedDatabase::new_with_access_control_and_audit(
                &test_db.pool,
                encryption_manager,
                access_control.clone(),
                audit_logger.clone(),
            ).await.unwrap()
        );
        
        let gdpr = Arc::new(GDPRCompliance::new_with_database(database.clone()));
        
        let deps = GrpcServerDependencies {
            database,
            gdpr,
            access_control: Some(access_control),
            audit_logger: Some(audit_logger.clone()),
        };
        
        let service = MimirServiceImpl::new_with_deps(deps);
        (service, audit_logger)
    }

    #[tokio::test]
    async fn test_store_data_with_access_control() {
        let (service, audit_logger) = setup_service().await;
        
        let request = Request::new(StoreDataRequest {
            user_id: "user1".to_string(),
            data: b"Secret data".to_vec(),
        });
        
        // TODO: Add user context to request metadata
        // For now, we'll test the basic flow
        let response = service.store_data(request).await;
        
        // Should succeed (access control will be checked internally)
        assert!(response.is_ok());
        
        // Verify audit log was created
        let logs = audit_logger.get_user_audit_logs("user1").await.unwrap();
        assert!(logs.iter().any(|log| matches!(log.event_type, AuditEvent::DataStored)));
    }

    #[tokio::test]
    async fn test_retrieve_data_with_access_control() {
        let (service, audit_logger) = setup_service().await;
        
        // First store data
        let store_request = Request::new(StoreDataRequest {
            user_id: "user1".to_string(),
            data: b"Secret data".to_vec(),
        });
        let store_response = service.store_data(store_request).await.unwrap();
        let data_id = store_response.into_inner().data_id;
        
        // Retrieve data
        let retrieve_request = Request::new(RetrieveDataRequest {
            data_id: data_id.clone(),
            user_id: "user1".to_string(),
        });
        
        let response = service.retrieve_data(retrieve_request).await;
        assert!(response.is_ok());
        
        // Verify audit log was created
        let logs = audit_logger.get_user_audit_logs("user1").await.unwrap();
        assert!(logs.iter().any(|log| matches!(log.event_type, AuditEvent::DataRetrieved)));
    }

    #[tokio::test]
    async fn test_delete_data_with_access_control() {
        let (service, audit_logger) = setup_service().await;
        
        // First store data
        let store_request = Request::new(StoreDataRequest {
            user_id: "user1".to_string(),
            data: b"Secret data".to_vec(),
        });
        let store_response = service.store_data(store_request).await.unwrap();
        let data_id = store_response.into_inner().data_id;
        
        // Delete data
        let delete_request = Request::new(DeleteDataRequest {
            data_id: data_id.clone(),
            user_id: "user1".to_string(),
        });
        
        let response = service.delete_data(delete_request).await;
        assert!(response.is_ok());
        
        // Verify audit log was created
        let logs = audit_logger.get_user_audit_logs("user1").await.unwrap();
        assert!(logs.iter().any(|log| matches!(log.event_type, AuditEvent::DataDeleted)));
    }
}

#[cfg(test)]
mod tests {
    use mimir::access_control::{AccessControlManager, Role, Resource, Action, UserContext};

    #[tokio::test]
    async fn test_access_control_workflow() {
        let acm = AccessControlManager::new();
        
        // Register users with different roles
        acm.register_user("admin".to_string(), Role::Admin).await;
        acm.register_user("user1".to_string(), Role::User).await;
        acm.register_user("user2".to_string(), Role::User).await;
        acm.register_user("service".to_string(), Role::Service).await;
        acm.register_user("readonly".to_string(), Role::ReadOnly).await;
        
        // Test admin access
        assert!(acm.check_permission("admin", Resource::UserData, Action::Read).await.is_ok());
        assert!(acm.check_permission("admin", Resource::UserData, Action::Delete).await.is_ok());
        assert!(acm.check_permission("admin", Resource::AuditLogs, Action::Read).await.is_ok());
        
        // Test user access to own data
        assert!(acm.check_user_data_access("user1", "user1", Action::Read).await.is_ok());
        assert!(acm.check_user_data_access("user1", "user1", Action::Update).await.is_ok());
        
        // Test user cannot access other user's data
        assert!(acm.check_user_data_access("user1", "user2", Action::Read).await.is_err());
        
        // Test service access
        assert!(acm.check_user_data_access("service", "user1", Action::Read).await.is_ok());
        assert!(acm.check_user_data_access("service", "user1", Action::Update).await.is_ok());
        assert!(acm.check_user_data_access("service", "user1", Action::Delete).await.is_err());
        
        // Test readonly access
        assert!(acm.check_permission("readonly", Resource::UserData, Action::Read).await.is_ok());
        assert!(acm.check_permission("readonly", Resource::UserData, Action::Create).await.is_err());
    }

    #[tokio::test]
    async fn test_unauthorized_access_prevention() {
        let acm = AccessControlManager::new();
        
        // Register regular user
        acm.register_user("user1".to_string(), Role::User).await;
        
        // User should not be able to access audit logs
        assert!(acm.check_permission("user1", Resource::AuditLogs, Action::Read).await.is_err());
        
        // User should not be able to access configuration
        assert!(acm.check_permission("user1", Resource::Configuration, Action::Read).await.is_err());
        assert!(acm.check_permission("user1", Resource::Configuration, Action::Update).await.is_err());
    }

    #[tokio::test]
    async fn test_user_context_validation() {
        let acm = AccessControlManager::new();
        
        // Register user
        acm.register_user("user1".to_string(), Role::User).await;
        
        // Valid context
        let valid_context = UserContext::new("user1".to_string(), Role::User);
        assert!(acm.validate_user_context(&valid_context).await.is_ok());
        
        // Invalid context (wrong role)
        let invalid_context = UserContext::new("user1".to_string(), Role::Admin);
        assert!(acm.validate_user_context(&invalid_context).await.is_err());
        
        // Invalid context (user not found)
        let unknown_context = UserContext::new("unknown".to_string(), Role::User);
        assert!(acm.validate_user_context(&unknown_context).await.is_err());
    }

    #[tokio::test]
    async fn test_cross_user_data_access_prevention() {
        let acm = AccessControlManager::new();
        
        // Register two users
        acm.register_user("alice".to_string(), Role::User).await;
        acm.register_user("bob".to_string(), Role::User).await;
        
        // Alice can access her own data
        assert!(acm.check_user_data_access("alice", "alice", Action::Read).await.is_ok());
        assert!(acm.check_user_data_access("alice", "alice", Action::Update).await.is_ok());
        assert!(acm.check_user_data_access("alice", "alice", Action::Delete).await.is_ok());
        
        // Bob can access his own data
        assert!(acm.check_user_data_access("bob", "bob", Action::Read).await.is_ok());
        
        // Alice cannot access Bob's data
        assert!(acm.check_user_data_access("alice", "bob", Action::Read).await.is_err());
        assert!(acm.check_user_data_access("alice", "bob", Action::Update).await.is_err());
        assert!(acm.check_user_data_access("alice", "bob", Action::Delete).await.is_err());
        
        // Bob cannot access Alice's data
        assert!(acm.check_user_data_access("bob", "alice", Action::Read).await.is_err());
    }

    #[tokio::test]
    async fn test_role_hierarchy() {
        let acm = AccessControlManager::new();
        
        // Register users with different roles
        acm.register_user("admin".to_string(), Role::Admin).await;
        acm.register_user("service".to_string(), Role::Service).await;
        acm.register_user("user".to_string(), Role::User).await;
        acm.register_user("readonly".to_string(), Role::ReadOnly).await;
        
        // Admin > Service > User > ReadOnly in terms of permissions
        
        // Admin can do everything
        assert!(acm.check_permission("admin", Resource::UserData, Action::Delete).await.is_ok());
        assert!(acm.check_permission("admin", Resource::AuditLogs, Action::Read).await.is_ok());
        assert!(acm.check_permission("admin", Resource::Configuration, Action::Update).await.is_ok());
        
        // Service can read/write user data and read audit logs
        assert!(acm.check_permission("service", Resource::UserData, Action::Read).await.is_ok());
        assert!(acm.check_permission("service", Resource::UserData, Action::Update).await.is_ok());
        assert!(acm.check_permission("service", Resource::AuditLogs, Action::Read).await.is_ok());
        assert!(acm.check_permission("service", Resource::UserData, Action::Delete).await.is_err());
        
        // User can read/write their own data
        assert!(acm.check_permission("user", Resource::UserData, Action::Read).await.is_ok());
        assert!(acm.check_permission("user", Resource::UserData, Action::Update).await.is_ok());
        assert!(acm.check_permission("user", Resource::AuditLogs, Action::Read).await.is_err());
        
        // ReadOnly can only read
        assert!(acm.check_permission("readonly", Resource::UserData, Action::Read).await.is_ok());
        assert!(acm.check_permission("readonly", Resource::UserData, Action::Update).await.is_err());
    }
}

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use thiserror::Error;
use tracing::{debug, warn};

use super::permissions::{Permission, Role, Resource, Action};

#[derive(Debug, Error)]
pub enum AccessControlError {
    #[error("Access denied: {0}")]
    AccessDenied(String),
    #[error("User not found: {0}")]
    UserNotFound(String),
    #[error("Invalid user context")]
    InvalidContext,
}

/// User context containing authentication and authorization information
#[derive(Debug, Clone)]
pub struct UserContext {
    pub user_id: String,
    pub role: Role,
}

impl UserContext {
    pub fn new(user_id: String, role: Role) -> Self {
        Self { user_id, role }
    }
}

/// Access Control Manager implementing RBAC
pub struct AccessControlManager {
    /// Maps user_id to role
    user_roles: Arc<RwLock<HashMap<String, Role>>>,
}

impl AccessControlManager {
    pub fn new() -> Self {
        Self {
            user_roles: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Register a user with a role
    pub async fn register_user(&self, user_id: String, role: Role) {
        let mut roles = self.user_roles.write().await;
        roles.insert(user_id.clone(), role);
        debug!("Registered user {} with role {:?}", user_id, role);
    }

    /// Get role for a user
    pub async fn get_user_role(&self, user_id: &str) -> Option<Role> {
        let roles = self.user_roles.read().await;
        roles.get(user_id).copied()
    }

    /// Validate user context
    pub async fn validate_user_context(&self, context: &UserContext) -> Result<(), AccessControlError> {
        let stored_role = self.get_user_role(&context.user_id).await;
        
        match stored_role {
            Some(role) if role == context.role => {
                debug!("User context validated for {}", context.user_id);
                Ok(())
            }
            Some(role) => {
                warn!(
                    "Role mismatch for user {}: expected {:?}, got {:?}",
                    context.user_id, role, context.role
                );
                Err(AccessControlError::InvalidContext)
            }
            None => {
                warn!("User not found: {}", context.user_id);
                Err(AccessControlError::UserNotFound(context.user_id.clone()))
            }
        }
    }

    /// Check if a user has permission to perform an action on a resource
    pub async fn check_permission(
        &self,
        user_id: &str,
        resource: Resource,
        action: Action,
    ) -> Result<(), AccessControlError> {
        let role = self.get_user_role(user_id).await
            .ok_or_else(|| AccessControlError::UserNotFound(user_id.to_string()))?;
        
        let permission = Permission::new(role, resource, action);
        
        if permission.is_allowed() {
            debug!(
                "Permission granted for user {} (role {:?}) to {:?} on {:?}",
                user_id, role, action, resource
            );
            Ok(())
        } else {
            warn!(
                "Permission denied for user {} (role {:?}) to {:?} on {:?}",
                user_id, role, action, resource
            );
            Err(AccessControlError::AccessDenied(format!(
                "User {} (role {:?}) cannot {:?} {:?}",
                user_id, role, action, resource
            )))
        }
    }

    /// Check if a user can access another user's data
    pub async fn check_user_data_access(
        &self,
        accessor_id: &str,
        data_owner_id: &str,
        action: Action,
    ) -> Result<(), AccessControlError> {
        let role = self.get_user_role(accessor_id).await
            .ok_or_else(|| AccessControlError::UserNotFound(accessor_id.to_string()))?;
        
        // Admin and Service roles can access any user's data
        if matches!(role, Role::Admin | Role::Service | Role::ReadOnly) {
            return self.check_permission(accessor_id, Resource::UserData, action).await;
        }
        
        // Regular users can only access their own data
        if accessor_id == data_owner_id {
            return self.check_permission(accessor_id, Resource::UserData, action).await;
        }
        
        warn!(
            "User {} attempted to access user {}'s data",
            accessor_id, data_owner_id
        );
        Err(AccessControlError::AccessDenied(format!(
            "User {} cannot access user {}'s data",
            accessor_id, data_owner_id
        )))
    }
}

impl Default for AccessControlManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_register_and_get_user_role() {
        let acm = AccessControlManager::new();
        
        acm.register_user("user1".to_string(), Role::User).await;
        acm.register_user("admin1".to_string(), Role::Admin).await;
        
        assert_eq!(acm.get_user_role("user1").await, Some(Role::User));
        assert_eq!(acm.get_user_role("admin1").await, Some(Role::Admin));
        assert_eq!(acm.get_user_role("unknown").await, None);
    }

    #[tokio::test]
    async fn test_validate_user_context_success() {
        let acm = AccessControlManager::new();
        acm.register_user("user1".to_string(), Role::User).await;
        
        let context = UserContext::new("user1".to_string(), Role::User);
        assert!(acm.validate_user_context(&context).await.is_ok());
    }

    #[tokio::test]
    async fn test_validate_user_context_role_mismatch() {
        let acm = AccessControlManager::new();
        acm.register_user("user1".to_string(), Role::User).await;
        
        let context = UserContext::new("user1".to_string(), Role::Admin);
        assert!(acm.validate_user_context(&context).await.is_err());
    }

    #[tokio::test]
    async fn test_validate_user_context_user_not_found() {
        let acm = AccessControlManager::new();
        
        let context = UserContext::new("unknown".to_string(), Role::User);
        assert!(acm.validate_user_context(&context).await.is_err());
    }

    #[tokio::test]
    async fn test_check_permission_admin() {
        let acm = AccessControlManager::new();
        acm.register_user("admin1".to_string(), Role::Admin).await;
        
        // Admin should have access to everything
        assert!(acm.check_permission("admin1", Resource::UserData, Action::Read).await.is_ok());
        assert!(acm.check_permission("admin1", Resource::UserData, Action::Delete).await.is_ok());
        assert!(acm.check_permission("admin1", Resource::AuditLogs, Action::Read).await.is_ok());
        assert!(acm.check_permission("admin1", Resource::Configuration, Action::Update).await.is_ok());
    }

    #[tokio::test]
    async fn test_check_permission_user() {
        let acm = AccessControlManager::new();
        acm.register_user("user1".to_string(), Role::User).await;
        
        // User should have access to UserData
        assert!(acm.check_permission("user1", Resource::UserData, Action::Read).await.is_ok());
        assert!(acm.check_permission("user1", Resource::UserData, Action::Create).await.is_ok());
        assert!(acm.check_permission("user1", Resource::UserData, Action::Update).await.is_ok());
        assert!(acm.check_permission("user1", Resource::UserData, Action::Delete).await.is_ok());
        assert!(acm.check_permission("user1", Resource::UserData, Action::Export).await.is_ok());
        
        // User should NOT have access to AuditLogs or Configuration
        assert!(acm.check_permission("user1", Resource::AuditLogs, Action::Read).await.is_err());
        assert!(acm.check_permission("user1", Resource::Configuration, Action::Read).await.is_err());
    }

    #[tokio::test]
    async fn test_check_permission_service() {
        let acm = AccessControlManager::new();
        acm.register_user("service1".to_string(), Role::Service).await;
        
        // Service should have read/write access to UserData
        assert!(acm.check_permission("service1", Resource::UserData, Action::Read).await.is_ok());
        assert!(acm.check_permission("service1", Resource::UserData, Action::Create).await.is_ok());
        assert!(acm.check_permission("service1", Resource::UserData, Action::Update).await.is_ok());
        
        // Service should NOT have delete/export access to UserData
        assert!(acm.check_permission("service1", Resource::UserData, Action::Delete).await.is_err());
        assert!(acm.check_permission("service1", Resource::UserData, Action::Export).await.is_err());
        
        // Service should have read access to AuditLogs
        assert!(acm.check_permission("service1", Resource::AuditLogs, Action::Read).await.is_ok());
    }

    #[tokio::test]
    async fn test_check_permission_readonly() {
        let acm = AccessControlManager::new();
        acm.register_user("readonly1".to_string(), Role::ReadOnly).await;
        
        // ReadOnly should only have read access
        assert!(acm.check_permission("readonly1", Resource::UserData, Action::Read).await.is_ok());
        assert!(acm.check_permission("readonly1", Resource::AuditLogs, Action::Read).await.is_ok());
        
        // ReadOnly should NOT have write access
        assert!(acm.check_permission("readonly1", Resource::UserData, Action::Create).await.is_err());
        assert!(acm.check_permission("readonly1", Resource::UserData, Action::Update).await.is_err());
        assert!(acm.check_permission("readonly1", Resource::UserData, Action::Delete).await.is_err());
    }

    #[tokio::test]
    async fn test_check_permission_user_not_found() {
        let acm = AccessControlManager::new();
        
        let result = acm.check_permission("unknown", Resource::UserData, Action::Read).await;
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), AccessControlError::UserNotFound(_)));
    }

    #[tokio::test]
    async fn test_check_user_data_access_own_data() {
        let acm = AccessControlManager::new();
        acm.register_user("user1".to_string(), Role::User).await;
        
        // User should be able to access their own data
        assert!(acm.check_user_data_access("user1", "user1", Action::Read).await.is_ok());
        assert!(acm.check_user_data_access("user1", "user1", Action::Update).await.is_ok());
        assert!(acm.check_user_data_access("user1", "user1", Action::Delete).await.is_ok());
    }

    #[tokio::test]
    async fn test_check_user_data_access_other_user_data() {
        let acm = AccessControlManager::new();
        acm.register_user("user1".to_string(), Role::User).await;
        acm.register_user("user2".to_string(), Role::User).await;
        
        // User should NOT be able to access another user's data
        assert!(acm.check_user_data_access("user1", "user2", Action::Read).await.is_err());
        assert!(acm.check_user_data_access("user1", "user2", Action::Update).await.is_err());
        assert!(acm.check_user_data_access("user1", "user2", Action::Delete).await.is_err());
    }

    #[tokio::test]
    async fn test_check_user_data_access_admin_can_access_any() {
        let acm = AccessControlManager::new();
        acm.register_user("admin1".to_string(), Role::Admin).await;
        acm.register_user("user1".to_string(), Role::User).await;
        
        // Admin should be able to access any user's data
        assert!(acm.check_user_data_access("admin1", "user1", Action::Read).await.is_ok());
        assert!(acm.check_user_data_access("admin1", "user1", Action::Update).await.is_ok());
        assert!(acm.check_user_data_access("admin1", "user1", Action::Delete).await.is_ok());
    }

    #[tokio::test]
    async fn test_check_user_data_access_service_can_access_any() {
        let acm = AccessControlManager::new();
        acm.register_user("service1".to_string(), Role::Service).await;
        acm.register_user("user1".to_string(), Role::User).await;
        
        // Service should be able to access any user's data (for read/write)
        assert!(acm.check_user_data_access("service1", "user1", Action::Read).await.is_ok());
        assert!(acm.check_user_data_access("service1", "user1", Action::Update).await.is_ok());
        
        // But not delete
        assert!(acm.check_user_data_access("service1", "user1", Action::Delete).await.is_err());
    }
}

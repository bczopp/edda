use serde::{Deserialize, Serialize};

/// Represents different roles in the system
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Role {
    /// System administrator with full access
    Admin,
    /// Regular user with access to their own data
    User,
    /// Service account for inter-service communication
    Service,
    /// Read-only access (e.g., for analytics, compliance)
    ReadOnly,
}

/// Represents resources in the system
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Resource {
    /// User data stored in the database
    UserData,
    /// Audit logs
    AuditLogs,
    /// System configuration
    Configuration,
}

/// Represents actions that can be performed on resources
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Action {
    /// Read/query data
    Read,
    /// Create/store new data
    Create,
    /// Update existing data
    Update,
    /// Delete data
    Delete,
    /// Export data (GDPR)
    Export,
}

/// Represents a permission (role + resource + action)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Permission {
    pub role: Role,
    pub resource: Resource,
    pub action: Action,
}

impl Permission {
    pub fn new(role: Role, resource: Resource, action: Action) -> Self {
        Self { role, resource, action }
    }

    /// Check if this permission is allowed
    pub fn is_allowed(&self) -> bool {
        match (self.role, self.resource, self.action) {
            // Admin has full access to everything
            (Role::Admin, _, _) => true,
            
            // Users can read/create/update/delete/export their own data
            (Role::User, Resource::UserData, Action::Read) => true,
            (Role::User, Resource::UserData, Action::Create) => true,
            (Role::User, Resource::UserData, Action::Update) => true,
            (Role::User, Resource::UserData, Action::Delete) => true,
            (Role::User, Resource::UserData, Action::Export) => true,
            
            // Users cannot access audit logs or configuration
            (Role::User, Resource::AuditLogs, _) => false,
            (Role::User, Resource::Configuration, _) => false,
            
            // Service accounts can read/create/update user data
            (Role::Service, Resource::UserData, Action::Read) => true,
            (Role::Service, Resource::UserData, Action::Create) => true,
            (Role::Service, Resource::UserData, Action::Update) => true,
            (Role::Service, Resource::UserData, Action::Delete) => false, // Services cannot delete
            (Role::Service, Resource::UserData, Action::Export) => false, // Services cannot export
            
            // Service accounts can read audit logs
            (Role::Service, Resource::AuditLogs, Action::Read) => true,
            (Role::Service, Resource::AuditLogs, _) => false,
            
            // Service accounts cannot access configuration
            (Role::Service, Resource::Configuration, _) => false,
            
            // ReadOnly can only read
            (Role::ReadOnly, _, Action::Read) => true,
            (Role::ReadOnly, _, _) => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_admin_has_full_access() {
        assert!(Permission::new(Role::Admin, Resource::UserData, Action::Read).is_allowed());
        assert!(Permission::new(Role::Admin, Resource::UserData, Action::Create).is_allowed());
        assert!(Permission::new(Role::Admin, Resource::UserData, Action::Update).is_allowed());
        assert!(Permission::new(Role::Admin, Resource::UserData, Action::Delete).is_allowed());
        assert!(Permission::new(Role::Admin, Resource::AuditLogs, Action::Read).is_allowed());
        assert!(Permission::new(Role::Admin, Resource::Configuration, Action::Update).is_allowed());
    }

    #[test]
    fn test_user_can_access_own_data() {
        assert!(Permission::new(Role::User, Resource::UserData, Action::Read).is_allowed());
        assert!(Permission::new(Role::User, Resource::UserData, Action::Create).is_allowed());
        assert!(Permission::new(Role::User, Resource::UserData, Action::Update).is_allowed());
        assert!(Permission::new(Role::User, Resource::UserData, Action::Delete).is_allowed());
        assert!(Permission::new(Role::User, Resource::UserData, Action::Export).is_allowed());
    }

    #[test]
    fn test_user_cannot_access_audit_logs() {
        assert!(!Permission::new(Role::User, Resource::AuditLogs, Action::Read).is_allowed());
        assert!(!Permission::new(Role::User, Resource::AuditLogs, Action::Create).is_allowed());
    }

    #[test]
    fn test_user_cannot_access_configuration() {
        assert!(!Permission::new(Role::User, Resource::Configuration, Action::Read).is_allowed());
        assert!(!Permission::new(Role::User, Resource::Configuration, Action::Update).is_allowed());
    }

    #[test]
    fn test_service_can_read_and_write_user_data() {
        assert!(Permission::new(Role::Service, Resource::UserData, Action::Read).is_allowed());
        assert!(Permission::new(Role::Service, Resource::UserData, Action::Create).is_allowed());
        assert!(Permission::new(Role::Service, Resource::UserData, Action::Update).is_allowed());
    }

    #[test]
    fn test_service_cannot_delete_or_export_user_data() {
        assert!(!Permission::new(Role::Service, Resource::UserData, Action::Delete).is_allowed());
        assert!(!Permission::new(Role::Service, Resource::UserData, Action::Export).is_allowed());
    }

    #[test]
    fn test_readonly_can_only_read() {
        assert!(Permission::new(Role::ReadOnly, Resource::UserData, Action::Read).is_allowed());
        assert!(Permission::new(Role::ReadOnly, Resource::AuditLogs, Action::Read).is_allowed());
        assert!(!Permission::new(Role::ReadOnly, Resource::UserData, Action::Create).is_allowed());
        assert!(!Permission::new(Role::ReadOnly, Resource::UserData, Action::Delete).is_allowed());
    }
}

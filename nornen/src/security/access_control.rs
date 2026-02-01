use thiserror::Error;
use std::collections::HashMap;
use tonic::Request;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Permission {
    /// Register new providers
    RegisterProvider,
    /// Update existing providers
    UpdateProvider,
    /// Query providers
    QueryProviders,
    /// List all providers
    ListProviders,
    /// Update provider status
    UpdateProviderStatus,
    /// Coordinate requests
    CoordinateRequest,
    /// Access monitoring/metrics
    ViewMetrics,
    /// Admin operations
    Admin,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Role {
    /// Regular user - can query and coordinate
    User,
    /// Provider - can register and update own providers
    Provider,
    /// Admin - full access
    Admin,
}

impl Role {
    pub fn permissions(&self) -> Vec<Permission> {
        match self {
            Role::User => vec![
                Permission::QueryProviders,
                Permission::CoordinateRequest,
            ],
            Role::Provider => vec![
                Permission::QueryProviders,
                Permission::CoordinateRequest,
                Permission::RegisterProvider,
                Permission::UpdateProvider,
                Permission::UpdateProviderStatus,
            ],
            Role::Admin => vec![
                Permission::QueryProviders,
                Permission::CoordinateRequest,
                Permission::RegisterProvider,
                Permission::UpdateProvider,
                Permission::UpdateProviderStatus,
                Permission::ListProviders,
                Permission::ViewMetrics,
                Permission::Admin,
            ],
        }
    }

    pub fn has_permission(&self, permission: &Permission) -> bool {
        self.permissions().contains(permission)
    }
}

#[derive(Debug, Error)]
pub enum AccessControlError {
    #[error("Access denied: {0}")]
    AccessDenied(String),
    #[error("Invalid user ID")]
    InvalidUserId,
    #[error("Missing authentication")]
    MissingAuthentication,
}

/// Access control checker
pub struct AccessControl {
    // Role assignments (user_id -> role)
    role_assignments: HashMap<String, Role>,
    // Default role for unknown users
    default_role: Role,
}

impl AccessControl {
    pub fn new() -> Self {
        Self {
            role_assignments: HashMap::new(),
            default_role: Role::User,
        }
    }

    pub fn with_default_role(default_role: Role) -> Self {
        Self {
            role_assignments: HashMap::new(),
            default_role,
        }
    }

    /// Assign a role to a user
    pub fn assign_role(&mut self, user_id: &str, role: Role) {
        self.role_assignments.insert(user_id.to_string(), role);
    }

    /// Get user role
    pub fn get_role(&self, user_id: &str) -> &Role {
        self.role_assignments.get(user_id).unwrap_or(&self.default_role)
    }

    /// Check if user has permission
    pub fn check_permission(&self, user_id: &str, permission: &Permission) -> Result<(), AccessControlError> {
        let role = self.get_role(user_id);
        if role.has_permission(permission) {
            Ok(())
        } else {
            Err(AccessControlError::AccessDenied(
                format!("User {} with role {:?} does not have permission {:?}", user_id, role, permission)
            ))
        }
    }

    /// Extract user ID from gRPC request metadata
    /// In production, this would extract from JWT token or similar
    pub fn extract_user_id<T>(request: &Request<T>) -> Result<String, AccessControlError> {
        // Try to get user_id from metadata
        let metadata = request.metadata();
        
        // Check for user_id header
        if let Some(user_id) = metadata.get("user_id") {
            return Ok(user_id.to_str()
                .map_err(|_| AccessControlError::InvalidUserId)?
                .to_string());
        }
        
        // Check for authorization header (could contain JWT with user_id)
        if let Some(auth) = metadata.get("authorization") {
            // In production, decode JWT and extract user_id
            // For now, use a simple format: "Bearer user_id" or "user_id"
            let auth_str = auth.to_str()
                .map_err(|_| AccessControlError::MissingAuthentication)?;
            
            if let Some(user_id) = auth_str.strip_prefix("Bearer ") {
                return Ok(user_id.to_string());
            }
            
            // Fallback: use entire auth string as user_id (for testing)
            return Ok(auth_str.to_string());
        }
        
        // Default: use "system" user for internal requests
        // In production, this should be an error
        Ok("system".to_string())
    }

    /// Check access for a gRPC request
    pub fn check_access<T>(&self, request: &Request<T>, permission: &Permission) -> Result<String, AccessControlError> {
        let user_id = Self::extract_user_id(request)?;
        self.check_permission(&user_id, permission)?;
        Ok(user_id)
    }
}

impl Default for AccessControl {
    fn default() -> Self {
        Self::new()
    }
}

/// Helper macro for access control checks in gRPC handlers
#[macro_export]
macro_rules! require_permission {
    ($access_control:expr, $request:expr, $permission:expr) => {
        $access_control.check_access($request, &$permission)
            .map_err(|e| tonic::Status::permission_denied(e.to_string()))?
    };
}

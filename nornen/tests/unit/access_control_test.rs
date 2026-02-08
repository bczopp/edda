use nornen::security::{AccessControl, Permission, Role};
use tonic::Request;

#[tokio::test]
async fn test_role_permissions() {
    // Test User role
    let user_role = Role::User;
    assert!(user_role.has_permission(&Permission::QueryProviders));
    assert!(user_role.has_permission(&Permission::CoordinateRequest));
    assert!(!user_role.has_permission(&Permission::RegisterProvider));
    assert!(!user_role.has_permission(&Permission::Admin));

    // Test Provider role
    let provider_role = Role::Provider;
    assert!(provider_role.has_permission(&Permission::QueryProviders));
    assert!(provider_role.has_permission(&Permission::RegisterProvider));
    assert!(provider_role.has_permission(&Permission::UpdateProvider));
    assert!(!provider_role.has_permission(&Permission::ListProviders));
    assert!(!provider_role.has_permission(&Permission::Admin));

    // Test Admin role
    let admin_role = Role::Admin;
    assert!(admin_role.has_permission(&Permission::QueryProviders));
    assert!(admin_role.has_permission(&Permission::RegisterProvider));
    assert!(admin_role.has_permission(&Permission::ListProviders));
    assert!(admin_role.has_permission(&Permission::ViewMetrics));
    assert!(admin_role.has_permission(&Permission::Admin));
}

#[tokio::test]
async fn test_access_control_check_permission() {
    let mut access_control = AccessControl::new();
    
    // Assign roles
    access_control.assign_role("user1", Role::User);
    access_control.assign_role("provider1", Role::Provider);
    access_control.assign_role("admin1", Role::Admin);

    // Test User permissions
    assert!(access_control.check_permission("user1", &Permission::QueryProviders).is_ok());
    assert!(access_control.check_permission("user1", &Permission::CoordinateRequest).is_ok());
    assert!(access_control.check_permission("user1", &Permission::RegisterProvider).is_err());

    // Test Provider permissions
    assert!(access_control.check_permission("provider1", &Permission::RegisterProvider).is_ok());
    assert!(access_control.check_permission("provider1", &Permission::UpdateProvider).is_ok());
    assert!(access_control.check_permission("provider1", &Permission::ListProviders).is_err());

    // Test Admin permissions
    assert!(access_control.check_permission("admin1", &Permission::ListProviders).is_ok());
    assert!(access_control.check_permission("admin1", &Permission::ViewMetrics).is_ok());
    assert!(access_control.check_permission("admin1", &Permission::Admin).is_ok());
}

#[tokio::test]
async fn test_access_control_default_role() {
    let access_control = AccessControl::new();
    
    // Unknown user should get default role (User)
    assert!(access_control.check_permission("unknown", &Permission::QueryProviders).is_ok());
    assert!(access_control.check_permission("unknown", &Permission::RegisterProvider).is_err());
}

#[tokio::test]
async fn test_access_control_custom_default_role() {
    let access_control = AccessControl::with_default_role(Role::Admin);
    
    // Unknown user should get Admin role
    assert!(access_control.check_permission("unknown", &Permission::Admin).is_ok());
    assert!(access_control.check_permission("unknown", &Permission::ListProviders).is_ok());
}

#[tokio::test]
async fn test_extract_user_id_from_metadata() {
    let access_control = AccessControl::new();
    
    // Test with user_id header
    let mut request = Request::new(());
    request.metadata_mut().insert("user_id", "test_user".parse().unwrap());
    
    let user_id = AccessControl::extract_user_id(&request).unwrap();
    assert_eq!(user_id, "test_user");
}

#[tokio::test]
async fn test_extract_user_id_from_authorization() {
    let access_control = AccessControl::new();
    
    // Test with authorization header (Bearer format)
    let mut request = Request::new(());
    request.metadata_mut().insert("authorization", "Bearer test_user".parse().unwrap());
    
    let user_id = AccessControl::extract_user_id(&request).unwrap();
    assert_eq!(user_id, "test_user");
}

#[tokio::test]
async fn test_extract_user_id_fallback_to_system() {
    let access_control = AccessControl::new();
    
    // Test with no metadata - should default to "system"
    let request = Request::new(());
    
    let user_id = AccessControl::extract_user_id(&request).unwrap();
    assert_eq!(user_id, "system");
}

#[tokio::test]
async fn test_check_access_with_request() {
    let mut access_control = AccessControl::new();
    access_control.assign_role("admin1", Role::Admin);
    access_control.assign_role("user1", Role::User);
    
    // Test admin access
    let mut admin_request = Request::new(());
    admin_request.metadata_mut().insert("user_id", "admin1".parse().unwrap());
    
    let user_id = access_control.check_access(&admin_request, &Permission::Admin).unwrap();
    assert_eq!(user_id, "admin1");
    
    // Test user access denied
    let mut user_request = Request::new(());
    user_request.metadata_mut().insert("user_id", "user1".parse().unwrap());
    
    let result = access_control.check_access(&user_request, &Permission::Admin);
    assert!(result.is_err());
}

#[tokio::test]
async fn test_role_assignment_update() {
    let mut access_control = AccessControl::new();
    
    // Initially user has User role
    assert!(access_control.check_permission("user1", &Permission::RegisterProvider).is_err());
    
    // Upgrade to Provider
    access_control.assign_role("user1", Role::Provider);
    assert!(access_control.check_permission("user1", &Permission::RegisterProvider).is_ok());
    
    // Upgrade to Admin
    access_control.assign_role("user1", Role::Admin);
    assert!(access_control.check_permission("user1", &Permission::Admin).is_ok());
}

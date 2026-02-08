//! Tests for Heimdall Integration (Authentication)

use ragnarok::services::HeimdallIntegration;

#[tokio::test]
async fn test_heimdall_client_connect_unreachable() {
    let result = HeimdallIntegration::connect("127.0.0.1:59999".to_string()).await;
    assert!(result.is_err());
}

#[tokio::test]
#[ignore] // Requires running Heimdall service
async fn test_heimdall_authenticate_user() {
    let client = HeimdallIntegration::connect("127.0.0.1:50050".to_string())
        .await
        .expect("connect to heimdall");
    
    let token = client.authenticate("test_user", "test_password").await;
    assert!(token.is_ok());
}

#[tokio::test]
#[ignore] // Requires running Heimdall service
async fn test_heimdall_validate_token() {
    let client = HeimdallIntegration::connect("127.0.0.1:50050".to_string())
        .await
        .expect("connect to heimdall");
    
    let token = client.authenticate("test_user", "test_password")
        .await
        .expect("authenticate");
    
    let is_valid = client.validate_token(&token).await;
    assert!(is_valid.is_ok());
    assert!(is_valid.unwrap());
}

#[test]
fn test_heimdall_integration_new() {
    let integration = HeimdallIntegration::new("127.0.0.1:50050".to_string());
    assert_eq!(integration.address(), "127.0.0.1:50050");
}

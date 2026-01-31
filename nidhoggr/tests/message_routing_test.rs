mod utils;

use nidhoggr::routing::MessageRouter;
use nidhoggr::connection::ConnectionManager;
use nidhoggr::clients::ClientManager;
use std::sync::Arc;
use ratatoskr::messages::RatatoskrRequest;
use ratatoskr::proto::ratatoskr::MessageType;
use uuid::Uuid;

fn test_endpoints() -> nidhoggr::utils::config::ServiceEndpoints {
    utils::test_helpers::get_test_service_endpoints()
}

#[tokio::test]
async fn test_message_router_routes_business_request_to_nornen() {
    let connection_manager = Arc::new(ConnectionManager::new());
    let client_manager = Arc::new(ClientManager::new(test_endpoints()).await.unwrap());
    let router = MessageRouter::new(connection_manager, client_manager);
    
    let request = RatatoskrRequest::new_business_request(
        Uuid::new_v4().to_string(),
        "test-device".to_string(),
        "test-user".to_string(),
        b"test payload".to_vec(),
    );
    
    // This will fail until we implement routing
    let result = router.route_message(request).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_message_router_routes_payment_request_to_heidrun() {
    let connection_manager = Arc::new(ConnectionManager::new());
    let client_manager = Arc::new(ClientManager::new(test_endpoints()).await.unwrap());
    let router = MessageRouter::new(connection_manager, client_manager);
    
    let request = RatatoskrRequest::new_business_request(
        Uuid::new_v4().to_string(),
        "test-device".to_string(),
        "test-user".to_string(),
        b"payment payload".to_vec(),
    );
    
    // This will fail until we implement routing
    let result = router.route_message(request).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_message_router_handles_unknown_message_type() {
    let connection_manager = Arc::new(ConnectionManager::new());
    let client_manager = Arc::new(ClientManager::new(test_endpoints()).await.unwrap());
    let router = MessageRouter::new(connection_manager, client_manager);
    
    let mut request = RatatoskrRequest::new_business_request(
        Uuid::new_v4().to_string(),
        "test-device".to_string(),
        "test-user".to_string(),
        b"test payload".to_vec(),
    );
    // Set to unknown type
    request.message_type = MessageType::Unknown as i32;
    
    let result = router.route_message(request).await;
    assert!(result.is_err());
}

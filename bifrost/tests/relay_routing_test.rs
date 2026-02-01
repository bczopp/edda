//! Tests for Phase 9.2: Relay Manager, AsgardRelayClient, YggdrasilRelayClient.

use bifrost::message::{BifrostMessage, MessageType};
use bifrost::routing::{AsgardRelayClient, RelayClient, RelayManager, YggdrasilRelayClient};
use std::sync::Arc;

fn sample_message() -> BifrostMessage {
    BifrostMessage {
        message_id: "relay-1".to_string(),
        message_type: MessageType::Message,
        source_device_id: "src".to_string(),
        target_device_id: "tgt".to_string(),
        payload: serde_json::json!({}),
        timestamp: 0,
        protocol_version: None,
    }
}

#[tokio::test]
async fn relay_manager_tries_relays_in_order_and_falls_back() {
    let asgard = Arc::new(AsgardRelayClient::unconfigured());
    let yggdrasil = Arc::new(YggdrasilRelayClient::unconfigured());
    let mgr = RelayManager::new(Some(asgard), Some(yggdrasil));
    let msg = sample_message();
    let res = mgr.route_message(&msg).await;
    assert!(res.is_err());
    let err_str = res.unwrap_err().to_string();
    assert!(err_str.contains("Asgard") || err_str.contains("relay"));
}

#[tokio::test]
async fn relay_manager_succeeds_when_first_relay_succeeds() {
    let asgard = Arc::new(AsgardRelayClient::mock_success());
    let yggdrasil = Arc::new(YggdrasilRelayClient::unconfigured());
    let mgr = RelayManager::new(Some(asgard), Some(yggdrasil));
    let msg = sample_message();
    let res = mgr.route_message(&msg).await;
    assert!(res.is_ok());
}

#[tokio::test]
async fn relay_manager_falls_back_to_second_relay_when_first_fails() {
    let asgard = Arc::new(AsgardRelayClient::unconfigured());
    let yggdrasil = Arc::new(YggdrasilRelayClient::mock_success());
    let mgr = RelayManager::new(Some(asgard), Some(yggdrasil));
    let msg = sample_message();
    let res = mgr.route_message(&msg).await;
    assert!(res.is_ok());
}

#[tokio::test]
async fn relay_manager_returns_last_error_when_all_fail() {
    let asgard = Arc::new(AsgardRelayClient::unconfigured());
    let yggdrasil = Arc::new(YggdrasilRelayClient::unconfigured());
    let mgr = RelayManager::new(Some(asgard), Some(yggdrasil));
    let msg = sample_message();
    let res = mgr.route_message(&msg).await;
    let err = res.unwrap_err();
    assert!(err.to_string().contains("Yggdrasil") || err.to_string().contains("relay"));
}

#[tokio::test]
async fn asgard_relay_client_unconfigured_returns_error() {
    let client = AsgardRelayClient::unconfigured();
    let msg = sample_message();
    let res = client.route_message(&msg).await;
    assert!(res.is_err());
}

#[tokio::test]
async fn yggdrasil_relay_client_unconfigured_returns_error() {
    let client = YggdrasilRelayClient::unconfigured();
    let msg = sample_message();
    let res = client.route_message(&msg).await;
    assert!(res.is_err());
}

//! Tests for Phase 8.3.1: Yggdrasil Discovery Client (request, response, device list).

use bifrost::discovery::yggdrasil::{DeviceInfo, YggdrasilDiscoveryClient, YggdrasilDiscoveryStub};
use std::sync::Arc;

#[tokio::test]
async fn list_devices_returns_devices_from_provider() {
    let stub = YggdrasilDiscoveryStub::with_devices(vec![
        DeviceInfo {
            device_id: "dev-1".to_string(),
            user_id: "user-1".to_string(),
            host: "192.168.1.10".to_string(),
            port: 50051,
        },
        DeviceInfo {
            device_id: "dev-2".to_string(),
            user_id: "user-1".to_string(),
            host: "192.168.1.11".to_string(),
            port: 50051,
        },
    ]);
    let client = YggdrasilDiscoveryClient::new(Arc::new(stub));
    let devices = client.list_devices("user-1").await.unwrap();
    assert_eq!(devices.len(), 2);
    assert_eq!(devices[0].device_id, "dev-1");
    assert_eq!(devices[0].host, "192.168.1.10");
    assert_eq!(devices[1].device_id, "dev-2");
}

#[tokio::test]
async fn list_devices_returns_empty_when_stub_empty() {
    let stub = YggdrasilDiscoveryStub::with_devices(vec![]);
    let client = YggdrasilDiscoveryClient::new(Arc::new(stub));
    let devices = client.list_devices("user-1").await.unwrap();
    assert!(devices.is_empty());
}

#[tokio::test]
async fn list_devices_propagates_provider_error() {
    let stub = YggdrasilDiscoveryStub::failing("discovery unavailable");
    let client = YggdrasilDiscoveryClient::new(Arc::new(stub));
    let result = client.list_devices("user-1").await;
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("discovery unavailable"));
}

#[tokio::test]
async fn device_info_has_ws_endpoint() {
    let d = DeviceInfo {
        device_id: "d1".to_string(),
        user_id: "u1".to_string(),
        host: "host.local".to_string(),
        port: 8080,
    };
    assert_eq!(d.ws_url(), "ws://host.local:8080");
}

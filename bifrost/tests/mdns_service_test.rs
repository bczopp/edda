// bifrost/tests/mdns_service_test.rs
//! Integration tests for mDNS Service (Local Device Discovery)
//!
//! Tests cover:
//! - Service announcement (register device)
//! - Service discovery (find other devices)
//! - Service record parsing
//! - Error handling

use bifrost::discovery::{DiscoveredDevice, MDNSService, MDNSServiceStub};
use std::net::{IpAddr, Ipv4Addr};
use std::sync::Arc;
use std::time::Duration;

#[tokio::test]
async fn test_mdns_service_creation() {
    // GIVEN: Device-Parameter
    let device_id = "test-device-001".to_string();
    let port = 9001;
    
    // WHEN: MDNSService mit Stub erstellt wird
    let stub = MDNSServiceStub::new(device_id.clone(), port);
    let service = MDNSService::new(Arc::new(stub));
    
    // THEN: Service sollte erfolgreich erstellt werden
    let devices = service.get_discovered_devices().await;
    assert_eq!(devices.len(), 0);
}

#[tokio::test]
async fn test_mdns_service_register_unregister() {
    // GIVEN: Ein MDNSService mit Stub
    let device_id = "test-device-002".to_string();
    let port = 9002;
    let stub = Arc::new(MDNSServiceStub::new(device_id, port));
    let service = MDNSService::new(stub.clone());
    
    // WHEN: Service registriert wird
    let register_result = service.register().await;
    
    // THEN: Registrierung sollte erfolgreich sein
    assert!(register_result.is_ok());
    assert!(stub.is_registered().await);
    
    // WHEN: Service unregistriert wird
    let unregister_result = service.unregister().await;
    
    // THEN: Unregistrierung sollte erfolgreich sein
    assert!(unregister_result.is_ok());
    assert!(!stub.is_registered().await);
}

#[tokio::test]
async fn test_mdns_service_discovery_empty() {
    // GIVEN: Ein MDNSService mit Stub (keine simulierten Devices)
    let device_id = "test-device-003".to_string();
    let port = 9003;
    let stub = Arc::new(MDNSServiceStub::new(device_id, port));
    let service = MDNSService::new(stub.clone());
    
    // WHEN: Discovery durchgeführt wird
    let discovery_timeout = Duration::from_millis(100);
    let devices = service.browse(discovery_timeout).await;
    
    // THEN: Discovery sollte erfolgreich sein mit leerer Liste
    assert!(devices.is_ok());
    assert_eq!(devices.unwrap().len(), 0);
}

#[tokio::test]
async fn test_mdns_service_discovery_with_simulated_devices() {
    // GIVEN: Ein MDNSService mit simulierten Devices
    let device_id = "test-device-004".to_string();
    let port = 9004;
    let stub = Arc::new(MDNSServiceStub::new(device_id, port));
    
    // Simulierte Devices hinzufügen
    stub.add_simulated_device(DiscoveredDevice {
        device_id: "device-1".to_string(),
        ip_address: IpAddr::V4(Ipv4Addr::new(192, 168, 1, 100)),
        port: 9001,
        hostname: "device-1.local".to_string(),
    }).await;
    
    stub.add_simulated_device(DiscoveredDevice {
        device_id: "device-2".to_string(),
        ip_address: IpAddr::V4(Ipv4Addr::new(192, 168, 1, 101)),
        port: 9002,
        hostname: "device-2.local".to_string(),
    }).await;
    
    let service = MDNSService::new(stub.clone());
    
    // WHEN: Discovery durchgeführt wird
    let devices = service.browse(Duration::from_millis(100)).await.unwrap();
    
    // THEN: Beide Devices sollten gefunden werden
    assert_eq!(devices.len(), 2);
    assert!(devices.iter().any(|d| d.device_id == "device-1"));
    assert!(devices.iter().any(|d| d.device_id == "device-2"));
}

#[tokio::test]
async fn test_mdns_service_clear_devices() {
    // GIVEN: Ein MDNSService mit Stub
    let device_id = "test-device-005".to_string();
    let port = 9005;
    let stub = Arc::new(MDNSServiceStub::new(device_id, port));
    let service = MDNSService::new(stub.clone());
    
    // WHEN: Devices gecleart werden
    service.clear_discovered_devices().await;
    
    // THEN: Device-Liste sollte leer sein
    let devices = service.get_discovered_devices().await;
    assert_eq!(devices.len(), 0);
}

#[tokio::test]
async fn test_mdns_service_filters_own_device() {
    // GIVEN: Ein registrierter MDNSService mit eigenem Device als simuliertes Device
    let device_id = "test-device-own".to_string();
    let port = 9007;
    let stub = Arc::new(MDNSServiceStub::new(device_id.clone(), port));
    
    // Füge eigenes Device als simuliertes Device hinzu (sollte gefiltert werden)
    stub.add_simulated_device(DiscoveredDevice {
        device_id: device_id.clone(),
        ip_address: IpAddr::V4(Ipv4Addr::new(192, 168, 1, 200)),
        port,
        hostname: "test-device-own.local".to_string(),
    }).await;
    
    // Füge anderes Device hinzu
    stub.add_simulated_device(DiscoveredDevice {
        device_id: "other-device".to_string(),
        ip_address: IpAddr::V4(Ipv4Addr::new(192, 168, 1, 201)),
        port: 9008,
        hostname: "other-device.local".to_string(),
    }).await;
    
    let service = MDNSService::new(stub.clone());
    
    // WHEN: Discovery durchgeführt wird
    let devices = service.browse(Duration::from_millis(100)).await.unwrap();
    
    // THEN: Eigenes Device sollte nicht in der Liste sein
    assert_eq!(devices.len(), 1);
    assert!(!devices.iter().any(|d| d.device_id == device_id));
    assert!(devices.iter().any(|d| d.device_id == "other-device"));
}

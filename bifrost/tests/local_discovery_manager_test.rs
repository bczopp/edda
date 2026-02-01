// bifrost/tests/local_discovery_manager_test.rs
//! Integration tests for LocalDiscoveryManager
//!
//! Tests cover:
//! - Discovery-Requests senden
//! - Discovery-Responses verarbeiten
//! - Discovery-Timeouts behandeln
//! - Device-Liste aktualisieren

use bifrost::discovery::{
    DiscoveredDevice, LocalDiscoveryConfig, LocalDiscoveryManager, MDNSService, MDNSServiceStub,
};
use std::net::{IpAddr, Ipv4Addr};
use std::sync::Arc;
use std::time::Duration;

fn create_test_manager(device_id: &str, port: u16) -> (LocalDiscoveryManager, Arc<MDNSServiceStub>) {
    let stub = Arc::new(MDNSServiceStub::new(device_id.to_string(), port));
    let service = MDNSService::new(stub.clone());
    let config = LocalDiscoveryConfig::default();
    let manager = LocalDiscoveryManager::new(service, config, device_id.to_string());
    (manager, stub)
}

#[tokio::test]
async fn test_local_discovery_manager_creation() {
    // GIVEN: Discovery-Konfiguration
    let (manager, _) = create_test_manager("test-device-101", 9101);
    
    // THEN: Manager sollte erfolgreich erstellt sein
    assert_eq!(manager.own_device_id, "test-device-101");
}

#[tokio::test]
async fn test_local_discovery_manager_announcement() {
    // GIVEN: Ein LocalDiscoveryManager
    let (manager, stub) = create_test_manager("test-device-102", 9102);
    
    // WHEN: Announcement gestartet wird
    let start_result = manager.start_announcement().await;
    
    // THEN: Announcement sollte erfolgreich starten
    assert!(start_result.is_ok());
    assert!(stub.is_registered().await);
    
    // WHEN: Announcement gestoppt wird
    let stop_result = manager.stop_announcement().await;
    
    // THEN: Stop sollte erfolgreich sein
    assert!(stop_result.is_ok());
    assert!(!stub.is_registered().await);
}

#[tokio::test]
async fn test_local_discovery_manager_discover_empty() {
    // GIVEN: Ein LocalDiscoveryManager ohne simulierte Devices
    let (manager, _) = create_test_manager("test-device-103", 9103);
    
    // WHEN: discover_devices() aufgerufen wird
    let devices = manager.discover_devices().await;
    
    // THEN: Discovery sollte erfolgreich sein (leere Liste ist OK)
    assert!(devices.is_ok());
    assert_eq!(devices.unwrap().len(), 0);
}

#[tokio::test]
async fn test_local_discovery_manager_discover_with_devices() {
    // GIVEN: Ein LocalDiscoveryManager mit simulierten Devices
    let (manager, stub) = create_test_manager("test-device-104", 9104);
    
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
    
    // WHEN: discover_devices() aufgerufen wird
    let devices = manager.discover_devices().await.unwrap();
    
    // THEN: Beide Devices sollten gefunden werden
    assert_eq!(devices.len(), 2);
    assert!(devices.iter().any(|d| d.device_id == "device-1"));
    assert!(devices.iter().any(|d| d.device_id == "device-2"));
}

#[tokio::test]
async fn test_local_discovery_manager_get_devices() {
    // GIVEN: Ein LocalDiscoveryManager
    let (manager, _) = create_test_manager("test-device-105", 9105);
    
    // WHEN: get_devices() aufgerufen wird (vor Discovery)
    let devices = manager.get_devices().await;
    
    // THEN: Liste sollte leer sein
    assert_eq!(devices.len(), 0);
}

#[tokio::test]
async fn test_local_discovery_manager_clear() {
    // GIVEN: Ein LocalDiscoveryManager mit simulierten Devices
    let (manager, stub) = create_test_manager("test-device-106", 9106);
    
    stub.add_simulated_device(DiscoveredDevice {
        device_id: "device-1".to_string(),
        ip_address: IpAddr::V4(Ipv4Addr::new(192, 168, 1, 100)),
        port: 9001,
        hostname: "device-1.local".to_string(),
    }).await;
    
    // Discovery durchführen
    manager.discover_devices().await.unwrap();
    
    // WHEN: clear_devices() aufgerufen wird
    manager.clear_devices().await;
    
    // THEN: Device-Liste sollte leer sein
    let devices = manager.get_devices().await;
    assert_eq!(devices.len(), 0);
}

#[tokio::test]
async fn test_local_discovery_manager_filters_own_device() {
    // GIVEN: Ein LocalDiscoveryManager mit eigenem Device als simuliertes Device
    let own_device_id = "test-device-own-107".to_string();
    let (manager, stub) = create_test_manager(&own_device_id, 9107);
    
    // Eigenes Device hinzufügen (sollte gefiltert werden)
    stub.add_simulated_device(DiscoveredDevice {
        device_id: own_device_id.clone(),
        ip_address: IpAddr::V4(Ipv4Addr::new(192, 168, 1, 200)),
        port: 9107,
        hostname: "test-device-own-107.local".to_string(),
    }).await;
    
    // Anderes Device hinzufügen
    stub.add_simulated_device(DiscoveredDevice {
        device_id: "other-device".to_string(),
        ip_address: IpAddr::V4(Ipv4Addr::new(192, 168, 1, 201)),
        port: 9108,
        hostname: "other-device.local".to_string(),
    }).await;
    
    // WHEN: Discovery durchgeführt wird
    let devices = manager.discover_devices().await.unwrap();
    
    // THEN: Eigenes Device sollte nicht in der Liste sein
    assert_eq!(devices.len(), 1);
    assert!(!devices.iter().any(|d| d.device_id == own_device_id));
    assert!(devices.iter().any(|d| d.device_id == "other-device"));
}

#[tokio::test]
async fn test_local_discovery_manager_continuous_config_not_set() {
    // GIVEN: Ein LocalDiscoveryManager ohne continuous config
    let stub = Arc::new(MDNSServiceStub::new("test-device-108".to_string(), 9108));
    let service = MDNSService::new(stub);
    let config = LocalDiscoveryConfig {
        discovery_timeout: Duration::from_millis(100),
        continuous_interval: None,
    };
    let manager = Arc::new(LocalDiscoveryManager::new(
        service,
        config,
        "test-device-108".to_string(),
    ));
    
    // WHEN: start_continuous_discovery() aufgerufen wird
    let result = manager.start_continuous_discovery().await;
    
    // THEN: Fehler sollte zurückgegeben werden (nicht konfiguriert)
    assert!(result.is_err());
}

#[tokio::test]
async fn test_local_discovery_manager_multiple_discoveries() {
    // GIVEN: Ein LocalDiscoveryManager
    let (manager, _) = create_test_manager("test-device-109", 9109);
    
    // WHEN: Mehrere Discoveries nacheinander durchgeführt werden
    let devices1 = manager.discover_devices().await;
    let devices2 = manager.discover_devices().await;
    
    // THEN: Beide sollten erfolgreich sein
    assert!(devices1.is_ok());
    assert!(devices2.is_ok());
}

#[tokio::test]
async fn test_local_discovery_manager_updates_device_list() {
    // GIVEN: Ein LocalDiscoveryManager
    let (manager, stub) = create_test_manager("test-device-110", 9110);
    
    // Erstes Device hinzufügen
    stub.add_simulated_device(DiscoveredDevice {
        device_id: "device-1".to_string(),
        ip_address: IpAddr::V4(Ipv4Addr::new(192, 168, 1, 100)),
        port: 9001,
        hostname: "device-1.local".to_string(),
    }).await;
    
    // Erste Discovery
    let devices1 = manager.discover_devices().await.unwrap();
    assert_eq!(devices1.len(), 1);
    
    // Zweites Device hinzufügen
    stub.add_simulated_device(DiscoveredDevice {
        device_id: "device-2".to_string(),
        ip_address: IpAddr::V4(Ipv4Addr::new(192, 168, 1, 101)),
        port: 9002,
        hostname: "device-2.local".to_string(),
    }).await;
    
    // Zweite Discovery
    let devices2 = manager.discover_devices().await.unwrap();
    
    // THEN: Device-Liste sollte aktualisiert werden
    assert_eq!(devices2.len(), 2);
    
    // Cached list sollte auch aktualisiert sein
    let cached = manager.get_devices().await;
    assert_eq!(cached.len(), 2);
}

// Device resolver tests (resolve cryptic ID → display name + capabilities).

mod common;

use common::{
    minimal_capabilities, resolver_with_one_device, sensor_actuator_capabilities,
};
use jotunheim_esp32::capability::{DeviceResolver, InMemoryDeviceResolver, tools_for_device};

#[test]
fn resolve_returns_none_for_unknown_id() {
    let r = InMemoryDeviceResolver::new();
    assert!(r.resolve("ESP-7F2A").is_none());
}

#[test]
fn register_and_resolve() {
    let caps = sensor_actuator_capabilities(
        "ESP-7F2A",
        "Living Room Sensor",
        &["DHT22"],
        &[],
    );
    let resolver = InMemoryDeviceResolver::new();
    resolver.register(
        "ESP-7F2A".to_string(),
        "Wohnzimmer-Sensor".to_string(),
        caps,
    );
    let resolved = resolver.resolve("ESP-7F2A").unwrap();
    assert_eq!(resolved.device_id, "ESP-7F2A");
    assert_eq!(resolved.display_name, "Wohnzimmer-Sensor");
    assert_eq!(resolved.capabilities.device_name, "Living Room Sensor");
}

#[test]
fn list_device_ids() {
    let resolver = InMemoryDeviceResolver::new();
    let caps1 = minimal_capabilities("d1", "n1");
    let caps2 = minimal_capabilities("d2", "n2");
    resolver.register("d1".to_string(), "Device 1".to_string(), caps1);
    resolver.register("d2".to_string(), "Device 2".to_string(), caps2);
    let ids = resolver.list_device_ids();
    assert_eq!(ids.len(), 2);
    assert!(ids.contains(&"d1".to_string()));
    assert!(ids.contains(&"d2".to_string()));
}

#[test]
fn unregister_removes_device() {
    let resolver = InMemoryDeviceResolver::new();
    let caps = minimal_capabilities("x", "n");
    resolver.register("x".to_string(), "X".to_string(), caps);
    assert!(resolver.resolve("x").is_some());
    resolver.unregister("x");
    assert!(resolver.resolve("x").is_none());
}

#[test]
fn tools_for_device_returns_generated_tools_with_prefix() {
    let resolver = resolver_with_one_device(
        "ESP-1",
        "Küche",
        sensor_actuator_capabilities("ESP-1", "Kitchen", &["DHT22"], &[]),
    );
    let tools = tools_for_device(resolver.as_ref(), "ESP-1").unwrap();
    assert!(!tools.is_empty());
    assert!(tools.iter().any(|t| t.name.starts_with("Küche_") && t.name.contains("dht22")));
}

//! Test data generators for Jotunheim capabilities, resolvers, and devices.
//! Reduces duplication across integration tests (Phase 1.2.2).

use jotunheim_esp32::capability::{DeviceCapabilityBuilder, InMemoryDeviceResolver};
use jotunheim_esp32::grpc::proto::jotunheim_capability::JotunheimCapabilities;
use std::sync::Arc;

/// Default device type used in generated capabilities.
pub const DEFAULT_DEVICE_TYPE: &str = "ESP32";
/// Default protocol version for generated capabilities.
pub const DEFAULT_PROTOCOL_VERSION: &str = "1";
/// Default firmware version for generated capabilities.
pub const DEFAULT_FIRMWARE_VERSION: &str = "0";

/// Returns minimal `JotunheimCapabilities` with only device identity (no sensors/actuators).
pub fn minimal_capabilities(device_id: &str, device_name: &str) -> JotunheimCapabilities {
    DeviceCapabilityBuilder::new()
        .device_id(device_id)
        .device_name(device_name)
        .device_type(DEFAULT_DEVICE_TYPE)
        .firmware_version(DEFAULT_FIRMWARE_VERSION)
        .protocol_version(DEFAULT_PROTOCOL_VERSION)
        .build()
}

/// Returns capabilities with the given sensors and optional actuators.
pub fn sensor_actuator_capabilities(
    device_id: &str,
    device_name: &str,
    sensors: &[&str],
    actuators: &[&str],
) -> JotunheimCapabilities {
    let mut b = DeviceCapabilityBuilder::new()
        .device_id(device_id)
        .device_name(device_name)
        .device_type(DEFAULT_DEVICE_TYPE)
        .firmware_version(DEFAULT_FIRMWARE_VERSION)
        .protocol_version(DEFAULT_PROTOCOL_VERSION);
    if !sensors.is_empty() {
        b = b.sensors(sensors);
    }
    if !actuators.is_empty() {
        b = b.actuators(actuators);
    }
    b.build()
}

/// Returns capabilities with DHT22 and LED (common fixture).
pub fn dht22_led_capabilities(device_id: &str, device_name: &str) -> JotunheimCapabilities {
    sensor_actuator_capabilities(device_id, device_name, &["DHT22"], &["LED"])
}

/// Returns an `InMemoryDeviceResolver` with one registered device.
pub fn resolver_with_one_device(
    device_id: &str,
    display_name: &str,
    capabilities: JotunheimCapabilities,
) -> Arc<InMemoryDeviceResolver> {
    let resolver = Arc::new(InMemoryDeviceResolver::new());
    resolver.register(
        device_id.to_string(),
        display_name.to_string(),
        capabilities,
    );
    resolver
}

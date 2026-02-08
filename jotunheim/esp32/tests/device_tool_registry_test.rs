// Device Tool Registry tests (auto-discovery of sensors/actuators → tool definitions).

mod common;

use common::sensor_actuator_capabilities;
use jotunheim_esp32::capability::{
    generate_tools_from_capabilities, DeviceCapabilityBuilder, GeneratedToolDef,
};

#[test]
fn generate_tools_from_sensors_and_actuators() {
    let caps = sensor_actuator_capabilities("dev1", "Test", &["DHT22", "BMP280"], &["LED"]);
    let tools = generate_tools_from_capabilities(&caps, None);
    let names: Vec<&str> = tools.iter().map(|t| t.name.as_str()).collect();
    assert!(names.contains(&"read_dht22_temperature"));
    assert!(names.contains(&"read_dht22_humidity"));
    assert!(names.contains(&"read_bmp280_temperature"));
    assert!(names.contains(&"read_bmp280_pressure"));
    assert!(names.contains(&"set_led"));
}

#[test]
fn prefix_avoids_name_clash() {
    let caps = sensor_actuator_capabilities("kitchen", "Kitchen", &["DHT22"], &[]);
    let tools = generate_tools_from_capabilities(&caps, Some("kitchen"));
    let names: Vec<&str> = tools.iter().map(|t| t.name.as_str()).collect();
    assert!(names.contains(&"kitchen_read_dht22_temperature"));
}

#[test]
fn unknown_sensor_gets_generic_tool() {
    let caps = sensor_actuator_capabilities("d", "n", &["CustomSensor"], &[]);
    let tools = generate_tools_from_capabilities(&caps, None);
    assert!(!tools.is_empty());
    assert!(tools.iter().any(|t| t.name.contains("read_sensor") && t.name.contains("customsensor")));
}

#[test]
fn gpio_digital_adds_gpio_read_write() {
    let caps = DeviceCapabilityBuilder::new()
        .device_id("d")
        .device_name("n")
        .device_type("ESP32")
        .firmware_version("0")
        .protocol_version("1")
        .gpio_pins(&[2, 4])
        .gpio_digital(true)
        .build();
    let tools = generate_tools_from_capabilities(&caps, None);
    assert!(tools.iter().any(|t| t.name == "gpio_read"));
    assert!(tools.iter().any(|t| t.name == "gpio_write"));
}

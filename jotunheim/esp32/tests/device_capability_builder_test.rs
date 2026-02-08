// DeviceCapabilityBuilder tests (Phase 4.1.1, TDD).

use jotunheim_esp32::capability::DeviceCapabilityBuilder;

#[test]
fn build_with_minimal_device_info_produces_valid_capabilities() {
    let cap = DeviceCapabilityBuilder::new()
        .device_id("dev-1")
        .device_name("TestDevice")
        .device_type("ESP32")
        .firmware_version("0.1.0")
        .protocol_version("1.0")
        .build();
    assert_eq!(cap.device_id, "dev-1");
    assert_eq!(cap.device_name, "TestDevice");
    assert_eq!(cap.device_type, "ESP32");
    assert_eq!(cap.firmware_version, "0.1.0");
    assert_eq!(cap.protocol_version, "1.0");
    assert!(cap.tools.is_empty());
}

#[test]
fn build_with_hardware_includes_gpio_and_interfaces() {
    let cap = DeviceCapabilityBuilder::new()
        .device_id("d")
        .device_name("n")
        .device_type("ESP32")
        .firmware_version("0")
        .protocol_version("1")
        .gpio_pins(&[1, 2, 3])
        .gpio_digital(true)
        .gpio_analog(true)
        .gpio_pwm(true)
        .interfaces(&["I2C", "SPI"])
        .sensors(&["DHT22"])
        .actuators(&["LED"])
        .build();
    let h = cap.hardware.as_ref().unwrap();
    let gpio = h.gpio.as_ref().unwrap();
    assert_eq!(gpio.available_pins, [1, 2, 3]);
    assert!(gpio.digital);
    assert!(gpio.analog);
    assert!(gpio.pwm);
    assert_eq!(h.interfaces, ["I2C", "SPI"]);
    assert_eq!(h.sensors, ["DHT22"]);
    assert_eq!(h.actuators, ["LED"]);
}

#[test]
fn build_with_resources_includes_limits() {
    let cap = DeviceCapabilityBuilder::new()
        .device_id("d")
        .device_name("n")
        .device_type("ESP32")
        .firmware_version("0")
        .protocol_version("1")
        .max_memory_kb(256)
        .max_concurrent_tools(4)
        .build();
    let r = cap.resources.as_ref().unwrap();
    assert_eq!(r.max_memory_kb, 256);
    assert_eq!(r.max_concurrent_tools, 4);
}

#[test]
fn build_with_tools_includes_tool_list() {
    let cap = DeviceCapabilityBuilder::new()
        .device_id("d")
        .device_name("n")
        .device_type("ESP32")
        .firmware_version("0")
        .protocol_version("1")
        .add_tool("led_on", "Turn LED on", "void")
        .build();
    assert_eq!(cap.tools.len(), 1);
    assert_eq!(cap.tools[0].name, "led_on");
    assert_eq!(cap.tools[0].description, "Turn LED on");
    assert_eq!(cap.tools[0].return_type, "void");
}

#[test]
fn build_with_features_sets_protocol_features() {
    let cap = DeviceCapabilityBuilder::new()
        .device_id("d")
        .device_name("n")
        .device_type("ESP32")
        .firmware_version("0")
        .protocol_version("1")
        .streaming(true)
        .compression(false)
        .encryption(true)
        .build();
    let f = cap.features.as_ref().unwrap();
    assert!(f.streaming);
    assert!(!f.compression);
    assert!(f.encryption);
}

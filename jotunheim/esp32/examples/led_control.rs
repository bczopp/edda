//! LED-Control-Example (Phase 12.2.1).
//! Builds device capabilities with LED actuator and prints a minimal workflow.
//! On ESP32 you would wire GPIO and call hardware APIs; here we only demonstrate the API.

use jotunheim_esp32::capability::DeviceCapabilityBuilder;

fn main() {
    let caps = DeviceCapabilityBuilder::new()
        .device_id("led-example")
        .device_name("LED Controller")
        .device_type("ESP32")
        .firmware_version("0.1.0")
        .protocol_version("1.0")
        .actuators(&["LED"])
        .gpio_pins(&[2])
        .gpio_digital(true)
        .add_tool("led_on", "Turn LED on", "void")
        .add_tool("led_off", "Turn LED off", "void")
        .build();
    println!("LED-Control device: {} (type: {})", caps.device_name, caps.device_type);
    println!("Actuators: {:?}", caps.hardware.as_ref().map(|h| &h.actuators));
    println!("Tools: {} registered", caps.tools.len());
}

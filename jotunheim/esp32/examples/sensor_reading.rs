//! Sensor-Reading-Example (Phase 12.2.1).
//! Builds device capabilities for a sensor node and shows resource-monitor usage.
//! On ESP32 you would read from I2C/ADC; here we only demonstrate the API.

use jotunheim_esp32::capability::DeviceCapabilityBuilder;
use jotunheim_esp32::resources::ResourceMonitor;

fn main() {
    let caps = DeviceCapabilityBuilder::new()
        .device_id("sensor-example")
        .device_name("Room Sensor")
        .device_type("ESP32")
        .firmware_version("0.1.0")
        .protocol_version("1.0")
        .sensors(&["DHT22", "BMP280"])
        .interfaces(&["I2C"])
        .max_memory_kb(128)
        .build();
    println!("Sensor device: {} (type: {})", caps.device_name, caps.device_type);
    println!("Sensors: {:?}", caps.hardware.as_ref().map(|h| &h.sensors));

    let mon = ResourceMonitor::new(80, 120, 1024);
    mon.set_ram_usage_kb(32);
    mon.set_cpu_usage_percent(10);
    println!("RAM: {} KB, CPU: {}%", mon.ram_usage_kb(), mon.cpu_usage_percent());
}

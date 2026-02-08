//! DeviceCapabilityBuilder (Phase 4.1.1, TDD).

use crate::grpc::proto::jotunheim_capability::{
    GpioInfo, HardwareCapabilities, JotunheimCapabilities, ProtocolFeatures,
    ResourceLimits, Tool,
};

/// Builds JotunheimCapabilities from device info, hardware, resources, and features.
pub struct DeviceCapabilityBuilder {
    device_id: String,
    device_name: String,
    device_type: String,
    firmware_version: String,
    protocol_version: String,
    tools: Vec<Tool>,
    gpio_pins: Vec<i32>,
    gpio_digital: bool,
    gpio_analog: bool,
    gpio_pwm: bool,
    interfaces: Vec<String>,
    sensors: Vec<String>,
    actuators: Vec<String>,
    max_memory_kb: Option<u32>,
    max_cpu_percent: Option<u32>,
    max_concurrent_tools: Option<u32>,
    streaming: bool,
    compression: bool,
    encryption: bool,
}

impl Default for DeviceCapabilityBuilder {
    fn default() -> Self {
        Self {
            device_id: String::new(),
            device_name: String::new(),
            device_type: String::new(),
            firmware_version: String::new(),
            protocol_version: String::new(),
            tools: Vec::new(),
            gpio_pins: Vec::new(),
            gpio_digital: false,
            gpio_analog: false,
            gpio_pwm: false,
            interfaces: Vec::new(),
            sensors: Vec::new(),
            actuators: Vec::new(),
            max_memory_kb: None,
            max_cpu_percent: None,
            max_concurrent_tools: None,
            streaming: false,
            compression: false,
            encryption: false,
        }
    }
}

impl DeviceCapabilityBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn device_id(mut self, v: &str) -> Self {
        self.device_id = v.to_string();
        self
    }
    pub fn device_name(mut self, v: &str) -> Self {
        self.device_name = v.to_string();
        self
    }
    pub fn device_type(mut self, v: &str) -> Self {
        self.device_type = v.to_string();
        self
    }
    pub fn firmware_version(mut self, v: &str) -> Self {
        self.firmware_version = v.to_string();
        self
    }
    pub fn protocol_version(mut self, v: &str) -> Self {
        self.protocol_version = v.to_string();
        self
    }

    pub fn add_tool(mut self, name: &str, description: &str, return_type: &str) -> Self {
        self.tools.push(Tool {
            name: name.to_string(),
            description: description.to_string(),
            parameters: Vec::new(),
            return_type: return_type.to_string(),
        });
        self
    }

    pub fn gpio_pins(mut self, pins: &[i32]) -> Self {
        self.gpio_pins = pins.to_vec();
        self
    }
    pub fn gpio_digital(mut self, v: bool) -> Self {
        self.gpio_digital = v;
        self
    }
    pub fn gpio_analog(mut self, v: bool) -> Self {
        self.gpio_analog = v;
        self
    }
    pub fn gpio_pwm(mut self, v: bool) -> Self {
        self.gpio_pwm = v;
        self
    }
    pub fn interfaces(mut self, v: &[&str]) -> Self {
        self.interfaces = v.iter().map(|s| (*s).to_string()).collect();
        self
    }
    pub fn sensors(mut self, v: &[&str]) -> Self {
        self.sensors = v.iter().map(|s| (*s).to_string()).collect();
        self
    }
    pub fn actuators(mut self, v: &[&str]) -> Self {
        self.actuators = v.iter().map(|s| (*s).to_string()).collect();
        self
    }

    pub fn max_memory_kb(mut self, v: u32) -> Self {
        self.max_memory_kb = Some(v);
        self
    }
    pub fn max_cpu_percent(mut self, v: u32) -> Self {
        self.max_cpu_percent = Some(v);
        self
    }
    pub fn max_concurrent_tools(mut self, v: u32) -> Self {
        self.max_concurrent_tools = Some(v);
        self
    }

    pub fn streaming(mut self, v: bool) -> Self {
        self.streaming = v;
        self
    }
    pub fn compression(mut self, v: bool) -> Self {
        self.compression = v;
        self
    }
    pub fn encryption(mut self, v: bool) -> Self {
        self.encryption = v;
        self
    }

    pub fn build(self) -> JotunheimCapabilities {
        let hardware = if self.gpio_pins.is_empty()
            && self.interfaces.is_empty()
            && self.sensors.is_empty()
            && self.actuators.is_empty()
        {
            None
        } else {
            Some(HardwareCapabilities {
                gpio: Some(GpioInfo {
                    available_pins: self.gpio_pins,
                    digital: self.gpio_digital,
                    analog: self.gpio_analog,
                    pwm: self.gpio_pwm,
                }),
                interfaces: self.interfaces,
                sensors: self.sensors,
                actuators: self.actuators,
            })
        };
        let resources = if self.max_memory_kb.is_none()
            && self.max_cpu_percent.is_none()
            && self.max_concurrent_tools.is_none()
        {
            None
        } else {
            Some(ResourceLimits {
                max_memory_kb: self.max_memory_kb.unwrap_or(0),
                max_cpu_percent: self.max_cpu_percent.unwrap_or(0),
                max_concurrent_tools: self.max_concurrent_tools.unwrap_or(0),
            })
        };
        let features = Some(ProtocolFeatures {
            streaming: self.streaming,
            compression: self.compression,
            encryption: self.encryption,
        });
        JotunheimCapabilities {
            device_id: self.device_id,
            device_name: self.device_name,
            device_type: self.device_type,
            firmware_version: self.firmware_version,
            protocol_version: self.protocol_version,
            tools: self.tools,
            hardware,
            resources,
            features,
        }
    }
}

//! Device Tool Registry: Auto-generate tool definitions from sensors/actuators/GPIO.
//! So users don't have to create tool-calling definitions manually.
//! See docs/DEVICE_TOOL_AUTO_DISCOVERY.md.

use crate::grpc::proto::jotunheim_capability::JotunheimCapabilities;
use std::collections::{HashMap, HashSet};

/// One parameter of a generated tool.
#[derive(Debug, Clone)]
pub struct GeneratedParam {
    pub name: String,
    pub param_type: String, // "string" | "number" | "boolean"
    pub required: bool,
    pub description: Option<String>,
}

/// Tool definition generated from device capabilities (no script; script is added by Loki/platform).
#[derive(Debug, Clone)]
pub struct GeneratedToolDef {
    pub name: String,
    pub description: String,
    pub parameters: Vec<GeneratedParam>,
    pub return_type: String, // "string" | "number" | "boolean" | "void"
}

/// Registry: sensor/actuator type → list of tool definitions.
fn builtin_sensor_tools() -> HashMap<String, Vec<GeneratedToolDef>> {
    let mut m = HashMap::new();
    m.insert(
        "DHT22".to_string(),
        vec![
            GeneratedToolDef {
                name: "read_dht22_temperature".to_string(),
                description: "Read temperature in °C from DHT22 sensor".to_string(),
                parameters: vec![GeneratedParam {
                    name: "pin".to_string(),
                    param_type: "number".to_string(),
                    required: true,
                    description: Some("GPIO pin number".to_string()),
                }],
                return_type: "number".to_string(),
            },
            GeneratedToolDef {
                name: "read_dht22_humidity".to_string(),
                description: "Read relative humidity in % from DHT22 sensor".to_string(),
                parameters: vec![GeneratedParam {
                    name: "pin".to_string(),
                    param_type: "number".to_string(),
                    required: true,
                    description: Some("GPIO pin number".to_string()),
                }],
                return_type: "number".to_string(),
            },
        ],
    );
    m.insert(
        "DS18B20".to_string(),
        vec![GeneratedToolDef {
            name: "read_ds18b20_temperature".to_string(),
            description: "Read temperature in °C from DS18B20 sensor".to_string(),
            parameters: vec![GeneratedParam {
                name: "pin".to_string(),
                param_type: "number".to_string(),
                required: true,
                description: Some("GPIO pin number (1-Wire)".to_string()),
            }],
            return_type: "number".to_string(),
        }],
    );
    m.insert(
        "BMP280".to_string(),
        vec![
            GeneratedToolDef {
                name: "read_bmp280_temperature".to_string(),
                description: "Read temperature in °C from BMP280 (I2C)".to_string(),
                parameters: vec![],
                return_type: "number".to_string(),
            },
            GeneratedToolDef {
                name: "read_bmp280_pressure".to_string(),
                description: "Read pressure in hPa from BMP280 (I2C)".to_string(),
                parameters: vec![],
                return_type: "number".to_string(),
            },
        ],
    );
    m
}

fn builtin_actuator_tools() -> HashMap<String, Vec<GeneratedToolDef>> {
    let mut m = HashMap::new();
    m.insert(
        "LED".to_string(),
        vec![GeneratedToolDef {
            name: "set_led".to_string(),
            description: "Turn LED on or off".to_string(),
            parameters: vec![
                GeneratedParam {
                    name: "pin".to_string(),
                    param_type: "number".to_string(),
                    required: true,
                    description: Some("GPIO pin number".to_string()),
                },
                GeneratedParam {
                    name: "on".to_string(),
                    param_type: "boolean".to_string(),
                    required: true,
                    description: Some("true = on, false = off".to_string()),
                },
            ],
            return_type: "void".to_string(),
        }],
    );
    m.insert(
        "Relay".to_string(),
        vec![GeneratedToolDef {
            name: "set_relay".to_string(),
            description: "Set relay on or off".to_string(),
            parameters: vec![
                GeneratedParam {
                    name: "pin".to_string(),
                    param_type: "number".to_string(),
                    required: true,
                    description: None,
                },
                GeneratedParam {
                    name: "on".to_string(),
                    param_type: "boolean".to_string(),
                    required: true,
                    description: None,
                },
            ],
            return_type: "void".to_string(),
        }],
    );
    m.insert(
        "Motor".to_string(),
        vec![GeneratedToolDef {
            name: "set_motor".to_string(),
            description: "Set motor speed or direction".to_string(),
            parameters: vec![
                GeneratedParam {
                    name: "pin".to_string(),
                    param_type: "number".to_string(),
                    required: true,
                    description: None,
                },
                GeneratedParam {
                    name: "value".to_string(),
                    param_type: "number".to_string(),
                    required: true,
                    description: Some("Speed 0..100 or -100..100 for direction".to_string()),
                },
            ],
            return_type: "void".to_string(),
        }],
    );
    m
}

/// Generic fallback tools for unknown sensor/actuator types.
fn generic_sensor_tool(sensor_type: &str) -> GeneratedToolDef {
    GeneratedToolDef {
        name: format!("read_sensor_{}", sensor_type.to_lowercase().replace(' ', "_")),
        description: format!("Read value from sensor type {}", sensor_type),
        parameters: vec![
            GeneratedParam {
                name: "pin".to_string(),
                param_type: "number".to_string(),
                required: false,
                description: Some("GPIO pin if applicable".to_string()),
            },
        ],
        return_type: "number".to_string(),
    }
}

fn generic_actuator_tool(actuator_type: &str) -> GeneratedToolDef {
    GeneratedToolDef {
        name: format!("set_actuator_{}", actuator_type.to_lowercase().replace(' ', "_")),
        description: format!("Set actuator type {}", actuator_type),
        parameters: vec![
            GeneratedParam {
                name: "pin".to_string(),
                param_type: "number".to_string(),
                required: true,
                description: None,
            },
            GeneratedParam {
                name: "value".to_string(),
                param_type: "string".to_string(),
                required: true,
                description: Some("Value or command".to_string()),
            },
        ],
        return_type: "void".to_string(),
    }
}

/// Generate tool definitions from device capabilities.
/// Optional `prefix` (e.g. device_id or device_name) avoids name clashes when multiple devices are present.
pub fn generate_tools_from_capabilities(
    caps: &JotunheimCapabilities,
    prefix: Option<&str>,
) -> Vec<GeneratedToolDef> {
    let sensor_map = builtin_sensor_tools();
    let actuator_map = builtin_actuator_tools();
    let prefix = prefix.unwrap_or("").to_string();
    let prefix_name = |name: &str| {
        if prefix.is_empty() {
            name.to_string()
        } else {
            format!("{}_{}", prefix, name)
        }
    };

    let mut out = Vec::new();
    let mut seen_sensors = HashSet::new();
    let mut seen_actuators = HashSet::new();

    if let Some(hw) = &caps.hardware {
        for s in &hw.sensors {
            if seen_sensors.insert(s.as_str()) {
                let tools = sensor_map
                    .get(s)
                    .cloned()
                    .unwrap_or_else(|| vec![generic_sensor_tool(s)]);
                for mut t in tools {
                    t.name = prefix_name(&t.name);
                    out.push(t);
                }
            }
        }
        for a in &hw.actuators {
            if seen_actuators.insert(a.as_str()) {
                let tools = actuator_map
                    .get(a)
                    .cloned()
                    .unwrap_or_else(|| vec![generic_actuator_tool(a)]);
                for mut t in tools {
                    t.name = prefix_name(&t.name);
                    out.push(t);
                }
            }
        }
        if let Some(gpio) = &hw.gpio {
            if gpio.digital {
                out.push(GeneratedToolDef {
                    name: prefix_name("gpio_read"),
                    description: "Read digital GPIO pin (high/low)".to_string(),
                    parameters: vec![GeneratedParam {
                        name: "pin".to_string(),
                        param_type: "number".to_string(),
                        required: true,
                        description: None,
                    }],
                    return_type: "boolean".to_string(),
                });
                out.push(GeneratedToolDef {
                    name: prefix_name("gpio_write"),
                    description: "Write digital GPIO pin high or low".to_string(),
                    parameters: vec![
                        GeneratedParam {
                            name: "pin".to_string(),
                            param_type: "number".to_string(),
                            required: true,
                            description: None,
                        },
                        GeneratedParam {
                            name: "high".to_string(),
                            param_type: "boolean".to_string(),
                            required: true,
                            description: None,
                        },
                    ],
                    return_type: "void".to_string(),
                });
            }
        }
    }

    out
}

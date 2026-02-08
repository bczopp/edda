//! Actuator controller – delegates to HardwareAccess (Phase 6.5.1).

use std::sync::Arc;

use crate::error::Result;
use crate::hardware::HardwareAccess;

/// Actuator controller: LED, relay, motor (optional).
pub struct ActuatorController {
    hardware: Arc<dyn HardwareAccess>,
}

impl ActuatorController {
    pub fn new(hardware: Arc<dyn HardwareAccess>) -> Self {
        Self { hardware }
    }

    /// Set LED brightness 0.0–1.0.
    pub fn set_led(&self, actuator_id: &str, brightness: f64) -> Result<()> {
        let v = brightness.clamp(0.0, 1.0);
        self.hardware.control_actuator(actuator_id, v)
    }

    /// Set relay on/off (true = on, false = off).
    pub fn set_relay(&self, actuator_id: &str, on: bool) -> Result<()> {
        let v = if on { 1.0 } else { 0.0 };
        self.hardware.control_actuator(actuator_id, v)
    }

    /// Set motor value 0.0–1.0 (optional).
    pub fn set_motor(&self, actuator_id: &str, value: f64) -> Result<()> {
        let v = value.clamp(0.0, 1.0);
        self.hardware.control_actuator(actuator_id, v)
    }
}

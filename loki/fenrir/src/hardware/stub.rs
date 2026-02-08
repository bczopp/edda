//! Stub implementation of HardwareAccess for tests and when no hardware is present.

use std::collections::HashMap;
use std::sync::RwLock;

use super::HardwareAccess;
use crate::error::{FenrirError, Result};

/// Stub implementation: in-memory state, no real hardware.
pub struct StubHardwareAccess {
    gpio: RwLock<HashMap<u8, bool>>,
}

impl Default for StubHardwareAccess {
    fn default() -> Self {
        Self {
            gpio: RwLock::new(HashMap::new()),
        }
    }
}

impl StubHardwareAccess {
    pub fn new() -> Self {
        Self::default()
    }
}

impl HardwareAccess for StubHardwareAccess {
    fn read_gpio(&self, pin: u8) -> Result<bool> {
        let g = self.gpio.read().map_err(|e| FenrirError::Gpio(e.to_string()))?;
        Ok(*g.get(&pin).unwrap_or(&false))
    }

    fn write_gpio(&self, pin: u8, value: bool) -> Result<()> {
        let mut g = self.gpio.write().map_err(|e| FenrirError::Gpio(e.to_string()))?;
        g.insert(pin, value);
        Ok(())
    }

    fn read_sensor(&self, _sensor_id: &str) -> Result<f64> {
        // Stub: return fixed value
        Ok(0.0)
    }

    fn control_actuator(&self, _actuator_id: &str, _value: f64) -> Result<()> {
        Ok(())
    }
}

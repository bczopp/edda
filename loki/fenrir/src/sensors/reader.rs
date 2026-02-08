//! Sensor reader – delegates to HardwareAccess (Phase 6.4.1).

use std::sync::Arc;

use crate::error::Result;
use crate::hardware::HardwareAccess;

/// Sensor reader: temperature (°C), humidity (0–100), motion (optional).
pub struct SensorReader {
    hardware: Arc<dyn HardwareAccess>,
}

impl SensorReader {
    pub fn new(hardware: Arc<dyn HardwareAccess>) -> Self {
        Self { hardware }
    }

    /// Read temperature in °C.
    pub fn read_temperature(&self, sensor_id: &str) -> Result<f64> {
        self.hardware.read_sensor(sensor_id)
    }

    /// Read humidity 0.0–100.0 (%).
    pub fn read_humidity(&self, sensor_id: &str) -> Result<f64> {
        self.hardware.read_sensor(sensor_id)
    }

    /// Read motion (true = motion detected; 0.0 = false, non-zero = true).
    pub fn read_motion(&self, sensor_id: &str) -> Result<bool> {
        let v = self.hardware.read_sensor(sensor_id)?;
        Ok(v != 0.0)
    }
}

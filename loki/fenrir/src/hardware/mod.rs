//! Hardware access abstraction (Phase 6.2 – HardwareAccess trait).

mod stub;

pub use stub::StubHardwareAccess;

use crate::error::Result;

/// Hardware access abstraction for GPIO, sensors, and actuators.
pub trait HardwareAccess: Send + Sync {
    /// Read GPIO pin value (true = high, false = low).
    fn read_gpio(&self, pin: u8) -> Result<bool>;

    /// Write GPIO pin value.
    fn write_gpio(&self, pin: u8, value: bool) -> Result<()>;

    /// Read sensor value by ID (e.g. temperature in °C, humidity 0–100).
    fn read_sensor(&self, sensor_id: &str) -> Result<f64>;

    /// Control actuator by ID (e.g. LED brightness 0.0–1.0, relay on/off).
    fn control_actuator(&self, actuator_id: &str, value: f64) -> Result<()>;
}

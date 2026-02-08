//! GPIO controller – delegates to HardwareAccess (Phase 6.3.1).

use std::sync::Arc;

use crate::error::Result;
use crate::hardware::HardwareAccess;

/// GPIO controller: read, write, optional PWM.
pub struct GPIOController {
    hardware: Arc<dyn HardwareAccess>,
}

impl GPIOController {
    pub fn new(hardware: Arc<dyn HardwareAccess>) -> Self {
        Self { hardware }
    }

    /// Read GPIO pin (high = true, low = false).
    pub fn read(&self, pin: u8) -> Result<bool> {
        self.hardware.read_gpio(pin)
    }

    /// Write GPIO pin value.
    pub fn write(&self, pin: u8, value: bool) -> Result<()> {
        self.hardware.write_gpio(pin, value)
    }

    /// Set PWM duty cycle 0.0–1.0 (optional; stub returns NotAvailable).
    pub fn set_pwm(&self, _pin: u8, _duty: f64) -> Result<()> {
        Err(crate::error::FenrirError::NotAvailable(
            "PWM not implemented".to_string(),
        ))
    }
}

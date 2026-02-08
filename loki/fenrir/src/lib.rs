//! Fenrir – Hardware-Control Service (Loki sub-service).
//! GPIO, Sensors, Actuators; script-accessible via Loki.

pub mod actuators;
pub mod error;
pub mod gpio;
pub mod hardware;
pub mod script;
pub mod sensors;

pub use actuators::ActuatorController;
pub use error::{FenrirError, Result};
pub use gpio::GPIOController;
pub use hardware::{HardwareAccess, StubHardwareAccess};
pub use script::FenrirScriptAPI;
pub use sensors::SensorReader;

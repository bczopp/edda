//! Fenrir script API – Lua bindings for GPIO, sensors, actuators (Phase 6.6.1).

use std::sync::Arc;

use mlua::{UserData, UserDataMethods};

use crate::actuators::ActuatorController;
use crate::error::Result;
use crate::gpio::GPIOController;
use crate::sensors::SensorReader;

/// Script API exposed to Lua: gpio_read, gpio_write, sensor_read, actuator_set.
pub struct FenrirScriptAPI {
    gpio: Arc<GPIOController>,
    sensors: Arc<SensorReader>,
    actuators: Arc<ActuatorController>,
}

impl FenrirScriptAPI {
    pub fn new(
        gpio: Arc<GPIOController>,
        sensors: Arc<SensorReader>,
        actuators: Arc<ActuatorController>,
    ) -> Self {
        Self { gpio, sensors, actuators }
    }

    /// Register this API as global "fenrir" in the given Lua state.
    pub fn register_into(&self, lua: &mlua::Lua) -> Result<()> {
        lua.globals()
            .set("fenrir", self.clone_for_lua())
            .map_err(|e| crate::error::FenrirError::NotAvailable(e.to_string()))?;
        Ok(())
    }

    fn clone_for_lua(&self) -> FenrirScriptAPI {
        FenrirScriptAPI {
            gpio: Arc::clone(&self.gpio),
            sensors: Arc::clone(&self.sensors),
            actuators: Arc::clone(&self.actuators),
        }
    }
}

impl Clone for FenrirScriptAPI {
    fn clone(&self) -> Self {
        Self {
            gpio: Arc::clone(&self.gpio),
            sensors: Arc::clone(&self.sensors),
            actuators: Arc::clone(&self.actuators),
        }
    }
}

impl UserData for FenrirScriptAPI {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("gpio_read", |_, this, pin: u8| {
            this.gpio
                .read(pin)
                .map_err(|e| mlua::Error::external(e))
        });
        methods.add_method("gpio_write", |_, this, (pin, value): (u8, bool)| {
            this.gpio
                .write(pin, value)
                .map_err(|e| mlua::Error::external(e))
        });
        methods.add_method("sensor_read", |_, this, sensor_id: String| {
            this.sensors
                .read_temperature(&sensor_id)
                .map_err(|e| mlua::Error::external(e))
        });
        methods.add_method("actuator_set", |_, this, (actuator_id, value): (String, f64)| {
            this.actuators
                .set_led(&actuator_id, value)
                .map_err(|e| mlua::Error::external(e))
        });
    }
}

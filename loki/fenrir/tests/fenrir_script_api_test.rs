//! Tests for FenrirScriptAPI – Lua bindings (TDD – Phase 6.6.1).

use fenrir::{
    ActuatorController, FenrirScriptAPI, GPIOController, HardwareAccess, SensorReader,
    StubHardwareAccess,
};
use mlua::Lua;
use std::sync::Arc;

fn api() -> FenrirScriptAPI {
    let hw = Arc::new(StubHardwareAccess::new());
    let gpio = Arc::new(GPIOController::new(Arc::clone(&hw)));
    let sensors = Arc::new(SensorReader::new(Arc::clone(&hw)));
    let actuators = Arc::new(ActuatorController::new(hw));
    FenrirScriptAPI::new(gpio, sensors, actuators)
}

#[test]
fn fenrir_script_api_register_and_gpio() {
    let api = api();
    let lua = Lua::new();
    api.register_into(&lua).unwrap();
    lua.load("fenrir:gpio_write(1, true)").exec().unwrap();
    let v: bool = lua.load("return fenrir:gpio_read(1)").eval().unwrap();
    assert!(v);
}

#[test]
fn fenrir_script_api_sensor_read() {
    let api = api();
    let lua = Lua::new();
    api.register_into(&lua).unwrap();
    let v: f64 = lua
        .load("return fenrir:sensor_read(\"temp1\")")
        .eval()
        .unwrap();
    assert_eq!(v, 0.0);
}

#[test]
fn fenrir_script_api_actuator_set() {
    let api = api();
    let lua = Lua::new();
    api.register_into(&lua).unwrap();
    lua.load("fenrir:actuator_set(\"led1\", 0.5)")
        .exec()
        .unwrap();
}

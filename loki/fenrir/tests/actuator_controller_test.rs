//! Tests for ActuatorController (TDD – Phase 6.5.1).

use fenrir::{actuators::ActuatorController, HardwareAccess, StubHardwareAccess};
use std::sync::Arc;

fn controller() -> ActuatorController {
    ActuatorController::new(Arc::new(StubHardwareAccess::new()))
}

#[test]
fn actuator_controller_set_led() {
    let ctrl = controller();
    assert!(ctrl.set_led("led1", 0.5).is_ok());
    assert!(ctrl.set_led("led1", 1.0).is_ok());
}

#[test]
fn actuator_controller_set_relay() {
    let ctrl = controller();
    assert!(ctrl.set_relay("relay1", true).is_ok());
    assert!(ctrl.set_relay("relay1", false).is_ok());
}

#[test]
fn actuator_controller_set_motor_optional() {
    let ctrl = controller();
    let _ = ctrl.set_motor("motor1", 0.8);
}

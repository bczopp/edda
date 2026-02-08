//! Tests for GPIOController (TDD – Phase 6.3.1).

use fenrir::{gpio::GPIOController, HardwareAccess, StubHardwareAccess};
use std::sync::Arc;

fn controller() -> GPIOController {
    GPIOController::new(Arc::new(StubHardwareAccess::new()))
}

#[test]
fn gpio_controller_read_default_low() {
    let ctrl = controller();
    assert_eq!(ctrl.read(0).unwrap(), false);
}

#[test]
fn gpio_controller_write_then_read() {
    let ctrl = controller();
    ctrl.write(3, true).unwrap();
    assert_eq!(ctrl.read(3).unwrap(), true);
    ctrl.write(3, false).unwrap();
    assert_eq!(ctrl.read(3).unwrap(), false);
}

#[test]
fn gpio_controller_multiple_pins() {
    let ctrl = controller();
    ctrl.write(1, true).unwrap();
    ctrl.write(2, false).unwrap();
    assert_eq!(ctrl.read(1).unwrap(), true);
    assert_eq!(ctrl.read(2).unwrap(), false);
}

#[test]
fn gpio_controller_set_pwm_placeholder() {
    let ctrl = controller();
    // PWM optional: currently returns NotAvailable
    let res = ctrl.set_pwm(4, 0.5);
    assert!(res.is_err());
}

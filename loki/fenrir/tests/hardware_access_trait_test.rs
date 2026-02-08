//! Tests for HardwareAccess trait (TDD – Phase 6.2.1).

use fenrir::{HardwareAccess, StubHardwareAccess};

#[test]
fn stub_read_gpio_default_low() {
    let hw = StubHardwareAccess::new();
    assert_eq!(hw.read_gpio(0).unwrap(), false);
    assert_eq!(hw.read_gpio(1).unwrap(), false);
}

#[test]
fn stub_write_then_read_gpio() {
    let hw = StubHardwareAccess::new();
    hw.write_gpio(2, true).unwrap();
    assert_eq!(hw.read_gpio(2).unwrap(), true);
    hw.write_gpio(2, false).unwrap();
    assert_eq!(hw.read_gpio(2).unwrap(), false);
}

#[test]
fn stub_read_sensor_returns_value() {
    let hw = StubHardwareAccess::new();
    let v = hw.read_sensor("temp").unwrap();
    assert_eq!(v, 0.0);
}

#[test]
fn stub_control_actuator_ok() {
    let hw = StubHardwareAccess::new();
    assert!(hw.control_actuator("led", 0.5).is_ok());
}

#[test]
fn trait_object_send_sync() {
    let hw: Box<dyn HardwareAccess> = Box::new(StubHardwareAccess::new());
    let _ = hw.read_gpio(0);
}

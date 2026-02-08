//! Tests for SensorReader (TDD – Phase 6.4.1).

use fenrir::{sensors::SensorReader, HardwareAccess, StubHardwareAccess};
use std::sync::Arc;

fn reader() -> SensorReader {
    SensorReader::new(Arc::new(StubHardwareAccess::new()))
}

#[test]
fn sensor_reader_read_temperature() {
    let r = reader();
    let v = r.read_temperature("temp1").unwrap();
    assert_eq!(v, 0.0);
}

#[test]
fn sensor_reader_read_humidity() {
    let r = reader();
    let v = r.read_humidity("hum1").unwrap();
    assert_eq!(v, 0.0);
}

#[test]
fn sensor_reader_read_motion_default_false() {
    let r = reader();
    let v = r.read_motion("pir1").unwrap();
    assert!(!v);
}

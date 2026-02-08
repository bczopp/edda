// MemoryMonitor tests (Phase 6.1.1, TDD).

use jotunheim_esp32::resources::MemoryMonitor;

#[test]
fn new_sets_thresholds_and_zero_usage() {
    let m = MemoryMonitor::new(100, 200);
    assert_eq!(m.current_usage_kb(), 0);
    assert!(!m.is_warning());
    assert!(!m.is_exhausted());
}

#[test]
fn set_usage_below_warning_no_alert() {
    let m = MemoryMonitor::new(100, 200);
    m.set_usage_kb(50);
    assert!(!m.is_warning());
    assert!(!m.is_exhausted());
}

#[test]
fn set_usage_at_warning_threshold_triggers_warning() {
    let m = MemoryMonitor::new(100, 200);
    m.set_usage_kb(100);
    assert!(m.is_warning());
    assert!(!m.is_exhausted());
}

#[test]
fn set_usage_above_exhaustion_triggers_exhausted() {
    let m = MemoryMonitor::new(100, 200);
    m.set_usage_kb(200);
    assert!(m.is_warning());
    assert!(m.is_exhausted());
}

#[test]
fn set_usage_above_exhaustion_threshold() {
    let m = MemoryMonitor::new(100, 200);
    m.set_usage_kb(250);
    assert!(m.is_exhausted());
}

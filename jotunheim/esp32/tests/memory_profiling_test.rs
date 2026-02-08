// Memory-profiling and optimization tests (Phase 10.1.1).
// Validates bounded allocations and pool usage; < 10KB RAM target is for ESP32 on device.

use jotunheim_esp32::capability::DeviceCapabilityBuilder;
use jotunheim_esp32::resources::{MemoryMonitor, MemoryPool};

#[test]
fn memory_pool_does_not_exceed_max_buffers() {
    let pool = MemoryPool::new(4, 256);
    let mut bufs = Vec::new();
    for _ in 0..4 {
        bufs.push(pool.acquire().expect("acquire"));
    }
    assert!(pool.acquire().is_none());
    for b in bufs {
        pool.release(b);
    }
    assert_eq!(pool.acquire().is_some(), true);
}

#[test]
fn memory_monitor_warning_and_exhaustion_at_thresholds() {
    let mon = MemoryMonitor::new(10, 20);
    mon.set_usage_kb(9);
    assert!(!mon.is_warning() && !mon.is_exhausted());
    mon.set_usage_kb(10);
    assert!(mon.is_warning());
    mon.set_usage_kb(20);
    assert!(mon.is_exhausted());
}

#[test]
fn capability_builder_produces_bounded_output() {
    let mut b = DeviceCapabilityBuilder::new()
        .device_id("id")
        .device_name("n")
        .device_type("ESP32")
        .firmware_version("0")
        .protocol_version("1");
    for i in 0..8 {
        b = b.add_tool(&format!("tool_{}", i), "desc", "void");
    }
    let cap = b.build();
    assert_eq!(cap.tools.len(), 8);
    assert!(cap.device_id.len() < 256);
}

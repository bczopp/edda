// Resource-constraint tests (Phase 13.3.1).

use jotunheim_esp32::resources::{BandwidthMonitor, MemoryMonitor, ResourceMonitor, TaskScheduler};

#[test]
fn memory_exhaustion_detected() {
    let mon = MemoryMonitor::new(50, 100);
    mon.set_usage_kb(40);
    assert!(!mon.is_exhausted());
    assert!(!mon.is_warning());
    mon.set_usage_kb(50);
    assert!(mon.is_warning());
    mon.set_usage_kb(100);
    assert!(mon.is_exhausted());
}

#[test]
fn bandwidth_throttle_when_over_limit() {
    let mon = BandwidthMonitor::new(1000);
    assert!(!mon.should_throttle());
    mon.add_sent(600);
    mon.add_received(500);
    assert!(mon.should_throttle());
    mon.tick();
    assert!(!mon.should_throttle());
    assert_eq!(mon.sent_bytes(), 0);
    assert_eq!(mon.received_bytes(), 0);
}

#[test]
fn task_scheduler_respects_max_queued() {
    let sched = TaskScheduler::new(2);
    sched.submit(1, Box::new(|| {}));
    sched.submit(2, Box::new(|| {}));
    sched.submit(3, Box::new(|| {})); // over limit, may be dropped
    let ran = sched.run_next();
    assert!(ran);
    let ran2 = sched.run_next();
    assert!(ran2);
    let ran3 = sched.run_next();
    assert!(!ran3); // queue empty or only 2 were queued
}

#[test]
fn resource_monitor_aggregates_limits() {
    let mon = ResourceMonitor::new(80, 120, 5000);
    mon.set_ram_usage_kb(100);
    mon.set_cpu_usage_percent(90);
    mon.add_network_sent(3000);
    mon.add_network_received(2000);
    assert_eq!(mon.ram_usage_kb(), 100);
    assert_eq!(mon.cpu_usage_percent(), 90);
    assert_eq!(mon.network_sent_bytes(), 3000);
    assert_eq!(mon.network_received_bytes(), 2000);
}

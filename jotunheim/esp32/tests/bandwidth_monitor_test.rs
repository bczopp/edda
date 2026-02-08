// BandwidthMonitor tests (Phase 6.3.1, TDD).

use jotunheim_esp32::resources::BandwidthMonitor;

#[test]
fn add_traffic_increments_usage() {
    let mon = BandwidthMonitor::new(1000);
    mon.add_sent(100);
    mon.add_received(200);
    assert_eq!(mon.sent_bytes(), 100);
    assert_eq!(mon.received_bytes(), 200);
}

#[test]
fn should_throttle_when_over_limit() {
    let mon = BandwidthMonitor::new(100);
    mon.add_sent(50);
    mon.add_received(60);
    assert!(mon.should_throttle());
}

#[test]
fn should_not_throttle_when_under_limit() {
    let mon = BandwidthMonitor::new(1000);
    mon.add_sent(100);
    mon.add_received(200);
    assert!(!mon.should_throttle());
}

#[test]
fn tick_resets_usage() {
    let mon = BandwidthMonitor::new(100);
    mon.add_sent(80);
    mon.add_received(80);
    mon.tick();
    assert_eq!(mon.sent_bytes(), 0);
    assert_eq!(mon.received_bytes(), 0);
}

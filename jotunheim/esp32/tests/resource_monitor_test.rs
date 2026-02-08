// ResourceMonitor tests (Phase 11.2.1, TDD).

use jotunheim_esp32::resources::ResourceMonitor;

#[test]
fn ram_usage_tracked() {
    let mon = ResourceMonitor::new(100, 200, 10_000);
    assert_eq!(mon.ram_usage_kb(), 0);
    mon.set_ram_usage_kb(50);
    assert_eq!(mon.ram_usage_kb(), 50);
}

#[test]
fn cpu_usage_tracked() {
    let mon = ResourceMonitor::new(100, 200, 10_000);
    assert_eq!(mon.cpu_usage_percent(), 0);
    mon.set_cpu_usage_percent(75);
    assert_eq!(mon.cpu_usage_percent(), 75);
}

#[test]
fn network_usage_tracked() {
    let mon = ResourceMonitor::new(100, 200, 10_000);
    assert_eq!(mon.network_sent_bytes(), 0);
    assert_eq!(mon.network_received_bytes(), 0);
    mon.add_network_sent(100);
    mon.add_network_received(200);
    assert_eq!(mon.network_sent_bytes(), 100);
    assert_eq!(mon.network_received_bytes(), 200);
}

#[test]
fn network_tick_resets_window() {
    let mon = ResourceMonitor::new(100, 200, 10_000);
    mon.add_network_sent(50);
    mon.add_network_received(50);
    mon.network_tick();
    assert_eq!(mon.network_sent_bytes(), 0);
    assert_eq!(mon.network_received_bytes(), 0);
}

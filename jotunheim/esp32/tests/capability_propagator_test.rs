// CapabilityPropagator tests (Phase 4.3.1, TDD).

use jotunheim_esp32::capability::{CapabilityPropagator, DeviceCapabilityBuilder};
use std::sync::Arc;
use tokio::sync::Mutex;

#[tokio::test]
async fn on_connect_emits_connected_event() {
    let caps = DeviceCapabilityBuilder::new()
        .device_id("dev-1")
        .device_name("Test")
        .device_type("ESP32")
        .firmware_version("0.1")
        .protocol_version("1.0")
        .build();
    let received = Arc::new(Mutex::new(None::<String>));
    let received_clone = received.clone();
    let propagator = CapabilityPropagator::new(caps).with_event_callback(move |ev| {
        let r = received_clone.clone();
        tokio::spawn(async move {
            *r.lock().await = Some(ev.event_type);
        });
    });
    propagator.on_connect().await;
    tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
    let r = received.lock().await;
    assert_eq!(r.as_deref(), Some("connected"));
}

#[tokio::test]
async fn emit_event_sends_event_type_and_capabilities() {
    let caps = DeviceCapabilityBuilder::new()
        .device_id("d")
        .device_name("n")
        .device_type("ESP32")
        .firmware_version("0")
        .protocol_version("1")
        .build();
    let event_type = Arc::new(Mutex::new(None::<String>));
    let event_type_clone = event_type.clone();
    let propagator = CapabilityPropagator::new(caps.clone()).with_event_callback(move |ev| {
        let et = event_type_clone.clone();
        tokio::spawn(async move {
            *et.lock().await = Some(ev.event_type);
        });
    });
    propagator.emit_event("updated");
    tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
    let et = event_type.lock().await;
    assert_eq!(et.as_deref(), Some("updated"));
}

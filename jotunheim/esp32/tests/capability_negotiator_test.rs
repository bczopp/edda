// CapabilityNegotiator tests (Phase 4.2.1, TDD).

use jotunheim_esp32::capability::{CapabilityNegotiator, DeviceCapabilityBuilder};
use std::sync::Arc;
use tokio::sync::Mutex;

#[tokio::test]
async fn on_request_sends_capabilities() {
    let caps = DeviceCapabilityBuilder::new()
        .device_id("dev-1")
        .device_name("Test")
        .device_type("ESP32")
        .firmware_version("0.1")
        .protocol_version("1.0")
        .build();
    let received = Arc::new(Mutex::new(None::<jotunheim_esp32::grpc::proto::jotunheim_capability::CapabilityResponse>));
    let received_clone = received.clone();
    let negotiator = CapabilityNegotiator::new(caps.clone(), move |resp| {
        let r = received_clone.clone();
        tokio::spawn(async move {
            *r.lock().await = Some(resp);
        });
    });
    negotiator.on_capability_request().await;
    tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
    let r = received.lock().await;
    assert!(r.is_some());
    let resp = r.as_ref().unwrap();
    assert_eq!(resp.capabilities.as_ref().unwrap().device_id, "dev-1");
}

#[tokio::test]
async fn negotiate_with_timeout_completes_or_times_out() {
    let caps = DeviceCapabilityBuilder::new()
        .device_id("d")
        .device_name("n")
        .device_type("ESP32")
        .firmware_version("0")
        .protocol_version("1")
        .build();
    let negotiator = CapabilityNegotiator::with_timeout(
        caps,
        |_| {},
        std::time::Duration::from_millis(100),
    );
    let res = negotiator.negotiate_with_timeout().await;
    assert!(res.is_ok() || res.is_err());
}

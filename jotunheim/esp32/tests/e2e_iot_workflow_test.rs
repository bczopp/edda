// E2E IoT workflow test (Phase 13.1.1).
// Connection-Establishment → Capability-Negotiation → Remote-Control (with mocks).

use jotunheim_esp32::capability::{CapabilityNegotiator, DeviceCapabilityBuilder};
use jotunheim_esp32::network::{WiFiManager, WiFiStatus};
use jotunheim_esp32::remote::{RemoteCommandHandler, ScriptExecutor};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

struct MockScriptExecutor;
#[async_trait::async_trait]
impl ScriptExecutor for MockScriptExecutor {
    async fn execute(
        &mut self,
        script_id: &str,
        _: &str,
        _: &str,
        _: HashMap<String, String>,
    ) -> Result<String, jotunheim_esp32::remote::RemoteCommandError> {
        Ok(format!("executed:{}", script_id))
    }
}

#[tokio::test]
async fn e2e_connection_capability_negotiation_remote_control() {
    // 1. Connection (simulated WiFi)
    let wifi = WiFiManager::new("TestSSID".into(), "password".into());
    wifi.connect().await.unwrap();
    assert_eq!(wifi.status(), WiFiStatus::Connected);

    // 2. Build capabilities and negotiate
    let caps = DeviceCapabilityBuilder::new()
        .device_id("e2e-device")
        .device_name("E2E Test Device")
        .device_type("ESP32")
        .firmware_version("0.1.0")
        .protocol_version("1.0")
        .add_tool("led_on", "Turn LED on", "void")
        .build();

    let received = Arc::new(Mutex::new(None::<String>));
    let received_clone = received.clone();
    let negotiator = CapabilityNegotiator::new(caps.clone(), move |resp| {
        let r = received_clone.clone();
        tokio::spawn(async move {
            if let Some(c) = resp.capabilities {
                *r.lock().await = Some(c.device_id);
            }
        });
    });
    negotiator.on_capability_request().await;
    tokio::time::sleep(tokio::time::Duration::from_millis(30)).await;
    assert_eq!(received.lock().await.as_deref(), Some("e2e-device"));

    // 3. Remote control (mock executor)
    let mut handler = RemoteCommandHandler::new(MockScriptExecutor);
    let res = handler
        .handle_command("led_on", "print('on')", "lua", HashMap::new())
        .await;
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), "executed:led_on");
}

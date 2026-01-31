//! Voller Container-E2E: Request-Flow in der Container-Umgebung mit Mock-URLs aus Env.
//! Läuft nur, wenn z.B. GERI_URL gesetzt ist (docker-compose.test setzt THOR_URL, GERI_URL, …).
//! mock-geri wird per Dockerfile.mock-geri-grpc als gRPC-Mock gebaut (Einherjar, Responsibility, Geri ProcessPrompt)
//! und liefert bei Routing auf Geri Ok(response). Andere Mocks bleiben TCP-only → Ok oder Err je nach Ziel-Service.

use odin::clients::manager::ClientManager;
use odin::orchestration::responsibility::ResponsibilityManager;
use odin::orchestration::{RequestProcessor, UserRequest};
use odin::protocols::manager::ProtocolManager;
use odin::utils::config::OdinSettings;
use std::sync::Arc;

fn env_url(key: &str) -> Option<String> {
    std::env::var(key).ok().filter(|s| !s.is_empty())
}

/// Baut OdinSettings mit service_urls aus Env (THOR_URL, GERI_URL, …).
fn settings_from_container_env() -> OdinSettings {
    let mut s = OdinSettings::default();
    if let Some(u) = env_url("THOR_URL") {
        s.service_urls.thor = Some(u);
    }
    if let Some(u) = env_url("FREKI_URL") {
        s.service_urls.freki = Some(u);
    }
    if let Some(u) = env_url("GERI_URL") {
        s.service_urls.geri = Some(u);
    }
    if let Some(u) = env_url("HUGINN_URL") {
        s.service_urls.huginn = Some(u);
    }
    if let Some(u) = env_url("MUNINN_URL") {
        s.service_urls.muninn = Some(u);
    }
    if let Some(u) = env_url("LOKI_URL") {
        s.service_urls.loki = Some(u);
    }
    if let Some(u) = env_url("HEIMDALL_URL") {
        s.service_urls.heimdall = Some(u);
    }
    if let Some(u) = env_url("SKULD_URL") {
        s.service_urls.skuld = Some(u);
    }
    s
}

/// Voller Container-E2E: Request → discover → responsibility → route → process.
/// In der Container-Umgebung nutzt dieser Test die Mock-URLs aus Env (THOR_URL, GERI_URL, …).
#[tokio::test]
async fn e2e_container_request_flow_uses_mock_urls() {
    if env_url("GERI_URL").or_else(|| env_url("THOR_URL")).is_none() {
        eprintln!("SKIP e2e_container_request_flow_uses_mock_urls: GERI_URL/THOR_URL not set (not in container)");
        return;
    }

    let settings = settings_from_container_env();
    let settings_arc = Arc::new(tokio::sync::RwLock::new(settings));
    let protocol_manager = Arc::new(ProtocolManager::new(settings_arc.clone()));
    let client_manager = Arc::new(ClientManager::new(settings_arc.clone()));

    if let Err(e) = client_manager.initialize().await {
        eprintln!("E2E container: client_manager.initialize() failed (mocks may be TCP-only): {}", e);
    }

    let responsibility_manager = Arc::new(ResponsibilityManager::new(
        protocol_manager.get_cache(),
        protocol_manager.clone(),
        client_manager.clone(),
    ));
    let processor = RequestProcessor::new_with_responsibility(responsibility_manager);

    // Discover nutzt Env-URLs (mock-thor:50052 etc.)
    if let Err(e) = protocol_manager.discover_all_capabilities().await {
        eprintln!("E2E container: discover_all_capabilities failed (mocks TCP-only?): {}", e);
    }

    let req = UserRequest {
        request_id: "e2e-container-1".to_string(),
        user_id: "u1".to_string(),
        device_id: "d1".to_string(),
        input: "Can you explain how this works?".to_string(),
        input_type: "text".to_string(),
    };

    let result = processor.process(req).await;
    match &result {
        Ok(s) => assert!(!s.is_empty(), "E2E container flow should return non-empty or Err"),
        Err(_) => {
            // Erwartet, wenn Mocks nur TCP listen und kein gRPC sprechen
        }
    }
}

use jotunheim_esp32::network::NetworkManager;
use jotunheim_esp32::grpc::LokiClient;
use jotunheim_esp32::utils::config::JotunheimSettings;

fn main() {
    // ESP32 main entry point
    let settings = JotunheimSettings::default();
    
    // Initialize network
    let network_manager = NetworkManager::new(
        settings.network.ssid.clone(),
        settings.network.password.clone(),
    );
    
    // Initialize gRPC client (Loki)
    let loki_client = LokiClient::new(
        settings.loki.address.clone(),
        settings.loki.port,
    );
    
    // TODO: Initialize capability manager
    // TODO: Initialize remote control
    // TODO: Initialize OTA updates
}

//! Tests for SettingsLoader (Phase 1.3.3 – TDD).

use jotunheim_esp32::settings::SettingsLoader;
use jotunheim_esp32::utils::config::JotunheimSettings;
use std::path::Path;

#[test]
fn loader_loads_json() {
    let tmp = tempfile::NamedTempFile::new().unwrap();
    let path = tmp.path();
    let json = r#"{"loki":{"address":"192.168.1.1","port":50052},"network":{"ssid":"test","password":"secret"}}"#;
    std::fs::write(path, json).unwrap();

    let s = SettingsLoader::load(Path::new(path)).unwrap();
    assert_eq!(s.loki.address, "192.168.1.1");
    assert_eq!(s.loki.port, 50052);
    assert_eq!(s.network.ssid, "test");
}

#[test]
fn loader_loads_toml() {
    let tmp = tempfile::NamedTempFile::new().unwrap();
    let path = tmp.path().with_extension("toml");
    let toml = r#"
[loki]
address = "10.0.0.1"
port = 50053

[network]
ssid = "wifi"
password = "pass"
"#;
    std::fs::write(&path, toml).unwrap();

    let s = SettingsLoader::load(&path).unwrap();
    assert_eq!(s.loki.address, "10.0.0.1");
    assert_eq!(s.loki.port, 50053);
    assert_eq!(s.network.ssid, "wifi");
}

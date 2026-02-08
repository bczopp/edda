//! Config tests: RagnarokSettings default and optional thor.

use ragnarok::utils::config::RagnarokSettings;

#[test]
fn test_settings_default_has_no_thor() {
    let s = RagnarokSettings::default();
    assert!(s.thor.is_none());
    assert_eq!(s.odin.port, 50051);
}

#[test]
fn test_settings_deserialize_with_thor() {
    let json = r#"{"odin":{"address":"127.0.0.1","port":50051},"thor":{"address":"192.168.1.1","port":50052}}"#;
    let s: RagnarokSettings = serde_json::from_str(json).unwrap();
    assert!(s.thor.is_some());
    let thor = s.thor.unwrap();
    assert_eq!(thor.address, "192.168.1.1");
    assert_eq!(thor.port, 50052);
}

#[test]
fn test_settings_deserialize_without_thor() {
    let json = r#"{"odin":{"address":"127.0.0.1","port":50051}}"#;
    let s: RagnarokSettings = serde_json::from_str(json).unwrap();
    assert!(s.thor.is_none());
}

#[test]
fn test_settings_default_has_no_geri_freki() {
    let s = RagnarokSettings::default();
    assert!(s.geri.is_none());
    assert!(s.freki.is_none());
}

#[test]
fn test_settings_deserialize_with_geri_freki() {
    let json = r#"{"odin":{"address":"127.0.0.1","port":50051},"geri":{"address":"127.0.0.1","port":50053},"freki":{"address":"127.0.0.1","port":50054}}"#;
    let s: RagnarokSettings = serde_json::from_str(json).unwrap();
    assert!(s.geri.is_some());
    assert!(s.freki.is_some());
    assert_eq!(s.geri.as_ref().unwrap().port, 50053);
    assert_eq!(s.freki.as_ref().unwrap().port, 50054);
}

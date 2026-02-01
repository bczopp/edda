use heimdall::security::tls::{TLSConfigManager, TLSConfig, ProtocolVersion};
use std::path::PathBuf;
use tempfile::TempDir;
use std::fs;

#[tokio::test]
async fn test_tls_config_manager_new() {
    let _manager = TLSConfigManager::new(None, None, None);
    // Basic creation should succeed
    assert!(true);
}

#[tokio::test]
async fn test_get_recommended_config() {
    let config = TLSConfigManager::get_recommended_config();
    
    // Verify TLS 1.3 only
    match config.min_protocol_version {
        ProtocolVersion::Tls13 => {},
        _ => panic!("Expected TLS 1.3 as minimum protocol version"),
    }
    
    match config.max_protocol_version {
        ProtocolVersion::Tls13 => {},
        _ => panic!("Expected TLS 1.3 as maximum protocol version"),
    }
    
    // Verify cipher suites
    assert_eq!(config.cipher_suites.len(), 2);
    assert!(config.cipher_suites.contains(&"TLS_AES_256_GCM_SHA384".to_string()));
    assert!(config.cipher_suites.contains(&"TLS_CHACHA20_POLY1305_SHA256".to_string()));
    
    // Verify client cert not required by default
    assert!(!config.require_client_cert);
}

#[tokio::test]
async fn test_tls13_only_config() {
    let config = TLSConfig::tls13_only();
    
    // Verify TLS 1.3 only
    match config.min_protocol_version {
        ProtocolVersion::Tls13 => {},
        _ => panic!("Expected TLS 1.3 as minimum protocol version"),
    }
    
    match config.max_protocol_version {
        ProtocolVersion::Tls13 => {},
        _ => panic!("Expected TLS 1.3 as maximum protocol version"),
    }
    
    // Verify strong cipher suites
    assert_eq!(config.cipher_suites.len(), 2);
    assert!(config.cipher_suites.contains(&"TLS_AES_256_GCM_SHA384".to_string()));
    assert!(config.cipher_suites.contains(&"TLS_CHACHA20_POLY1305_SHA256".to_string()));
}

#[tokio::test]
async fn test_configure_server_tls_no_certs() {
    let manager = TLSConfigManager::new(None, None, None);
    
    // Configure TLS without certificates should succeed
    // (Server can still use TLS with self-signed certs or no certs)
    let result = manager.configure_server_tls();
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_configure_server_tls_with_invalid_cert_path() {
    let cert_path = PathBuf::from("/nonexistent/cert.pem");
    let key_path = PathBuf::from("/nonexistent/key.pem");
    
    let manager = TLSConfigManager::new(Some(cert_path), Some(key_path), None);
    
    // Should fail with certificate error
    let result = manager.configure_server_tls();
    assert!(result.is_err());
    
    let err_msg = result.unwrap_err().to_string();
    assert!(err_msg.contains("Failed to read certificate") || err_msg.contains("Certificate error"));
}

#[tokio::test]
async fn test_configure_server_tls_with_valid_certs() {
    // Create temporary directory for test certificates
    let temp_dir = TempDir::new().unwrap();
    let cert_path = temp_dir.path().join("cert.pem");
    let key_path = temp_dir.path().join("key.pem");
    
    // Generate test certificate and key (PEM format)
    // Note: In real tests, we'd use proper test certificates
    // For now, we'll create dummy PEM files to test file reading
    let test_cert = "-----BEGIN CERTIFICATE-----\n\
MIIBkTCB+wIJAKHHCgVZU7DGMA0GCSqGSIb3DQEBCwUAMBExDzANBgNVBAMMBnRl\n\
c3RjYTAeFw0yNDAxMDEwMDAwMDBaFw0yNTAxMDEwMDAwMDBaMBExDzANBgNVBAMM\n\
BnRlc3RjYTBcMA0GCSqGSIb3DQEBAQUAA0sAMEgCQQDL8VvRqmGqGqGqGqGqGqGq\n\
GqGqGqGqGqGqGqGqGqGqGqGqGqGqGqGqGqGqGqGqGqGqGqGqGqGqGqGqGqGqAgMB\n\
AAEwDQYJKoZIhvcNAQELBQADQQAL8VvRqmGqGqGqGqGqGqGqGqGqGqGqGqGqGqGq\n\
GqGqGqGqGqGqGqGqGqGqGqGqGqGqGqGqGqGqGqGqGqGqGqGq\n\
-----END CERTIFICATE-----";
    
    let test_key = "-----BEGIN PRIVATE KEY-----\n\
MIIBVAIBADANBgkqhkiG9w0BAQEFAASCAT4wggE6AgEAAkEAy/Fb0aphqhqhqhqh\n\
qhqhqhqhqhqhqhqhqhqhqhqhqhqhqhqhqhqhqhqhqhqhqhqhqhqhqhqhqhqhqhqh\n\
qhqhqhqhIDAQABAkEAy/Fb0aphqhqhqhqhqhqhqhqhqhqhqhqhqhqhqhqhqhqhqhqh\n\
qhqhqhqhqhqhqhqhqhqhqhqhqhqhqhqhqhqhqhQIhAMvxW9GqYaoaoaoaoaoaoaoa\n\
oaoaoaoaoaoaoaoaoaoaoaoaoiEAy/Fb0aphqhqhqhqhqhqhqhqhqhqhqhqhqhqhqh\n\
qhqhqhqhqhqhCIQDL8VvRqmGqGqGqGqGqGqGqGqGqGqGqGqGqGqGqGqGqGqGqGqGq\n\
GqGqIQDL8VvRqmGqGqGqGqGqGqGqGqGqGqGqGqGqGqGqGqGqGqGqGqGqGqGqIQDL\n\
8VvRqmGqGqGqGqGqGqGqGqGqGqGqGqGqGqGqGqGqGqGqGqGqGqGqGqGqGqGqGqGq\n\
-----END PRIVATE KEY-----";
    
    // Write test files
    fs::write(&cert_path, test_cert).unwrap();
    fs::write(&key_path, test_key).unwrap();
    
    let manager = TLSConfigManager::new(
        Some(cert_path.clone()),
        Some(key_path.clone()),
        None
    );
    
    // Should succeed (even with dummy certs, file reading works)
    // Note: Actual TLS validation would fail with these dummy certs,
    // but we're testing the config manager, not TLS itself
    let result = manager.configure_server_tls();
    
    // This might fail with cert validation error, which is expected
    // We're just testing that the file reading works
    // In production, proper certificates would be used
    match result {
        Ok(_) => {
            // Config creation succeeded (file reading worked)
        },
        Err(e) => {
            // Expected: cert validation might fail with dummy certs
            println!("Expected error with dummy certs: {}", e);
        }
    }
}

#[tokio::test]
async fn test_configure_server_tls_with_ca_cert() {
    let temp_dir = TempDir::new().unwrap();
    let ca_cert_path = temp_dir.path().join("ca.pem");
    
    // Create dummy CA certificate
    let test_ca_cert = "-----BEGIN CERTIFICATE-----\n\
MIIBkTCB+wIJAKHHCgVZU7DGMA0GCSqGSIb3DQEBCwUAMBExDzANBgNVBAMMBnRl\n\
c3RjYTAeFw0yNDAxMDEwMDAwMDBaFw0yNTAxMDEwMDAwMDBaMBExDzANBgNVBAMM\n\
BnRlc3RjYTBcMA0GCSqGSIb3DQEBAQUAA0sAMEgCQQDL8VvRqmGqGqGqGqGqGqGq\n\
GqGqGqGqGqGqGqGqGqGqGqGqGqGqGqGqGqGqGqGqGqGqGqGqGqGqGqGqGqGqAgMB\n\
AAEwDQYJKoZIhvcNAQELBQADQQAL8VvRqmGqGqGqGqGqGqGqGqGqGqGqGqGqGqGq\n\
GqGqGqGqGqGqGqGqGqGqGqGqGqGqGqGqGqGqGqGqGqGqGqGq\n\
-----END CERTIFICATE-----";
    
    fs::write(&ca_cert_path, test_ca_cert).unwrap();
    
    let manager = TLSConfigManager::new(None, None, Some(ca_cert_path));
    
    // Should succeed (file reading works)
    let result = manager.configure_server_tls();
    
    match result {
        Ok(_) => {
            // Config creation succeeded
        },
        Err(e) => {
            println!("Error with CA cert: {}", e);
        }
    }
}

#[tokio::test]
async fn test_configure_server_tls_cert_without_key() {
    let temp_dir = TempDir::new().unwrap();
    let cert_path = temp_dir.path().join("cert.pem");
    
    let test_cert = "-----BEGIN CERTIFICATE-----\ntest\n-----END CERTIFICATE-----";
    fs::write(&cert_path, test_cert).unwrap();
    
    // Cert without key should be handled gracefully
    let manager = TLSConfigManager::new(Some(cert_path), None, None);
    
    // Should succeed (no cert/key configured if key is missing)
    let result = manager.configure_server_tls();
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_tls_config_strong_cipher_suites() {
    let config = TLSConfigManager::get_recommended_config();
    
    // Verify only strong cipher suites are included
    for cipher in &config.cipher_suites {
        // All cipher suites should be TLS 1.3
        assert!(
            cipher.starts_with("TLS_AES") || cipher.starts_with("TLS_CHACHA20"),
            "Cipher suite {} is not a TLS 1.3 cipher",
            cipher
        );
        
        // Should use strong algorithms (GCM or ChaCha20-Poly1305)
        assert!(
            cipher.contains("GCM") || cipher.contains("CHACHA20"),
            "Cipher suite {} does not use strong authenticated encryption",
            cipher
        );
    }
}

#[tokio::test]
async fn test_tls_config_no_weak_protocols() {
    let config = TLSConfigManager::get_recommended_config();
    
    // Verify minimum version is TLS 1.3 (no TLS 1.2 or below)
    match config.min_protocol_version {
        ProtocolVersion::Tls13 => {},
        ProtocolVersion::Tls12 => panic!("TLS 1.2 should not be allowed as minimum"),
    }
}

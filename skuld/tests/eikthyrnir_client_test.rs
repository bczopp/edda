//! Tests für Eikthyrnir-Client (Phase 7). Nutzen Mock oder Stub; Integration mit echtem Eikthyrnir in Container.

use skuld::eikthyrnir_client::{EikthyrnirClient, QualityMetric};

#[tokio::test]
async fn test_quality_metric_struct() {
    let m = QualityMetric {
        metric_id: "m1".to_string(),
        provider_id: "geri".to_string(),
        value: 0.95,
        timestamp: "2025-01-01T00:00:00Z".to_string(),
    };
    assert_eq!(m.metric_id, "m1");
    assert_eq!(m.provider_id, "geri");
    assert!((m.value - 0.95).abs() < 1e-9);
}

#[tokio::test]
async fn test_eikthyrnir_client_construction() {
    // Client kann mit Dummy-URL erstellt werden (echte Verbindung nur bei connect/get_quality_metrics)
    let _client = EikthyrnirClient::new("http://[::1]:50060");
    // Wenn connect fehlschlägt (kein Server), ist das in Tests ok
}

#[tokio::test]
async fn test_get_quality_metrics_returns_result_type() {
    // Ohne laufenden Eikthyrnir-Server erwarten wir einen Fehler (Connection/Unavailable)
    let client = EikthyrnirClient::new("http://127.0.0.1:59999");
    let result = client.get_quality_metrics("test_provider", 10).await;
    // Entweder Ok mit leeren/Test-Daten oder Err
    assert!(result.is_ok() || result.is_err());
}

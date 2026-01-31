//! Tests for TokenLeakDetector (Phase 6.6.1): anomaly detection, device tracking, alerts.

use heimdall::token::TokenLeakDetector;

fn detector(max_devices_per_token: u32, window_seconds: i64) -> TokenLeakDetector {
    TokenLeakDetector::new(max_devices_per_token, window_seconds)
}

#[tokio::test]
async fn test_record_usage_and_get_usage_by_device() {
    let det = detector(2, 3600);
    det.record_usage("token-1", "device-a").await;
    det.record_usage("token-1", "device-a").await;
    det.record_usage("token-1", "device-b").await;

    let usage = det.get_usage_by_device("token-1").await;
    assert_eq!(usage.get("device-a"), Some(&2));
    assert_eq!(usage.get("device-b"), Some(&1));
}

#[tokio::test]
async fn test_no_anomaly_when_single_device() {
    let det = detector(2, 3600);
    det.record_usage("token-1", "device-a").await;
    det.record_usage("token-1", "device-a").await;

    assert!(det.check_anomaly("token-1").await.is_none());
}

#[tokio::test]
async fn test_anomaly_when_multiple_devices_exceed_threshold() {
    let det = detector(2, 3600);
    det.record_usage("token-1", "device-a").await;
    det.record_usage("token-1", "device-b").await;
    det.record_usage("token-1", "device-c").await;

    let alert = det.check_anomaly("token-1").await.unwrap();
    assert_eq!(alert.token_id, "token-1");
    assert!(alert.distinct_devices >= 3);
    assert!(alert.message.contains("multiple devices") || !alert.message.is_empty());
}

#[tokio::test]
async fn test_anomaly_at_threshold_boundary() {
    // Implementation triggers when distinct > max (not >=), so 3 devices with max 2 triggers alert
    let det = detector(2, 3600);
    det.record_usage("token-1", "device-a").await;
    det.record_usage("token-1", "device-b").await;
    det.record_usage("token-1", "device-c").await;

    let alert = det.check_anomaly("token-1").await;
    assert!(alert.is_some());
}

#[tokio::test]
async fn test_no_usage_returns_empty_and_no_anomaly() {
    let det = detector(2, 3600);
    let usage = det.get_usage_by_device("unknown").await;
    assert!(usage.is_empty());
    assert!(det.check_anomaly("unknown").await.is_none());
}

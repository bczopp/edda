//! Tests for Phase 15.2.1: AnomalyDetector (patterns, score, alerts).

use bifrost::security::anomaly::{AnomalyDetector, AnomalyEvent};
use std::sync::Arc;
use std::time::Duration;

#[test]
fn no_events_zero_score() {
    let detector = AnomalyDetector::new(10, Duration::from_secs(60), 70);
    assert_eq!(detector.get_anomaly_score(), 0);
}

#[test]
fn many_connects_in_short_time_raises_score() {
    let detector = Arc::new(AnomalyDetector::new(5, Duration::from_secs(60), 70));
    for i in 0..10 {
        detector.record(AnomalyEvent::Connect {
            connection_id: format!("conn-{}", i),
            device_id: format!("device-{}", i),
        });
    }
    let score = detector.get_anomaly_score();
    assert!(score > 0);
}

#[test]
fn check_alert_returns_some_when_score_above_threshold() {
    let detector = Arc::new(AnomalyDetector::new(2, Duration::from_secs(60), 10));
    for i in 0..20 {
        detector.record(AnomalyEvent::Connect {
            connection_id: format!("c-{}", i),
            device_id: format!("d-{}", i),
        });
    }
    let alert = detector.check_alert();
    assert!(alert.is_some());
    assert!(alert.unwrap().score >= 10);
}

#[test]
fn check_alert_returns_none_when_below_threshold() {
    let detector = AnomalyDetector::new(100, Duration::from_secs(1), 99);
    detector.record(AnomalyEvent::Connect {
        connection_id: "c1".to_string(),
        device_id: "d1".to_string(),
    });
    let alert = detector.check_alert();
    assert!(alert.is_none() || alert.unwrap().score < 99);
}

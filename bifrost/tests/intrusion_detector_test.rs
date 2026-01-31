//! Tests for Phase 15.2.2: IntrusionDetector (patterns, alerts, block).

use bifrost::security::{IntrusionDetector, IntrusionEvent, SecurityAlert};
use std::sync::Arc;

#[test]
fn no_events_no_alert() {
    let detector = IntrusionDetector::new(3, 5);
    assert!(detector.check_alert().is_none());
}

#[test]
fn repeated_failed_auth_triggers_alert() {
    let detector = Arc::new(IntrusionDetector::new(2, 5));
    detector.record(IntrusionEvent::FailedAuth {
        connection_id: "c1".to_string(),
        device_id: "d1".to_string(),
    });
    detector.record(IntrusionEvent::FailedAuth {
        connection_id: "c1".to_string(),
        device_id: "d1".to_string(),
    });
    detector.record(IntrusionEvent::FailedAuth {
        connection_id: "c1".to_string(),
        device_id: "d1".to_string(),
    });
    let alert = detector.check_alert();
    assert!(alert.is_some());
    assert!(matches!(alert.unwrap().kind, bifrost::security::AlertKind::RepeatedFailedAuth));
}

#[test]
fn repeated_failed_auth_blocks_connection() {
    let detector = Arc::new(IntrusionDetector::new(2, 5));
    detector.record(IntrusionEvent::FailedAuth {
        connection_id: "c1".to_string(),
        device_id: "d1".to_string(),
    });
    detector.record(IntrusionEvent::FailedAuth {
        connection_id: "c1".to_string(),
        device_id: "d1".to_string(),
    });
    detector.record(IntrusionEvent::FailedAuth {
        connection_id: "c1".to_string(),
        device_id: "d1".to_string(),
    });
    assert!(detector.should_block_connection("c1"));
    assert!(detector.should_block_device("d1"));
}

#[test]
fn unknown_connection_not_blocked() {
    let detector = IntrusionDetector::new(5, 5);
    assert!(!detector.should_block_connection("c1"));
    assert!(!detector.should_block_device("d1"));
}

#[test]
fn invalid_message_count_triggers_alert() {
    let detector = Arc::new(IntrusionDetector::new(5, 2));
    detector.record(IntrusionEvent::InvalidMessage {
        connection_id: "c1".to_string(),
    });
    detector.record(IntrusionEvent::InvalidMessage {
        connection_id: "c1".to_string(),
    });
    detector.record(IntrusionEvent::InvalidMessage {
        connection_id: "c1".to_string(),
    });
    let alert = detector.check_alert();
    assert!(alert.is_some());
}

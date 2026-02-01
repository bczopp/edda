//! Tests for Phase 5.4.1: ConnectionStatusTracker (ACTIVE, IDLE, SUSPICIOUS, BLOCKED; Heimdall updates; propagate to clients).

use bifrost::connection::{ConnectionStatus, ConnectionStatusTracker};
use std::sync::mpsc;

#[test]
fn new_tracker_starts_empty() {
    let tracker = ConnectionStatusTracker::new(None);
    assert!(tracker.get_status("conn-1").is_none());
}

#[test]
fn update_and_get_status() {
    let tracker = ConnectionStatusTracker::new(None);
    tracker.update_status("conn-1", ConnectionStatus::Active);
    assert_eq!(tracker.get_status("conn-1"), Some(ConnectionStatus::Active));
}

#[test]
fn update_status_overwrites() {
    let tracker = ConnectionStatusTracker::new(None);
    tracker.update_status("conn-1", ConnectionStatus::Active);
    tracker.update_status("conn-1", ConnectionStatus::Blocked);
    assert_eq!(tracker.get_status("conn-1"), Some(ConnectionStatus::Blocked));
}

#[test]
fn multiple_connections_tracked_independently() {
    let tracker = ConnectionStatusTracker::new(None);
    tracker.update_status("conn-1", ConnectionStatus::Active);
    tracker.update_status("conn-2", ConnectionStatus::Idle);
    tracker.update_status("conn-3", ConnectionStatus::Suspicious);
    assert_eq!(tracker.get_status("conn-1"), Some(ConnectionStatus::Active));
    assert_eq!(tracker.get_status("conn-2"), Some(ConnectionStatus::Idle));
    assert_eq!(tracker.get_status("conn-3"), Some(ConnectionStatus::Suspicious));
}

#[test]
fn status_propagated_to_channel_on_update() {
    let (tx, rx) = mpsc::channel();
    let tracker = ConnectionStatusTracker::new(Some(tx));
    tracker.update_status("conn-1", ConnectionStatus::Active);
    let (id, status) = rx.recv().unwrap();
    assert_eq!(id, "conn-1");
    assert_eq!(status, ConnectionStatus::Active);

    tracker.update_status("conn-1", ConnectionStatus::Blocked);
    let (id2, status2) = rx.recv().unwrap();
    assert_eq!(id2, "conn-1");
    assert_eq!(status2, ConnectionStatus::Blocked);
}

#[test]
fn remove_connection() {
    let tracker = ConnectionStatusTracker::new(None);
    tracker.update_status("conn-1", ConnectionStatus::Active);
    tracker.remove("conn-1");
    assert!(tracker.get_status("conn-1").is_none());
}

#[test]
fn remove_nonexistent_is_noop() {
    let tracker = ConnectionStatusTracker::new(None);
    tracker.remove("conn-1");
    assert!(tracker.get_status("conn-1").is_none());
}

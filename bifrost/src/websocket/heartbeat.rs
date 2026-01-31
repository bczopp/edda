//! Heartbeat mechanism (Phase 6.2.3): configurable send interval, timeout on missing heartbeats.

use std::time::{Duration, Instant};

/// Decides when to send heartbeats and when to consider a connection timed out (no heartbeat received).
pub struct HeartbeatManager {
    send_interval: Duration,
    timeout: Duration,
    last_sent: Instant,
    last_received: Instant,
}

impl HeartbeatManager {
    /// Creates a new heartbeat manager.
    /// * `send_interval` – send a heartbeat when this much time has passed since last sent.
    /// * `timeout` – consider connection timed out when no heartbeat received for this long.
    pub fn new(send_interval: Duration, timeout: Duration) -> Self {
        let now = Instant::now();
        Self {
            send_interval,
            timeout,
            last_sent: now,
            last_received: now,
        }
    }

    /// Returns true when a heartbeat should be sent (interval elapsed since last sent).
    pub fn should_send_heartbeat(&self) -> bool {
        self.last_sent.elapsed() >= self.send_interval
    }

    /// Returns true when no heartbeat was received for the configured timeout (connection timed out).
    pub fn should_timeout(&self) -> bool {
        self.last_received.elapsed() > self.timeout
    }

    /// Call when a heartbeat was sent.
    pub fn record_sent(&mut self) {
        self.last_sent = Instant::now();
    }

    /// Call when a heartbeat was received.
    pub fn record_received(&mut self) {
        self.last_received = Instant::now();
    }
}

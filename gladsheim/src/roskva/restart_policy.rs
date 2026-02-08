//! Restart policy and attempt tracking for auto-restart (Phase 5.3.1, 5.3.2).

use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;

/// Policy for when and how to restart unhealthy services.
#[derive(Debug, Clone)]
pub struct RestartPolicy {
    pub enabled: bool,
    pub max_attempts: u32,
    pub no_restart_services: HashSet<String>,
    pub base_backoff: Duration,
    pub max_backoff: Duration,
}

impl Default for RestartPolicy {
    fn default() -> Self {
        Self {
            enabled: true,
            max_attempts: 5,
            no_restart_services: HashSet::new(),
            base_backoff: Duration::from_secs(1),
            max_backoff: Duration::from_secs(60),
        }
    }
}

impl RestartPolicy {
    pub fn new(enabled: bool, max_attempts: u32) -> Self {
        Self {
            enabled,
            max_attempts,
            ..Default::default()
        }
    }

    /// Whether a restart is allowed for this service given current attempt count.
    pub fn should_allow_restart(&self, service_name: &str, current_attempts: u32) -> bool {
        if !self.enabled {
            return false;
        }
        if self.no_restart_services.contains(service_name) {
            return false;
        }
        current_attempts < self.max_attempts
    }

    /// Exponential backoff duration for the given attempt (0-based).
    pub fn backoff_duration(&self, attempt: u32) -> Duration {
        let secs = self.base_backoff.as_secs().saturating_mul(1 << attempt.min(31));
        let duration = Duration::from_secs(secs);
        if duration > self.max_backoff {
            self.max_backoff
        } else {
            duration
        }
    }

    pub fn add_no_restart(&mut self, service_name: String) {
        self.no_restart_services.insert(service_name);
    }

    pub fn remove_no_restart(&mut self, service_name: &str) -> bool {
        self.no_restart_services.remove(service_name)
    }
}

/// Tracks restart attempts per service (Phase 5.3.2).
#[derive(Clone, Default)]
pub struct RestartAttemptTracker {
    attempts: Arc<RwLock<HashMap<String, u32>>>,
}

impl RestartAttemptTracker {
    pub fn new() -> Self {
        Self::default()
    }

    pub async fn get(&self, service_name: &str) -> u32 {
        let m = self.attempts.read().await;
        *m.get(service_name).unwrap_or(&0)
    }

    pub async fn increment(&self, service_name: &str) -> u32 {
        let mut m = self.attempts.write().await;
        let v = m.entry(service_name.to_string()).or_insert(0);
        *v = v.saturating_add(1);
        *v
    }

    pub async fn reset(&self, service_name: &str) {
        let mut m = self.attempts.write().await;
        m.remove(service_name);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_restart_policy_disabled_no_restart() {
        let policy = RestartPolicy::new(false, 5);
        assert!(!policy.should_allow_restart("svc", 0));
    }

    #[test]
    fn test_restart_policy_within_max_attempts() {
        let policy = RestartPolicy::new(true, 3);
        assert!(policy.should_allow_restart("svc", 0));
        assert!(policy.should_allow_restart("svc", 2));
        assert!(!policy.should_allow_restart("svc", 3));
        assert!(!policy.should_allow_restart("svc", 4));
    }

    #[test]
    fn test_restart_policy_no_restart_list() {
        let mut policy = RestartPolicy::default();
        policy.add_no_restart("critical".to_string());
        assert!(!policy.should_allow_restart("critical", 0));
        assert!(policy.should_allow_restart("other", 0));
        assert!(policy.remove_no_restart("critical"));
        assert!(policy.should_allow_restart("critical", 0));
    }

    #[test]
    fn test_backoff_exponential() {
        let policy = RestartPolicy {
            base_backoff: Duration::from_secs(1),
            max_backoff: Duration::from_secs(300),
            ..Default::default()
        };
        assert_eq!(policy.backoff_duration(0), Duration::from_secs(1));
        assert_eq!(policy.backoff_duration(1), Duration::from_secs(2));
        assert_eq!(policy.backoff_duration(2), Duration::from_secs(4));
        assert_eq!(policy.backoff_duration(3), Duration::from_secs(8));
    }

    #[test]
    fn test_backoff_capped_at_max() {
        let policy = RestartPolicy {
            base_backoff: Duration::from_secs(10),
            max_backoff: Duration::from_secs(30),
            ..Default::default()
        };
        assert_eq!(policy.backoff_duration(0), Duration::from_secs(10));
        assert_eq!(policy.backoff_duration(1), Duration::from_secs(20));
        assert_eq!(policy.backoff_duration(2), Duration::from_secs(30));
        assert_eq!(policy.backoff_duration(3), Duration::from_secs(30));
    }

    #[tokio::test]
    async fn test_restart_attempt_tracker() {
        let tracker = RestartAttemptTracker::new();
        assert_eq!(tracker.get("svc").await, 0);
        assert_eq!(tracker.increment("svc").await, 1);
        assert_eq!(tracker.increment("svc").await, 2);
        assert_eq!(tracker.get("svc").await, 2);
        tracker.reset("svc").await;
        assert_eq!(tracker.get("svc").await, 0);
    }
}

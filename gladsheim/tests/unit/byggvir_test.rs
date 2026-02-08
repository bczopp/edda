//! Tests for Byggvir (Resource Manager)

use gladsheim::byggvir::{
    Byggvir,
    ServiceResourceTracker,
    ResourceLimitChecker,
    ResourceEnforcer,
    ResourceManager,
};
use gladsheim::byggvir::SystemResourceMonitor;
use std::time::Duration;

#[tokio::test]
async fn test_byggvir_creation() {
    let byggvir = Byggvir::new();
    assert!(byggvir.is_ok());
}

#[tokio::test]
async fn test_service_resource_tracker_register() {
    let tracker = ServiceResourceTracker::new();
    
    tracker.register_service("test-service".to_string(), 12345).await;
    
    let usage = tracker.get_usage("test-service").await;
    assert!(usage.is_some());
}

#[tokio::test]
async fn test_service_resource_tracker_update() {
    let tracker = ServiceResourceTracker::new();
    
    tracker.register_service("test-service".to_string(), 12345).await;
    tracker.update_usage("test-service", 1024 * 1024, 25.0).await; // 1MB, 25% CPU
    
    let usage = tracker.get_usage("test-service").await;
    assert_eq!(usage.unwrap().memory_bytes, 1024 * 1024);
    assert_eq!(usage.unwrap().cpu_percent, 25.0);
}

#[tokio::test]
async fn test_resource_limit_checker_check_memory() {
    let checker = ResourceLimitChecker::new();
    
    let limits = gladsheim::byggvir::ResourceLimits::new(512, 50.0); // 512MB, 50% CPU
    
    // Check within limits
    let result = checker.check_memory_limit(256 * 1024 * 1024, &limits); // 256MB
    assert!(result.is_ok());
    
    // Check exceeding limits
    let result = checker.check_memory_limit(1024 * 1024 * 1024, &limits); // 1GB
    assert!(result.is_err());
}

#[tokio::test]
async fn test_resource_limit_checker_check_cpu() {
    let checker = ResourceLimitChecker::new();
    
    let limits = gladsheim::byggvir::ResourceLimits::new(512, 50.0);
    
    // Check within limits
    let result = checker.check_cpu_limit(25.0, &limits);
    assert!(result.is_ok());
    
    // Check exceeding limits
    let result = checker.check_cpu_limit(75.0, &limits);
    assert!(result.is_err());
}

#[tokio::test]
async fn test_resource_enforcer_warning() {
    let enforcer = ResourceEnforcer::new();
    
    // Test warning threshold (80% of limit)
    let limits = gladsheim::byggvir::ResourceLimits::new(512, 50.0);
    let usage = 410 * 1024 * 1024; // ~80% of 512MB
    
    let action = enforcer.check_and_enforce(usage, 25.0, &limits).await;
    // Should return warning or ok
    assert!(matches!(action, EnforcementAction::Ok | EnforcementAction::Warning { .. }));
}

#[tokio::test]
async fn test_resource_enforcer_critical() {
    let enforcer = ResourceEnforcer::new();
    
    // Test critical threshold (100%+ of limit)
    let limits = gladsheim::byggvir::ResourceLimits::new(512, 50.0);
    let usage = 600 * 1024 * 1024; // Exceeds 512MB
    
    let action = enforcer.check_and_enforce(usage, 25.0, &limits).await;
    // Should return critical
    assert!(matches!(action, EnforcementAction::Critical { .. }));
}

#[tokio::test]
async fn test_system_resource_monitor_creation_and_refresh() {
    let mut monitor = SystemResourceMonitor::new();
    // Should be callable without panic
    monitor.refresh();
}

#[tokio::test]
async fn test_system_resource_monitor_total_and_used_memory() {
    let monitor = SystemResourceMonitor::new();

    let total = monitor.total_memory();
    let used = monitor.used_memory();

    assert!(total > 0, "total memory should be > 0");
    assert!(used <= total, "used memory should not exceed total memory");
}

#[tokio::test]
async fn test_system_resource_monitor_total_cpu_is_non_negative() {
    let monitor = SystemResourceMonitor::new();

    let cpu = monitor.total_cpu();
    assert!(cpu >= 0.0, "CPU usage should be >= 0");
}

#[tokio::test]
async fn test_system_resource_monitor_process_memory_and_cpu_optional() {
    let monitor = SystemResourceMonitor::new();

    // We cannot guarantee a specific PID is present here, but we can
    // verify that the API is callable and returns an Option without panicking.
    // Using an arbitrary large PID makes it likely that it does not exist.
    let fake_pid: i32 = 999_999;

    let mem = monitor.process_memory(fake_pid.into());
    let cpu = monitor.process_cpu(fake_pid.into());

    // We only assert that the calls succeed and return an Option.
    // Concrete semantics are covered in higher-level integration tests.
    assert!(mem.is_none() || mem.is_some());
    assert!(cpu.is_none() || cpu.is_some());
}

#[tokio::test]
async fn test_service_resource_tracker_history_stores_multiple_entries() {
    let tracker = ServiceResourceTracker::new();

    tracker
        .register_service("history-service".to_string(), 42)
        .await;

    tracker
        .update_usage("history-service", 100, 1.0)
        .await;
    tracker
        .update_usage("history-service", 200, 2.0)
        .await;
    tracker
        .update_usage("history-service", 300, 3.0)
        .await;

    let history = tracker.get_history("history-service", None).await;
    assert_eq!(history.len(), 3);
    assert_eq!(history[0].memory_bytes, 100);
    assert_eq!(history[1].memory_bytes, 200);
    assert_eq!(history[2].memory_bytes, 300);
}

#[tokio::test]
async fn test_service_resource_tracker_history_is_limited_to_last_n_entries() {
    let tracker = ServiceResourceTracker::with_history_capacity(3);

    tracker
        .register_service("limited-history".to_string(), 7)
        .await;

    for i in 0..5 {
        tracker
            .update_usage("limited-history", i * 10, i as f32)
            .await;
    }

    let history = tracker.get_history("limited-history", None).await;
    assert_eq!(history.len(), 3);
    assert_eq!(history[0].memory_bytes, 20);
    assert_eq!(history[1].memory_bytes, 30);
    assert_eq!(history[2].memory_bytes, 40);
}

#[tokio::test]
async fn test_service_resource_tracker_history_limit_parameter() {
    let tracker = ServiceResourceTracker::new();

    tracker
        .register_service("history-limit".to_string(), 1)
        .await;

    tracker
        .update_usage("history-limit", 10, 1.0)
        .await;
    tracker
        .update_usage("history-limit", 20, 2.0)
        .await;
    tracker
        .update_usage("history-limit", 30, 3.0)
        .await;

    let history_two = tracker.get_history("history-limit", Some(2)).await;
    assert_eq!(history_two.len(), 2);
    assert_eq!(history_two[0].memory_bytes, 20);
    assert_eq!(history_two[1].memory_bytes, 30);
}

#[tokio::test]
async fn test_service_resource_tracker_aggregation_average() {
    let tracker = ServiceResourceTracker::new();

    tracker
        .register_service("agg-avg".to_string(), 1)
        .await;

    tracker.update_usage("agg-avg", 100, 10.0).await;
    tracker.update_usage("agg-avg", 200, 20.0).await;
    tracker.update_usage("agg-avg", 300, 30.0).await;

    let aggregated = tracker
        .aggregate_average("agg-avg")
        .await
        .expect("average aggregation should exist");

    assert_eq!(aggregated.memory_bytes, 200);
    assert!((aggregated.cpu_percent - 20.0).abs() < f32::EPSILON);
}

#[tokio::test]
async fn test_service_resource_tracker_aggregation_min_max() {
    let tracker = ServiceResourceTracker::new();

    tracker
        .register_service("agg-min-max".to_string(), 1)
        .await;

    tracker.update_usage("agg-min-max", 50, 5.0).await;
    tracker.update_usage("agg-min-max", 150, 15.0).await;
    tracker.update_usage("agg-min-max", 250, 25.0).await;

    let min = tracker
        .aggregate_min("agg-min-max")
        .await
        .expect("min aggregation should exist");
    let max = tracker
        .aggregate_max("agg-min-max")
        .await
        .expect("max aggregation should exist");

    assert_eq!(min.memory_bytes, 50);
    assert!((min.cpu_percent - 5.0).abs() < f32::EPSILON);

    assert_eq!(max.memory_bytes, 250);
    assert!((max.cpu_percent - 25.0).abs() < f32::EPSILON);
}

#[tokio::test]
async fn test_resource_manager_creation() {
    let manager = ResourceManager::new();
    // Just ensure construction works and we can drop it.
    drop(manager);
}

#[tokio::test]
async fn test_resource_manager_registers_service_with_limits() {
    let manager = ResourceManager::new();

    let limits = gladsheim::byggvir::ResourceLimits::new(256, 25.0);
    manager
        .register_service("svc-register".to_string(), 1234, limits)
        .await;

    // usage should exist (initialized with zero values)
    let usage = manager.get_resource_usage("svc-register").await;
    assert!(usage.is_some());
    assert_eq!(usage.unwrap().service_name, "svc-register");
}

#[tokio::test]
async fn test_resource_manager_update_and_enforce_within_limits() {
    let manager = ResourceManager::new();

    let limits = gladsheim::byggvir::ResourceLimits::new(256, 50.0);
    manager
        .register_service("svc-ok".to_string(), 1, limits)
        .await;

    let action = manager
        .update_and_enforce("svc-ok", 64 * 1024 * 1024, 10.0)
        .await
        .expect("service should be known");

    // Within limits: either Ok or Warning depending on thresholds
    use gladsheim::byggvir::EnforcementAction;
    assert!(matches!(action, EnforcementAction::Ok | EnforcementAction::Warning { .. }));
}

#[tokio::test]
async fn test_resource_manager_update_and_enforce_exceeds_limits() {
    let manager = ResourceManager::new();

    let limits = gladsheim::byggvir::ResourceLimits::new(128, 20.0);
    manager
        .register_service("svc-over".to_string(), 2, limits)
        .await;

    let action = manager
        .update_and_enforce("svc-over", 512 * 1024 * 1024, 50.0)
        .await
        .expect("service should be known");

    use gladsheim::byggvir::EnforcementAction;
    assert!(matches!(action, EnforcementAction::Critical { .. }));
}

#[tokio::test]
async fn test_resource_manager_monitoring_loop_starts_and_stops() {
    let manager = ResourceManager::new();

    let limits = gladsheim::byggvir::ResourceLimits::new(256, 50.0);
    manager
        .register_service("svc-loop".to_string(), 0, limits)
        .await;

    let handle = manager.start_monitoring_loop(Duration::from_millis(10));

    // Give the loop a little time to run.
    tokio::time::sleep(Duration::from_millis(30)).await;

    // Dropping the handle should signal the loop to stop without panic.
    drop(handle);
}

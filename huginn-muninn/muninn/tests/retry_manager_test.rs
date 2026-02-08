//! Tests for Retry Manager

use muninn::retry::RetryManager;
use std::time::Duration;

#[tokio::test]
async fn test_retry_manager_new() {
    let retry = RetryManager::new(3, Duration::from_millis(100));
    assert_eq!(retry.max_retries(), 3);
    assert_eq!(retry.initial_delay(), Duration::from_millis(100));
}

#[tokio::test]
async fn test_retry_manager_execute_success() {
    let retry = RetryManager::new(3, Duration::from_millis(10));
    let result = retry.execute(|| async {
        Ok::<String, String>("Success".to_string())
    }).await;
    
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "Success");
}

#[tokio::test]
async fn test_retry_manager_execute_failure_no_retry() {
    let retry = RetryManager::new(0, Duration::from_millis(10));
    let mut attempts = 0;
    
    let result = retry.execute(|| {
        attempts += 1;
        async move {
            Err::<String, String>("Error".to_string())
        }
    }).await;
    
    assert!(result.is_err());
    assert_eq!(attempts, 1); // Should only try once (no retries)
}

#[tokio::test]
async fn test_retry_manager_execute_with_retries() {
    let retry = RetryManager::new(3, Duration::from_millis(10));
    let mut attempts = 0;
    
    let result = retry.execute(|| {
        attempts += 1;
        async move {
            if attempts < 3 {
                Err::<String, String>("Temporary error".to_string())
            } else {
                Ok::<String, String>("Success".to_string())
            }
        }
    }).await;
    
    assert!(result.is_ok());
    assert_eq!(attempts, 3); // Should retry 2 times (total 3 attempts)
}

#[tokio::test]
async fn test_retry_manager_execute_max_retries_exceeded() {
    let retry = RetryManager::new(3, Duration::from_millis(10));
    let mut attempts = 0;
    
    let result = retry.execute(|| {
        attempts += 1;
        async move {
            Err::<String, String>("Persistent error".to_string())
        }
    }).await;
    
    assert!(result.is_err());
    assert_eq!(attempts, 4); // Should try 3 times + 1 initial = 4 total attempts
}

#[tokio::test]
async fn test_retry_manager_exponential_backoff() {
    let retry = RetryManager::new(3, Duration::from_millis(10));
    let delays = retry.calculate_delays();
    
    // Exponential backoff: initial, initial*2, initial*4
    assert_eq!(delays.len(), 3);
    assert_eq!(delays[0], Duration::from_millis(10));
    assert_eq!(delays[1], Duration::from_millis(20));
    assert_eq!(delays[2], Duration::from_millis(40));
}

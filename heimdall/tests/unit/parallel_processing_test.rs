//! Tests für Parallel-Processing (Phase 15.1.1).
//! Parallele Verarbeitung von Security-Checks (Token-Validierung, Permission-Check).

#[cfg(test)]
mod tests {
    use heimdall::utils::performance::ParallelProcessor;
    use std::sync::atomic::{AtomicU32, Ordering};

    #[tokio::test]
    async fn parallel_token_validations_run_concurrently() {
        let counter = std::sync::Arc::new(AtomicU32::new(0));
        let tokens = vec!["token1".to_string(), "token2".to_string(), "token3".to_string()];

        let validator = {
            let counter = std::sync::Arc::clone(&counter);
            move |token: String| {
                let counter = std::sync::Arc::clone(&counter);
                Box::pin(async move {
                    counter.fetch_add(1, Ordering::SeqCst);
                    if token.starts_with("token") {
                        Ok(())
                    } else {
                        Err("invalid".into())
                    }
                })
            }
        };

        let results = ParallelProcessor::validate_tokens_parallel(tokens, validator).await;

        assert_eq!(results.len(), 3);
        assert!(results.iter().all(|r| r.is_ok()));
        assert_eq!(counter.load(Ordering::SeqCst), 3);
    }

    #[tokio::test]
    async fn parallel_permission_checks_run_concurrently() {
        let counter = std::sync::Arc::new(AtomicU32::new(0));
        let checks = vec![
            ("dev1".to_string(), "user1".to_string(), "resource".to_string(), "read".to_string()),
            ("dev2".to_string(), "user2".to_string(), "resource".to_string(), "write".to_string()),
        ];

        let checker = {
            let counter = std::sync::Arc::clone(&counter);
            move |_d: String, _u: String, _r: String, _a: String| {
                let counter = std::sync::Arc::clone(&counter);
                Box::pin(async move {
                    counter.fetch_add(1, Ordering::SeqCst);
                    Ok(true)
                })
            }
        };

        let results = ParallelProcessor::check_permissions_parallel(checks, checker).await;

        assert_eq!(results.len(), 2);
        assert!(results.iter().all(|r| r.is_ok()));
        assert_eq!(counter.load(Ordering::SeqCst), 2);
    }

    #[tokio::test]
    async fn parallel_token_validation_handles_failures() {
        let tokens = vec!["ok".to_string(), "fail".to_string()];
        let validator = |token: String| {
            Box::pin(async move {
                if token == "ok" {
                    Ok(())
                } else {
                    Err::<(), Box<dyn std::error::Error + Send + Sync>>("validation failed".into())
                }
            })
        };

        let results = ParallelProcessor::validate_tokens_parallel(tokens, validator).await;

        assert_eq!(results.len(), 2);
        assert!(results[0].is_ok());
        assert!(results[1].is_err());
    }
}

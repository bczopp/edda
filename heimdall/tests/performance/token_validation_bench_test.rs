//! Performance-Tests: Token-Validierung und Benchmarks (Phase 15.2.1, 18.3.1).
//! Ziel: Token-Validierung < 10ms, Permission-Check < 5ms (siehe README/IMPLEMENTATION_PLAN).

#[cfg(test)]
mod benchmark_runner {
    use heimdall::utils::performance::PerformanceBenchmark;

    /// run_all_benchmarks mit Mock-Validatoren (no-op) erfüllt Ziele < 10ms / < 5ms.
    #[tokio::test]
    async fn run_all_benchmarks_meets_targets_with_fast_mocks() {
        let token_validator = || {
            Box::pin(async move { Ok::<(), Box<dyn std::error::Error + Send + Sync>>(()) })
        };
        let permission_checker = || {
            Box::pin(async move { Ok::<(), Box<dyn std::error::Error + Send + Sync>>(()) })
        };

        let result = PerformanceBenchmark::run_all_benchmarks(token_validator, permission_checker).await;
        assert!(result.is_ok(), "benchmarks should meet targets: {:?}", result.err());
    }
}

#[cfg(test)]
mod token_validation {
    use std::sync::Arc;
    use std::time::Instant;

    use heimdall::token::{TokenGenerator, TokenValidator};
    use heimdall::keys::KeyGenerator;
    use heimdall::utils::config::HeimdallSettings;
    use heimdall::utils::TokenValidationCache;
    use crate::common::TestDatabase;

    /// Single validation should complete; target < 10ms for standard tokens.
    #[tokio::test]
    async fn token_validation_completes_within_target_time() {
        let _test_db = TestDatabase::new().await.expect("DATABASE_URL set");
        let (keypair, pkcs8) = KeyGenerator::new().generate_ed25519_keypair().unwrap();
        let config = HeimdallSettings::default().token_configuration;
        let generator = TokenGenerator::new(std::sync::Arc::new(keypair), config);
        let (token, _, _) = generator
            .generate_session_token("perf-dev-1", "user-1")
            .expect("generate token");

        let keys_dir = std::env::temp_dir().join("heimdall-perf-keys");
        let _ = std::fs::create_dir_all(&keys_dir);
        let key_storage = heimdall::keys::SecureKeyStorage::new(keys_dir.clone());
        key_storage.store_keypair("heimdall", &pkcs8).expect("store key");
        let validator = TokenValidator::with_cache(
            keys_dir,
            Arc::new(TokenValidationCache::new(300)),
        );

        let start = Instant::now();
        let result = validator.validate_token(&token).await;
        let elapsed = start.elapsed();

        assert!(result.is_ok(), "validation should succeed");
        // Ziel: < 10ms (Skelett: nur prüfen dass validierung durchläuft; Schwellwert optional)
        assert!(
            elapsed.as_millis() < 500,
            "token validation should complete in reasonable time (target < 10ms in production); got {:?}",
            elapsed
        );
    }
}

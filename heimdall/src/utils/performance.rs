use std::sync::Arc;
use tokio::sync::RwLock;
use std::time::{Duration, Instant};
use tracing::{info, warn};

/// Performance metrics for Heimdall operations
#[derive(Debug, Clone)]
pub struct PerformanceMetrics {
    pub token_validation_count: u64,
    pub token_validation_total_time: Duration,
    pub permission_check_count: u64,
    pub permission_check_total_time: Duration,
    pub connection_validation_count: u64,
    pub connection_validation_total_time: Duration,
}

impl PerformanceMetrics {
    pub fn new() -> Self {
        Self {
            token_validation_count: 0,
            token_validation_total_time: Duration::ZERO,
            permission_check_count: 0,
            permission_check_total_time: Duration::ZERO,
            connection_validation_count: 0,
            connection_validation_total_time: Duration::ZERO,
        }
    }

    pub fn token_validation_avg_time(&self) -> Duration {
        if self.token_validation_count == 0 {
            return Duration::ZERO;
        }
        self.token_validation_total_time / self.token_validation_count as u32
    }

    pub fn permission_check_avg_time(&self) -> Duration {
        if self.permission_check_count == 0 {
            return Duration::ZERO;
        }
        self.permission_check_total_time / self.permission_check_count as u32
    }

    pub fn connection_validation_avg_time(&self) -> Duration {
        if self.connection_validation_count == 0 {
            return Duration::ZERO;
        }
        self.connection_validation_total_time / self.connection_validation_count as u32
    }
}

/// Performance monitor for tracking operation times
pub struct PerformanceMonitor {
    metrics: Arc<RwLock<PerformanceMetrics>>,
    token_validation_target: Duration,
    permission_check_target: Duration,
}

impl PerformanceMonitor {
    pub fn new() -> Self {
        Self {
            metrics: Arc::new(RwLock::new(PerformanceMetrics::new())),
            token_validation_target: Duration::from_millis(10), // < 10ms target
            permission_check_target: Duration::from_millis(5),   // < 5ms target
        }
    }

    /// Record token validation time
    pub async fn record_token_validation(&self, duration: Duration) {
        let mut metrics = self.metrics.write().await;
        metrics.token_validation_count += 1;
        metrics.token_validation_total_time += duration;

        if duration > self.token_validation_target {
            warn!(
                "Token validation took {}ms (target: {}ms)",
                duration.as_millis(),
                self.token_validation_target.as_millis()
            );
        }
    }

    /// Record permission check time
    pub async fn record_permission_check(&self, duration: Duration) {
        let mut metrics = self.metrics.write().await;
        metrics.permission_check_count += 1;
        metrics.permission_check_total_time += duration;

        if duration > self.permission_check_target {
            warn!(
                "Permission check took {}ms (target: {}ms)",
                duration.as_millis(),
                self.permission_check_target.as_millis()
            );
        }
    }

    /// Record connection validation time
    pub async fn record_connection_validation(&self, duration: Duration) {
        let mut metrics = self.metrics.write().await;
        metrics.connection_validation_count += 1;
        metrics.connection_validation_total_time += duration;
    }

    /// Get current metrics
    pub async fn get_metrics(&self) -> PerformanceMetrics {
        self.metrics.read().await.clone()
    }

    /// Reset metrics
    pub async fn reset(&self) {
        let mut metrics = self.metrics.write().await;
        *metrics = PerformanceMetrics::new();
    }
}

/// Parallel processor for security checks
pub struct ParallelProcessor;

impl ParallelProcessor {
    /// Process multiple token validations in parallel
    pub async fn validate_tokens_parallel<F, R>(
        tokens: Vec<String>,
        validator: F,
    ) -> Vec<Result<R, Box<dyn std::error::Error + Send + Sync>>>
    where
        F: Fn(String) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<R, Box<dyn std::error::Error + Send + Sync>>> + Send>> + Send + Sync + Clone + 'static,
        R: Send + 'static,
    {
        use futures::future::join_all;
        
        let futures: Vec<_> = tokens
            .into_iter()
            .map(|token| {
                let validator = validator.clone();
                tokio::spawn(async move {
                    validator(token).await
                })
            })
            .collect();

        let results = join_all(futures).await;
        results
            .into_iter()
            .map(|r| r.unwrap_or_else(|e| Err(Box::new(e) as Box<dyn std::error::Error + Send + Sync>)))
            .collect()
    }

    /// Process multiple permission checks in parallel
    pub async fn check_permissions_parallel<F, R>(
        checks: Vec<(String, String, String, String)>,
        checker: F,
    ) -> Vec<Result<R, Box<dyn std::error::Error + Send + Sync>>>
    where
        F: Fn(String, String, String, String) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<R, Box<dyn std::error::Error + Send + Sync>>> + Send>> + Send + Sync + Clone + 'static,
        R: Send + 'static,
    {
        use futures::future::join_all;
        
        let futures: Vec<_> = checks
            .into_iter()
            .map(|(device_id, user_id, resource_type, action)| {
                let checker = checker.clone();
                tokio::spawn(async move {
                    checker(device_id, user_id, resource_type, action).await
                })
            })
            .collect();

        let results = join_all(futures).await;
        results
            .into_iter()
            .map(|r| r.unwrap_or_else(|e| Err(Box::new(e) as Box<dyn std::error::Error + Send + Sync>)))
            .collect()
    }
}

/// Performance benchmark runner
pub struct PerformanceBenchmark;

impl PerformanceBenchmark {
    /// Benchmark token validation performance
    pub async fn benchmark_token_validation<F>(
        validator: F,
        iterations: u32,
    ) -> Duration
    where
        F: Fn() -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<(), Box<dyn std::error::Error + Send + Sync>>> + Send>>,
    {
        let start = Instant::now();
        
        for _ in 0..iterations {
            let _ = validator().await;
        }
        
        start.elapsed() / iterations
    }

    /// Benchmark permission check performance
    pub async fn benchmark_permission_check<F>(
        checker: F,
        iterations: u32,
    ) -> Duration
    where
        F: Fn() -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<(), Box<dyn std::error::Error + Send + Sync>>> + Send>>,
    {
        let start = Instant::now();
        
        for _ in 0..iterations {
            let _ = checker().await;
        }
        
        start.elapsed() / iterations
    }

    /// Run all performance benchmarks and verify targets are met
    pub async fn run_all_benchmarks<F1, F2>(
        token_validator: F1,
        permission_checker: F2,
    ) -> Result<(), String>
    where
        F1: Fn() -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<(), Box<dyn std::error::Error + Send + Sync>>> + Send>>,
        F2: Fn() -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<(), Box<dyn std::error::Error + Send + Sync>>> + Send>>,
    {
        const ITERATIONS: u32 = 1000;
        const TOKEN_TARGET: Duration = Duration::from_millis(10);
        const PERMISSION_TARGET: Duration = Duration::from_millis(5);

        let token_avg = Self::benchmark_token_validation(token_validator, ITERATIONS).await;
        let permission_avg = Self::benchmark_permission_check(permission_checker, ITERATIONS).await;

        info!(
            "Performance benchmarks: Token validation: {:?} (target: {:?}), Permission check: {:?} (target: {:?})",
            token_avg, TOKEN_TARGET, permission_avg, PERMISSION_TARGET
        );

        if token_avg > TOKEN_TARGET {
            return Err(format!(
                "Token validation performance target not met: {:?} > {:?}",
                token_avg, TOKEN_TARGET
            ));
        }

        if permission_avg > PERMISSION_TARGET {
            return Err(format!(
                "Permission check performance target not met: {:?} > {:?}",
                permission_avg, PERMISSION_TARGET
            ));
        }

        Ok(())
    }
}

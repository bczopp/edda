use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;
use std::collections::VecDeque;
use chrono::{DateTime, Utc};
use tracing::debug;

const MAX_SAMPLES: usize = 1000;
const TIME_WINDOW_SECONDS: u64 = 60;

#[derive(Debug, Clone)]
pub struct PerformanceStats {
    pub query_count: u64,
    pub write_count: u64,
    pub avg_query_time_ms: f64,
    pub avg_write_time_ms: f64,
    pub min_query_time_ms: f64,
    pub max_query_time_ms: f64,
    pub min_write_time_ms: f64,
    pub max_write_time_ms: f64,
    pub queries_per_second: f64,
    pub writes_per_second: f64,
    pub pool_size: u32,
    pub pool_idle: usize,
    pub pool_is_closed: bool,
    pub last_updated: DateTime<Utc>,
}

#[derive(Debug, Clone)]
struct TimedSample {
    duration: Duration,
    timestamp: DateTime<Utc>,
}

pub struct PerformanceMonitor {
    query_samples: Arc<RwLock<VecDeque<TimedSample>>>,
    write_samples: Arc<RwLock<VecDeque<TimedSample>>>,
    pool_stats: Arc<RwLock<PoolStatsSnapshot>>,
    window_start: Arc<RwLock<DateTime<Utc>>>,
}

#[derive(Debug, Clone)]
struct PoolStatsSnapshot {
    size: u32,
    idle: usize,
    is_closed: bool,
}

impl PerformanceMonitor {
    pub fn new() -> Self {
        Self {
            query_samples: Arc::new(RwLock::new(VecDeque::new())),
            write_samples: Arc::new(RwLock::new(VecDeque::new())),
            pool_stats: Arc::new(RwLock::new(PoolStatsSnapshot {
                size: 0,
                idle: 0,
                is_closed: false,
            })),
            window_start: Arc::new(RwLock::new(Utc::now())),
        }
    }

    pub async fn record_query_time(&self, duration: Duration) {
        let mut samples = self.query_samples.write().await;
        samples.push_back(TimedSample {
            duration,
            timestamp: Utc::now(),
        });
        
        // Keep only recent samples
        if samples.len() > MAX_SAMPLES {
            samples.pop_front();
        }
        
        // Clean old samples outside time window
        self.cleanup_old_samples(&mut samples).await;
    }

    pub async fn record_write_time(&self, duration: Duration) {
        let mut samples = self.write_samples.write().await;
        samples.push_back(TimedSample {
            duration,
            timestamp: Utc::now(),
        });
        
        // Keep only recent samples
        if samples.len() > MAX_SAMPLES {
            samples.pop_front();
        }
        
        // Clean old samples outside time window
        self.cleanup_old_samples(&mut samples).await;
    }

    pub async fn update_pool_stats(&self, size: u32, idle: usize, is_closed: bool) {
        let mut stats = self.pool_stats.write().await;
        stats.size = size;
        stats.idle = idle;
        stats.is_closed = is_closed;
    }

    async fn cleanup_old_samples(&self, samples: &mut VecDeque<TimedSample>) {
        let cutoff = Utc::now() - chrono::Duration::seconds(TIME_WINDOW_SECONDS as i64);
        while let Some(front) = samples.front() {
            if front.timestamp < cutoff {
                samples.pop_front();
            } else {
                break;
            }
        }
    }

    fn calculate_stats(samples: &VecDeque<TimedSample>) -> (u64, f64, f64, f64) {
        if samples.is_empty() {
            return (0, 0.0, 0.0, 0.0);
        }
        
        let count = samples.len() as u64;
        let sum: f64 = samples.iter()
            .map(|s| s.duration.as_secs_f64() * 1000.0)
            .sum();
        let avg = sum / count as f64;
        
        let min = samples.iter()
            .map(|s| s.duration.as_secs_f64() * 1000.0)
            .fold(f64::INFINITY, f64::min);
        
        let max = samples.iter()
            .map(|s| s.duration.as_secs_f64() * 1000.0)
            .fold(0.0, f64::max);
        
        (count, avg, min, max)
    }

    fn calculate_throughput(samples: &VecDeque<TimedSample>) -> f64 {
        if samples.is_empty() {
            return 0.0;
        }
        
        let oldest = samples.front().map(|s| s.timestamp);
        let newest = samples.back().map(|s| s.timestamp);
        
        if let (Some(oldest), Some(newest)) = (oldest, newest) {
            let duration = (newest - oldest).num_seconds();
            if duration > 0 {
                samples.len() as f64 / duration as f64
            } else {
                samples.len() as f64
            }
        } else {
            0.0
        }
    }

    pub async fn get_stats(&self) -> PerformanceStats {
        let query_samples = self.query_samples.read().await;
        let write_samples = self.write_samples.read().await;
        let pool_stats = self.pool_stats.read().await;
        
        let (query_count, avg_query, min_query, max_query) = Self::calculate_stats(&query_samples);
        let (write_count, avg_write, min_write, max_write) = Self::calculate_stats(&write_samples);
        
        let queries_per_second = Self::calculate_throughput(&query_samples);
        let writes_per_second = Self::calculate_throughput(&write_samples);
        
        PerformanceStats {
            query_count,
            write_count,
            avg_query_time_ms: avg_query,
            avg_write_time_ms: avg_write,
            min_query_time_ms: min_query,
            max_query_time_ms: max_query,
            min_write_time_ms: min_write,
            max_write_time_ms: max_write,
            queries_per_second,
            writes_per_second,
            pool_size: pool_stats.size,
            pool_idle: pool_stats.idle,
            pool_is_closed: pool_stats.is_closed,
            last_updated: Utc::now(),
        }
    }

    pub async fn reset_stats(&self) {
        let mut query_samples = self.query_samples.write().await;
        let mut write_samples = self.write_samples.write().await;
        query_samples.clear();
        write_samples.clear();
        debug!("Performance stats reset");
    }

    /// Export metrics in Prometheus format
    pub async fn export_prometheus_metrics(&self) -> String {
        let stats = self.get_stats().await;
        
        format!(
            "# HELP mimir_query_duration_seconds Average query duration in seconds\n\
             # TYPE mimir_query_duration_seconds gauge\n\
             mimir_query_duration_seconds {{}} {}\n\
             \n\
             # HELP mimir_write_duration_seconds Average write duration in seconds\n\
             # TYPE mimir_write_duration_seconds gauge\n\
             mimir_write_duration_seconds {{}} {}\n\
             \n\
             # HELP mimir_queries_total Total number of queries\n\
             # TYPE mimir_queries_total counter\n\
             mimir_queries_total {{}} {}\n\
             \n\
             # HELP mimir_writes_total Total number of writes\n\
             # TYPE mimir_writes_total counter\n\
             mimir_writes_total {{}} {}\n\
             \n\
             # HELP mimir_queries_per_second Queries per second\n\
             # TYPE mimir_queries_per_second gauge\n\
             mimir_queries_per_second {{}} {}\n\
             \n\
             # HELP mimir_writes_per_second Writes per second\n\
             # TYPE mimir_writes_per_second gauge\n\
             mimir_writes_per_second {{}} {}\n\
             \n\
             # HELP mimir_pool_size Database connection pool size\n\
             # TYPE mimir_pool_size gauge\n\
             mimir_pool_size {{}} {}\n\
             \n\
             # HELP mimir_pool_idle Idle connections in pool\n\
             # TYPE mimir_pool_idle gauge\n\
             mimir_pool_idle {{}} {}\n",
            stats.avg_query_time_ms / 1000.0,
            stats.avg_write_time_ms / 1000.0,
            stats.query_count,
            stats.write_count,
            stats.queries_per_second,
            stats.writes_per_second,
            stats.pool_size,
            stats.pool_idle,
        )
    }
}

impl Default for PerformanceMonitor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_performance_monitor_new() {
        let monitor = PerformanceMonitor::new();
        let stats = monitor.get_stats().await;
        assert_eq!(stats.query_count, 0);
        assert_eq!(stats.write_count, 0);
    }

    #[tokio::test]
    async fn test_calculate_stats_empty() {
        let samples = VecDeque::new();
        let (count, avg, min, max) = PerformanceMonitor::calculate_stats(&samples);
        assert_eq!(count, 0);
        assert_eq!(avg, 0.0);
    }

    #[tokio::test]
    async fn test_export_prometheus_metrics() {
        let monitor = PerformanceMonitor::new();
        monitor.record_query_time(Duration::from_millis(25)).await;
        
        let metrics = monitor.export_prometheus_metrics().await;
        assert!(metrics.contains("mimir_query_duration_seconds"));
        assert!(metrics.contains("mimir_queries_total"));
    }
}

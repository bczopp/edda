//! Performance-Alert-Manager (Phase 15.2.2): Threshold-basierte Alerts bei Performance-Problemen.

use crate::utils::MetricsCollector;

/// Einzelner Performance-Alert (Indexing oder Search über Schwellwert).
#[derive(Debug, Clone, PartialEq)]
pub enum PerformanceAlert {
    /// Durchschnittliche Indexing-Zeit über Schwellwert.
    IndexingSlow { current_avg_ms: f64 },
    /// Durchschnittliche Search-Zeit über Schwellwert.
    SearchSlow { current_avg_ms: f64 },
}

/// Prüft Metriken gegen Schwellwerte und liefert ggf. Alerts.
pub struct PerformanceAlertManager {
    max_avg_indexing_ms: f64,
    max_avg_search_ms: f64,
}

impl PerformanceAlertManager {
    pub fn new(max_avg_indexing_ms: f64, max_avg_search_ms: f64) -> Self {
        Self {
            max_avg_indexing_ms,
            max_avg_search_ms,
        }
    }

    /// Liest Metriken und gibt alle Alerts zurück, deren Schwellwerte überschritten sind.
    pub fn check_alerts(&self, metrics: &MetricsCollector) -> Vec<PerformanceAlert> {
        let mut alerts = Vec::new();
        let avg_indexing = metrics.get_avg_indexing_time_ms();
        if avg_indexing > 0.0 && avg_indexing > self.max_avg_indexing_ms {
            alerts.push(PerformanceAlert::IndexingSlow {
                current_avg_ms: avg_indexing,
            });
        }
        let avg_search = metrics.get_avg_search_time_ms();
        if avg_search > 0.0 && avg_search > self.max_avg_search_ms {
            alerts.push(PerformanceAlert::SearchSlow {
                current_avg_ms: avg_search,
            });
        }
        alerts
    }
}

use crate::performance::{PerformanceTracker, PerformanceWindow};
use crate::selection::{EfficiencyInput, EfficiencyScoreCalculator};
use crate::model::ModelInfo;

pub struct DynamicEfficiencyCalculator {
    performance_tracker: PerformanceTracker,
    base_calculator: EfficiencyScoreCalculator,
}

impl DynamicEfficiencyCalculator {
    pub fn new(performance_tracker: PerformanceTracker, base_calculator: EfficiencyScoreCalculator) -> Self {
        Self {
            performance_tracker,
            base_calculator,
        }
    }

    pub async fn calculate_score(&self, model: &ModelInfo) -> f64 {
        let metrics = self.performance_tracker.get_metrics(&model.provider, &model.id).await;
        
        let mut input = EfficiencyInput {
            parameter_count: model.parameter_count,
            max_parameter_count: 175_000_000_000, // GPT-3.5 size as baseline
            hardware_score: if model.is_local { 1.0 } else { 0.5 },
            uptime_percentage: metrics.as_ref().map(|m| m.success_rate() * 100.0),
            error_rate: metrics.as_ref().map(|m| 1.0 - m.success_rate()),
            ping_ms: metrics.as_ref().map(|m| m.average_latency_ms() as u32),
            max_ping_ms: 2000,
            distance_km: if model.is_local { Some(0.0) } else { Some(500.0) }, // Default remote distance
            max_distance_km: 2000.0,
            is_local: model.is_local,
            cost_per_token: model.cost_per_token_input, // Simplification: use input cost
            max_cost_per_token: 0.0001, // Baseline
        };

        // Windowed metrics can be used for more dynamic behavior
        if let Some(recent_metrics) = self.performance_tracker.get_windowed_metrics(&model.provider, &model.id, PerformanceWindow::Last1Min).await {
            // Update with recent data if available
            input.ping_ms = Some(recent_metrics.average_latency_ms() as u32);
            input.error_rate = Some(1.0 - recent_metrics.success_rate());
        }

        self.base_calculator.calculate(&input)
    }
}

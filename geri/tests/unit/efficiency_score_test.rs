//! Tests für Efficiency-Score-Calculator (Phase 7.1.1).

#[cfg(test)]
mod tests {
    use geri::selection::{EfficiencyInput, EfficiencyScoreCalculator};

    fn full_input() -> EfficiencyInput {
        EfficiencyInput {
            parameter_count: Some(8_000_000_000),
            max_parameter_count: 70_000_000_000,
            hardware_score: 1.0,
            uptime_percentage: Some(99.0),
            error_rate: Some(0.01),
            ping_ms: Some(50),
            max_ping_ms: 1000,
            distance_km: Some(0.0),
            max_distance_km: 10_000.0,
            is_local: true,
            cost_per_token: Some(0.000_01),
            max_cost_per_token: 0.001,
        }
    }

    #[test]
    fn calculate_returns_value_between_0_and_1() {
        let calc = EfficiencyScoreCalculator::default();
        let input = full_input();
        let score = calc.calculate(&input);
        assert!(score >= 0.0 && score <= 1.0);
    }

    #[test]
    fn model_size_score_higher_for_larger_model() {
        let calc = EfficiencyScoreCalculator::default();
        let mut input = full_input();
        input.parameter_count = Some(70_000_000_000);
        input.max_parameter_count = 70_000_000_000;
        let score_full = calc.calculate(&input);
        input.parameter_count = Some(1_000_000_000);
        let score_small = calc.calculate(&input);
        assert!(score_full > score_small);
    }

    #[test]
    fn hardware_score_contributes() {
        let calc = EfficiencyScoreCalculator::default();
        let mut input_high = full_input();
        input_high.hardware_score = 1.0;
        let mut input_low = full_input();
        input_low.hardware_score = 0.0;
        assert!(calc.calculate(&input_high) > calc.calculate(&input_low));
    }

    #[test]
    fn reliability_score_higher_for_high_uptime_low_error() {
        let calc = EfficiencyScoreCalculator::default();
        let mut input_good = full_input();
        input_good.uptime_percentage = Some(100.0);
        input_good.error_rate = Some(0.0);
        let mut input_bad = full_input();
        input_bad.uptime_percentage = Some(50.0);
        input_bad.error_rate = Some(0.5);
        assert!(calc.calculate(&input_good) > calc.calculate(&input_bad));
    }

    #[test]
    fn latency_score_higher_for_lower_ping() {
        let calc = EfficiencyScoreCalculator::default();
        let mut input_fast = full_input();
        input_fast.ping_ms = Some(10);
        let mut input_slow = full_input();
        input_slow.ping_ms = Some(900);
        assert!(calc.calculate(&input_fast) > calc.calculate(&input_slow));
    }

    #[test]
    fn distance_score_local_is_max() {
        let calc = EfficiencyScoreCalculator::default();
        let mut input_local = full_input();
        input_local.is_local = true;
        input_local.distance_km = Some(0.0);
        let mut input_remote = full_input();
        input_remote.is_local = false;
        input_remote.distance_km = Some(5000.0);
        input_remote.max_distance_km = 10_000.0;
        assert!(calc.calculate(&input_local) > calc.calculate(&input_remote));
    }

    #[test]
    fn cost_score_higher_for_lower_cost() {
        let calc = EfficiencyScoreCalculator::default();
        let mut input_cheap = full_input();
        input_cheap.cost_per_token = Some(0.000_001);
        let mut input_expensive = full_input();
        input_expensive.cost_per_token = Some(0.0005);
        assert!(calc.calculate(&input_cheap) > calc.calculate(&input_expensive));
    }

    #[test]
    fn calculate_with_missing_optionals_uses_defaults() {
        let calc = EfficiencyScoreCalculator::default();
        let input = EfficiencyInput {
            parameter_count: None,
            max_parameter_count: 1,
            hardware_score: 0.5,
            uptime_percentage: None,
            error_rate: None,
            ping_ms: None,
            max_ping_ms: 1000,
            distance_km: None,
            max_distance_km: 10_000.0,
            is_local: false,
            cost_per_token: None,
            max_cost_per_token: 1.0,
        };
        let score = calc.calculate(&input);
        assert!(score >= 0.0 && score <= 1.0);
    }

    #[test]
    fn weights_sum_to_one() {
        let calc = EfficiencyScoreCalculator::default();
        let w = calc.weights();
        let sum = w.model_size + w.hardware + w.reliability + w.latency + w.distance + w.cost;
        assert!((sum - 1.0).abs() < 0.001);
    }
}

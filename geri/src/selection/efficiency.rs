//! Efficiency-Score-Calculator (Phase 7.1.1): Multi-Faktor-Bewertung für Model-Auswahl.

/// Gewichtungen der sechs Faktoren (Summe = 1.0).
#[derive(Debug, Clone)]
pub struct EfficiencyWeights {
    pub model_size: f64,
    pub hardware: f64,
    pub reliability: f64,
    pub latency: f64,
    pub distance: f64,
    pub cost: f64,
}

impl Default for EfficiencyWeights {
    fn default() -> Self {
        Self {
            model_size: 0.20,
            hardware: 0.15,
            reliability: 0.20,
            latency: 0.25,
            distance: 0.10,
            cost: 0.10,
        }
    }
}

/// Eingabewerte für die Efficiency-Berechnung eines Models.
#[derive(Debug, Clone)]
pub struct EfficiencyInput {
    /// Parameter-Anzahl des Models (für Size-Score).
    pub parameter_count: Option<u64>,
    /// Max. Parameter-Anzahl zur Normalisierung (z. B. größtes verfügbares Model).
    pub max_parameter_count: u64,
    /// Hardware-Score 0.0–1.0 (z. B. GPU/Quality).
    pub hardware_score: f64,
    /// Uptime in Prozent (0–100).
    pub uptime_percentage: Option<f64>,
    /// Fehlerrate 0.0–1.0.
    pub error_rate: Option<f64>,
    /// Ping/Latency in ms.
    pub ping_ms: Option<u32>,
    /// Max. akzeptabler Ping (ms), z. B. 1000.
    pub max_ping_ms: u32,
    /// Entfernung in km (bei Remote).
    pub distance_km: Option<f64>,
    /// Max. Entfernung zur Normalisierung.
    pub max_distance_km: f64,
    /// True = lokales Model (Distance-Score = 1.0).
    pub is_local: bool,
    /// Kosten pro Token (z. B. Dollar).
    pub cost_per_token: Option<f64>,
    /// Max. Kosten pro Token zur Normalisierung.
    pub max_cost_per_token: f64,
}

/// Berechnet den Gesamt-Efficiency-Score aus den sechs gewichteten Teil-Scores.
#[derive(Debug, Clone, Default)]
pub struct EfficiencyScoreCalculator {
    weights: EfficiencyWeights,
}

impl EfficiencyScoreCalculator {
    /// Erstellt einen Calculator mit Standard-Gewichtungen.
    pub fn new(weights: EfficiencyWeights) -> Self {
        Self { weights }
    }

    /// Liefert die verwendeten Gewichtungen.
    pub fn weights(&self) -> &EfficiencyWeights {
        &self.weights
    }

    /// Berechnet den Gesamt-Score (0.0–1.0).
    pub fn calculate(&self, input: &EfficiencyInput) -> f64 {
        let size = self.model_size_score(input);
        let hw = self.hardware_score(input);
        let rel = self.reliability_score(input);
        let lat = self.latency_score(input);
        let dist = self.distance_score(input);
        let cost = self.cost_score(input);
        (size * self.weights.model_size)
            + (hw * self.weights.hardware)
            + (rel * self.weights.reliability)
            + (lat * self.weights.latency)
            + (dist * self.weights.distance)
            + (cost * self.weights.cost)
    }

    /// Model-Size-Score: parameter_count / max_parameter_count (fehlt = 0.5).
    fn model_size_score(&self, input: &EfficiencyInput) -> f64 {
        match (input.parameter_count, input.max_parameter_count) {
            (Some(p), max) if max > 0 => (p as f64 / max as f64).min(1.0),
            _ => 0.5,
        }
    }

    /// Hardware-Score 0.0–1.0 (geclippt).
    fn hardware_score(&self, input: &EfficiencyInput) -> f64 {
        input.hardware_score.clamp(0.0, 1.0)
    }

    /// Reliability: (uptime/100) * (1 - error_rate). Fehlt = 0.5.
    fn reliability_score(&self, input: &EfficiencyInput) -> f64 {
        let uptime = input.uptime_percentage.unwrap_or(50.0) / 100.0;
        let err = input.error_rate.unwrap_or(0.0).min(1.0);
        (uptime * (1.0 - err)).clamp(0.0, 1.0)
    }

    /// Latency-Score: 1.0 - (ping_ms / max_ping_ms). Fehlt = 0.5.
    fn latency_score(&self, input: &EfficiencyInput) -> f64 {
        match (input.ping_ms, input.max_ping_ms) {
            (Some(p), max) if max > 0 => (1.0 - (p as f64 / max as f64)).max(0.0),
            _ => 0.5,
        }
    }

    /// Distance-Score: lokal = 1.0, sonst 1.0 - (distance_km / max). Fehlt = 0.5.
    fn distance_score(&self, input: &EfficiencyInput) -> f64 {
        if input.is_local {
            return 1.0;
        }
        match (input.distance_km, input.max_distance_km) {
            (Some(d), max) if max > 0.0 => (1.0 - (d / max)).max(0.0),
            _ => 0.5,
        }
    }

    /// Cost-Score: 1.0 - (cost / max_cost). Fehlt = 0.5.
    fn cost_score(&self, input: &EfficiencyInput) -> f64 {
        match (input.cost_per_token, input.max_cost_per_token) {
            (Some(c), max) if max > 0.0 => (1.0 - (c / max)).max(0.0),
            _ => 0.5,
        }
    }
}

//! Weighted-Round-Robin Load-Balancer für Model-Auswahl.

use std::sync::atomic::{AtomicUsize, Ordering};

/// Gewichteter Round-Robin: wählt das nächste Model anhand von Gewichten.
#[derive(Debug)]
pub struct LoadBalancer {
    /// (model_name, weight) – weight = Anzahl Slots pro Runde
    models: Vec<(String, u32)>,
    total_slots: u32,
    counter: AtomicUsize,
}

impl LoadBalancer {
    /// Erstellt einen LoadBalancer aus Model-Namen und Gewichten.
    /// Gewicht 0 wird ignoriert.
    pub fn new(weighted_models: Vec<(String, u32)>) -> Self {
        let models: Vec<(String, u32)> = weighted_models
            .into_iter()
            .filter(|(_, w)| *w > 0)
            .collect();
        let total_slots: u32 = models.iter().map(|(_, w)| w).sum();
        Self {
            models,
            total_slots,
            counter: AtomicUsize::new(0),
        }
    }

    /// Nächstes Model nach Round-Robin (zyklisch).
    pub fn next(&self) -> Option<String> {
        if self.models.is_empty() || self.total_slots == 0 {
            return None;
        }
        let idx = self.counter.fetch_add(1, Ordering::Relaxed) as u32 % self.total_slots;
        let mut slot = 0u32;
        for (name, w) in &self.models {
            slot += *w;
            if idx < slot {
                return Some(name.clone());
            }
        }
        self.models.first().map(|(n, _)| n.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_returns_none() {
        let lb = LoadBalancer::new(vec![]);
        assert!(lb.next().is_none());
    }

    #[test]
    fn test_single_model_always_returns_it() {
        let lb = LoadBalancer::new(vec![("gpt-4".to_string(), 1)]);
        assert_eq!(lb.next().as_deref(), Some("gpt-4"));
        assert_eq!(lb.next().as_deref(), Some("gpt-4"));
    }

    #[test]
    fn test_weighted_round_robin() {
        let lb = LoadBalancer::new(vec![
            ("a".to_string(), 1),
            ("b".to_string(), 2),
        ]);
        // total_slots=3: a once, b twice per cycle
        let mut seen = vec![];
        for _ in 0..6 {
            seen.push(lb.next().unwrap());
        }
        let a_count = seen.iter().filter(|s| *s == "a").count();
        let b_count = seen.iter().filter(|s| *s == "b").count();
        assert_eq!(a_count, 2);
        assert_eq!(b_count, 4);
    }
}

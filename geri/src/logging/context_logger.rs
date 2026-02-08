//! Context-Logger (Phase 18.2.1): Request-Context in Logs, Trace-IDs.

use std::collections::HashMap;

/// Hält Trace-ID und optionale Request-Kontext-Felder für strukturierte Logs.
#[derive(Debug, Clone)]
pub struct ContextLogger {
    trace_id: String,
    context: HashMap<String, String>,
}

impl ContextLogger {
    /// Erstellt einen Context-Logger mit der angegebenen Trace-ID (für Request-Tracking).
    pub fn new(trace_id: String) -> Self {
        Self {
            trace_id,
            context: HashMap::new(),
        }
    }

    /// Liefert die Trace-ID.
    pub fn trace_id(&self) -> &str {
        &self.trace_id
    }

    /// Fügt ein Kontext-Feld hinzu (z. B. model_id, request_type).
    pub fn add_field(&mut self, key: &str, value: &str) {
        self.context.insert(key.to_string(), value.to_string());
    }

    /// Erzeugt eine strukturierte Log-Zeile (trace_id=… key=value …) für Logging/Tracing.
    pub fn to_log_string(&self) -> String {
        let mut parts = vec![format!("trace_id={}", self.trace_id)];
        let mut keys: Vec<_> = self.context.keys().collect();
        keys.sort();
        for k in keys {
            if let Some(v) = self.context.get(k) {
                parts.push(format!("{}={}", k, v));
            }
        }
        parts.join(" ")
    }
}

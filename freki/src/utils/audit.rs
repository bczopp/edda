//! Audit Logger (Phase 15.1.2): Document-Indexing-, Document-Access-, Query-Events für Compliance/Traceability.

use std::sync::Arc;
use tracing::info;

/// Sink für Audit-Events (Default: Tracing; Tests: Recording).
pub trait AuditSink: Send + Sync {
    fn log_document_indexed(&self, document_id: &str);
    fn log_document_accessed(&self, document_id: &str);
    fn log_query(&self, request_id: &str, limit: u32);
}

/// Schreibt Audit-Events ins Tracing (target "audit") für Filterung in Log-Aggregation.
pub struct TracingAuditSink;

impl AuditSink for TracingAuditSink {
    fn log_document_indexed(&self, document_id: &str) {
        info!(target: "audit", event = "document_indexed", document_id = %document_id);
    }
    fn log_document_accessed(&self, document_id: &str) {
        info!(target: "audit", event = "document_accessed", document_id = %document_id);
    }
    fn log_query(&self, request_id: &str, limit: u32) {
        info!(target: "audit", event = "query", request_id = %request_id, limit = limit);
    }
}

/// Audit-Logger: leitet Events an einen konfigurierbaren Sink (z. B. Tracing).
pub struct AuditLogger {
    sink: Arc<dyn AuditSink>,
}

impl AuditLogger {
    pub fn new(sink: Arc<dyn AuditSink>) -> Self {
        Self { sink }
    }

    /// Erstellt einen Logger mit Standard-Tracing-Sink.
    pub fn with_tracing() -> Self {
        Self::new(Arc::new(TracingAuditSink))
    }

    pub fn log_document_indexed(&self, document_id: &str) {
        self.sink.log_document_indexed(document_id);
    }

    pub fn log_document_accessed(&self, document_id: &str) {
        self.sink.log_document_accessed(document_id);
    }

    pub fn log_query(&self, request_id: &str, limit: u32) {
        self.sink.log_query(request_id, limit);
    }
}

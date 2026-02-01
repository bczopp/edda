#[cfg(test)]
pub mod utils;

#[cfg(test)]
mod unit {
    pub mod embedding_test;
    pub mod chunking_test;
    pub mod auto_indexing_test;
    pub mod batch_indexing_test;
    pub mod change_detector_test;
    pub mod full_reindex_test;
    pub mod incremental_update_test;
    pub mod watch_folder_test;
    pub mod query_embedding_test;
    pub mod document_ranker_test;
    pub mod context_extractor_test;
    pub mod context_formatter_test;
    pub mod indexing_error_handler_test;
    pub mod retrieval_error_handler_test;
    pub mod connection_retry_test;
    pub mod metrics_collector_test;
    pub mod request_validator_test;
    pub mod data_deletion_test;
    pub mod data_export_test;
    pub mod test_generators_test;
    pub mod audit_logger_test;
    pub mod performance_alert_test;
    pub mod parallel_indexing_perf_test;
    pub mod gdpr_compliance_test;
    pub mod security_test_suite;
}

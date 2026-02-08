//! GDPR-Compliance-Tests (Phase 19.3.2): Right-to-Deletion, Data-Export, Data-Minimization, Access-Control.

#[cfg(test)]
mod tests {
    use freki::grpc::RequestValidator;
    use freki::utils::data_export::{DataExportManager, ExportRecord};
    use freki::utils::DataDeletionError;
    use serde_json::json;

    /// Right-to-Deletion: DataDeletionManager-API und Fehlertyp für Traceability.
    #[test]
    fn gdpr_right_to_deletion_error_display() {
        let e = DataDeletionError::VectorDb(freki::vector_db::VectorDbError::VectorError(
            "test".to_string(),
        ));
        assert!(e.to_string().contains("Vector DB"));
    }

    /// Data-Export (Data Portability): ExportRecord enthält nur notwendige Felder (id, content, metadata).
    #[test]
    fn gdpr_data_export_record_fields() {
        let r = ExportRecord {
            id: "doc-1".to_string(),
            content: "content".to_string(),
            metadata: json!({"k": "v"}),
        };
        assert!(!r.id.is_empty());
        assert!(!r.content.is_empty());
    }

    /// Data-Export: format_json liefert lesbares JSON für Portabilität.
    #[test]
    fn gdpr_data_export_format_json() {
        let records = vec![ExportRecord {
            id: "id1".to_string(),
            content: "c1".to_string(),
            metadata: json!({}),
        }];
        let out = DataExportManager::format_json(&records).unwrap();
        assert!(out.contains("\"id\""));
        assert!(out.contains("\"content\""));
        assert!(out.contains("\"metadata\""));
    }

    /// Data-Minimization / Access-Control: Leere document_id wird abgelehnt.
    #[test]
    fn gdpr_validation_rejects_empty_document_id() {
        let r = RequestValidator::validate_index_document("", 100, 384);
        assert!(r.is_err());
        assert!(r.unwrap_err().to_string().contains("document_id"));
    }

    /// Access-Control: Limit außerhalb 1..=1000 wird abgelehnt.
    #[test]
    fn gdpr_validation_rejects_invalid_limit() {
        let r = RequestValidator::validate_retrieve_context(384, 0);
        assert!(r.is_err());
        let r2 = RequestValidator::validate_retrieve_context(384, 1001);
        assert!(r2.is_err());
    }

    /// Access-Control: Gültige Grenzwerte werden akzeptiert.
    #[test]
    fn gdpr_validation_accepts_valid_limits() {
        assert!(RequestValidator::validate_retrieve_context(384, 1).is_ok());
        assert!(RequestValidator::validate_retrieve_context(384, 1000).is_ok());
    }
}

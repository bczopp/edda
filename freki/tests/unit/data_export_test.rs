//! Unit tests for DataExportManager and DataExportError (Phase 17.2.1).

#[cfg(test)]
mod tests {
    use freki::utils::data_export::{DataExportError, DataExportManager, ExportRecord};
    use freki::vector_db::VectorDbError;
    use serde_json::json;

    #[test]
    fn test_data_export_error_display() {
        let e = DataExportError::VectorDb(VectorDbError::VectorError("test".to_string()));
        assert!(e.to_string().contains("Vector DB"));
    }

    #[test]
    fn test_data_export_error_from_vector_db_error() {
        let v = VectorDbError::ConnectionError("conn".to_string());
        let e: DataExportError = v.into();
        assert!(e.to_string().contains("conn"));
    }

    #[test]
    fn test_data_export_error_serialization() {
        let e = DataExportError::Serialization("invalid".to_string());
        assert!(e.to_string().contains("Serialization"));
    }

    #[test]
    fn test_format_json_empty() {
        let records: Vec<ExportRecord> = vec![];
        let out = DataExportManager::format_json(&records).unwrap();
        assert_eq!(out, "[]");
    }

    #[test]
    fn test_format_json_single() {
        let records = vec![ExportRecord {
            id: "doc-1".to_string(),
            content: "hello".to_string(),
            metadata: json!({"k": "v"}),
        }];
        let out = DataExportManager::format_json(&records).unwrap();
        assert!(out.contains("\"id\": \"doc-1\""));
        assert!(out.contains("\"content\": \"hello\""));
        assert!(out.contains("\"metadata\""));
    }

    #[test]
    fn test_format_csv_empty() {
        let records: Vec<ExportRecord> = vec![];
        let out = DataExportManager::format_csv(&records).unwrap();
        assert_eq!(out, "id,content,metadata\n");
    }

    #[test]
    fn test_format_csv_single() {
        let records = vec![ExportRecord {
            id: "doc-1".to_string(),
            content: "hello, world".to_string(),
            metadata: json!({}),
        }];
        let out = DataExportManager::format_csv(&records).unwrap();
        assert!(out.starts_with("id,content,metadata\n"));
        assert!(out.contains("\"doc-1\""));
        assert!(out.contains("\"hello, world\""));
    }

    #[test]
    fn test_format_csv_escapes_quotes() {
        let records = vec![ExportRecord {
            id: "id".to_string(),
            content: "say \"hi\"".to_string(),
            metadata: json!(null),
        }];
        let out = DataExportManager::format_csv(&records).unwrap();
        assert!(out.contains("\"say \"\"hi\"\"\""));
    }
}

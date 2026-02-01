//! Data-Export-Manager (Phase 17.2.1): Indizierte Dokumente/Metadaten exportieren (JSON, CSV).

use crate::vector_db::VectorDbClient;
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DataExportError {
    #[error("Vector DB error: {0}")]
    VectorDb(#[from] crate::vector_db::VectorDbError),
    #[error("Serialization error: {0}")]
    Serialization(String),
}

/// Ein Eintrag für den Export (ein indizierter Chunk/Punkt).
#[derive(Debug, Clone, Serialize)]
pub struct ExportRecord {
    pub id: String,
    pub content: String,
    pub metadata: serde_json::Value,
}

/// Exportiert indizierte Dokumente als JSON oder CSV (GDPR Data Portability).
pub struct DataExportManager {
    vector_db: VectorDbClient,
    collection_name: String,
}

impl DataExportManager {
    pub fn new(vector_db: VectorDbClient, collection_name: String) -> Self {
        Self {
            vector_db,
            collection_name,
        }
    }

    /// Export als JSON-Array (Indent für Lesbarkeit).
    pub fn format_json(records: &[ExportRecord]) -> Result<String, DataExportError> {
        serde_json::to_string_pretty(records).map_err(|e| DataExportError::Serialization(e.to_string()))
    }

    /// Export als CSV (id, content, metadata als JSON-String); Felder in Anführungszeichen, " escaped als "".
    pub fn format_csv(records: &[ExportRecord]) -> Result<String, DataExportError> {
        fn escape(s: &str) -> String {
            format!("\"{}\"", s.replace('"', "\"\""))
        }
        let mut out = String::from("id,content,metadata\n");
        for r in records {
            out.push_str(&escape(&r.id));
            out.push(',');
            out.push_str(&escape(&r.content));
            out.push(',');
            out.push_str(&escape(&r.metadata.to_string()));
            out.push('\n');
        }
        Ok(out)
    }

    /// Holt Records aus der Collection (scroll) und exportiert als JSON.
    pub async fn export_json(&self, max_records: Option<u32>) -> Result<String, DataExportError> {
        let raw = self
            .vector_db
            .scroll_all(&self.collection_name, max_records.unwrap_or(10_000))
            .await?;
        let records: Vec<ExportRecord> = raw
            .into_iter()
            .map(|(id, content, meta_str)| ExportRecord {
                id,
                content,
                metadata: serde_json::from_str(&meta_str).unwrap_or(serde_json::Value::Null),
            })
            .collect();
        Self::format_json(&records)
    }

    /// Holt Records aus der Collection (scroll) und exportiert als CSV.
    pub async fn export_csv(&self, max_records: Option<u32>) -> Result<String, DataExportError> {
        let raw = self
            .vector_db
            .scroll_all(&self.collection_name, max_records.unwrap_or(10_000))
            .await?;
        let records: Vec<ExportRecord> = raw
            .into_iter()
            .map(|(id, content, meta_str)| ExportRecord {
                id,
                content,
                metadata: serde_json::from_str(&meta_str).unwrap_or(serde_json::Value::Null),
            })
            .collect();
        Self::format_csv(&records)
    }
}

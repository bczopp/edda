# Freki gRPC API Documentation

## Übersicht

Freki stellt einen gRPC-Service bereit für Document-Indexing und Context-Retrieval (RAG). Der Service nutzt Qdrant als Vector-Database und unterstützt Embedding-basierte Ähnlichkeitssuche.

**Service-Name**: `freki.FrekiService`  
**Standard-Port**: 50053 (konfigurierbar via `freki.json`)

## Service-Definition

```protobuf
service FrekiService {
    rpc IndexDocument(IndexDocumentRequest) returns (IndexDocumentResponse);
    rpc RetrieveContext(RetrieveContextRequest) returns (RetrieveContextResponse);
}
```

---

## RPCs

### IndexDocument

Indiziert ein Dokument mit Embedding in der Vector-Database.

**Request**: `IndexDocumentRequest`

| Feld | Typ | Beschreibung | Validierung |
|------|-----|--------------|-------------|
| `document_id` | `string` | Eindeutige Dokument-ID | Nicht leer, max 255 Zeichen |
| `content` | `string` | Dokumentinhalt | Max 10 MB |
| `embedding` | `bytes` | Vector-Embedding (bincode-serialisiert `Vec<f32>`) | Nicht leer, Dimension muss mit Collection übereinstimmen |
| `metadata` | `map<string, string>` | Optionale Metadaten | - |

**Response**: `IndexDocumentResponse`

| Feld | Typ | Beschreibung |
|------|-----|--------------|
| `success` | `bool` | `true` bei Erfolg |
| `message` | `string` | Erfolgs- oder Fehlermeldung |

**Workflow**:
1. Request-Validierung (document_id, content-Größe, embedding)
2. Embedding deserialisieren (bincode)
3. Document erstellen (id, content, metadata)
4. In Vector-Database indizieren (Upsert)
5. Audit-Log: `log_document_indexed(document_id)`

**Error-Codes**:
- `INVALID_ARGUMENT` (3): Ungültiges Embedding-Format, leere document_id, content zu groß
- `INTERNAL` (13): Fehler beim Indexieren (Vector-DB-Fehler)

**Beispiel** (Rust mit tonic):

```rust
use freki::freki::freki_service_client::FrekiServiceClient;
use freki::freki::{IndexDocumentRequest, IndexDocumentResponse};
use bincode;

let mut client = FrekiServiceClient::connect("http://localhost:50053").await?;

let embedding: Vec<f32> = vec![0.1; 384]; // Beispiel: 384-Dimension
let embedding_bytes = bincode::serialize(&embedding)?;

let request = tonic::Request::new(IndexDocumentRequest {
    document_id: "doc-123".to_string(),
    content: "This is sample content.".to_string(),
    embedding: embedding_bytes,
    metadata: [("source".to_string(), "example".to_string())].into_iter().collect(),
});

let response: tonic::Response<IndexDocumentResponse> = client.index_document(request).await?;
println!("Success: {}", response.into_inner().success);
```

---

### RetrieveContext

Ruft relevante Dokumente basierend auf Query-Embedding ab (RAG-Context).

**Request**: `RetrieveContextRequest`

| Feld | Typ | Beschreibung | Validierung |
|------|-----|--------------|-------------|
| `query_embedding` | `bytes` | Query-Embedding (bincode-serialisiert `Vec<f32>`) | Nicht leer, Dimension muss mit Collection übereinstimmen |
| `limit` | `uint64` | Maximale Anzahl zurückzugebender Dokumente | 1..=1000 |
| `collection_name` | `string` | Collection-Name (optional, verwendet Default wenn leer) | - |

**Response**: `RetrieveContextResponse`

| Feld | Typ | Beschreibung |
|------|-----|--------------|
| `documents` | `repeated RetrievedDocument` | Relevante Dokumente (sortiert nach Score, absteigend) |
| `relevance_scores` | `repeated float` | Relevanz-Scores (parallel zu `documents`) |

**RetrievedDocument**:

| Feld | Typ | Beschreibung |
|------|-----|--------------|
| `id` | `string` | Dokument-ID |
| `content` | `string` | Dokumentinhalt (Chunk-Text) |
| `metadata` | `map<string, string>` | Metadaten |
| `score` | `float` | Ähnlichkeits-Score (0.0-1.0, höher = relevanter) |

**Workflow**:
1. Request-Validierung (query_embedding, limit)
2. Query-Embedding deserialisieren (bincode)
3. Vector-Search in Qdrant (Cosine-Similarity)
4. Dokumente nach Score ranken
5. Top-K Dokumente zurückgeben
6. Audit-Log: `log_query(request_id, limit)`, `log_document_accessed(doc_id)` pro Dokument

**Error-Codes**:
- `INVALID_ARGUMENT` (3): Ungültiges Query-Embedding-Format, limit außerhalb 1..=1000
- `INTERNAL` (13): Fehler beim Retrieval (Vector-DB-Fehler)

**Beispiel** (Rust mit tonic):

```rust
use freki::freki::freki_service_client::FrekiServiceClient;
use freki::freki::{RetrieveContextRequest, RetrieveContextResponse};
use bincode;

let mut client = FrekiServiceClient::connect("http://localhost:50053").await?;

let query_embedding: Vec<f32> = vec![0.2; 384];
let query_bytes = bincode::serialize(&query_embedding)?;

let request = tonic::Request::new(RetrieveContextRequest {
    query_embedding: query_bytes,
    limit: 10,
    collection_name: "".to_string(), // Verwendet Default-Collection
});

let response: tonic::Response<RetrieveContextResponse> = client.retrieve_context(request).await?;
let ctx = response.into_inner();

for (doc, score) in ctx.documents.iter().zip(ctx.relevance_scores.iter()) {
    println!("Doc {}: {} (score: {})", doc.id, doc.content, score);
}
```

---

## Error-Handling

### gRPC Status-Codes

| Code | Name | Verwendung |
|------|------|------------|
| `OK` (0) | Success | Request erfolgreich verarbeitet |
| `INVALID_ARGUMENT` (3) | Invalid Argument | Ungültige Request-Parameter (leere IDs, falsches Embedding-Format, ungültiges limit) |
| `INTERNAL` (13) | Internal Error | Server-seitiger Fehler (Vector-DB-Fehler, Indexing-Fehler) |

### Fehlerbehandlung im Client

```rust
match client.index_document(request).await {
    Ok(response) => {
        // Erfolg
    }
    Err(status) => {
        match status.code() {
            tonic::Code::InvalidArgument => {
                eprintln!("Invalid request: {}", status.message());
            }
            tonic::Code::Internal => {
                eprintln!("Server error: {}", status.message());
            }
            _ => {
                eprintln!("Unexpected error: {}", status);
            }
        }
    }
}
```

---

## Request-Workflows

### Vollständiger RAG-Workflow

1. **Dokumente indizieren**:
   ```rust
   // Für jedes Dokument:
   // 1. Embedding generieren (z. B. via Embedding-Model)
   let embedding = embedding_model.embed_text(&document_content).await?;
   // 2. Via Freki indizieren
   client.index_document(IndexDocumentRequest { ... }).await?;
   ```

2. **Query-Embedding generieren**:
   ```rust
   // Query-Text zu Embedding konvertieren (gleiches Modell wie für Dokumente)
   let query_embedding = embedding_model.embed_text(&user_query).await?;
   ```

3. **Context abrufen**:
   ```rust
   let context = client.retrieve_context(RetrieveContextRequest {
       query_embedding: bincode::serialize(&query_embedding)?,
       limit: 10,
       collection_name: "".to_string(),
   }).await?;
   ```

4. **Context für LLM verwenden**:
   ```rust
   let context_text: String = context.documents
       .iter()
       .map(|d| format!("[{}] {}", d.id, d.content))
       .collect::<Vec<_>>()
       .join("\n\n");
   // An LLM (z. B. Geri) weitergeben
   ```

---

## Performance & Limits

- **Embedding-Dimension**: Muss mit Collection-Dimension übereinstimmen (Standard: 384 für all-MiniLM-L6-v2)
- **Content-Größe**: Max 10 MB pro Dokument
- **Limit**: 1-1000 Dokumente pro Retrieve-Request
- **Latenz-Ziele**:
  - IndexDocument: < 1s pro Dokument
  - RetrieveContext: < 100ms (Production), ≤ 150ms (CI)

---

## Audit-Logging

Alle Operationen werden geloggt:
- **IndexDocument**: `log_document_indexed(document_id)`
- **RetrieveContext**: `log_query(request_id, limit)`, `log_document_accessed(doc_id)` pro zurückgegebenem Dokument

Logs sind über `tracing` mit Target `audit` verfügbar (siehe `src/utils/audit.rs`).

---

## Integration mit Odin

Odin nutzt Freki für RAG-Context-Anreicherung:

1. Odin empfängt User-Request
2. Odin generiert Query-Embedding (via Embedding-Model)
3. Odin ruft `FrekiService::RetrieveContext` auf
4. Freki liefert relevante Dokumente
5. Odin formatiert Context und gibt an Geri (LLM) weiter

Siehe auch: `docs/rust-services-overview.md` (Odin ↔ Freki Integration).

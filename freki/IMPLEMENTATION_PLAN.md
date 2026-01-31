# IMPLEMENTATION_PLAN - Freki (RAG Service)

## Übersicht

Dieser Plan beschreibt die kleinstmöglichen Schritte zur Implementierung von Freki - dem RAG (Retrieval Augmented Generation) Service. Freki verwaltet Vector Database, Document Indexing, Context Retrieval und Prompt Enrichment.

**Mythologische Bedeutung**: Freki ist einer von Odins Wölfen.

**Programmiersprache**: Rust

## Entschiedene Konfiguration

### Vector-Database-Wahl
✅ **ENTSCHEIDUNG**: Qdrant
**Begründung**: Rust-native, lokal, beste Performance, production-ready, robuste Features

### Embedding-Model-Standard
✅ **ENTSCHEIDUNG**: all-MiniLM-L6-v2
**Begründung**: Sentence Transformers, bester Kompromiss zwischen Quality und Performance, bewährt

### Document-Storage
✅ **ENTSCHEIDUNG**: Vector-Database
**Begründung**: Integriert, weniger Verwaltung, konsistente Architektur

### Chunking-Bibliothek
✅ **ENTSCHEIDUNG**: tiktoken
**Begründung**: Standard-Tool, zuverlässig, bewährt

### Cache-Storage
✅ **ENTSCHEIDUNG**: Redis
**Begründung**: Schnell, persistent, production-ready, separate Service ermöglicht Skalierung

---

## Phase 1: Projekt-Setup & Grundstruktur

### 1.1 Projekt-Initialisierung

**Abhängigkeiten**: Keine
**Erforderliche USER-Eingaben**: Vector-Database-Wahl

#### 1.1.1 Cargo-Projekt erstellen
- [ ] `Cargo.toml` erstellen
- [ ] Basis-Dependencies definieren
  - Async Runtime (tokio)
  - gRPC (tonic, prost)
  - Serialization (serde, serde_json)
  - Logging (tracing, tracing-subscriber)
  - Error-Handling (anyhow, thiserror)
  - Vector-Database-Client (z.B. qdrant-client)
- [ ] `.gitignore` erstellen

#### 1.1.2 Verzeichnisstruktur erstellen
- [ ] `src/main.rs` erstellen
- [ ] `src/lib.rs` erstellen
- [ ] `src/vector_db/` für Vector-Database-Integration erstellen
- [ ] `src/embedding/` für Embedding-Generation erstellen
- [ ] `src/indexing/` für Document-Indexing erstellen
- [ ] `src/retrieval/` für Context-Retrieval erstellen
- [ ] `src/chunking/` für Document-Chunking erstellen
- [ ] `src/grpc/` für gRPC-Service erstellen
- [ ] `src/utils/` für Utilities erstellen
- [ ] `config/` für Konfigurationsdateien erstellen
- [ ] `tests/` für Tests erstellen

#### 1.1.3 Build-System einrichten
- [ ] Build-Scripts in `Cargo.toml` definieren
- [ ] Code-Generierungs-Pipeline einrichten (Protobuf → Rust)
- [ ] Cargo-Features definieren (z.B. `qdrant`, `chroma`, `pinecone`)

### 1.2 Test-Infrastruktur

**Abhängigkeiten**: 1.1 (Projekt-Initialisierung)

#### 1.2.1 Container-Setup für Tests
- [ ] `Dockerfile` für Test-Umgebung erstellen
- [ ] Docker Compose für Test-Services konfigurieren
  - Vector-Database-Container (Qdrant/Chroma/etc.)
  - Mock-Odin-Service
  - Embedding-Model-Container (optional)
  - Redis-Container (falls Redis-Cache gewählt)
- [ ] Test-Container-Startup-Scripts erstellen
- [ ] **WICHTIG**: Alle Tests müssen in Containern laufen - keine lokalen Dependencies, Tools oder Services auf der Entwicklungsmaschine installieren

#### 1.2.2 Test-Framework konfigurieren
- [ ] Test-Dependencies hinzufügen (tokio-test, mockall, etc.)
- [ ] Test-Utilities und Helpers erstellen
- [ ] Mock-Setup für Odin und Geri
- [ ] Test-Document-Generators erstellen
- [ ] Test-Embedding-Generators erstellen

#### 1.2.3 CI/CD-Pipeline
- [ ] GitHub Actions / GitLab CI Workflow erstellen
- [ ] Automatische Test-Ausführung bei Commits konfigurieren
- [ ] Code-Coverage-Reporting einrichten (cargo-tarpaulin)
- [ ] Linting und Formatting (cargo clippy, cargo fmt)

### 1.3 Projekt-Konfiguration

**Abhängigkeiten**: 1.1 (Projekt-Initialisierung)

#### 1.3.1 Settings-System Design
- [ ] Settings-Schema definieren (JSON oder TOML)
- [ ] Settings-Struktur entwerfen (Freki-spezifisch)
  - gRPC-Port
  - Vector-Database-Konfiguration
  - Embedding-Model-Settings
  - Chunking-Settings
  - Watch-Folder-Settings
  - Cache-Settings (falls Caching)

#### 1.3.2 Settings-Validierung
- [ ] Rust-Structs für Settings definieren
- [ ] Tests für Settings-Validierung schreiben
- [ ] Settings-Validator implementieren (TDD)
  - Schema-Validierung
  - Range-Checks
  - Format-Validierung
- [ ] Tests ausführen und bestehen

#### 1.3.3 Settings-Loader
- [ ] Tests für Settings-Loader schreiben
- [ ] Settings-Loader implementieren (TDD)
  - JSON/TOML Parsing
  - Environment-Variable-Override
  - Default-Settings
- [ ] Hot-Reload-Mechanismus implementieren (TDD)
  - File-Watcher für Settings-Datei
  - Settings-Reload ohne Service-Restart
- [ ] Tests ausführen und bestehen

---

## Phase 2: Protobuf & gRPC Setup

### 2.1 Protobuf Definitions

**Abhängigkeiten**: 1.1 (Projekt-Initialisierung)

#### 2.1.1 Shared Protobuf-Projekt verwenden
❓ **HINWEIS**: Protobuf-Definitions sollten in einem separaten Projekt sein (z.B. `edda-protocols`)
- [ ] Freki als Dependency zu Protobuf-Projekt hinzufügen
- [ ] Protobuf-Definitions importieren

#### 2.1.2 WolfRequest/WolfResponse Protocol
- [ ] `WolfRequest.proto` definieren (falls nicht vorhanden)
  - ModelType enum (RAG, LLM)
  - RequestMetadata
- [ ] `WolfResponse.proto` definieren (falls nicht vorhanden)
  - RAGContext
  - RetrievedDocument
  - ResponseMetadata
- [ ] Code-Generierung konfigurieren

### 2.2 gRPC Server Implementation

**Abhängigkeiten**: 2.1 (Protobuf Definitions)

#### 2.2.1 gRPC Server Setup
- [ ] Tests für gRPC-Server-Setup schreiben
- [ ] gRPC-Server-Setup implementieren (TDD)
  - tonic-Server konfigurieren
  - Port-Konfiguration
  - Health-Check-Service
- [ ] Tests ausführen und bestehen

#### 2.2.2 Wolf-Service Implementation
- [ ] Tests für Wolf-Service schreiben
- [ ] `WolfServiceImpl` implementieren (TDD)
  - `ProcessRequest` RPC implementieren (empfängt WolfRequest, gibt WolfResponse zurück)
  - Request-Validation
  - Error-Handling (gRPC Status-Codes)
- [ ] Tests ausführen und bestehen

---

## Phase 3: Vector-Database-Integration

### 3.1 Vector-Database-Client

**Abhängigkeiten**: 1.1 (Projekt-Initialisierung)
**Erforderliche USER-Eingaben**: Vector-Database-Wahl

#### 3.1.1 Database-Client-Interface
- [ ] Tests für Database-Interface schreiben
- [ ] `VectorDatabaseClient` Trait definieren
  - `insert_vectors()`
  - `search_vectors()`
  - `update_vectors()`
  - `delete_vectors()`
  - `create_collection()`
  - `delete_collection()`
- [ ] Tests ausführen und bestehen

#### 3.1.2 Qdrant Client (Option A - empfohlen)
❓ **HINWEIS**: Nur wenn Qdrant gewählt wurde
- [ ] Tests für Qdrant-Client schreiben
- [ ] `QdrantClient` implementieren (TDD)
  - Connection zu Qdrant
  - Collection-Management
  - Vector-Insert/Search/Update/Delete
- [ ] Tests ausführen und bestehen

#### 3.1.3 Alternative Database Clients (Option B-E)
❓ **HINWEIS**: Je nach gewählter Database
- [ ] Tests für gewählten Client schreiben
- [ ] Client implementieren (TDD)
- [ ] Tests ausführen und bestehen

### 3.2 Collection Management

**Abhängigkeiten**: 3.1 (Vector-Database-Client)

#### 3.2.1 Collection Manager
- [ ] Tests für Collection-Manager schreiben
- [ ] `CollectionManager` implementieren (TDD)
  - Collections erstellen
  - Collections auflisten
  - Collections löschen
  - Collection-Configuration verwalten
- [ ] Tests ausführen und bestehen

---

## Phase 4: Embedding-Model-Integration

### 4.1 Embedding-Model-Interface

**Abhängigkeiten**: Keine
**Erforderliche USER-Eingaben**: Embedding-Model-Standard

#### 4.1.1 Embedding-Model-Trait
- [ ] Tests für Embedding-Model-Interface schreiben
- [ ] `EmbeddingModel` Trait definieren
  - `embed_text()`
  - `embed_batch()`
  - `get_model_name()`
  - `get_vector_dimension()`
- [ ] Tests ausführen und bestehen

### 4.2 Lokale Embedding-Models

**Abhängigkeiten**: 4.1 (Embedding-Model-Interface)

#### 4.2.1 Sentence-Transformers Integration
- [ ] Tests für Sentence-Transformers schreiben
- [ ] `SentenceTransformersModel` implementieren (TDD)
  - Model-Loading (z.B. all-MiniLM-L6-v2)
  - Text-Embedding
  - Batch-Embedding
- [ ] Tests ausführen und bestehen

#### 4.2.2 BGE-Models Integration (Optional)
- [ ] Tests für BGE-Models schreiben
- [ ] `BGEModel` implementieren (TDD)
- [ ] Tests ausführen und bestehen

### 4.3 Cloud Embedding-Models (Optional)

**Abhängigkeiten**: 4.1 (Embedding-Model-Interface)

#### 4.3.1 OpenAI Embeddings Integration
❓ **HINWEIS**: Nur wenn User API-Keys hinterlegt hat
- [ ] Tests für OpenAI-Embeddings schreiben
- [ ] `OpenAIEmbeddings` implementieren (TDD)
  - API-Client
  - Text-Embedding via API
- [ ] Tests ausführen und bestehen

### 4.4 Model-Registry

**Abhängigkeiten**: 4.2 (Lokale Models), 4.3 (Cloud Models - optional)

#### 4.4.1 Model-Registry Implementation
- [ ] Tests für Model-Registry schreiben
- [ ] `ModelRegistry` implementieren (TDD)
  - Verfügbare Models registrieren
  - Model-Selection (Standard, typ-spezifisch)
  - Model-Health-Check
- [ ] Tests ausführen und bestehen

---

## Phase 5: Document-Chunking

### 5.1 Chunking-Interface

**Abhängigkeiten**: Keine
**Erforderliche USER-Eingaben**: Chunking-Bibliothek-Wahl

#### 5.1.1 Chunking-Trait
- [ ] Tests für Chunking-Interface schreiben
- [ ] `DocumentChunker` Trait definieren
  - `chunk_document()`
  - `get_chunk_size()`
  - `get_overlap_size()`
- [ ] Tests ausführen und bestehen

### 5.2 Semantic-Chunking-Implementierung

**Abhängigkeiten**: 5.1 (Chunking-Interface)

#### 5.2.1 Sentence-Boundary-Detection
- [ ] Tests für Sentence-Boundary-Detection schreiben
- [ ] `SentenceBoundaryDetector` implementieren (TDD)
  - Dokument in Sätze aufteilen
  - Satzgrenzen erkennen
- [ ] Tests ausführen und bestehen

#### 5.2.2 Semantic-Similarity-Grouping
- [ ] Tests für Semantic-Similarity-Grouping schreiben
- [ ] `SemanticGrouper` implementieren (TDD)
  - Sätze nach semantischer Ähnlichkeit gruppieren
  - Semantic-Threshold anwenden
  - Chunks basierend auf Gruppen erstellen
- [ ] Tests ausführen und bestehen

#### 5.2.3 Max-Size-Constraint
- [ ] Tests für Max-Size-Constraint schreiben
- [ ] `MaxSizeEnforcer` implementieren (TDD)
  - Max-Größe als Constraint anwenden (1000 Tokens)
  - Token-Counting (tiktoken)
  - Chunks bei Überschreitung splitten
- [ ] Tests ausführen und bestehen

#### 5.2.4 Overlap-Implementierung
- [ ] Tests für Overlap schreiben
- [ ] `OverlapManager` implementieren (TDD)
  - Letzte N Tokens eines Chunks als erste Tokens des nächsten
  - Overlap-Berechnung (100 Tokens)
  - Overlap-Insertion
- [ ] Tests ausführen und bestehen

### 5.3 Semantic-Chunker

**Abhängigkeiten**: 5.2 (Semantic-Chunking-Implementierung)

#### 5.3.1 Complete Semantic-Chunker
- [ ] Tests für Complete-Semantic-Chunker schreiben
- [ ] `SemanticChunker` implementieren (TDD)
  - Alle Chunking-Komponenten kombinieren
  - Chunking-Parameter konfigurierbar
  - Chunking-Workflow orchestrieren
- [ ] Tests ausführen und bestehen

---

## Phase 6: Document-Indexing

### 6.1 Document-Parser

**Abhängigkeiten**: Keine

#### 6.1.1 Document-Parser-Interface
- [ ] Tests für Document-Parser-Interface schreiben
- [ ] `DocumentParser` Trait definieren
  - `parse_document()`
  - `supports_file_type()`
- [ ] Tests ausführen und bestehen

#### 6.1.2 File-Type-Specific Parsers
- [ ] Tests für Text-Parser schreiben (.txt, .md)
- [ ] `TextParser` implementieren (TDD)
- [ ] Tests ausführen und bestehen
- [ ] Tests für PDF-Parser schreiben (.pdf)
- [ ] `PDFParser` implementieren (TDD)
- [ ] Tests ausführen und bestehen
- [ ] Tests für DOCX-Parser schreiben (.docx)
- [ ] `DOCXParser` implementieren (TDD)
- [ ] Tests ausführen und bestehen
- [ ] (Optional) Weitere Parser nach Bedarf

### 6.2 Document-Metadata-Management

**Abhängigkeiten**: 6.1 (Document-Parser)

#### 6.2.1 Metadata-Extractor
- [ ] Tests für Metadata-Extractor schreiben
- [ ] `MetadataExtractor` implementieren (TDD)
  - Metadaten aus Dokumenten extrahieren
  - Standard-Metadaten (Title, Author, Created-Date, etc.)
  - Custom-Metadaten
- [ ] Tests ausführen und bestehen

### 6.3 Indexing-Pipeline

**Abhängigkeiten**: 6.1 (Document-Parser), 5.3 (Semantic-Chunker), 4.4 (Model-Registry), 3.2 (Collection Management)

#### 6.3.1 Indexing-Manager
- [ ] Tests für Indexing-Manager schreiben
- [ ] `IndexingManager` implementieren (TDD)
  - Dokument laden
  - Dokument parsen
  - Dokument chunken
  - Embeddings erstellen
  - Chunks in Vector-Database indizieren
- [ ] Tests ausführen und bestehen

#### 6.3.2 Batch-Indexing
- [ ] Tests für Batch-Indexing schreiben
- [ ] `BatchIndexingManager` implementieren (TDD)
  - Multiple Dokumente parallel indizieren
  - Batch-Size-Limits
  - Progress-Tracking
- [ ] Tests ausführen und bestehen

---

## Phase 7: Document-Updates

### 7.1 Change-Detection

**Abhängigkeiten**: 6.3 (Indexing-Pipeline)

#### 7.1.1 Document-Change-Detector
- [ ] Tests für Change-Detection schreiben
- [ ] `DocumentChangeDetector` implementieren (TDD)
  - Dokument-Hash berechnen
  - Änderungen erkennen
  - Geänderte Teile identifizieren
- [ ] Tests ausführen und bestehen

### 7.2 Incremental-Update

**Abhängigkeiten**: 7.1 (Change-Detection), 6.3 (Indexing-Pipeline)

#### 7.2.1 Incremental-Update-Manager
- [ ] Tests für Incremental-Update schreiben
- [ ] `IncrementalUpdateManager` implementieren (TDD)
  - Geänderte Chunks identifizieren
  - Nur geänderte Chunks re-indizieren
  - Embeddings für geänderte Chunks neu erstellen
  - Vector-Database selektiv aktualisieren
- [ ] Tests ausführen und bestehen

### 7.3 Full-Re-Indexing

**Abhängigkeiten**: 6.3 (Indexing-Pipeline)

#### 7.3.1 Full-Re-Indexing-Manager
- [ ] Tests für Full-Re-Indexing schreiben
- [ ] `FullReIndexingManager` implementieren (TDD)
  - Alte Chunks aus Vector-Database entfernen
  - Dokument vollständig neu chunken
  - Neue Embeddings erstellen
  - Neue Chunks indizieren
  - Background-Processing
- [ ] Tests ausführen und bestehen

---

## Phase 8: Watch-Folder-Funktionalität

### 8.1 File-System-Watcher

**Abhängigkeiten**: 6.3 (Indexing-Pipeline), 7.2 (Incremental-Update), 7.3 (Full-Re-Indexing)

#### 8.1.1 Watch-Folder-Manager
- [ ] Tests für Watch-Folder schreiben
- [ ] `WatchFolderManager` implementieren (TDD)
  - Folder überwachen
  - Neue Dateien erkennen
  - Geänderte Dateien erkennen
  - Gelöschte Dateien erkennen
  - Event-Handling
- [ ] Tests ausführen und bestehen

#### 8.1.2 Auto-Indexing-Manager
- [ ] Tests für Auto-Indexing schreiben
- [ ] `AutoIndexingManager` implementieren (TDD)
  - Neue Dateien automatisch indizieren
  - Geänderte Dateien re-indizieren (Incremental oder Full)
  - Gelöschte Dateien aus Index entfernen
- [ ] Tests ausführen und bestehen

---

## Phase 9: Context-Retrieval

### 9.1 Vector-Search

**Abhängigkeiten**: 3.1 (Vector-Database-Client), 4.4 (Model-Registry)

#### 9.1.1 Query-Embedding-Generator
- [ ] Tests für Query-Embedding schreiben
- [ ] `QueryEmbeddingGenerator` implementieren (TDD)
  - Query-Text zu Embedding konvertieren
  - Gleiche Model wie für Dokumente verwenden
- [ ] Tests ausführen und bestehen

#### 9.1.2 Similarity-Search-Manager
- [ ] Tests für Similarity-Search schreiben
- [ ] `SimilaritySearchManager` implementieren (TDD)
  - Vector-Search durchführen (Cosine, Dot Product, Euclidean)
  - Top-K Retrieval
  - Threshold-Filtering
- [ ] Tests ausführen und bestehen

### 9.2 Document-Ranking

**Abhängigkeiten**: 9.1 (Vector-Search)

#### 9.2.1 Document-Ranker
- [ ] Tests für Document-Ranking schreiben
- [ ] `DocumentRanker` implementieren (TDD)
  - Dokumente nach Relevanz-Score ranken
  - Threshold anwenden
  - Top-K filtern
- [ ] Tests ausführen und bestehen

### 9.3 Context-Extraction

**Abhängigkeiten**: 9.2 (Document-Ranking)

#### 9.3.1 Context-Extractor
- [ ] Tests für Context-Extraction schreiben
- [ ] `ContextExtractor` implementieren (TDD)
  - Relevante Text-Passagen extrahieren
  - Multiple Dokumente kombinieren
  - Kontext-Länge optimieren (optional, kann auch von Geri übernommen werden)
- [ ] Tests ausführen und bestehen

### 9.4 Context-Formatting

**Abhängigkeiten**: 9.3 (Context-Extraction)

#### 9.4.1 Context-Formatter
- [ ] Tests für Context-Formatting schreiben
- [ ] `ContextFormatter` implementieren (TDD)
  - Context für LLM formatieren
  - Strukturiertes Format (`[Document 1: document_id] content...`)
  - Metadaten beibehalten für Traceability
  - RAGContext erstellen
- [ ] Tests ausführen und bestehen

---

## Phase 10: Caching (Optional)

### 10.1 Cache-Implementierung

**Abhängigkeiten**: 4.2 (Embedding-Models), 9.1 (Vector-Search)
**Erforderliche USER-Eingaben**: Cache-Storage-Wahl

❓ **HINWEIS**: Diese Phase wird nur ausgeführt, wenn Caching gewählt wurde.

#### 10.1.1 Embedding-Cache
- [ ] Tests für Embedding-Cache schreiben
- [ ] `EmbeddingCache` implementieren (TDD)
  - Embeddings cachen (Chunk-ID → Embedding)
  - Cache-Key: `{document_id}_{chunk_id}_{model_name}_{model_version}`
  - Cache-Hit/Miss-Handling
  - Cache-Invalidation bei Dokument-Update
- [ ] Tests ausführen und bestehen

#### 10.1.2 Query-Result-Cache
- [ ] Tests für Query-Result-Cache schreiben
- [ ] `QueryResultCache` implementieren (TDD)
  - Häufige Queries cachen
  - Retrieval-Results cachen
  - TTL-basierte Expiration
  - Cache-Invalidation bei Index-Updates
- [ ] Tests ausführen und bestehen

---

## Phase 11: Hybrid-Search (Optional)

### 11.1 Keyword-Search

**Abhängigkeiten**: 3.1 (Vector-Database-Client)

#### 11.1.1 Keyword-Search-Manager
- [ ] Tests für Keyword-Search schreiben
- [ ] `KeywordSearchManager` implementieren (TDD)
  - Full-Text-Search in Dokumenten
  - Keyword-Matching
  - BM25-Ranking
- [ ] Tests ausführen und bestehen

### 11.2 Hybrid-Search-Combiner

**Abhängigkeiten**: 9.1 (Vector-Search), 11.1 (Keyword-Search)

#### 11.2.1 Hybrid-Search-Manager
- [ ] Tests für Hybrid-Search schreiben
- [ ] `HybridSearchManager` implementieren (TDD)
  - Vector-Search + Keyword-Search kombinieren
  - Score-Fusion (z.B. Reciprocal Rank Fusion)
  - Re-Ranking
- [ ] Tests ausführen und bestehen

---

## Phase 12: Re-Ranking (Optional)

### 12.1 Cross-Encoder Integration

**Abhängigkeiten**: 9.2 (Document-Ranking)

#### 12.1.1 Cross-Encoder-Model
- [ ] Tests für Cross-Encoder schreiben
- [ ] `CrossEncoderModel` implementieren (TDD)
  - Cross-Encoder-Model laden
  - Query + Document → Relevance-Score
  - Re-Ranking
- [ ] Tests ausführen und bestehen

---

## Phase 13: Error-Handling & Resilience

### 13.1 Indexing-Error-Handling

**Abhängigkeiten**: 6.3 (Indexing-Pipeline)

#### 13.1.1 Indexing-Error-Handler
- [ ] Tests für Indexing-Error-Handler schreiben
- [ ] `IndexingErrorHandler` implementieren (TDD)
  - Indexing-Fehler kategorisieren
  - Retry-Strategie
  - Fallback-Mechanismen
  - Error-Logging
  - User-Benachrichtigung
- [ ] Tests ausführen und bestehen

### 13.2 Retrieval-Error-Handling

**Abhängigkeiten**: 9.4 (Context-Formatting)

#### 13.2.1 Retrieval-Error-Handler
- [ ] Tests für Retrieval-Error-Handler schreiben
- [ ] `RetrievalErrorHandler` implementieren (TDD)
  - Retrieval-Fehler kategorisieren
  - Retry-Strategie
  - Fallback zu alternativen Routen
  - Error-Logging
- [ ] Tests ausführen und bestehen

### 13.3 Vector-Database-Connection-Resilience

**Abhängigkeiten**: 3.1 (Vector-Database-Client)

#### 13.3.1 Connection-Retry-Manager
- [ ] Tests für Connection-Retry schreiben
- [ ] `ConnectionRetryManager` implementieren (TDD)
  - Sofortiger Reconnect-Versuch
  - Exponential-Backoff
  - Maximale Retry-Versuche
  - Connection-Monitoring
- [ ] Tests ausführen und bestehen

---

## Phase 14: Performance-Optimization

### 14.1 Indexing-Performance

**Abhängigkeiten**: 6.3 (Indexing-Pipeline)

#### 14.1.1 Parallel-Indexing
- [ ] Performance-Tests für Parallel-Indexing schreiben
- [ ] Parallel-Indexing optimieren
  - Multiple Dokumente parallel verarbeiten
  - Thread-Pool für Embedding-Generation
  - Batch-Insert in Vector-Database
- [ ] Performance-Tests ausführen und Benchmarks erreichen

### 14.2 Search-Performance

**Abhängigkeiten**: 9.1 (Vector-Search)

#### 14.2.1 Index-Optimizations
- [ ] Performance-Tests für Search schreiben
- [ ] Index-Optimierungen implementieren
  - HNSW-Index (falls Qdrant)
  - IVF-Index (falls andere Databases)
  - Index-Tuning
- [ ] Performance-Tests ausführen und Benchmarks erreichen (< 100ms)

---

## Phase 15: Monitoring & Logging

### 15.1 Structured Logging

**Abhängigkeiten**: 1.1 (Projekt-Initialisierung)

#### 15.1.1 Logging Setup
- [ ] Structured-Logging konfigurieren (tracing)
- [ ] Log-Levels definieren (trace, debug, info, warn, error)
- [ ] Context-Tracking implementieren
- [ ] Log-Rotation konfigurieren

#### 15.1.2 Audit Logging
- [ ] Tests für Audit-Logging schreiben
- [ ] `AuditLogger` implementieren (TDD)
  - Document-Indexing-Events loggen
  - Document-Access-Events loggen
  - Query-Events loggen
- [ ] Tests ausführen und bestehen

### 15.2 Performance Monitoring

**Abhängigkeiten**: 14.1 (Indexing-Performance), 14.2 (Search-Performance)

#### 15.2.1 Metrics Collector
- [ ] Tests für Metrics-Collector schreiben
- [ ] `MetricsCollector` implementieren (TDD)
  - Performance-Metriken sammeln (Search-Zeit, Indexing-Zeit)
  - Query-Volumes tracken
  - Resource-Usage-Metriken sammeln
- [ ] Tests ausführen und bestehen

#### 15.2.2 Performance Alerts
- [ ] Tests für Performance-Alerts schreiben
- [ ] `PerformanceAlertManager` implementieren (TDD)
  - Alerts bei Performance-Problemen
  - Threshold-basierte Alerts
- [ ] Tests ausführen und bestehen

---

## Phase 16: Security & Data-Privacy

### 16.1 Input Validation

**Abhängigkeiten**: 2.2 (gRPC Server Implementation)

#### 16.1.1 Request Validator
- [ ] Tests für Request-Validation schreiben
- [ ] `RequestValidator` implementieren (TDD)
  - WolfRequest-Validation
  - Input-Sanitization
  - Malicious-Content-Detection (optional)
- [ ] Tests ausführen und bestehen

### 16.2 Document-Security

**Abhängigkeiten**: 6.1 (Document-Parser)

#### 16.2.1 Document-Encryption (Optional)
- [ ] Tests für Document-Encryption schreiben
- [ ] `DocumentEncryptionManager` implementieren (TDD)
  - Sensitive Dokumente verschlüsselt speichern
  - Decryption für Retrieval
- [ ] Tests ausführen und bestehen

#### 16.2.2 Malware-Scanning (Optional)
- [ ] Tests für Malware-Scanning schreiben
- [ ] `MalwareScanner` implementieren (TDD)
  - Dokumente auf Malware scannen
  - Integration mit Anti-Virus-Tools
- [ ] Tests ausführen und bestehen

### 16.3 Access-Control

**Abhängigkeiten**: 2.2 (gRPC Server Implementation)

#### 16.3.1 Access-Control-Manager
- [ ] Tests für Access-Control schreiben
- [ ] `AccessControlManager` implementieren (TDD)
  - Document-Access-Control
  - User-Permissions
  - Authorization
- [ ] Tests ausführen und bestehen

---

## Phase 17: GDPR-Compliance

### 17.1 Right-to-Deletion

**Abhängigkeiten**: 6.3 (Indexing-Pipeline), 3.1 (Vector-Database-Client)

#### 17.1.1 Data-Deletion-Manager
- [ ] Tests für Data-Deletion schreiben
- [ ] `DataDeletionManager` implementieren (TDD)
  - Sichere Datenlöschung
  - Dokumente aus Index entfernen
  - Embeddings aus Vector-Database löschen
  - Metadata löschen
- [ ] Tests ausführen und bestehen

### 17.2 Data-Export

**Abhängigkeiten**: 6.3 (Indexing-Pipeline)

#### 17.2.1 Data-Export-Manager
- [ ] Tests für Data-Export schreiben
- [ ] `DataExportManager` implementieren (TDD)
  - Indizierte Dokumente exportieren
  - Metadata exportieren
  - Exportformat (JSON, CSV, etc.)
- [ ] Tests ausführen und bestehen

---

## Phase 18: Documentation

### 18.1 API Documentation

**Abhängigkeiten**: 2.2 (gRPC Server Implementation)

#### 18.1.1 gRPC Service Documentation
- [ ] gRPC-Service-Documentation erstellen
  - WolfRequest/WolfResponse dokumentieren
  - Request-Workflows dokumentieren
  - Error-Codes dokumentieren
- [ ] Code-Examples erstellen

### 18.2 Code Documentation

**Abhängigkeiten**: Alle vorherigen Phasen

#### 18.2.1 Rust Documentation
- [ ] Alle Public-APIs mit Rustdoc dokumentieren
- [ ] Code-Examples in Rustdoc hinzufügen
- [ ] Rustdoc generieren (`cargo doc`)

#### 18.2.2 Architecture Documentation
- [ ] Architecture-Diagramm erstellen
- [ ] Indexing-Flow-Diagramm erstellen
- [ ] Retrieval-Flow-Diagramm erstellen

### 18.3 User Documentation

**Abhängigkeiten**: Alle vorherigen Phasen

#### 18.3.1 Integration Guide
- [ ] Integration-Guide für Odin erstellen
  - Wie Odin Freki nutzt
  - WolfRequest-Examples
  - RAGContext-Examples

---

## Phase 19: Testing & Quality-Assurance

### 19.1 Integration Testing

**Abhängigkeiten**: Alle vorherigen Phasen

#### 19.1.1 End-to-End Tests
- [ ] E2E-Tests für komplette RAG-Workflows schreiben
  - Document-Indexing → Query → Context-Retrieval
  - Watch-Folder → Auto-Indexing
  - Document-Update → Re-Indexing
- [ ] E2E-Tests ausführen und bestehen

#### 19.1.2 Load Testing
- [ ] Load-Tests schreiben
  - Hohe Query-Volumes testen
  - Batch-Indexing-Performance testen
  - Concurrent-Queries testen
- [ ] Load-Tests ausführen und Benchmarks erreichen

### 19.2 Performance Testing

**Abhängigkeiten**: 14.1 (Indexing-Performance), 14.2 (Search-Performance)

#### 19.2.1 Performance Benchmarks
- [ ] Performance-Benchmarks definieren
  - Vector-Search-Zeit (< 100ms)
  - Indexing-Zeit (< 1s pro Dokument)
  - Throughput (Queries/Sekunde)
- [ ] Performance-Tests schreiben und ausführen

### 19.3 Security Testing

**Abhängigkeiten**: 16.1 (Security & Data-Privacy)

#### 19.3.1 Security Test Suite
- [ ] Comprehensive Security-Tests ausführen
  - Input-Validation-Tests
  - Access-Control-Tests
  - Encryption-Tests (falls implementiert)
- [ ] Security-Tests bestehen

#### 19.3.2 GDPR Compliance Testing
- [ ] GDPR-Compliance-Tests schreiben
  - Data-Minimization-Tests
  - Right-to-Deletion-Tests
  - Data-Export-Tests
  - Access-Control-Tests
- [ ] GDPR-Compliance-Tests ausführen und bestehen

---

## Zusammenfassung

**Gesamtanzahl Phasen**: 19
**Gesamtanzahl Schritte**: ~300+

**Kritische Abhängigkeiten**:
1. Vector-Database-Wahl (beeinflusst gesamte Vector-DB-Integration)
2. Embedding-Model-Standard (beeinflusst Embedding-Quality)
3. Document-Storage-Wahl (beeinflusst Document-Management)
4. Chunking-Bibliothek-Wahl (beeinflusst Chunking-Implementierung)
5. Cache-Storage-Wahl (beeinflusst Performance)

**Offene Fragen für USER**:
1. Vector-Database (Qdrant empfohlen, Chroma, Pinecone, Weaviate, Milvus)
2. Embedding-Model-Standard (all-MiniLM-L6-v2 empfohlen, all-mpnet-base-v2, BGE-small-en-v1.5, Custom)
3. Document-Storage (Filesystem, Vector-Database, Separate Database)
4. Chunking-Bibliothek (tiktoken + eigene Impl., text-splitter, eigene vollständige Impl.)
5. Cache-Storage (In-Memory, Redis, Vector-Database, Kein Cache)

**Hinweise**:
- Alle Schritte folgen TDD (Tests zuerst, dann Implementierung)
- **ALLE Tests müssen in Containern laufen** - keine lokalen Dependencies, Tools oder Services auf der Entwicklungsmaschine installieren
- Container-Setup muss vollständig isolierte Test-Umgebung bereitstellen
- Alle Schritte sind kleinstmöglich aufgeteilt
- Abhängigkeiten zwischen Phasen sind klar definiert
- Offene Fragen sind klar markiert (❓)
- Performance ist kritisch: < 100ms Vector-Search, < 1s Indexing pro Dokument
- Privacy ist kritisch: Lokale Verarbeitung bevorzugt, GDPR-Compliance
- Rust-Implementierung: Optimiert für Performance und Memory-Effizienz

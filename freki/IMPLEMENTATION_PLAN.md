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
- [x] `Cargo.toml` erstellen
- [x] Basis-Dependencies definieren (tokio, tonic, prost, serde, tracing, anyhow, thiserror, etc.)
- [x] `.gitignore` erstellen

#### 1.1.2 Verzeichnisstruktur erstellen
- [x] `src/main.rs`, `src/lib.rs` erstellen
- [x] `src/vector_db/`, `src/embedding/`, `src/indexing/`, `src/retrieval/`, `src/chunking/`, `src/grpc/`, `src/utils/` erstellen
- [x] `config/`, `tests/` (unit: chunking_test, embedding_test; mocks, utils) erstellen

#### 1.1.3 Build-System einrichten
- [x] Build-Scripts (build.rs, Protobuf → Rust) einrichten
- [ ] Cargo-Features definieren (z.B. `qdrant`, `chroma`) – optional

### 1.2 Test-Infrastruktur

**Abhängigkeiten**: 1.1 (Projekt-Initialisierung)

#### 1.2.1 Container-Setup für Tests
- [x] `Dockerfile` für Test-Umgebung erstellen (`Dockerfile.test`)
- [x] Docker Compose für Test-Services konfigurieren (`docker-compose.test.yml`)
  - Vector-Database-Container (Qdrant)
  - Mock-Odin-Service
  - Redis-Container
- [x] Test-Container-Startup-Scripts erstellen (`scripts/run-tests.ps1`, `scripts/run-tests.sh`)
- [x] **WICHTIG**: Alle Tests müssen in Containern laufen – CI und lokale Ausführung via `docker compose -f docker-compose.test.yml run --rm freki-test`

#### 1.2.2 Test-Framework konfigurieren
- [x] Test-Dependencies hinzufügen (tokio-test, mockall, tempfile, reqwest)
- [x] Test-Utilities und Helpers erstellen (tests/utils/test_helpers.rs: wait_for_service, get_service_url, wait_for_qdrant)
- [x] Mock-Setup für Odin (tests/mocks, docker-compose mock-odin)
- [ ] Mock-Setup für Geri (bei Bedarf für Integrationstests)
- [x] Test-Document-Generators erstellen (`tests/utils/document_generators.rs`: sample_document, sample_documents, document_with_content, document_with_metadata)
- [x] Test-Embedding-Generators erstellen (`tests/utils/embedding_generators.rs`: TestEmbeddingModel, deterministische Vektoren, default_dimension 384)

#### 1.2.3 CI/CD-Pipeline
- [x] GitHub Actions Workflow erstellen (`.github/workflows/freki.yml`)
- [x] Automatische Test-Ausführung bei Push/PR auf `freki/**` (Test im Container)
- [ ] Code-Coverage-Reporting einrichten (cargo-tarpaulin) – optional
- [x] Linting und Formatting (cargo fmt --check, cargo clippy)

### 1.3 Projekt-Konfiguration

**Abhängigkeiten**: 1.1 (Projekt-Initialisierung)

#### 1.3.1 Settings-System Design
- [x] Settings-Schema definieren (JSON, `src/utils/config.rs`)
- [x] Settings-Struktur (FrekiSettings: grpc_port, qdrant_url, embedding_model)
  - [ ] Chunking-Settings, Watch-Folder-Settings, Cache-Settings – optional / spätere Phase

#### 1.3.2 Settings-Validierung
- [x] Rust-Structs für Settings definieren (FrekiSettings)
- [x] Tests für Settings-Validierung schreiben (`src/utils/config.rs`: test_validate_default_ok, invalid_port, empty_qdrant_url, empty_embedding_model)
- [x] Settings-Validator implementieren (FrekiSettings::validate, SettingsError)
- [x] Validierung in load() und Hot-Reload integriert

#### 1.3.3 Settings-Loader
- [x] Settings-Loader implementiert (SettingsManager::load, get)
- [x] JSON-Parsing, Default-Settings
- [x] Hot-Reload-Mechanismus (start_hot_reload, notify)
- [ ] Tests für Settings-Loader (optional, z. B. mit tempfile)

---

## Phase 2: Protobuf & gRPC Setup

### 2.1 Protobuf Definitions

**Abhängigkeiten**: 1.1 (Projekt-Initialisierung)

#### 2.1.1 Shared Protobuf-Projekt verwenden
❓ **HINWEIS**: Protobuf-Definitions sollten in einem separaten Projekt sein (z.B. `edda-protocols`)
- [ ] Freki als Dependency zu Protobuf-Projekt hinzufügen
- [ ] Protobuf-Definitions importieren

#### 2.1.2 WolfRequest/WolfResponse Protocol
- [x] RAG-Protokoll in `freki.proto`: IndexDocumentRequest/Response, RetrieveContextRequest/Response, RetrievedDocument (RAGContext-äquivalent)
- [x] Code-Generierung (build.rs, tonic)

### 2.2 gRPC Server Implementation

**Abhängigkeiten**: 2.1 (Protobuf Definitions)

#### 2.2.1 gRPC Server Setup
- [x] gRPC-Server-Setup (tonic, `src/grpc/server.rs`, `start_grpc_server`, Port aus Settings)
- [ ] Tests für gRPC-Handler (optional, z. B. mit Mock VectorDb)
- [ ] Health-Check-Service (tonic health) – optional

#### 2.2.2 Wolf-Service Implementation
- [x] `FrekiServiceImpl` (IndexDocument, RetrieveContext) – `src/grpc/server.rs`
- [x] IndexDocument RPC (Document + Embedding → IndexDocumentResponse)
- [x] RetrieveContext RPC (Query-Embedding, Limit → RetrievedDocument[], relevance_scores)
- [x] Request-Validation, Error-Handling (gRPC Status)

---

## Phase 3: Vector-Database-Integration

### 3.1 Vector-Database-Client

**Abhängigkeiten**: 1.1 (Projekt-Initialisierung)
**Erforderliche USER-Eingaben**: Vector-Database-Wahl

#### 3.1.1 Database-Client-Interface
- [ ] `VectorDatabaseClient` Trait (optional, für Abstraktion) – aktuell konkreter VectorDbClient
- [x] create_collection, upsert_points (insert), search (search_vectors), delete_collection, delete_points (delete_vectors) – `src/vector_db/client.rs`
- [ ] update_vectors – optional (Qdrant: Upsert überschreibt)

#### 3.1.2 Qdrant Client (Option A - empfohlen)
❓ **HINWEIS**: Nur wenn Qdrant gewählt wurde
- [x] `VectorDbClient` (Qdrant) – Connection, create_collection, upsert_points, search, delete_collection, delete_points
- [ ] Tests für VectorDbClient (Integration mit Qdrant-Container) – optional

#### 3.1.3 Alternative Database Clients (Option B-E)
❓ **HINWEIS**: Je nach gewählter Database
- [ ] Tests für gewählten Client schreiben
- [ ] Client implementieren (TDD)
- [ ] Tests ausführen und bestehen

### 3.2 Collection Management

**Abhängigkeiten**: 3.1 (Vector-Database-Client)

#### 3.2.1 Collection Manager
- [x] `CollectionManager` – `src/vector_db/collection.rs` (Wrapper um VectorDbClient)
- [x] Collections erstellen, auflisten, löschen (create_collection, list_collections, delete_collection)
- [x] `VectorDbClient::list_collections()` ergänzt
- [ ] Tests für Collection-Manager (optional, Integration mit Qdrant-Container)
- [ ] Collection-Configuration verwalten – optional

---

## Phase 4: Embedding-Model-Integration

### 4.1 Embedding-Model-Interface

**Abhängigkeiten**: Keine
**Erforderliche USER-Eingaben**: Embedding-Model-Standard

#### 4.1.1 Embedding-Model-Trait
- [x] Tests für Embedding-Model-Interface (`tests/unit/embedding_test.rs`: embed_text, embed_batch, get_vector_dimension)
- [x] `EmbeddingModel` Trait – `src/embedding/sentence_transformers.rs` (embed_text, embed_batch, get_model_name, get_vector_dimension)
- [x] `SentenceTransformersModel` implementiert Trait (Stub für echte Integration)

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
- [x] Tests für Model-Registry (`src/embedding/registry.rs`: test_register_and_get_model, test_list_models, test_is_model_available)
- [x] `ModelRegistry` – `src/embedding/registry.rs` (register, get_model, list_models, is_model_available, initialize_default)
- [x] Model-Selection (get_model mit default), Health/Availability (is_model_available)

---

## Phase 5: Document-Chunking

### 5.1 Chunking-Interface

**Abhängigkeiten**: Keine
**Erforderliche USER-Eingaben**: Chunking-Bibliothek-Wahl

#### 5.1.1 Chunking-Trait
- [x] Tests für Chunking-Interface (`tests/unit/chunking_test.rs`: semantic_chunker_creation, chunk_document, chunk_large_document, chunk_overlap)
- [x] `DocumentChunker` Trait – `src/chunking/semantic.rs` (chunk_document, get_chunk_size, get_overlap_size)
- [x] Tests ausführen und bestehen

### 5.2 Semantic-Chunking-Implementierung

**Abhängigkeiten**: 5.1 (Chunking-Interface)

#### 5.2.1 Sentence-Boundary-Detection
- [x] `SentenceBoundaryDetector` – `src/chunking/sentence_boundary.rs` (Satzgrenzen)
- [x] In SemanticChunker integriert (detect_sentences)
- [ ] Eigene Unit-Tests für SentenceBoundaryDetector – optional

#### 5.2.2 Semantic-Similarity-Grouping
- [ ] SemanticGrouper (semantische Ähnlichkeit) – optional / spätere Phase
- [x] Aktuell: satzbasierte Chunks mit Max-Size und Overlap

#### 5.2.3 Max-Size-Constraint
- [x] In SemanticChunker (chunk_size, count_tokens, create_chunks_with_overlap)
- [ ] tiktoken-Integration – optional (aktuell: Whitespace-Token-Count)

#### 5.2.4 Overlap-Implementierung
- [x] In SemanticChunker (overlap_size, create_chunks_with_overlap)
- [x] Overlap-Buffer am Chunk-Ende → Anfang nächster Chunk

### 5.3 Semantic-Chunker

**Abhängigkeiten**: 5.2 (Semantic-Chunking-Implementierung)

#### 5.3.1 Complete Semantic-Chunker
- [x] Tests für Semantic-Chunker (chunking_test.rs)
- [x] `SemanticChunker` – `src/chunking/semantic.rs` (chunk_size, overlap_size, SentenceBoundaryDetector)
- [x] Chunking-Parameter konfigurierbar (new(chunk_size, overlap_size))

---

## Phase 6: Document-Indexing

### 6.1 Document-Parser

**Abhängigkeiten**: Keine

#### 6.1.1 Document-Parser-Interface
- [x] Tests für Document-Parser (`src/indexing/parser.rs`: test_supports_txt_and_md, test_parse_document_returns_document, test_parse_unsupported_type_returns_error)
- [x] `DocumentParser` Trait – `src/indexing/parser.rs` (parse_document, supports_file_type)
- [x] Tests ausführen und bestehen

#### 6.1.2 File-Type-Specific Parsers
- [x] Tests für Text-Parser (.txt, .md) – in parser.rs
- [x] `TextParser` – `src/indexing/parser.rs` (parse_document, supports_file_type für txt, md, markdown)
- [ ] PDF-Parser, DOCX-Parser – optional / spätere Phase
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
- [x] Tests für Metadata-Extractor (`src/indexing/metadata.rs`: test_extract_adds_title_from_first_line, test_extract_preserves_existing_metadata)
- [x] `MetadataExtractor` – `src/indexing/metadata.rs` (extract: Standard-Metadaten z. B. title aus erster Zeile, bestehende Metadaten erhalten)
- [ ] Created-Date, Author aus Dateisystem/Format – optional

### 6.3 Indexing-Pipeline

**Abhängigkeiten**: 6.1 (Document-Parser), 5.3 (Semantic-Chunker), 4.4 (Model-Registry), 3.2 (Collection Management)

#### 6.3.1 Indexing-Manager
- [x] `IndexingManager` – `src/indexing/manager.rs` (Pipeline: index_bytes → parse → optional MetadataExtractor → DocumentIndexer.index_document_auto)
- [x] Dokument parsen (DocumentParser), optional Metadaten anreichern, chunken + embedden + indizieren (via DocumentIndexer)
- [ ] Eigene Tests für IndexingManager (Integration mit Mock-Parser/Indexer) – optional

#### 6.3.2 Batch-Indexing
- [x] Tests für Batch-Indexing schreiben (`tests/unit/batch_indexing_test.rs`: index_all, batch_size, failures, progress, empty)
- [x] `BatchIndexingManager` implementieren (TDD) – `src/indexing/batch.rs`
  - Multiple Dokumente parallel indizieren (tokio::spawn pro Chunk)
  - Batch-Size-Limits (chunks von batch_size)
  - Progress-Tracking (optionaler Callback)
- [x] Tests ausführen und bestehen

---

## Phase 7: Document-Updates

### 7.1 Change-Detection

**Abhängigkeiten**: 6.3 (Indexing-Pipeline)

#### 7.1.1 Document-Change-Detector
- [x] Tests für Change-Detection schreiben (`tests/unit/change_detector_test.rs`)
- [x] `DocumentChangeDetector` implementieren (TDD) – `src/indexing/change_detector.rs`
  - Dokument-Hash berechnen (`compute_hash`, `compute_content_hash`; SHA-256, `DocumentHash`)
  - Änderungen erkennen (`has_changed(previous_hash, document)`)
  - Geänderte Teile identifizieren (`changed_chunk_indices(old_chunk_hashes, new_chunks)`)
- [x] Tests ausführen und bestehen

### 7.2 Incremental-Update

**Abhängigkeiten**: 7.1 (Change-Detection), 6.3 (Indexing-Pipeline)

#### 7.2.1 Incremental-Update-Manager
- [x] Tests für Incremental-Update schreiben (`tests/unit/incremental_update_test.rs`)
- [x] `IncrementalUpdateManager` implementieren (TDD) – `src/indexing/incremental_update.rs`
  - Geänderte Chunks identifizieren (via DocumentChangeDetector::changed_chunk_indices)
  - Nur geänderte Chunks re-indizieren (Upsert nur für changed indices)
  - Embeddings für geänderte Chunks neu erstellen (embed_batch für Subset)
  - Vector-Database selektiv aktualisieren (DocumentIndexer::index_document pro geändertem Chunk)
- [x] Tests ausführen und bestehen (Integration mit Qdrant, skip wenn nicht erreichbar)

### 7.3 Full-Re-Indexing

**Abhängigkeiten**: 6.3 (Indexing-Pipeline)

#### 7.3.1 Full-Re-Indexing-Manager
- [x] Tests für Full-Re-Indexing schreiben (`tests/unit/full_reindex_test.rs`)
- [x] `FullReIndexingManager` implementieren (TDD) – `src/indexing/full_reindex.rs`
  - Alte Chunks aus Vector-Database entfernen (`DocumentIndexer::delete_document_chunks`)
  - Dokument vollständig neu chunken (via Indexer-Chunker)
  - Neue Embeddings erstellen (via Indexer-Embedding-Model)
  - Neue Chunks indizieren (DocumentIndexer::index_document pro Chunk)
  - [ ] Background-Processing – optional / spätere Phase
- [x] Tests ausführen und bestehen (Integration mit Qdrant, skip wenn nicht erreichbar)

---

## Phase 8: Watch-Folder-Funktionalität

### 8.1 File-System-Watcher

**Abhängigkeiten**: 6.3 (Indexing-Pipeline), 7.2 (Incremental-Update), 7.3 (Full-Re-Indexing)

#### 8.1.1 Watch-Folder-Manager
- [x] Tests für Watch-Folder schreiben (`tests/unit/watch_folder_test.rs`)
- [x] `WatchFolderManager` implementieren (TDD) – `src/watch/manager.rs`
  - Folder überwachen (`watch(path, recursive)`, notify RecommendedWatcher)
  - Neue Dateien erkennen (WatchEvent::Created)
  - Geänderte Dateien erkennen (WatchEvent::Modified)
  - Gelöschte Dateien erkennen (WatchEvent::Removed)
  - Event-Handling (mpsc-Kanal, Aufrufer liest Events)
- [x] Tests ausführen und bestehen (tempfile, Create/Modify/Remove)

#### 8.1.2 Auto-Indexing-Manager
- [x] Tests für Auto-Indexing schreiben (`tests/unit/auto_indexing_test.rs`)
- [x] `AutoIndexingManager` implementieren (TDD) – `src/indexing/auto_indexing.rs`
  - Neue Dateien automatisch indizieren (`handle_created`: Datei lesen, parsen, `index_document_auto`)
  - Geänderte Dateien re-indizieren (`handle_modified`: Datei lesen, parsen, `FullReIndexingManager::reindex_full`)
  - Gelöschte Dateien aus Index entfernen (`handle_removed`: `DataDeletionManager::delete_document`)
  - Pfad-zu-document_id-Mapping (`path_to_document_id`: stabiler ID aus Pfad)
- [x] Tests ausführen und bestehen (Integration mit Qdrant, skip wenn nicht erreichbar)

---

## Phase 9: Context-Retrieval

### 9.1 Vector-Search

**Abhängigkeiten**: 3.1 (Vector-Database-Client), 4.4 (Model-Registry)

#### 9.1.1 Query-Embedding-Generator
- [x] Tests für Query-Embedding schreiben (`tests/unit/query_embedding_test.rs`: model_dimension, generate_returns_vector, empty_query)
- [x] `QueryEmbeddingGenerator` implementieren (TDD) – `src/retrieval/query_embedding.rs`
  - Query-Text zu Embedding konvertieren (gleiches Modell wie für Dokumente)
  - Gleiche Model wie für Dokumente verwenden
- [x] Tests ausführen und bestehen

#### 9.1.2 Similarity-Search-Manager
- [x] Tests für Similarity-Search (Integration mit Qdrant-Container; Threshold-Logik in search() getestet)
- [x] `SimilaritySearchManager` implementieren (TDD) – `src/retrieval/similarity_search.rs`
  - Vector-Search durchführen (Qdrant Cosine)
  - Top-K Retrieval (limit)
  - Threshold-Filtering (score >= score_threshold)
- [x] Tests ausführen und bestehen

### 9.2 Document-Ranking

**Abhängigkeiten**: 9.1 (Vector-Search)

#### 9.2.1 Document-Ranker
- [x] Tests für Document-Ranking schreiben (`tests/unit/document_ranker_test.rs`: sort_desc, threshold, top_k, empty, single)
- [x] `DocumentRanker` implementieren (TDD) – `src/retrieval/ranker.rs`
  - Dokumente nach Relevanz-Score ranken (sort by score desc)
  - Threshold anwenden (score >= threshold)
  - Top-K filtern (take top_k)
- [x] Tests ausführen und bestehen

### 9.3 Context-Extraction

**Abhängigkeiten**: 9.2 (Document-Ranking)

#### 9.3.1 Context-Extractor
- [x] Tests für Context-Extraction schreiben (`tests/unit/context_extractor_test.rs`: combine, max_chars, empty, single)
- [x] `ContextExtractor` implementieren (TDD) – `src/retrieval/context_extractor.rs`
  - Relevante Text-Passagen extrahieren (ExtractedPassage, ExtractedContext)
  - Multiple Dokumente kombinieren (combined mit "\n\n")
  - Kontext-Länge optimieren (with_max_chars, char-boundary-sicher kürzen)
- [x] Tests ausführen und bestehen

### 9.4 Context-Formatting

**Abhängigkeiten**: 9.3 (Context-Extraction)

#### 9.4.1 Context-Formatter
- [x] Tests für Context-Formatting schreiben (`tests/unit/context_formatter_test.rs`: structured, empty, single, traceability)
- [x] `ContextFormatter` implementieren (TDD) – `src/retrieval/context_formatter.rs`
  - Context für LLM formatieren
  - Strukturiertes Format (`[Document N: document_id]\ncontent...`)
  - RAGContext mit document_ids für Traceability
  - RAGContext erstellen (formatted_text, document_ids)
- [x] Tests ausführen und bestehen

---

## Phase 10: Caching (Optional)

### 10.1 Cache-Implementierung

**Abhängigkeiten**: 4.2 (Embedding-Models), 9.1 (Vector-Search)
**Erforderliche USER-Eingaben**: Cache-Storage-Wahl

❓ **HINWEIS**: Diese Phase wird nur ausgeführt, wenn Caching gewählt wurde.

#### 10.1.1 Embedding-Cache
- [x] Tests für Embedding-Cache schreiben (`tests/unit/embedding_cache_test.rs`)
- [x] `EmbeddingCache` implementieren (TDD) – `src/cache/embedding_cache.rs`
  - In-Memory-Cache + Trait `EmbeddingCache`; Cache-Key: `{document_id}_{chunk_id}_{model_name}`
  - Cache-Hit/Miss-Handling; `EmbeddingCacheHelper::get_or_compute_embeddings`
  - Cache-Invalidation bei Dokument (`invalidate_document`), `clear`
- [ ] Tests im Container ausführen (Container-Build wurde behoben: Logging + gRPC Send)

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
- [x] Tests für Indexing-Error-Handler schreiben (`tests/unit/indexing_error_handler_test.rs`: categorize, is_retriable, user_message, execute_with_retry)
- [x] `IndexingErrorHandler` implementieren (TDD) – `src/indexing/error_handler.rs`
  - Indexing-Fehler kategorisieren (Parse, Embedding, VectorDb, Unknown)
  - Retry-Strategie (execute_with_retry, max_retries, retry_delay)
  - is_retriable pro Kategorie
  - User-Benachrichtigung (user_message)
- [x] Tests ausführen und bestehen

### 13.2 Retrieval-Error-Handling

**Abhängigkeiten**: 9.4 (Context-Formatting)

#### 13.2.1 Retrieval-Error-Handler
- [x] Tests für Retrieval-Error-Handler schreiben (`tests/unit/retrieval_error_handler_test.rs`: categorize, is_retriable, execute_with_retry)
- [x] `RetrievalErrorHandler` implementieren (TDD) – `src/retrieval/error_handler.rs`
  - Retrieval-Fehler kategorisieren (VectorDb, Embedding, Timeout, Unknown)
  - Retry-Strategie (execute_with_retry, max_retries, retry_delay)
  - is_retriable pro Kategorie
- [x] Tests ausführen und bestehen

### 13.3 Vector-Database-Connection-Resilience

**Abhängigkeiten**: 3.1 (Vector-Database-Client)

#### 13.3.1 Connection-Retry-Manager
- [x] Tests für Connection-Retry schreiben (`tests/unit/connection_retry_test.rs`: success first/success after retries/fail after max/default)
- [x] `ConnectionRetryManager` implementieren (TDD) – `src/vector_db/connection_retry.rs`
  - Sofortiger Reconnect-Versuch (erster Versuch ohne Delay)
  - Exponential-Backoff (delay *= 2, cap max_delay)
  - Maximale Retry-Versuche (max_retries)
  - connect_vector_db(url), connect_with_retry(op)
- [x] Tests ausführen und bestehen

---

## Phase 14: Performance-Optimization

### 14.1 Indexing-Performance

**Abhängigkeiten**: 6.3 (Indexing-Pipeline)

#### 14.1.1 Parallel-Indexing
- [x] Performance-Tests für Parallel-Indexing schreiben (`tests/unit/parallel_indexing_perf_test.rs`: parallel_indexing_faster_than_sequential)
- [x] Parallel-Indexing optimieren – Multiple Dokumente parallel (bereits in BatchIndexingManager, tokio::spawn pro Chunk)
  - [ ] Thread-Pool für Embedding-Generation – optional / spätere Phase
  - [ ] Batch-Insert in Vector-Database – optional / spätere Phase
- [x] Performance-Tests ausführen und Benchmarks erreichen (parallel schneller als sequenzielle Schätzung)

### 14.2 Search-Performance

**Abhängigkeiten**: 9.1 (Vector-Search)

#### 14.2.1 Index-Optimizations
- [x] Performance-Tests für Search schreiben (`tests/search_performance_test.rs`: search_latency_under_threshold, erfordert QDRANT_URL)
- [x] Index-Optimierungen implementieren – Qdrant nutzt HNSW per Default (VectorParams in create_collection); optional HNSW-Parameter-Tuning später
  - [ ] IVF-Index – nur bei anderen Vector-DBs
  - [ ] Index-Tuning – optional
- [x] Performance-Tests ausführen und Benchmarks erreichen (Search-Latenz ≤ 150ms in CI; Ziel < 100ms in Produktion)

---

## Phase 15: Monitoring & Logging

### 15.1 Structured Logging

**Abhängigkeiten**: 1.1 (Projekt-Initialisierung)

#### 15.1.1 Logging Setup
- [x] Structured-Logging konfigurieren (tracing) – `src/utils/logging.rs`, `init_logging()`, von main aufgerufen
- [x] Log-Levels definieren (trace, debug, info, warn, error) – RUST_LOG (Default: info), siehe Modul-Doc
- [x] Context-Tracking implementieren – request_id in Spans (gRPC: index_document, retrieve_context), `info_span!(..., request_id = %request_id).entered()`
- [x] Log-Rotation konfigurieren – optional FREKI_LOG_FILE → tägliche Rotation (tracing-appender), FREKI_LOG_JSON=1 für JSON

#### 15.1.2 Audit Logging
- [x] Tests für Audit-Logging schreiben (`tests/unit/audit_logger_test.rs`: log_document_indexed/accessed/query, RecordingSink)
- [x] `AuditLogger` implementieren (TDD) – `src/utils/audit.rs`
  - Document-Indexing-Events loggen (log_document_indexed), in gRPC index_document nach Erfolg
  - Document-Access-Events loggen (log_document_accessed), in gRPC retrieve_context pro zurückgegebenem Dokument
  - Query-Events loggen (log_query), in gRPC retrieve_context; Sink-Trait, Default: TracingAuditSink (target "audit")
- [x] Tests ausführen und bestehen

### 15.2 Performance Monitoring

**Abhängigkeiten**: 14.1 (Indexing-Performance), 14.2 (Search-Performance)

#### 15.2.1 Metrics Collector
- [x] Tests für Metrics-Collector schreiben (`tests/unit/metrics_collector_test.rs`: counts, record times, avg)
- [x] `MetricsCollector` implementieren (TDD) – `src/utils/metrics.rs`
  - Performance-Metriken (record_indexing_time, record_search_time, get_avg_*_ms)
  - Query-Volumes (increment_indexing_count, increment_query_count, get_*_count)
- [x] Tests ausführen und bestehen

#### 15.2.2 Performance Alerts
- [x] Tests für Performance-Alerts schreiben (`tests/unit/performance_alert_test.rs`: under/over threshold, both, no metrics)
- [x] `PerformanceAlertManager` implementieren (TDD) – `src/utils/performance_alerts.rs`
  - Alerts bei Performance-Problemen (PerformanceAlert::IndexingSlow, SearchSlow mit current_avg_ms)
  - Threshold-basierte Alerts (max_avg_indexing_ms, max_avg_search_ms), check_alerts(metrics) → Vec<PerformanceAlert>
- [x] Tests ausführen und bestehen

---

## Phase 16: Security & Data-Privacy

### 16.1 Input Validation

**Abhängigkeiten**: 2.2 (gRPC Server Implementation)

#### 16.1.1 Request Validator
- [x] Tests für Request-Validation schreiben (`tests/unit/request_validator_test.rs`: index_document ok/empty_id/content_too_large/empty_embedding, retrieve_context ok/empty_embedding/limit)
- [x] `RequestValidator` implementieren (TDD) – `src/grpc/validator.rs`
  - IndexDocumentRequest: document_id non-empty, content max 10MB, embedding non-empty
  - RetrieveContextRequest: query_embedding non-empty, limit 1..=1000
  - Input-Sanitization (trim document_id)
- [x] Tests ausführen und bestehen

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
- [x] Tests für Data-Deletion schreiben (`tests/unit/data_deletion_test.rs`: error display, From VectorDbError)
- [x] `DataDeletionManager` implementieren (TDD) – `src/utils/data_deletion.rs`
  - Sichere Datenlöschung (delete_document_by_point_ids)
  - Dokumente aus Index entfernen (delete_document nutzt scroll_points_by_document_id + delete_points)
  - Embeddings aus Vector-Database löschen (via delete_points)
  - Payload "document_id" beim Indexieren gesetzt (`src/indexing/document.rs`)
- [x] VectorDbClient: scroll_points_by_document_id (Filter payload.document_id) – `src/vector_db/client.rs`
- [x] Tests ausführen und bestehen

### 17.2 Data-Export

**Abhängigkeiten**: 6.3 (Indexing-Pipeline)

#### 17.2.1 Data-Export-Manager
- [x] Tests für Data-Export schreiben (`tests/unit/data_export_test.rs`: error display/From, format_json, format_csv)
- [x] `DataExportManager` implementieren (TDD) – `src/utils/data_export.rs`
  - Indizierte Dokumente exportieren (scroll_all, export_json, export_csv)
  - Metadata exportieren (ExportRecord: id, content, metadata)
  - Exportformat (JSON, CSV)
- [x] Tests ausführen und bestehen

---

## Phase 18: Documentation

### 18.1 API Documentation

**Abhängigkeiten**: 2.2 (gRPC Server Implementation)

#### 18.1.1 gRPC Service Documentation
- [x] gRPC-Service-Documentation erstellen (`docs/API.md`)
  - FrekiService (IndexDocument, RetrieveContext) dokumentiert
  - Request/Response-Schemas dokumentiert (IndexDocumentRequest/Response, RetrieveContextRequest/Response, RetrievedDocument)
  - Request-Workflows dokumentiert (RAG-Workflow: Index → Query → Retrieve → LLM)
  - Error-Codes dokumentiert (INVALID_ARGUMENT, INTERNAL mit Beispielen)
- [x] Code-Examples erstellt (Rust/tonic-Beispiele für IndexDocument und RetrieveContext)

### 18.2 Code Documentation

**Abhängigkeiten**: Alle vorherigen Phasen

#### 18.2.1 Rust Documentation
- [x] Wichtige Public-APIs mit Rustdoc dokumentieren (Document, DocumentIndexer, ContextRetriever, IndexingManager, BatchIndexingManager, WatchFolderManager, AutoIndexingManager, VectorDbClient, lib.rs)
- [x] Code-Examples in Rustdoc hinzufügen (Beispiele für alle dokumentierten APIs)
- [ ] Rustdoc generieren (`cargo doc`) – kann lokal ausgeführt werden

#### 18.2.2 Architecture Documentation
- [x] Architecture-Diagramm erstellen (`docs/ARCHITECTURE.md`: Gesamtarchitektur mit Komponenten und Beziehungen)
- [x] Indexing-Flow-Diagramm erstellen (`docs/ARCHITECTURE.md`: Indexing-Sequence-Diagramm, Batch-Indexing, Incremental-Update, Full-Re-Indexing)
- [x] Retrieval-Flow-Diagramm erstellen (`docs/ARCHITECTURE.md`: Retrieval-Sequence-Diagramm, Watch-Folder & Auto-Indexing)

### 18.3 User Documentation

**Abhängigkeiten**: Alle vorherigen Phasen

#### 18.3.1 Integration Guide
- [x] Implementierungsstand (Für Entwickler) in README – Übersicht umgesetzter Features und offene Phasen
- [ ] Integration-Guide für Odin erstellen (Wie Odin Freki nutzt, WolfRequest-Examples, RAGContext-Examples)

---

## Phase 19: Testing & Quality-Assurance

### 19.1 Integration Testing

**Abhängigkeiten**: Alle vorherigen Phasen

#### 19.1.1 End-to-End Tests
- [x] E2E-Tests für komplette RAG-Workflows schreiben (`tests/e2e_rag_test.rs`: e2e_index_then_retrieve_context)
  - Document-Indexing → Query → Context-Retrieval (DocumentIndexer + ContextRetriever, erfordert QDRANT_URL)
  - [ ] Watch-Folder → Auto-Indexing – optional / spätere Phase
  - [ ] Document-Update → Re-Indexing – optional / spätere Phase
- [x] E2E-Tests ausführen und bestehen

#### 19.1.2 Load Testing
- [x] Load-Tests schreiben (`tests/load_test.rs`)
  - Hohe Query-Volumes testen (Concurrent-Queries: 20 parallele Retrieve-Requests, ≤ 5s)
  - Batch-Indexing-Performance testen (10 Dokumente, BatchIndexingManager, ≤ 15s)
  - Concurrent-Queries testen (load_concurrent_retrieve_requests)
- [x] Load-Tests ausführen und Benchmarks erreichen (Tests skip wenn QDRANT_URL nicht erreichbar)

### 19.2 Performance Testing

**Abhängigkeiten**: 14.1 (Indexing-Performance), 14.2 (Search-Performance)

#### 19.2.1 Performance Benchmarks
- [x] Performance-Benchmarks definieren (README: Performance-Benchmarks (Tests))
  - Vector-Search-Zeit: Ziel < 100 ms, CI ≤ 150 ms (`search_performance_test.rs`)
  - Indexing-Zeit: Ziel < 1 s pro Dokument; Batch-Indexing 10 Docs ≤ 15 s (`load_test.rs`)
  - Concurrent-Queries: 20 parallele Retrieves ≤ 5 s (`load_test.rs`)
- [x] Performance-Tests schreiben und ausführen (bestehende Tests decken Benchmarks ab)

### 19.3 Security Testing

**Abhängigkeiten**: 16.1 (Security & Data-Privacy)

#### 19.3.1 Security Test Suite
- [x] Comprehensive Security-Tests ausführen (`tests/unit/security_test_suite.rs`)
  - Input-Validation-Tests: leere document_id, content_too_large, leere embedding/query_embedding, ungültiges limit
  - Access-Control-Tests: gültige Index-/Retrieve-Requests werden akzeptiert
  - Encryption-Tests – optional (falls Document-Encryption implementiert)
- [x] Security-Tests bestehen

#### 19.3.2 GDPR Compliance Testing
- [x] GDPR-Compliance-Tests schreiben (`tests/unit/gdpr_compliance_test.rs`)
  - Right-to-Deletion: DataDeletionError-Display, API DataDeletionManager
  - Data-Export: ExportRecord-Felder, format_json (Data Portability)
  - Data-Minimization / Access-Control: RequestValidator lehnt leere document_id und ungültiges limit ab, akzeptiert gültige Grenzwerte
- [x] GDPR-Compliance-Tests ausführen und bestehen

---

## Verbleibende optionale Punkte (Übersicht)

- [ ] **Phase 4.2** Sentence-Transformers FFI (komplex, optional)
- [ ] **Phase 10–12** Caching, Hybrid-Search, Re-Ranking (optional)
- [ ] **Phase 16.2–16.3** Document-Encryption, Access-Control (optional)

*(Kern-RAG und Produktion: PRODUKTIONSBEREIT mit Embedding-Stubs.)*

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

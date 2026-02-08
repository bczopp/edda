# Freki Architecture Documentation

## Übersicht

Dieses Dokument beschreibt die Architektur von Freki, dem RAG (Retrieval Augmented Generation) Service. Freki verwaltet Vector-Database-Operationen, Document-Indexing, Context-Retrieval und Prompt-Enrichment.

## Gesamtarchitektur

```mermaid
graph TB
    subgraph External["External Services"]
        Odin[Odin<br/>Orchestrator]
        Qdrant[Qdrant<br/>Vector Database]
    end
    
    subgraph FrekiService["Freki Service"]
        subgraph GRPC["gRPC Layer"]
            Server[FrekiServiceImpl<br/>gRPC Server]
            Validator[RequestValidator]
        end
        
        subgraph Indexing["Indexing Pipeline"]
            Parser[DocumentParser<br/>TextParser]
            Metadata[MetadataExtractor]
            Manager[IndexingManager]
            Indexer[DocumentIndexer]
            Batch[BatchIndexingManager]
            ChangeDet[DocumentChangeDetector]
            IncUpdate[IncrementalUpdateManager]
            FullReindex[FullReIndexingManager]
        end
        
        subgraph Retrieval["Retrieval Pipeline"]
            QueryEmb[QueryEmbeddingGenerator]
            SimSearch[SimilaritySearchManager]
            Ranker[DocumentRanker]
            Extractor[ContextExtractor]
            Formatter[ContextFormatter]
            Retriever[ContextRetriever]
        end
        
        subgraph Chunking["Chunking"]
            Chunker[SemanticChunker]
            SentenceDet[SentenceBoundaryDetector]
        end
        
        subgraph Embedding["Embedding"]
            EmbedModel[EmbeddingModel<br/>SentenceTransformers]
            Registry[ModelRegistry]
        end
        
        subgraph Watch["Watch & Auto-Indexing"]
            WatchMgr[WatchFolderManager]
            AutoIndex[AutoIndexingManager]
        end
        
        subgraph Utils["Utilities"]
            ErrorHandler[IndexingErrorHandler<br/>RetrievalErrorHandler]
            Audit[AuditLogger]
            Metrics[MetricsCollector]
            PerfAlert[PerformanceAlertManager]
            DataDel[DataDeletionManager]
            DataExp[DataExportManager]
        end
        
        subgraph VectorDB["Vector DB Client"]
            Client[VectorDbClient<br/>Qdrant Client]
            Collection[CollectionManager]
            Retry[ConnectionRetryManager]
        end
    end
    
    Odin -->|gRPC Requests| Server
    Server -->|Validates| Validator
    Validator -->|IndexDocument| Manager
    Validator -->|RetrieveContext| Retriever
    
    Manager -->|Parses| Parser
    Manager -->|Extracts Metadata| Metadata
    Manager -->|Indexes| Indexer
    
    Indexer -->|Chunks| Chunker
    Chunker -->|Detects Sentences| SentenceDet
    Indexer -->|Generates Embeddings| EmbedModel
    EmbedModel -->|Registered in| Registry
    
    Indexer -->|Stores| Client
    Client -->|Manages Collections| Collection
    Client -->|Retries on Error| Retry
    Client -->|Connects to| Qdrant
    
    Batch -->|Manages| Manager
    
    ChangeDet -->|Detects Changes| Indexer
    IncUpdate -->|Updates Changed Chunks| Indexer
    FullReindex -->|Re-indexes| Indexer
    
    WatchMgr -->|File Events| AutoIndex
    AutoIndex -->|Indexes/Re-indexes| Manager
    AutoIndex -->|Deletes| DataDel
    
    Retriever -->|Generates Query Embedding| QueryEmb
    QueryEmb -->|Uses| EmbedModel
    Retriever -->|Searches| SimSearch
    SimSearch -->|Queries| Client
    Retriever -->|Ranks| Ranker
    Retriever -->|Extracts| Extractor
    Retriever -->|Formats| Formatter
    
    Server -->|Logs| Audit
    Server -->|Collects| Metrics
    Metrics -->|Alerts| PerfAlert
    
    Manager -->|Handles Errors| ErrorHandler
    Retriever -->|Handles Errors| ErrorHandler
    
    Server -->|Returns Response| Odin
```

## Indexing-Flows

Es gibt zwei verschiedene Indexing-Flows in Freki:

1. **gRPC IndexDocument Flow**: Extern über gRPC API (Odin → Freki), Embedding bereits vorhanden
2. **Internal Auto-Indexing Flow**: Intern über Watch-Folder-Manager, automatische Embedding-Generierung

**Warum zwei verschiedene Flows?**

- **gRPC Flow**: Odin hat bereits das Embedding (z. B. von einem externen Embedding-Service) und möchte das Dokument direkt indizieren. Kein Chunking oder automatische Embedding-Generierung erforderlich.
- **Interner Flow**: Für lokale Dateien (z. B. über Watch-Folder) muss Freki das Dokument parsen, chunken und Embeddings generieren, bevor es indiziert werden kann.

**Verwendung**:
- **`DocumentIndexer.index_document(doc, embedding)`**: Für gRPC API (externes Embedding vorhanden)
- **`DocumentIndexer.index_document_auto(doc)`**: Für internes System (automatisches Chunking + Embedding)

### gRPC IndexDocument Flow (External API)

Dieser Flow wird von Odin über die gRPC API aufgerufen. Das Embedding ist bereits vorhanden und wird im Request mitgeliefert.

```mermaid
sequenceDiagram
    participant Odin
    participant Server as FrekiServiceImpl
    participant Validator as RequestValidator
    participant Indexer as DocumentIndexer
    participant VectorDB as VectorDbClient
    participant Qdrant
    participant Audit as AuditLogger
    
    Odin->>Server: IndexDocumentRequest<br/>(document_id, content, embedding, metadata)
    Server->>Validator: validate_index_request()
    Validator-->>Server: Validation OK
    
    Server->>Server: Deserialize embedding (bincode)
    Server->>Server: Create Document {id, content, metadata}
    
    Server->>Indexer: index_document(document, embedding)
    
    Note over Indexer: Embedding already provided,<br/>no automatic generation
    
    Indexer->>Indexer: Create chunk Document<br/>(id: document_id)
    Indexer->>VectorDB: upsert_points(collection, point)
    VectorDB->>Qdrant: HTTP POST /collections/{collection}/points
    Qdrant-->>VectorDB: Success
    VectorDB-->>Indexer: OK
    
    Indexer-->>Server: Success
    
    Server->>Audit: log_document_indexed(document_id)
    Server-->>Odin: IndexDocumentResponse {success: true}
```

### Internal Auto-Indexing Flow

Dieser Flow wird intern vom `IndexingManager` verwendet (z. B. über `WatchFolderManager`). Embedding und Chunking werden automatisch durchgeführt.

```mermaid
sequenceDiagram
    participant Caller as Caller<br/>(AutoIndexingManager)
    participant Manager as IndexingManager
    participant Parser as DocumentParser
    participant Metadata as MetadataExtractor
    participant Indexer as DocumentIndexer
    participant Chunker as SemanticChunker
    participant EmbedModel as EmbeddingModel
    participant VectorDB as VectorDbClient
    participant Qdrant
    
    Caller->>Manager: index_bytes(bytes, extension)
    Manager->>Parser: parse_document(bytes, extension)
    Parser-->>Manager: Document {id, content, metadata}
    
    Manager->>Metadata: extract(document)
    Metadata-->>Manager: Document with enriched metadata
    
    Manager->>Indexer: index_document_auto(document)
    
    alt Chunker configured
        Indexer->>Chunker: chunk_document(content)
        Chunker->>Chunker: detect_sentences()
        Chunker->>Chunker: create_chunks_with_overlap()
        Chunker-->>Indexer: Vec<String> chunks
    else No Chunker
        Indexer->>Indexer: Single chunk [content]
    end
    
    alt EmbeddingModel configured
        Indexer->>EmbedModel: embed_batch(chunks)
        EmbedModel-->>Indexer: Vec<Vec<f32>> embeddings
    else No EmbeddingModel
        Note over Indexer: Error: EmbeddingModel required<br/>for auto-indexing
    end
    
    loop For each chunk
        Indexer->>Indexer: Create chunk Document<br/>(id: "{doc_id}-chunk-{idx}")
        Indexer->>VectorDB: upsert_points(collection, points)
        VectorDB->>Qdrant: HTTP POST /collections/{collection}/points
        Qdrant-->>VectorDB: Success
        VectorDB-->>Indexer: OK
    end
    
    Indexer-->>Manager: Success
    Manager-->>Caller: Success
```

### Batch-Indexing-Flow

```mermaid
sequenceDiagram
    participant Client
    participant BatchMgr as BatchIndexingManager
    participant Manager as IndexingManager
    participant Indexer as DocumentIndexer
    
    Client->>BatchMgr: index_batch(items: Vec<(bytes, extension)>)
    
    BatchMgr->>BatchMgr: Split into batches (batch_size)
    
    loop For each batch
        par Parallel processing
            BatchMgr->>Manager: index_bytes(bytes, extension)
            Manager->>Indexer: index_document_auto(document)
            Indexer-->>Manager: Success
            Manager-->>BatchMgr: Success
        end
    end
    
    BatchMgr->>BatchMgr: Aggregate results<br/>(indexed_count, failed_count)
    BatchMgr-->>Client: BatchIndexingResult
```

### Incremental-Update-Flow

```mermaid
sequenceDiagram
    participant AutoIndex as AutoIndexingManager
    participant ChangeDet as DocumentChangeDetector
    participant IncUpdate as IncrementalUpdateManager
    participant Chunker as SemanticChunker
    participant EmbedModel as EmbeddingModel
    participant Indexer as DocumentIndexer
    participant VectorDB as VectorDbClient
    
    AutoIndex->>ChangeDet: compute_hash(document)
    ChangeDet-->>AutoIndex: DocumentHash
    
    AutoIndex->>ChangeDet: has_changed(old_hash, document)
    ChangeDet-->>AutoIndex: true
    
    AutoIndex->>IncUpdate: update_incremental(document, old_chunk_hashes)
    
    IncUpdate->>Chunker: chunk_document(content)
    Chunker-->>IncUpdate: Vec<String> chunks
    
    IncUpdate->>ChangeDet: changed_chunk_indices(old_hashes, chunks)
    ChangeDet-->>IncUpdate: Vec<usize> changed_indices
    
    IncUpdate->>IncUpdate: Extract changed chunks only
    
    IncUpdate->>EmbedModel: embed_batch(changed_chunks)
    EmbedModel-->>IncUpdate: Vec<Vec<f32>> embeddings
    
    loop For each changed chunk
        IncUpdate->>Indexer: index_document(chunk_doc, embedding)
        Indexer->>VectorDB: upsert_points() (Upsert = Update or Insert)
        VectorDB-->>Indexer: Success
    end
    
    IncUpdate-->>AutoIndex: IncrementalUpdateResult {updated_count}
```

### Full-Re-Indexing-Flow

```mermaid
sequenceDiagram
    participant AutoIndex as AutoIndexingManager
    participant FullReindex as FullReIndexingManager
    participant Indexer as DocumentIndexer
    participant VectorDB as VectorDbClient
    participant Chunker as SemanticChunker
    participant EmbedModel as EmbeddingModel
    
    AutoIndex->>FullReindex: reindex_full(document)
    
    FullReindex->>Indexer: delete_document_chunks(document_id)
    Indexer->>VectorDB: scroll_points_by_document_id(document_id)
    VectorDB-->>Indexer: Vec<PointId>
    Indexer->>VectorDB: delete_points(point_ids)
    VectorDB-->>Indexer: Success
    
    FullReindex->>Indexer: chunker()
    Indexer-->>FullReindex: Chunker
    
    FullReindex->>Chunker: chunk_document(content)
    Chunker-->>FullReindex: Vec<String> chunks
    
    FullReindex->>Indexer: embedding_model()
    Indexer-->>FullReindex: EmbeddingModel
    
    FullReindex->>EmbedModel: embed_batch(chunks)
    EmbedModel-->>FullReindex: Vec<Vec<f32>> embeddings
    
    loop For each chunk
        FullReindex->>Indexer: index_document(chunk_doc, embedding)
        Indexer->>VectorDB: upsert_points()
        VectorDB-->>Indexer: Success
    end
    
    FullReindex-->>AutoIndex: FullReIndexingResult {chunks_indexed}
```

## Retrieval-Flow

Der Retrieval-Flow beschreibt, wie relevanter Context basierend auf einer Query abgerufen wird.

```mermaid
sequenceDiagram
    participant Odin
    participant Server as FrekiServiceImpl
    participant Validator as RequestValidator
    participant Retriever as ContextRetriever
    participant QueryEmb as QueryEmbeddingGenerator
    participant EmbedModel as EmbeddingModel
    participant SimSearch as SimilaritySearchManager
    participant Ranker as DocumentRanker
    participant Extractor as ContextExtractor
    participant Formatter as ContextFormatter
    participant VectorDB as VectorDbClient
    participant Qdrant
    
    Odin->>Server: RetrieveContextRequest<br/>(query_embedding, limit)
    Server->>Validator: validate_retrieve_request()
    Validator-->>Server: Validation OK
    
    Server->>Retriever: retrieve(query_embedding, limit)
    
    alt Query text provided (not embedding)
        Retriever->>QueryEmb: generate(query_text)
        QueryEmb->>EmbedModel: embed_text(query_text)
        EmbedModel-->>QueryEmb: Vec<f32> embedding
        QueryEmb-->>Retriever: embedding
    else Embedding provided
        Note over Retriever: Uses provided embedding
    end
    
    Retriever->>SimSearch: search(embedding, limit, threshold)
    SimSearch->>VectorDB: search_vectors(collection, query_vector, limit)
    VectorDB->>Qdrant: HTTP POST /collections/{collection}/points/search
    Qdrant-->>VectorDB: SearchResult {points, scores}
    VectorDB-->>SimSearch: Vec<(Point, Score)>
    
    SimSearch->>SimSearch: Filter by threshold (score >= threshold)
    SimSearch-->>Retriever: Vec<(Point, Score)>
    
    Retriever->>Ranker: rank(documents, scores, limit, threshold)
    Ranker->>Ranker: Sort by score (descending)
    Ranker->>Ranker: Filter by threshold
    Ranker->>Ranker: Take top_k
    Ranker-->>Retriever: Vec<RetrievedDocument>
    
    Retriever->>Extractor: extract(documents, max_chars)
    Extractor->>Extractor: Combine document contents
    Extractor->>Extractor: Truncate to max_chars (char-boundary-safe)
    Extractor-->>Retriever: ExtractedContext
    
    Retriever->>Formatter: format(context)
    Formatter->>Formatter: Format as "[Document N: id]\ncontent..."
    Formatter-->>Retriever: RAGContext {formatted_text, document_ids}
    
    Retriever-->>Server: RetrievedContext {documents, relevance_scores}
    
    Server->>Server: AuditLogger.log_query(query_embedding)<br/>AuditLogger.log_document_accessed(doc_id) per doc
    
    Server-->>Odin: RetrieveContextResponse {documents, relevance_scores}
```

## Watch-Folder & Auto-Indexing-Flow

```mermaid
sequenceDiagram
    participant FileSystem
    participant WatchMgr as WatchFolderManager
    participant AutoIndex as AutoIndexingManager
    participant Manager as IndexingManager
    participant FullReindex as FullReIndexingManager
    participant DataDel as DataDeletionManager
    participant Indexer as DocumentIndexer
    
    WatchMgr->>WatchMgr: watch(path, recursive)
    WatchMgr->>FileSystem: Monitor file events (notify)
    
    FileSystem->>WatchMgr: Event: Created(path)
    WatchMgr->>WatchMgr: Send WatchEvent::Created
    WatchMgr->>AutoIndex: handle_created(path)
    
    AutoIndex->>AutoIndex: Read file bytes
    AutoIndex->>AutoIndex: Parse document (DocumentParser)
    AutoIndex->>AutoIndex: path_to_document_id(path)
    AutoIndex->>Manager: index_bytes(bytes, extension)
    Manager->>Indexer: index_document_auto(document)
    Indexer-->>Manager: Success
    Manager-->>AutoIndex: Success
    
    FileSystem->>WatchMgr: Event: Modified(path)
    WatchMgr->>WatchMgr: Send WatchEvent::Modified
    WatchMgr->>AutoIndex: handle_modified(path)
    
    AutoIndex->>AutoIndex: Read file bytes
    AutoIndex->>AutoIndex: Parse document
    AutoIndex->>FullReindex: reindex_full(document)
    FullReindex->>Indexer: delete_document_chunks(document_id)
    FullReindex->>Indexer: Re-index all chunks
    FullReindex-->>AutoIndex: Success
    
    FileSystem->>WatchMgr: Event: Removed(path)
    WatchMgr->>WatchMgr: Send WatchEvent::Removed
    WatchMgr->>AutoIndex: handle_removed(path)
    
    AutoIndex->>AutoIndex: path_to_document_id(path)
    AutoIndex->>DataDel: delete_document(document_id)
    DataDel->>Indexer: delete_document_chunks(document_id)
    Indexer-->>DataDel: Success
    DataDel-->>AutoIndex: Success
```

## Komponenten-Übersicht

### gRPC Layer
- **FrekiServiceImpl**: Haupt-gRPC-Server, implementiert `FrekiService` (IndexDocument, RetrieveContext)
- **RequestValidator**: Validiert gRPC-Requests (document_id, content-Größe, embedding, limit)

### Indexing Pipeline

**Wichtig**: Es gibt zwei verschiedene Indexing-Wege:
1. **gRPC API** (`index_document`): Embedding bereits vorhanden, kein automatisches Chunking/Embedding
2. **Internes System** (`index_document_auto`): Automatisches Chunking und Embedding-Generierung

Komponenten:
- **DocumentParser**: Parst Dokumente aus Bytes (TextParser für .txt, .md) - nur für internen Flow
- **MetadataExtractor**: Extrahiert Metadaten (z. B. Titel aus erster Zeile) - nur für internen Flow
- **IndexingManager**: Orchestriert interne Indexing-Pipeline (parse → metadata → index_document_auto)
- **DocumentIndexer**: Indiziert Dokumente in Vector-DB
  - `index_document(doc, embedding)`: Direktes Indexing mit vorhandenem Embedding (gRPC API)
  - `index_document_auto(doc)`: Automatisches Chunking + Embedding-Generierung (internes System)
- **BatchIndexingManager**: Indiziert mehrere Dokumente parallel in Batches (internes System)
- **DocumentChangeDetector**: Erkennt Dokument-Änderungen via SHA-256-Hash
- **IncrementalUpdateManager**: Re-indiziert nur geänderte Chunks
- **FullReIndexingManager**: Löscht alte Chunks und indiziert Dokument vollständig neu

### Retrieval Pipeline
- **QueryEmbeddingGenerator**: Generiert Embeddings für Queries
- **SimilaritySearchManager**: Führt Vector-Search durch (Cosine-Similarity)
- **DocumentRanker**: Rankt Dokumente nach Relevanz-Score
- **ContextExtractor**: Extrahiert relevante Text-Passagen
- **ContextFormatter**: Formatiert Context für LLM-Consumption
- **ContextRetriever**: Orchestriert Retrieval-Pipeline (query → search → rank → extract → format)

### Chunking
- **SemanticChunker**: Chunkt Dokumente semantisch (Satzgrenzen + Max-Size + Overlap)
- **SentenceBoundaryDetector**: Erkennt Satzgrenzen für semantisches Chunking

### Embedding
- **EmbeddingModel**: Trait für Embedding-Generierung (embed_text, embed_batch)
- **SentenceTransformersModel**: Implementierung für Sentence-Transformers-Models
- **ModelRegistry**: Verwaltet verfügbare Embedding-Models

### Watch & Auto-Indexing
- **WatchFolderManager**: Überwacht Ordner für Datei-Änderungen (notify)
- **AutoIndexingManager**: Verbindet Watch-Events mit Indexing (Created → index, Modified → reindex, Removed → delete)

### Utilities
- **IndexingErrorHandler**: Behandelt Indexing-Fehler (Kategorisierung, Retry)
- **RetrievalErrorHandler**: Behandelt Retrieval-Fehler (Kategorisierung, Retry)
- **AuditLogger**: Loggt Document-Indexing, Document-Access und Query-Events
- **MetricsCollector**: Sammelt Performance-Metriken (Indexing-Zeit, Search-Zeit, Volumes)
- **PerformanceAlertManager**: Generiert Alerts bei Performance-Problemen
- **DataDeletionManager**: Löscht Dokumente aus Index (GDPR Right-to-Deletion)
- **DataExportManager**: Exportiert indizierte Dokumente (GDPR Data Portability)

### Vector DB Client
- **VectorDbClient**: Client für Qdrant (create_collection, upsert_points, search_vectors, delete_points)
- **CollectionManager**: Verwaltet Collections (create, list, delete)
- **ConnectionRetryManager**: Retry-Logik für Vector-DB-Verbindungen (Exponential-Backoff)

## Datenfluss

### Indexing-Datenfluss

**gRPC IndexDocument Flow (External API)**:
```
IndexDocumentRequest {document_id, content, embedding, metadata}
  → FrekiServiceImpl (validate, deserialize embedding)
  → DocumentIndexer.index_document(document, embedding)
  → VectorDbClient.upsert_points()
  → Qdrant (Point with vector + payload)
```

**Internal Auto-Indexing Flow**:
```
File (bytes)
  → IndexingManager.index_bytes()
  → DocumentParser 
  → Document {id, content, metadata}
  → MetadataExtractor 
  → Document with enriched metadata
  → DocumentIndexer.index_document_auto()
  → SemanticChunker 
  → Vec<String> chunks
  → EmbeddingModel 
  → Vec<Vec<f32>> embeddings
  → VectorDbClient.upsert_points() (per chunk)
  → Qdrant (Points with vectors + payload)
```

### Retrieval-Datenfluss
```
Query (text or embedding)
  → QueryEmbeddingGenerator 
  → Vec<f32> query_embedding
  → SimilaritySearchManager 
  → VectorDbClient 
  → Qdrant (Search)
  → Vec<(Point, Score)>
  → DocumentRanker 
  → Vec<RetrievedDocument> (sorted, filtered)
  → ContextExtractor 
  → ExtractedContext (combined text)
  → ContextFormatter 
  → RAGContext {formatted_text, document_ids}
```

## Performance-Überlegungen

### Indexing-Performance
- **Parallel-Indexing**: BatchIndexingManager indiziert mehrere Dokumente parallel (tokio::spawn)
- **Batch-Embedding**: EmbeddingModel unterstützt `embed_batch` für effiziente Batch-Verarbeitung
- **Incremental-Updates**: Nur geänderte Chunks werden re-indiziert (weniger Embedding-Generierung)

### Retrieval-Performance
- **Vector-Search**: Qdrant nutzt HNSW-Index für schnelle Similarity-Search (< 100ms Ziel)
- **Threshold-Filtering**: Dokumente unter Threshold werden früh gefiltert
- **Top-K-Limitierung**: Nur Top-K Dokumente werden zurückgegeben

### Monitoring
- **MetricsCollector**: Trackt Indexing-Zeit, Search-Zeit, Volumes
- **PerformanceAlertManager**: Alerts bei Performance-Problemen (z. B. avg_indexing_ms > threshold)

## Sicherheit & Datenschutz

### Input-Validierung
- **RequestValidator**: Validiert alle gRPC-Requests (document_id, content-Größe, embedding, limit)

### GDPR-Compliance
- **DataDeletionManager**: Implementiert Right-to-Deletion (löscht alle Chunks eines Dokuments)
- **DataExportManager**: Implementiert Data Portability (exportiert indizierte Dokumente als JSON/CSV)

### Audit-Logging
- **AuditLogger**: Loggt alle Document-Indexing-, Document-Access- und Query-Events für Compliance

## Abhängigkeiten

### Externe Services
- **Qdrant**: Vector-Database für Embedding-Storage und Similarity-Search
- **Odin**: Orchestrator, sendet IndexDocument- und RetrieveContext-Requests

### Interne Abhängigkeiten
- **Embedding-Models**: Lokale Sentence-Transformers-Models (z. B. all-MiniLM-L6-v2)
- **File-System**: Für Watch-Folder-Funktionalität (notify)

## Erweiterbarkeit

### Optional Features
- **Caching** (Phase 10): Embedding-Cache und Query-Result-Cache für Performance
- **Hybrid-Search** (Phase 11): Keyword-Search + Vector-Search kombinieren
- **Re-Ranking** (Phase 12): Cross-Encoder für präzisere Relevanz-Bewertung
- **Document-Encryption** (Phase 16.2): Verschlüsselte Speicherung sensibler Dokumente
- **Access-Control** (Phase 16.3): User-basierte Zugriffskontrolle auf Dokumente

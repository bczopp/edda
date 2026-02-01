# Freki - RAG Service

## Übersicht

Freki ist einer von Odins Wölfen und stellt den RAG (Retrieval Augmented Generation) Service bereit. Er wird zuerst verwendet, um Prompts mit relevantem Kontext anzureichern, bevor sie an Geri (LLM) weitergegeben werden.

**Tests ausführen:** Von `freki/`: `docker compose -f docker-compose.test.yml run --rm freki-test` oder `./scripts/run-tests.sh` / `.\scripts\run-tests.ps1`. Von Repo-Root: `freki/scripts/run-tests.sh` bzw. `.\freki\scripts\run-tests.ps1`. **CI:** Bei Push/PR auf `freki/**` läuft die Pipeline [.github/workflows/freki.yml](../.github/workflows/freki.yml) (Test im Container, Lint).

**API-Dokumentation:** Siehe [docs/API.md](docs/API.md) für gRPC-Service-Dokumentation, Request/Response-Schemas, Error-Codes und Code-Beispiele.

### Implementierungsstand (Für Entwickler)

Aktuell umgesetzt (Details siehe [IMPLEMENTATION_PLAN.md](IMPLEMENTATION_PLAN.md)):

- **Vector-DB & gRPC**: Qdrant-Client, Collection-Management, gRPC-Server (IndexDocument, RetrieveContext), Request-Validierung.
- **Embedding & Chunking**: EmbeddingModel-Trait, Model-Registry, SemanticChunker, Sentence-Boundary.
- **Indexing**: DocumentIndexer, IndexingManager, BatchIndexingManager, Document-Parser (Text), Metadata-Extractor, DocumentChangeDetector (7.1.1), IncrementalUpdateManager (7.2.1), FullReIndexingManager (7.3.1), AutoIndexingManager (8.1.2).
- **Retrieval**: QueryEmbedding, SimilaritySearch, DocumentRanker, ContextExtractor/Formatter, ContextRetriever.
- **Resilience & Security**: Indexing/Retrieval-Error-Handler, ConnectionRetry, RequestValidator, DataDeletion/DataExport (GDPR).
- **Monitoring**: Structured Logging, AuditLogger, MetricsCollector, PerformanceAlertManager.
- **Watch-Folder** (Phase 8.1): WatchFolderManager (notify), WatchEvent (Created/Modified/Removed), Event-Kanal; AutoIndexingManager (8.1.2) verbindet Watch-Events mit Indexing (Created → index, Modified → reindex, Removed → delete).
- **Tests**: Container-Setup (Dockerfile.test, docker-compose.test.yml), E2E RAG, Load-Tests (Concurrent-Queries, Batch-Indexing), Search-Performance, Security/GDPR-Test-Suites, Performance-Benchmarks (README), Watch-Folder-Tests, Auto-Indexing-Tests.

Offen u. a.: Phase 10–12 (Caching, Hybrid-Search, Re-Ranking optional), Phase 16.2–16.3 (Document-Encryption, Access-Control optional), Phase 18.2.2 (Architecture-Diagramme).

**Rustdoc:** Wichtige Public-APIs sind mit Rustdoc-Kommentaren und Code-Beispielen dokumentiert. Generiere Dokumentation mit `cargo doc --open`.

## Verantwortlichkeiten

### 1. Vector Database Management
- Verwaltet Vector Database für Dokumente
- Erstellt und aktualisiert Embeddings
- Verwaltet Index-Struktur

### 2. Document Indexing
- Indiziert Dokumente für RAG
- Erstellt Embeddings aus Dokumenten
- Speichert Metadaten zu Dokumenten

### 3. Context Retrieval & Enrichment
- Sucht relevante Dokumente basierend auf Query
- Rankt Dokumente nach Relevanz
- Extrahiert relevanten Kontext

### 4. Prompt Enrichment
- Reichert Prompts mit abgerufenem Kontext an
- Formatiert Context für LLM-Consumption
- Optimiert Context-Länge

## Service-Interfaces

### Inputs
- `WolfRequest` mit `modelType: RAG` und `prompt`
- Document Upload Requests
- Index Update Requests

### Outputs
- `WolfResponse` mit `ragContext` (RAGContext)
- Index Status
- Document Metadata

## DTO-Definitionen

### WolfRequest

`WolfRequest` ist das DTO für Requests an Freki (RAG) und Geri (LLM).

**Protobuf-Definition:**
```protobuf
message WolfRequest {
  string request_id = 1;              // Eindeutige Request-ID
  ModelType model_type = 2;            // RAG oder LLM
  string prompt = 3;                   // User-Prompt
  optional RAGContext rag_context = 4; // RAG-Context (für LLM-Requests)
  optional RequestMetadata metadata = 5; // Zusätzliche Metadaten
}

enum ModelType {
  RAG = 0;   // RAG-Request (an Freki)
  LLM = 1;   // LLM-Request (an Geri)
}

message RequestMetadata {
  optional string session_id = 1;      // Session-ID für Kontext
  optional int32 max_tokens = 2;       // Maximale Token-Anzahl
  optional double temperature = 3;    // Temperature für LLM
  optional string language = 4;        // Sprache
}
```

**Validierungsregeln:**
- `request_id`: Muss eindeutig sein, UUID-Format empfohlen
- `model_type`: Muss RAG oder LLM sein
- `prompt`: Muss nicht leer sein, maximale Länge: 1.000.000 Zeichen
- `rag_context`: Optional, nur für LLM-Requests relevant

### WolfResponse

`WolfResponse` ist das DTO für Responses von Freki (RAG) und Geri (LLM).

**Protobuf-Definition:**
```protobuf
message WolfResponse {
  string request_id = 1;              // Request-ID (zur Zuordnung)
  ModelType model_type = 2;           // RAG oder LLM
  oneof response {
    RAGContext rag_context = 3;       // RAG-Context (von Freki)
    LLMResponse llm_response = 4;     // LLM-Response (von Geri)
  }
  optional ResponseMetadata metadata = 5; // Zusätzliche Metadaten
}

message RAGContext {
  repeated RetrievedDocument documents = 1; // Abgerufene Dokumente
  double retrieval_score = 2;              // Durchschnittlicher Retrieval-Score
}

message RetrievedDocument {
  string document_id = 1;            // Dokument-ID
  string content = 2;                // Dokument-Content (Chunk)
  double relevance_score = 3;         // Relevanz-Score
  map<string, string> metadata = 4;  // Dokument-Metadaten
}

message LLMResponse {
  string text = 1;                   // Generierter Text
  int32 tokens_used = 2;             // Anzahl verwendeter Tokens
  double latency_ms = 3;             // Latenz in Millisekunden
  optional double cost = 4;           // Kosten (für Cloud-LLMs)
  optional string model_used = 5;     // Verwendetes Model
}

message ResponseMetadata {
  int64 timestamp = 1;                // Timestamp
  optional string error = 2;          // Fehler-Message (falls vorhanden)
}
```

**Validierungsregeln:**
- `request_id`: Muss mit ursprünglichem Request übereinstimmen
- `model_type`: Muss mit Request übereinstimmen
- `rag_context`: Muss für RAG-Responses vorhanden sein
- `llm_response`: Muss für LLM-Responses vorhanden sein
- `documents`: Muss mindestens 1 Dokument enthalten (für RAG-Context)

## Workflow

1. **Query empfangen**
   - Odin sendet `WolfRequest` mit Prompt
   - Freki analysiert Query

2. **Embedding Generation**
   - Erstellt Embedding für Query
   - Verwendet Embedding-Model

3. **Vector Search**
   - Sucht in Vector Database nach ähnlichen Embeddings
   - Verwendet Similarity Search (Cosine, Euclidean, etc.)

4. **Document Ranking**
   - Rankt gefundene Dokumente nach Relevanz-Score
   - Filtert nach Threshold

5. **Context Extraction**
   - Extrahiert relevante Text-Passagen
   - Kombiniert mehrere Dokumente falls nötig

6. **Context Formatting**
   - Formatiert Context für LLM-Consumption
   - Erstellt `RAGContext` mit retrieved Documents
   - **Context-Format**: Dokumente werden in strukturiertem Format formatiert:
     ```
     [Document 1: document_id]
     content...
     
     [Document 2: document_id]
     content...
     ```
   - **Metadaten**: Dokument-Metadaten werden beibehalten für Traceability

7. **Response**
   - Sendet `WolfResponse` mit `ragContext` zurück
   - Odin verwendet `ragContext` für LLM-Request
   - **Hinweis**: Freki optimiert Context-Länge nicht - das übernimmt Geri (Context-Window-Management)

## Settings und Konfiguration

### Allgemeine Settings-Prinzipien

**Wichtig**: Diese Prinzipien gelten für alle Services und Platformen im Edda-System.

#### Settings-Format
- **Format**: Vermutlich JSON-Format (es sei denn im Rust-Kontext gibt es ein besseres Format, das ebenso einfach für Menschen zu verstehen ist)
- **Menschlich lesbar**: Settings-Dateien müssen für Menschen einfach zu verstehen und zu bearbeiten sein
- **Validierung**: Settings werden beim Laden validiert (Schema-Validierung)

#### Platform-Integration
- **Settings-Sammlung**: Platformen müssen alle Settings/Konfigurationsdateien sammeln, die auf dem Device bzw. auf der Platform aktuell verfügbar und aktiv sind
- **Frontend-Konfiguration**: Settings müssen über Settings im Frontend konfigurierbar gemacht werden
- **Zentrale Verwaltung**: Platform stellt zentrale Settings-Verwaltung zur Verfügung

#### Hot-Reload
- **Keine Neukompilierung**: Änderungen an den Settings sollen nicht dazu führen, dass das Projekt/der Service neu kompiliert werden muss
- **Runtime-Reload**: Die neuen Werte können einfach zur Laufzeit neu geladen werden
- **Service-Funktionen**: Services müssen entsprechende Funktionen zur Verfügung stellen (Hot-Reload, Settings-API, etc.)

#### Service-spezifische Settings
- **Projekt-spezifisch**: Was genau in einer Settings/Konfigurationsdatei steht, hängt sehr stark vom Service oder der Platform ab
- **Dokumentation**: Service-spezifische Settings müssen in der jeweiligen README dokumentiert werden
- **Beispiele**: Service-spezifische Settings-Beispiele sollten in der README enthalten sein

### Freki-spezifische Settings

**Settings-Inhalt (wird während Implementierung definiert)**
- Vector-Database-Konfiguration
- Embedding-Model-Einstellungen
- Chunking-Strategien
- Indexing-Einstellungen

## Technische Anforderungen

### Vector Database
- **Database-Auswahl**: 
  - **Standard-Database**: Qdrant ist Standard (lokal, performant, einfach)
  - **Database-Wechsel**: User kann zwischen verschiedenen Databases wechseln (Konfiguration)
  - **Migration-Tools**: Migration-Tools für Datenübertragung zwischen Databases
- **Database-Konfiguration**: 
  - **Konfiguration**: Vector-Database wird über Konfigurationsdatei konfiguriert
  - **Typ-spezifische Konfigurationen**: Verschiedene Konfigurationen für verschiedene Dokument-Typen (optional)
  - **Database-Updates**: Database-Updates werden automatisch gehandhabt (Schema-Migration)
- Unterstützung für verschiedene Vector DBs:
  - **Qdrant** (Standard, lokal)
  - **Chroma** (lokal)
  - **Pinecone** (Cloud, optional)
  - **Weaviate** (lokal/Cloud)
  - **Milvus** (lokal/Cloud)

### Embedding Models

**Einheitliches Model (Standard):**
- **Standard-Auswahl**: Beim ersten Start wird ein Standard-Embedding-Model gewählt
  - **Priorität**: Lokale Models bevorzugt (Privacy-First)
  - **Fallback**: Cloud-Models nur wenn lokale nicht verfügbar oder User explizit konfiguriert
  - **Empfohlenes Standard-Model**: `all-MiniLM-L6-v2` (Sentence Transformers, lokal, guter Kompromiss)
- **Automatische Auswahl**: System wählt bestes verfügbares lokales Model
- **Model-Registry**: Verfügbare Models werden in Registry verwaltet
- **Model-Health-Check**: System prüft Model-Verfügbarkeit und Performance

**Typ-spezifische Models (Option):**
- **Konfiguration**: User kann in Settings typ-spezifische Models konfigurieren
- **Format**: JSON-Konfiguration mit Mapping: `{ "document_type": "model_name" }`
- **Beispiele**:
  - PDF-Dokumente → spezialisiertes PDF-Embedding-Model
  - Code-Dateien → Code-spezifisches Embedding-Model
  - Markdown → Standard-Model
- **Fallback**: Falls typ-spezifisches Model nicht verfügbar, Fallback zu Standard-Model

**Embedding-Caching:**
- **Chunk-Embeddings**: Embeddings für Dokument-Chunks werden gecacht
- **Cache-Key**: `{document_id}_{chunk_id}_{model_name}_{model_version}`
- **Cache-Invalidierung**: Bei Dokument-Update wird Cache invalidiert
- **Cache-Speicherung**: Lokal in Vector-Database oder separatem Cache
- **Cache-Größe**: Konfigurierbar (z.B. max. 10GB)

**Unterstützung für verschiedene Embedding-Models:**
- **Lokale Models**:
  - Sentence Transformers (all-MiniLM-L6-v2, all-mpnet-base-v2, etc.)
  - BGE Models (BGE-small-en-v1.5, BGE-base-en-v1.5, etc.)
  - Custom lokale Models
- **Cloud-Models** (optional, nur wenn API-Keys hinterlegt):
  - OpenAI Embeddings (text-embedding-ada-002, text-embedding-3-small, etc.)
  - Cohere Embeddings
  - Custom Cloud-Models

### Indexing
- **Automatische Indizierung**: Watch-Folder überwacht Ordner, indiziert neue/geänderte Dateien automatisch
- **Manuelle Hinzufügung**: User kann Dokumente manuell hinzufügen (optional)
- **Chunking-Strategie**: Semantisches Chunking mit Max-Größe (siehe unten)
- **Dokument-Updates**: Incremental Update wenn möglich, sonst vollständige Re-Indizierung
- **Batch Indexing**: Batch Indexing für große Dokumentmengen
- **Metadata Indexing**: Metadaten werden indiziert

### Chunking-Implementierung

**Semantisches Chunking:**

**Bibliothek/Methode:**
- **Hauptmethode**: Sentence-Boundary-Detection + Semantic-Similarity
- **Bibliothek**: `tiktoken` für Token-Counting, `sentence-transformers` für Semantic-Similarity
- **Algorithmus**:
  1. Dokument wird in Sätze aufgeteilt (Sentence-Boundary-Detection)
  2. Sätze werden gruppiert basierend auf semantischer Ähnlichkeit
  3. Chunks werden erstellt, die natürliche semantische Grenzen respektieren
  4. Max-Größe wird als Constraint angewendet

**Chunking-Parameter:**
- **Max-Größe**: 1000 Tokens (konfigurierbar)
- **Min-Größe**: 200 Tokens (konfigurierbar)
- **Overlap**: 100 Tokens (konfigurierbar)
- **Semantic-Threshold**: 0.7 (konfigurierbar, für Semantic-Similarity)

**Overlap-Implementierung:**
- **Overlap-Mechanismus**: Letzte 100 Tokens eines Chunks werden als erste Tokens des nächsten Chunks wiederholt
- **Zweck**: Bessere Kontext-Erhaltung zwischen Chunks
- **Overlap-Berechnung**: 
  - Overlap wird am Ende des Chunks berechnet
  - Overlap wird am Anfang des nächsten Chunks eingefügt
  - Overlap wird nicht doppelt indiziert (nur einmal im Vector-Database)

**Dokument-Updates:**

**Incremental Update (wenn möglich):**
- **Änderungserkennung**: System erkennt geänderte Teile des Dokuments
- **Chunk-Vergleich**: Geänderte Chunks werden identifiziert
- **Selective Re-Indexing**: Nur geänderte Chunks werden neu indiziert
- **Embedding-Update**: Embeddings für geänderte Chunks werden neu erstellt
- **Vector-Database-Update**: Nur geänderte Vektoren werden aktualisiert

**Vollständige Re-Indizierung (falls nötig):**
- **Trigger**: Bei großen Änderungen (>50% des Dokuments) oder wenn Incremental nicht möglich
- **Prozess**: 
  1. Alte Chunks werden aus Vector-Database entfernt
  2. Dokument wird vollständig neu gechunkt
  3. Neue Embeddings werden erstellt
  4. Neue Chunks werden indiziert
- **Performance**: Wird im Hintergrund durchgeführt, blockiert nicht andere Operationen

### Search
- **Multi-Document-Retrieval**: Top-K mit Threshold als Minimum (K beste Dokumente über Threshold)
- **Similarity Search**: Similarity Search (Cosine, Dot Product, Euclidean)
- **Hybrid Search**: Hybrid Search (Vector + Keyword)
- **Filtering**: Filtering nach Metadaten
- **Re-ranking**: Re-ranking mit Cross-Encoders

## gRPC Communication

**gRPC Service Communication:**
- **Odin ↔ Freki**: gRPC für RAG-Services
- **Type-Safe**: Protobuf garantiert korrekte Service-Interfaces
- **Streaming**: Built-in Streaming für große Responses

**gRPC Connection-Management:**
- **Connection-Pooling**: Wiederverwendung von Verbindungen für bessere Performance
- **Connection Reuse**: Connections werden effizient wiederverwendet
- **Automatische Reconnection**: Kombination aus sofortigem Versuch + Exponential Backoff
  - Sofortiger Reconnect-Versuch bei Verbindungsabbruch
  - Nach erstem Fehler beginnt Exponential Backoff
  - Maximale Wartezeit (z.B. 60 Sekunden)
  - Kontinuierliche Versuche zur Wiederherstellung
- **Connection Monitoring**: Verbindungsstatus wird überwacht

**gRPC Error-Handling:**
- **gRPC Status-Codes**: gRPC-Fehler werden über Status-Codes behandelt
- **Retry-Mechanismen**: Automatischer Retry mit Exponential Backoff (siehe gemeinsame Klärungspunkte)
- **Timeout-Konfiguration**: Adaptive Timeouts mit Minimum/Maximum
- **Fallback**: Bei Fehler Fallback zu alternativen Routen

## Abhängigkeiten

### Keine Core Library

- **WICHTIG**: Es gibt keine Edda Core Library
- **Separate Projekte**: Wenn gemeinsame Komponenten benötigt werden (DTOs wie WolfRequest/WolfResponse, Protocols, Utils), sollte ein separates Projekt erstellt werden
- **Selektive Nutzung**: Dies hält Apps klein, da genau gewählt werden kann, was benötigt wird
- **Keine Abhängigkeit**: Freki sollte nicht auf Dateien/Protocols/Utils aus dem `edda` Verzeichnis verweisen (KEIN PROJEKT - nur Metadaten-Sammlung)

### Service-Abhängigkeiten

- **Odin**: Für Requests
- **Vector Database**: Für Embedding Storage
- **Embedding Models**: Für Embedding Generation
- **Document Storage**: Für Dokument-Verwaltung

## Integration

- **Odin**: Empfängt `WolfRequest` von Odin, sendet `WolfResponse` zurück
- **Geri**: Freki wird vor Geri verwendet, um Prompts anzureichern
- **Midgard**: Lokale Vector Database
- **Alfheim**: Cloud-based Vector Database (optimiert für Mobile)
- **Asgard**: Lokale Vector Database mit erweiterten Features

## Performance

### Caching-Strategien

**Gecachte Daten:**
- Embeddings für häufig verwendete Dokumente
- Häufige Queries und Retrieval-Ergebnisse
- Vector-Search-Ergebnisse
- Dokument-Metadaten

**Cache-Invalidierung:**
- Event-basiert: Bei Dokument-Updates, Index-Änderungen
- Timeout-basiert: Als Fallback, wenn Events fehlen
- Sofortige Invalidierung bei wichtigen Änderungen

**Cache-Sharing:**
- Kein direkter Cache-Sharing zwischen Devices
- Jedes Device hat eigenen Cache für optimale Performance

### Performance-Optimierungen
- **Indexing-Performance**: 
  - **Effiziente Indizierung**: Große Dokumentmengen werden effizient indiziert (Batch-Processing)
  - **Batch-Processing**: Batch-Processing für Indexing großer Dokumentmengen
  - **Indexing-Backlog**: Bei Indexing-Backlog werden Tasks priorisiert, wichtige Dokumente zuerst
- **Search-Performance**: 
  - **Search-Optimierung**: Optimierte Similarity Search für schnelle Retrieval
  - **Index-Optimierungen**: Optimierte Indizes für schnelle Vector-Search (HNSW, IVF, etc.)
  - **Search-Load**: Bei hoher Search-Load wird Caching verstärkt
- **Effiziente Vector Search**: Optimierte Similarity Search für schnelle Retrieval
- **Caching**: Intelligentes Caching für häufige Queries und Embeddings
- **Parallel Processing**: Parallele Verarbeitung für mehrere Queries
- **Embedding Caching**: Caching von Embeddings für häufig verwendete Dokumente

### Performance-Metriken
- Schnelle Vector-Search (< 100ms für Standard-Queries)
- Effizientes Indexing (< 1s pro Dokument)
- Hoher Durchsatz für parallele Queries

### Performance-Benchmarks (Tests)

Definierte Ziele und zugehörige Tests (siehe `IMPLEMENTATION_PLAN.md` Phase 19.2.1):

| Ziel | Schwellwert (CI) | Test |
|------|------------------|------|
| Vector-Search-Latenz | ≤ 150 ms | `tests/search_performance_test.rs` |
| Indexing pro Dokument | Ziel < 1 s | Unit/Load-Tests (Batch-Indexing) |
| Concurrent Queries (20) | ≤ 5 s gesamt | `tests/load_test.rs` (load_concurrent_retrieve_requests) |
| Batch-Indexing (10 Docs) | ≤ 15 s | `tests/load_test.rs` (load_batch_indexing_performance) |
| Parallel-Indexing | schneller als sequenziell | `tests/unit/parallel_indexing_perf_test.rs` |

Produktionsziele: Vector-Search < 100 ms, Indexing < 1 s/Dokument; CI-Schwellwerte sind bewusst großzügiger.

### Performance-Monitoring

**Performance-Monitoring:**
- Performance-Metriken: Response-Zeiten, Durchsatz, Resource-Usage
- Performance-Tracking für alle Queries und Indexing-Operations
- Kontinuierliche Überwachung und Performance-Optimierung
- Alerts bei Performance-Problemen

## Monitoring & Logging

### Strukturiertes Logging

**Strukturiertes Logging:**
- Structured Logging mit strukturierten Daten
- Log Levels: Verschiedene Log-Level (DEBUG, INFO, WARN, ERROR, etc.)
- Context Tracking: Context wird mitgeloggt
- Log Rotation: Automatische Log-Rotation
- Umfassendes Logging für Debugging und Monitoring

## Service-Ausfall-Behandlung

**Innerhalb einer Platform:**
- Fallback ist unnötig - Services müssen existieren, so bauen wir sie ja
- Services sind Teil der Platform-Installation

**Platformübergreifend:**
- Netzwerkplan verwenden für Service-Discovery
- Falls mit Yggdrasil verbunden: Netzwerkplan an Yggdrasil übertragen
- **WICHTIG**: Netzwerkplan darf unter keinen Umständen anderen Usern zugänglich gemacht werden

**Fallback-Strategien (nur platformübergreifend):**
- Alternative Vector-Database bei Ausfall
- Fallback zu alternativen Embedding-Models

**Service-Ausfall-Behandlung:**
- Automatischer Retry mit Exponential Backoff
- Sofortiger Fallback zu alternativen Services (nur platformübergreifend)
- User-Benachrichtigung bei komplettem Service-Ausfall

**User-Kommunikation:**
- Fehlermeldung an User, wenn alle Versuche fehlschlagen
- Error-Logging für Debugging
- User kann später erneut versuchen
- Transparente Fehlerbehandlung

## Datenschutz

### GDPR-Compliance

**Right to Deletion:**
- User kann alle Daten löschen ("Right to be forgotten")
- Sichere Datenlöschung
- Automatische Löschung nach Retention-Policy

**User-Rechte:**
- Right to Access: User können ihre Daten abrufen
- Right to Rectification: User können ihre Daten korrigieren
- Right to Data Portability: User können ihre Daten exportieren
- Right to Object: User können der Datenverarbeitung widersprechen

**Data-Minimization:**
- Nur notwendige Daten werden gespeichert
- Nur notwendige Daten werden indiziert
- Purpose Limitation: Daten nur für spezifische Zwecke verwendet
- Storage Limitation: Daten nur so lange gespeichert wie nötig

### Datenschutz-Features
- **Lokale Verarbeitung**: Dokumente werden bevorzugt lokal verarbeitet
- **Minimale Datensammlung**: Nur notwendige Daten werden indiziert
- **Keine Tracking-Daten**: Keine User-Tracking oder Analytics-Daten
- **Dokument-Privacy**: Dokumente bleiben lokal, werden nicht an Dritte weitergegeben
- **User Control**: User hat volle Kontrolle über indizierte Dokumente

### Compliance
- **GDPR-konform**: Unterstützung für GDPR-Anforderungen
- **Data Minimization**: Nur notwendige Daten werden indiziert
- **Right to Deletion**: User kann alle indizierten Dokumente löschen
- **Transparency**: User wird über Dokument-Verarbeitung informiert

## Sicherheit

### Security-Features
- **Secure Document Storage**: Verschlüsselte Speicherung von Dokumenten (optional)
- **Access Control**: Zugriffskontrolle für indizierte Dokumente
- **Input Validation**: Validierung aller Dokument-Inputs
- **Malware Scanning**: Optional Scanning von Dokumenten auf Malware
- **Audit Logging**: Logging aller Dokument-Zugriffe für Security-Audits

### Security-Best-Practices
- **Keine Hardcoded Secrets**: Keine Hardcoded Passwords oder Keys
- **Secure Defaults**: Sichere Standard-Konfigurationen
- **Regular Updates**: Automatische Security-Updates
- **Vulnerability Scanning**: Regelmäßiges Scanning für bekannte Vulnerabilities
- **Document Sanitization**: Sanitization von Dokumenten zum Schutz vor schädlichem Content

## Document Indexing Strategy

### Automatische Dokument-Indizierung

#### Kombination: Watch-Folder + manuelle Hinzufügung

**Watch-Folder (Automatisch)**
- **System überwacht Ordner**: System überwacht konfigurierte Ordner/Folder
- **Automatische Indizierung**: Neue oder geänderte Dateien werden automatisch indiziert
- **Kontinuierlich**: Indizierung erfolgt kontinuierlich im Hintergrund
- **User muss nichts tun**: User muss Dokumente nicht manuell hinzufügen

**Manuelle Hinzufügung (Optional)**
- **User KANN hinzufügen**: User kann neue Dokumente manuell hinzufügen
- **Optional**: Manuelle Hinzufügung ist optional, nicht verpflichtend
- **Flexibilität**: User hat Kontrolle über Indizierung

### Workflow: Dokument-Indizierung

1. **Dokument erkannt**
   - **Automatisch**: System erkennt neues/geändertes Dokument in Watch-Folder
   - **ODER manuell**: User fügt Dokument manuell hinzu

2. **Dokument-Verarbeitung**
   - Dokument wird geladen
   - Dokument-Typ wird erkannt (PDF, TXT, MD, DOCX, etc.)
   - Dokument wird geparst

3. **Chunking**
   - Dokument wird in Chunks aufgeteilt (siehe Chunking-Strategie)
   - Chunks werden erstellt

4. **Embedding-Erstellung**
   - Embeddings werden für jeden Chunk erstellt
   - Typ-spezifisches oder einheitliches Model wird verwendet

5. **Indizierung**
   - Chunks werden in Vector-Database indiziert
   - Metadaten werden gespeichert

### Chunking-Strategie

#### Kombination: Semantisches Chunking mit Max-Größe

**Semantisches Chunking**
- **Basierend auf Semantik**: Chunks werden basierend auf semantischer Bedeutung erstellt
- **Natürliche Grenzen**: Chunks enden an natürlichen Grenzen (z.B. Absätze, Kapitel)
- **Bessere Qualität**: Semantisches Chunking führt zu besseren Retrieval-Ergebnissen

**Max-Größe**
- **Maximale Chunk-Größe**: Chunks haben maximale Größe (z.B. 1000 Tokens)
- **Verhindert zu große Chunks**: Verhindert, dass Chunks zu groß werden
- **Context-Window**: Chunks passen in LLM Context-Window

#### Chunking-Parameter

**Semantische Grenzen**
- **Absätze**: Chunks enden an Absatz-Grenzen
- **Kapitel**: Chunks enden an Kapitel-Grenzen
- **Sätze**: Falls nötig, Chunks enden an Satz-Grenzen

**Max-Größe**
- **Token-Limit**: Maximale Anzahl von Tokens pro Chunk (z.B. 1000 Tokens)
- **Overlap**: Chunks können sich überlappen (z.B. 100 Tokens Overlap) für besseren Kontext
- **Min-Größe**: Minimale Chunk-Größe (z.B. 200 Tokens) für sinnvolle Chunks

### Dokument-Updates

#### Kombination: Incremental wenn möglich, sonst vollständige Re-Indizierung

**Incremental Update (wenn möglich)**
- **Nur geänderte Teile**: Nur geänderte Teile werden aktualisiert
- **Effizient**: Reduziert Rechenaufwand
- **Schnell**: Updates erfolgen schnell

**Vollständige Re-Indizierung (falls nötig)**
- **Falls Incremental nicht möglich**: Falls Incremental-Update nicht möglich ist, vollständige Re-Indizierung
- **Bei großen Änderungen**: Bei großen Änderungen wird vollständig re-indiziert
- **Sicherheit**: Garantiert Konsistenz

### Multi-Document-Retrieval

#### Kombination: Top-K mit Threshold als Minimum

**Top-K**
- **K beste Dokumente**: K beste Dokumente werden zurückgegeben (z.B. Top-5)
- **Ranking**: Dokumente werden nach Relevanz gerankt
- **Effizient**: Reduziert Anzahl der zurückgegebenen Dokumente

**Threshold als Minimum**
- **Minimum-Relevanz**: Nur Dokumente über bestimmter Relevanz (Threshold) werden berücksichtigt
- **Qualität**: Verhindert, dass irrelevante Dokumente zurückgegeben werden
- **Konfigurierbar**: Threshold kann konfiguriert werden

### Embedding-Erstellung

#### Kombination: Einheitliches Model als Standard, typ-spezifische als Option

**Einheitliches Model (Standard)**
- **Standard-Model**: Alle Dokument-Typen verwenden standardmäßig dasselbe Embedding-Model
- **Einfach**: Einfache Konfiguration
- **Konsistent**: Konsistente Embeddings über alle Dokument-Typen

**Typ-spezifische Models (Option)**
- **Optional**: User kann typ-spezifische Models für bestimmte Dokument-Typen konfigurieren
- **Optimiert**: Typ-spezifische Models können für bestimmte Typen optimiert sein
- **Flexibilität**: User hat Flexibilität bei Bedarf

## Implementierungs-Notizen

- Sollte verschiedene Vector DBs unterstützen (Plugin-Architektur)
- Muss effizientes Chunking für große Dokumente haben
- Sollte Caching für häufige Queries haben
- Muss Batch-Processing für Indexing unterstützen
- Sollte Monitoring für Search-Performance haben
- Muss automatische Indizierung unterstützen
- Sollte semantisches Chunking implementieren
- Muss Multi-Document-Retrieval unterstützen
- Sollte Hybrid Search unterstützen
- **Muss Watch-Folder-Funktionalität haben**: Automatische Indizierung von neuen/geänderten Dateien
- **Muss manuelle Dokument-Hinzufügung unterstützen**: Optional für User
- **Muss semantisches Chunking mit Max-Größe haben**: Chunks basierend auf Semantik, max. Größe beachten
- **Muss Incremental Updates haben**: Wenn möglich, nur geänderte Teile aktualisieren
- **Muss vollständige Re-Indizierung haben**: Falls nötig, komplett neu indizieren
- **Muss Top-K mit Threshold haben**: K beste Dokumente über Threshold
- **Muss einheitliches Embedding-Model als Standard haben**: Standard für alle Dokument-Typen
- **Muss typ-spezifische Models als Option unterstützen**: Optional für User
- **Performance**: Muss optimiert sein für schnelle Vector-Search und effizientes Indexing
- **Datenschutz**: Muss Privacy-by-Design implementieren und Dokument-Privacy gewährleisten
- **Sicherheit**: Muss robuste Security-Mechanismen haben, insbesondere für Dokument-Verarbeitung


# Freki - RAG Service

## Übersicht

Freki ist einer von Odins Wölfen und stellt den RAG (Retrieval Augmented Generation) Service bereit. Er wird zuerst verwendet, um Prompts mit relevantem Kontext anzureichern, bevor sie an Geri (LLM) weitergegeben werden.

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
   - Formatiert Context für LLM
   - Erstellt `RAGContext` mit retrieved Documents

7. **Response**
   - Sendet `WolfResponse` mit `ragContext` zurück
   - Odin verwendet `ragContext` für LLM-Request

## Technische Anforderungen

### Vector Database
- Unterstützung für verschiedene Vector DBs:
  - Qdrant
  - Chroma
  - Pinecone (Cloud)
  - Weaviate
  - Milvus

### Embedding Models
- **Einheitliches Model (Standard)**: Alle Dokument-Typen verwenden standardmäßig dasselbe Embedding-Model
- **Typ-spezifische Models (Option)**: User kann typ-spezifische Models für bestimmte Dokument-Typen konfigurieren
- Unterstützung für verschiedene Embedding-Models:
  - OpenAI Embeddings
  - Sentence Transformers
  - BGE Models
  - Custom Models

### Indexing
- **Automatische Indizierung**: Watch-Folder überwacht Ordner, indiziert neue/geänderte Dateien automatisch
- **Manuelle Hinzufügung**: User kann Dokumente manuell hinzufügen (optional)
- **Chunking-Strategie**: Semantisches Chunking mit Max-Größe (Chunks basierend auf Semantik, max. z.B. 1000 Tokens)
- **Dokument-Updates**: Incremental Update wenn möglich, sonst vollständige Re-Indizierung
- **Batch Indexing**: Batch Indexing für große Dokumentmengen
- **Metadata Indexing**: Metadaten werden indiziert

### Search
- **Multi-Document-Retrieval**: Top-K mit Threshold als Minimum (K beste Dokumente über Threshold)
- **Similarity Search**: Similarity Search (Cosine, Dot Product, Euclidean)
- **Hybrid Search**: Hybrid Search (Vector + Keyword)
- **Filtering**: Filtering nach Metadaten
- **Re-ranking**: Re-ranking mit Cross-Encoders

## Abhängigkeiten

- **Odin**: Für Requests
- **Vector Database**: Für Embedding Storage
- **Embedding Models**: Für Embedding Generation
- **Document Storage**: Für Dokument-Verwaltung
- **Edda Core Library**: DTOs (WolfRequest, WolfResponse)

## Integration

- **Odin**: Empfängt `WolfRequest` von Odin, sendet `WolfResponse` zurück
- **Geri**: Freki wird vor Geri verwendet, um Prompts anzureichern
- **Midgard**: Lokale Vector Database
- **Alfheim**: Cloud-based Vector Database (optimiert für Mobile)
- **Asgard**: Lokale Vector Database mit erweiterten Features

## Performance

### Performance-Optimierungen
- **Effiziente Vector Search**: Optimierte Similarity Search für schnelle Retrieval
- **Caching**: Intelligentes Caching für häufige Queries und Embeddings
- **Batch Processing**: Batch-Processing für Indexing großer Dokumentmengen
- **Parallel Processing**: Parallele Verarbeitung für mehrere Queries
- **Index Optimization**: Optimierte Indizes für schnelle Vector-Search
- **Embedding Caching**: Caching von Embeddings für häufig verwendete Dokumente

### Performance-Metriken
- Schnelle Vector-Search (< 100ms für Standard-Queries)
- Effizientes Indexing (< 1s pro Dokument)
- Hoher Durchsatz für parallele Queries

## Datenschutz

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


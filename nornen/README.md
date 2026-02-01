# Nornen (Urd, Verdandi) - Decision Service

## Übersicht

**Tests ausführen:** Von `nornen/`: `docker compose -f docker-compose.test.yml run --rm nornen-test` oder `./scripts/run-tests.sh` / `.\scripts\run-tests.ps1`. Von Repo-Root: `nornen/scripts/run-tests.sh` bzw. `.\nornen\scripts\run-tests.ps1`. **CI:** Bei Push/PR auf `nornen/**` läuft die Pipeline [.github/workflows/nornen.yml](../.github/workflows/nornen.yml) (Test im Container, Lint).

Nornen ist der Decision Service bei Yggdrasil, der Entscheidungen über eingehende/ausgehende Requests trifft, Provider-Registrierungen verwaltet und die Admin API bereitstellt.

**Mythologische Bedeutung**: Die Nornen sind die Schicksalsgöttinnen. Urd (Vergangenheit), Verdandi (Gegenwart). **Hinweis**: Skuld (Zukunft) ist ein separater Service, der auf allen Devices installiert werden muss.

**Programmiersprache**: Rust

**Wichtig**: Nornen ist nur bei Yggdrasil verfügbar. Skuld ist ein separater Service, der auf allen Devices installiert werden muss.

## Verantwortlichkeiten

### 1. Entscheidungen über Requests
- **Request-Entscheidungen**: 
  - **Entscheidungs-Algorithmen**: Intelligente Entscheidungs-Algorithmen für Request-Entscheidungen (Multi-Faktor-Bewertung)
  - **Entscheidungs-Konflikte**: Bei Entscheidungs-Konflikten wird höchste Priorität gewählt oder User wird um Klärung gebeten
  - Entscheidet über eingehende/ausgehende Requests
- **Request-Validation**: 
  - **Validation-Regeln**: Validation-Regeln basierend auf Business-Logik (Schema-Validierung, Policy-Checks)
  - **Validation-Fehler**: Bei Validation-Fehlern wird Fehler zurückgegeben, Request wird abgelehnt
  - Validiert Requests basierend auf Business-Logik
- **Request-Routing**: 
  - **Routing-Strategien**: Intelligente Routing-Strategien (Best-Path, Load-Balancing, etc.)
  - **Routing-Fehler**: Bei Routing-Fehlern wird automatisch auf alternative Route ausgewichen
  - Entscheidet über Request-Routing
- **Request-Priorisierung**: 
  - **Priorisierungs-Strategien**: Priorisierungs-Strategien (Critical > High > Medium > Low)
  - **Priorisierungs-Konflikte**: Bei Priorisierungs-Konflikten gewinnt höhere Priorität
  - Priorisiert Requests basierend auf verschiedenen Faktoren

### 2. Provider-Registrierung
- **Provider-Genehmigung**: Genehmigt oder lehnt Provider-Registrierungen ab
- **Provider-Verwaltung**: Verwaltet Provider-Registrierungen
- **Provider-Validation**: Validiert Provider-Capabilities und Requirements
- **Provider-Monitoring**: Überwacht Provider-Performance

### 3. User-Konfiguration
- **Konfiguration-Speicherung**: Speichert User-Konfiguration für Marketplace
- **Konfiguration-Verwaltung**: Verwaltet User-Konfigurationen
- **Konfiguration-Validation**: Validiert User-Konfigurationen
- **Konfiguration-Synchronisation**: Synchronisiert Konfigurationen zwischen Devices

### 4. Admin API
- **Health Check**: Health-Check-Endpoints für Monitoring
- **Dashboard-Informationen**: Bereitstellung von Dashboard-Daten
- **Monitoring-Daten**: Bereitstellung von Monitoring-Daten
- **Admin-Informationen**: Alle Informationen, die Admins benötigen

### 5. Analytics (Urd & Verdandi)
- **Urd (Vergangenheit)**: Historie, Request-History, historische Statistiken
- **Verdandi (Gegenwart)**: Aktuelle Statistiken, Real-time Analytics, Live-Metriken

## Service-Interfaces

### Inputs
- `DecisionRequest` (von Nidhöggr) - Requests für Entscheidungen
  - Provider-Registration-Requests
  - Request-Validation-Requests
  - Configuration-Requests
  - Admin-Requests

- `AnalyticsRequest` (von Yggdrasil) - Requests für Analytics
  - Provider-Analytics-Requests
  - Requester-Analytics-Requests
  - Historical-Data-Requests

### Outputs
- `DecisionResponse` (an Nidhöggr) - Entscheidungen und Responses
- `AnalyticsResponse` (an Yggdrasil) - Analytics-Daten und Statistiken

## Workflow

### Provider-Registrierung

1. **Provider-Registration-Request**
   - User sendet Provider-Registration-Request über Vedrfolnir
   - Nidhöggr leitet Request an Nornen weiter

2. **Nornen validiert Request**
   - Urd analysiert Provider-Historie (falls vorhanden)
   - Verdandi prüft aktuelle Provider-Requirements
   - Provider-Capabilities werden validiert
   - Provider-Requirements werden geprüft

3. **Entscheidung**
   - Nornen entscheidet über Genehmigung/Ablehnung
   - Bei Genehmigung: Provider wird registriert
   - Bei Ablehnung: Begründung wird zurückgegeben

4. **Response**
   - Nornen sendet Decision-Response an Nidhöggr
   - Nidhöggr sendet Response über Ratatoskr-Protocol zurück

### Request-Entscheidungen

1. **Request kommt an**
   - Request kommt über Nidhöggr an
   - Nidhöggr leitet Request an Nornen weiter

2. **Nornen analysiert Request**
   - Urd analysiert Request-Historie
   - Verdandi prüft aktuelle Request-Requirements
   - Request wird validiert

3. **Entscheidung**
   - Nornen entscheidet über Request-Behandlung
   - Request-Routing wird bestimmt
   - Request-Priorisierung wird festgelegt

4. **Response**
   - Nornen sendet Decision-Response
   - Request wird entsprechend behandelt

### Admin API

1. **Admin-Request**
   - Admin sendet Request an Admin-API
   - Request kommt über Nidhöggr an

2. **Nornen verarbeitet Request**
   - Urd liefert historische Daten
   - Verdandi liefert aktuelle Daten
   - Dashboard-Informationen werden zusammengestellt

3. **Response**
   - Nornen sendet Response mit Admin-Informationen
   - Dashboard-Daten werden zurückgegeben

## Der Brunnen (Mímisbrunnr)

**Mythologische Bedeutung**: Der Brunnen Mímisbrunnr ist die Quelle der Weisheit. In diesem Kontext ist der Brunnen die Datenbank, die von Mimir verwaltet wird.

**Datenbank-Verwaltung:**
- **Mimir verwaltet Datenbank**: Mimir (Privacy Database Service) verwaltet die Datenbank
- **Nornen nutzen Datenbank**: Nornen nutzen die Datenbank für Entscheidungen und Analytics
- **Datenbank-Zugriff**: Nornen greifen über Mimir auf die Datenbank zu

## Analytics-Features

### Provider Analytics
- **Request Statistics**: Anzahl von Requests pro Provider
- **Earnings**: Verdienst-Statistiken pro Provider
- **Quality Metrics**: Quality-Metriken pro Provider
- **Usage Patterns**: Nutzungsmuster pro Provider

### Requester Analytics
- **Request History**: Request-Historie pro Requester
- **Cost Analysis**: Kosten-Analyse pro Requester
- **Quality Metrics**: Quality-Metriken der verwendeten Provider
- **Usage Patterns**: Nutzungsmuster pro Requester

### Aggregation
- **Zeitbasierte Aggregation**: Aggregation von Daten über Zeiträume
- **Trend-Analyse**: Erkennung von Trends und Mustern
- **Predictive Analytics**: Vorhersage von Trends (durch Skuld, wenn verfügbar)

## Abhängigkeiten

### Keine Core Library

- **WICHTIG**: Es gibt keine Edda Core Library
- **Separate Projekte**: Wenn gemeinsame Komponenten benötigt werden (DTOs, Protocols, Utils), sollte ein separates Projekt erstellt werden
- **Selektive Nutzung**: Dies hält Apps klein, da genau gewählt werden kann, was benötigt wird
- **Keine Abhängigkeit**: Nornen sollte nicht auf Dateien/Protocols/Utils aus dem `edda` Verzeichnis verweisen (KEIN PROJEKT - nur Metadaten-Sammlung)

### Service-Abhängigkeiten

- **Mimir**: Privacy Database Service (für Datenbank-Zugriff)
- **Yggdrasil (Elixir)**: Hauptprozess, koordiniert alle Services

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

### Nornen-spezifische Settings

**Settings-Datei**: `config/nornen.json` (Beispiel: `config/nornen.json.example`)

**Settings-Struktur**:
```json
{
  "database": {
    "url": "postgresql://user:password@localhost:5432/nornen"
  },
  "mimir": {
    "url": "http://localhost:50051"
  },
  "grpc_port": 50052,
  "cache": {
    "enabled": true,
    "ttl_seconds": 300,
    "max_size": 1000
  },
  "audit": {
    "enabled": true,
    "backend": "postgres" // oder "mimir" oder "composite"
  },
  "monitoring": {
    "enabled": true
  }
}
```

**Settings-Felder**:
- `database.url`: PostgreSQL-Verbindungsstring (optional, wenn Mimir verwendet wird)
- `mimir.url`: Mimir gRPC-Service-URL (optional, wenn PostgreSQL verwendet wird)
- `grpc_port`: Port für den gRPC-Server (Standard: 50052)
- `cache.enabled`: Aktiviert Provider-Cache (Standard: true)
- `cache.ttl_seconds`: Time-to-Live für Cache-Einträge in Sekunden (Standard: 300)
- `cache.max_size`: Maximale Anzahl von Cache-Einträgen (Standard: 1000)
- `audit.enabled`: Aktiviert Audit-Logging (Standard: true)
- `audit.backend`: Audit-Backend ("postgres", "mimir", oder "composite")
- `monitoring.enabled`: Aktiviert Monitoring-Metriken (Standard: true)

**Hot-Reload**: Ja - Settings können zur Laufzeit neu geladen werden (via `notify` + `Arc<RwLock<Settings>>`)

## Integration

- **Nidhöggr**: Empfängt Requests von Nidhöggr
- **Mimir**: Nutzt Mimir für Datenbank-Zugriff
- **Yggdrasil (Elixir)**: Hauptprozess, koordiniert alle Services
- **Heidrun**: Für Payment-Informationen (falls nötig)
- **Eikthyrnir**: Für Quality-Metriken (falls nötig)

## Performance

### Performance-Optimierungen
- **Caching**: Intelligentes Caching für häufig abgerufene Daten
- **Async Processing**: Asynchrone Verarbeitung von Requests
- **Database-Optimization**: Optimierte Datenbankabfragen über Mimir
- **Batch-Processing**: Batch-Verarbeitung für Analytics-Requests

### Performance-Metriken
- Schnelle Entscheidungen (< 100ms für Standard-Entscheidungen)
- Effiziente Analytics-Abfragen (< 200ms für Standard-Analytics)
- Hoher Durchsatz (100+ Requests/Sekunde pro Instanz)

## Architektur

### Komponenten

#### Urd (Provider Registry)
- **Zweck**: Verwaltung von Provider-Registrierungen
- **Backend**: Mimir (primär) oder PostgreSQL (Legacy)
- **Features**:
  - Provider-Registrierung und -Updates
  - Capability-basierte Provider-Suche
  - Provider-Status-Verwaltung
  - Automatische Cache-Invalidierung bei Änderungen
  - Audit-Logging für alle Provider-Operationen

#### Verdandi (Request Router)
- **Zweck**: Intelligentes Routing von Requests zu Providern
- **Features**:
  - Capability-basierte Provider-Auswahl
  - Preference-basiertes Scoring
  - Round-Robin Load-Balancing
  - Fallback-Routing bei Fehlern
  - Cache-Integration für Performance

#### Nornen Coordinator
- **Zweck**: Orchestrierung von Urd und Verdandi
- **Features**:
  - Koordination von Request-Entscheidungen
  - Metriken-Sammlung
  - Health-Checks

### Datenbank-Backend

#### Mimir (Primär)
- **Verwendung**: Speicherung von Provider-Daten und Audit-Logs
- **Vorteile**: Privacy-fokussiert, GDPR-konform, zentrale Verwaltung
- **gRPC-Protokoll**: Kommunikation über Mimir gRPC-Service

#### PostgreSQL (Legacy)
- **Verwendung**: Fallback für Legacy-Systeme
- **Schema**: Siehe `migrations/001_initial_schema.sql`

### Caching

**ProviderCache**: In-Memory-Cache für Provider-Abfragen
- **TTL-basiert**: Einträge laufen nach konfigurierbarer Zeit ab
- **LRU-Eviction**: Älteste Einträge werden bei Max-Größe entfernt
- **Automatische Invalidierung**: Cache wird bei Provider-Änderungen automatisch invalidiert
- **Statistiken**: Cache-Hits/Misses werden getrackt

### Security & Access Control

**Role-Based Access Control (RBAC)**:
- **Rollen**:
  - `User`: Kann Provider abfragen und Requests koordinieren
  - `Provider`: Kann eigene Provider registrieren und aktualisieren
  - `Admin`: Vollzugriff auf alle Operationen
- **Berechtigungen**: Granulare Berechtigungen pro Operation
- **Authentifizierung**: User-ID wird aus gRPC-Metadaten extrahiert (`user_id` Header oder `authorization` Header)

### Audit-Logging

**Strukturiertes Audit-System**:
- **Backends**: PostgreSQL, Mimir, oder Composite (beide)
- **Events**: Provider-Registrierung, Updates, Status-Änderungen
- **Daten**: Event-Typ, Entity-ID, User-ID, Timestamp, Details (JSON)

### Monitoring

**MetricsCollector**:
- **Request-Metriken**: Gesamt/Success/Failed Requests, Durchschnittliche Response-Zeit, Requests/Sekunde
- **Provider-Metriken**: Per-Provider Statistiken (Requests, Erfolgsrate, Response-Zeit, Last Used)
- **System-Metriken**: Cache-Statistiken (Hits, Misses, Size, Hit-Rate)
- **Exposition**: Metriken über Coordinator verfügbar

## API-Dokumentation

### gRPC Services

#### NornenService
- **coordinate_request**: Koordiniert Request-Entscheidungen
  - **Berechtigung**: `CoordinateRequest`
  - **Input**: `CoordinateRequest` (request_id, request_type, context)
  - **Output**: `CoordinateResponse` (decision, provider_id, confidence, reasoning)

#### UrdService
- **register_provider**: Registriert einen neuen Provider
  - **Berechtigung**: `RegisterProvider`
  - **Input**: `RegisterProviderRequest` (provider_id, name, capabilities, endpoint, metadata)
  - **Output**: `RegisterProviderResponse` (success, message)

- **update_provider**: Aktualisiert einen bestehenden Provider
  - **Berechtigung**: `UpdateProvider`
  - **Input**: `UpdateProviderRequest` (provider_id, capabilities, status, metadata)
  - **Output**: `UpdateProviderResponse` (success)

- **query_providers**: Sucht Provider nach Capabilities
  - **Berechtigung**: `QueryProviders`
  - **Input**: `QueryProvidersRequest` (capabilities, status)
  - **Output**: `QueryProvidersResponse` (providers)

- **list_providers**: Listet alle Provider auf
  - **Berechtigung**: `ListProviders` (nur Admin)
  - **Input**: `ListProvidersRequest` (limit, offset)
  - **Output**: `ListProvidersResponse` (providers, total)

#### VerdandiService
- **route_request**: Routet einen Request zu einem Provider
  - **Berechtigung**: `CoordinateRequest`
  - **Input**: `RouteRequestRequest` (required_capabilities, context)
  - **Output**: `RouteRequestResponse` (provider_id, endpoint, confidence)

### Authentifizierung

**gRPC-Metadaten**:
- **Option 1**: `user_id` Header mit User-ID als Wert
- **Option 2**: `authorization` Header mit Format `Bearer <user_id>` (für zukünftige JWT-Integration)

**Beispiel** (mit `grpcurl`):
```bash
grpcurl -plaintext \
  -H "user_id: admin" \
  -d '{"provider_id": "test", "name": "Test Provider", "capabilities": ["llm"], "endpoint": "http://localhost:8080"}' \
  localhost:50052 nornen.UrdService/RegisterProvider
```

## Entwickler-Guide

### Projekt-Struktur

```
nornen/
├── src/
│   ├── main.rs              # Entry-Point
│   ├── lib.rs               # Library-Root
│   ├── urd/                 # Provider Registry (Urd)
│   │   └── registry.rs
│   ├── verdandi/            # Request Router (Verdandi)
│   │   └── router.rs
│   ├── coordinator/         # Nornen Coordinator
│   │   └── mod.rs
│   ├── grpc/                # gRPC Server
│   │   └── server.rs
│   ├── mimir_client/        # Mimir gRPC Client
│   │   └── client.rs
│   ├── cache/               # Provider Cache
│   │   └── provider_cache.rs
│   ├── audit/               # Audit Logging
│   │   └── logger.rs
│   ├── monitoring/          # Monitoring & Metrics
│   │   ├── metrics.rs
│   │   └── collector.rs
│   ├── security/            # Access Control
│   │   └── access_control.rs
│   └── utils/               # Utilities
│       └── config.rs
├── proto/                   # Protobuf Definitions
│   ├── nornen.proto
│   └── mimir/
│       └── mimir.proto
├── tests/                   # Tests
│   ├── unit/                # Unit Tests
│   ├── integration/         # Integration Tests
│   ├── mocks/               # Mock Services
│   └── utils/               # Test Utilities
├── migrations/              # Database Migrations
│   └── 001_initial_schema.sql
├── docker-compose.test.yml  # Test-Container-Setup
└── Cargo.toml
```

### Tests ausführen

**Alle Tests**:
```bash
# Von nornen/ Verzeichnis
docker compose -f docker-compose.test.yml run --rm nornen-test

# Oder mit Scripts
./scripts/run-tests.sh      # Linux/Mac
.\scripts\run-tests.ps1     # Windows
```

**Spezifische Tests**:
```bash
# Unit Tests
cargo test --lib

# Integration Tests
cargo test --test '*'

# Spezifischer Test
cargo test --lib access_control_test
```

### Entwicklung

**TDD-Workflow**:
1. Tests schreiben (Red)
2. Minimaler Code für grüne Tests (Green)
3. Refactoring (Refactor)

**Container-basierte Tests**:
- Alle Tests laufen in Containern
- Keine lokalen Dependencies erforderlich
- Mock-Services für externe Abhängigkeiten (z.B. Mock-Mimir)

### Code-Standards

- **TDD**: Test-Driven Development ist verpflichtend
- **Container-Tests**: Alle Tests müssen in Containern laufen
- **Error-Handling**: `thiserror` für strukturierte Fehler
- **Async**: `tokio` für asynchrone Operationen
- **Logging**: `tracing` für strukturiertes Logging

## Implementierungs-Notizen

- **Programmiersprache**: Rust
- **Nur bei Yggdrasil**: Nornen ist nur bei Yggdrasil verfügbar
- **Skuld ist separater Service**: Skuld ist nicht Teil von Nornen
- **Mimir-Integration**: Nutzt Mimir für Datenbank-Zugriff (primär)
- **PostgreSQL-Fallback**: Unterstützt PostgreSQL für Legacy-Systeme
- **gRPC-Kommunikation**: Kommuniziert mit anderen Services über gRPC
- **Caching**: ProviderCache für Performance-Optimierung
- **Security**: RBAC-basiertes Access-Control-System
- **Audit-Logging**: Strukturiertes Logging zu PostgreSQL/Mimir
- **Monitoring**: Metriken-Sammlung für Request- und Provider-Statistiken
- **Performance**: Optimiert für schnelle Entscheidungen und Analytics


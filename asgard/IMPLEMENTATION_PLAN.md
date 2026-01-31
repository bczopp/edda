# IMPLEMENTATION_PLAN - Asgard (Homeserver Platform)

## Übersicht

Dieser Plan beschreibt die kleinstmöglichen Schritte zur Implementierung von Asgard - der Homeserver Platform. Asgard ist eine Platform, die sich um Connections (Netzwerk, UI, etc.) kümmert, Device-Registry, Network-Management, Message-Routing, Lock-Management und weitere Server-Features bereitstellt.

## Offene Fragen (USER-INPUT ERFORDERLICH)

### Database-Wahl
❓ **FRAGE AN USER**: Welche Database soll für Asgard verwendet werden?
- Option A: PostgreSQL (Production-Server mit hohem Durchsatz, robuste Features)
- Option B: SQLite (Einfache Setups, Development, weniger Overhead)
- Option C: Beide unterstützen (PostgreSQL für Production, SQLite für Development/Testing)

**Auswirkung**: Beeinflusst die Database-Migrations, Performance-Optimierungen und Deployment-Strategie.

### Web Dashboard
❓ **FRAGE AN USER**: Soll ein optionales Web Dashboard in Phase 1 implementiert werden?
- Option A: Ja - Web Dashboard in Phase 1 (ermöglicht User nachzuvollziehen, was passiert)
- Option B: Nein - Web Dashboard in späteren Phasen (fokussiert auf Core-Funktionalität)
- Option C: Minimal - nur Admin-Panel in Phase 1, vollständiges Dashboard später

**Auswirkung**: Beeinflusst die Frontend-Implementierung und Projektstruktur.

### Web Dashboard Framework (falls Web Dashboard in Phase 1)
❓ **FRAGE AN USER**: Welches Framework für das Web Dashboard?
- Option A: React (große Community, viele Libraries)
- Option B: Vue (einfacher, leichtgewichtig)
- Option C: Svelte (modern, sehr performant)

**Auswirkung**: Beeinflusst die Frontend-Tooling und Build-Pipeline.

### API-Framework
❓ **FRAGE AN USER**: Welches Framework für REST API?
- Option A: Axum (modernes Rust Web-Framework, type-safe, async)
- Option B: Actix-Web (sehr performant, bewährt)
- Option C: Warp (leichtgewichtig, Filter-basiert)

**Auswirkung**: Beeinflusst die API-Implementierung und Performance.

### NAT-Traversal Bibliotheken
❓ **FRAGE AN USER** (aus unclear-items-analysis.md): Welche Rust-Bibliotheken für NAT-Traversal?
- Option A: `webrtc-rs` (vollständige WebRTC-Implementierung, STUN/TURN/ICE)
- Option B: Separate Libraries (`stun-rs`, `turn-rs`, `ice-rs`)
- Option C: Eigene Implementierung (mehr Kontrolle, mehr Aufwand)

**Auswirkung**: Beeinflusst die WAN-Connectivity und NAT-Traversal-Features (Phase 4).

### Protobuf-Code-Generierung
❓ **FRAGE AN USER**: Welches Tool für Protobuf → Rust-Generierung?
- Option A: `prost` (moderne, idiomatische Rust-Generierung)
- Option B: `protobuf-rust` (klassischer Generator)
- Option C: `tonic` mit `prost` (für gRPC + Protobuf)

**Auswirkung**: Beeinflusst die Code-Generierungs-Pipeline.

---

## Phase 1: Projekt-Setup & Grundstruktur

### 1.1 Projekt-Initialisierung

**Abhängigkeiten**: Keine
**Erforderliche USER-Eingaben**: Database-Wahl, API-Framework-Wahl

#### 1.1.1 Cargo-Projekt erstellen
- [x] `Cargo.toml` mit Workspace-Struktur erstellen
- [x] Basis-Dependencies definieren
  - Async Runtime (tokio)
  - Serialization (serde, serde_json)
  - Logging (tracing, tracing-subscriber)
  - Error-Handling (anyhow, thiserror)
- [x] `.gitignore` erstellen

#### 1.1.2 Verzeichnisstruktur erstellen
- [x] `src/main.rs` erstellen
- [x] `src/lib.rs` erstellen
- [x] `src/server/` für Server-Core erstellen
- [x] `src/services/` für Server-Services erstellen
- [x] `src/api/` für API-Endpoints erstellen
- [x] `src/utils/` für Utilities erstellen
- [ ] `config/` für Konfigurationsdateien erstellen
- [ ] `migrations/` für Database-Migrations erstellen
- [x] `tests/` für Tests erstellen

#### 1.1.3 Build-System einrichten
- [ ] Build-Scripts in `Cargo.toml` definieren
- [ ] Code-Generierungs-Pipeline einrichten (Protobuf → Rust)
- [ ] Cargo-Features definieren (z.B. `postgres`, `sqlite`)

### 1.2 Test-Infrastruktur

**Abhängigkeiten**: 1.1 (Projekt-Initialisierung)

#### 1.2.1 Container-Setup für Tests
- [x] `Dockerfile` für Test-Umgebung erstellen (Dockerfile.test)
- [x] Docker Compose für Test-Services konfigurieren (docker-compose.test.yml)
  - Database-Container (PostgreSQL/SQLite)
  - Mock-Services (Odin, Heimdall, etc.)
- [ ] Test-Container-Startup-Scripts erstellen
- [ ] **WICHTIG**: Alle Tests müssen in Containern laufen - keine lokalen Dependencies, Tools oder Services auf der Entwicklungsmaschine installieren

#### 1.2.2 Test-Framework konfigurieren
- [ ] Test-Dependencies hinzufügen (tokio-test, mockall, etc.)
- [ ] Test-Utilities und Helpers erstellen
- [ ] Mock-Setup für Services (Odin, Heimdall, Bifrost, etc.)
- [ ] Test-Database-Setup (automatisches Schema-Setup)

#### 1.2.3 CI/CD-Pipeline
- [ ] GitHub Actions / GitLab CI Workflow erstellen
- [ ] Automatische Test-Ausführung bei Commits konfigurieren
- [ ] Code-Coverage-Reporting einrichten (cargo-tarpaulin)
- [ ] Linting und Formatting (cargo clippy, cargo fmt)

### 1.3 Projekt-Konfiguration

**Abhängigkeiten**: 1.1 (Projekt-Initialisierung)

#### 1.3.1 Settings-System Design
- [ ] Settings-Schema definieren (JSON-Format oder TOML)
- [ ] Settings-Struktur entwerfen (Asgard-spezifisch)
  - Server-Konfiguration (Port, IP, etc.)
  - Database-Konfiguration
  - Service-Endpoints
  - Device-Registry-Einstellungen
  - Network-Management-Einstellungen
  - Lock-Management-Einstellungen
  - API-Konfiguration (Rate-Limits, etc.)

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
  - Settings-Reload ohne Server-Restart
- [ ] Tests ausführen und bestehen

---

## Phase 2: Database Integration

### 2.1 Database Setup

**Abhängigkeiten**: 1.1 (Projekt-Initialisierung)
**Erforderliche USER-Eingaben**: Database-Wahl (siehe oben)

#### 2.1.1 Database-Client Setup
- [ ] Database-Dependencies hinzufügen
  - Falls PostgreSQL: `sqlx` mit PostgreSQL-Feature
  - Falls SQLite: `sqlx` mit SQLite-Feature
  - Falls beide: beide Features
- [ ] Tests für Database-Connection schreiben
- [ ] Database-Connection-Manager implementieren (TDD)
  - Connection-Pooling
  - Connection-Timeout
  - Connection-Retry
- [ ] Tests ausführen und bestehen

#### 2.1.2 Database-Migration System
- [ ] Migration-Framework konfigurieren (`sqlx-cli`)
- [ ] Initial-Migrations erstellen
  - `devices` Table (siehe README.md Schema)
  - `connections` Table
  - `networks` Table
  - Indizes erstellen (siehe README.md)
- [ ] Tests für Migrations schreiben
- [ ] Migration-Runner implementieren (TDD)
- [ ] Tests ausführen und bestehen

### 2.2 Database Models

**Abhängigkeiten**: 2.1 (Database Setup)

#### 2.2.1 Device Model
- [ ] Tests für Device-Model schreiben
- [ ] `Device` Struct definieren
  - device_id: UUID
  - network_id: UUID
  - world_type: String
  - capabilities: JSON
  - hardware_spec: JSON
  - registered_at: Timestamp
  - last_seen: Timestamp
  - status: String
- [ ] CRUD-Operations implementieren (TDD)
  - Create Device
  - Read Device
  - Update Device
  - Delete Device
  - List Devices (mit Filters)
- [ ] Tests ausführen und bestehen

#### 2.2.2 Connection Model
- [ ] Tests für Connection-Model schreiben
- [ ] `Connection` Struct definieren
  - connection_id: UUID
  - source_device_id: UUID
  - target_device_id: UUID
  - established_at: Timestamp
  - last_heartbeat: Timestamp
  - status: String
- [ ] CRUD-Operations implementieren (TDD)
- [ ] Tests ausführen und bestehen

#### 2.2.3 Network Model
- [ ] Tests für Network-Model schreiben
- [ ] `Network` Struct definieren
  - network_id: UUID
  - name: String
  - created_at: Timestamp
  - owner_device_id: UUID
- [ ] CRUD-Operations implementieren (TDD)
- [ ] Tests ausführen und bestehen

---

## Phase 3: Protobuf & gRPC Integration

### 3.1 Protobuf Definitions

**Abhängigkeiten**: 1.1 (Projekt-Initialisierung)
**Erforderliche USER-Eingaben**: Protobuf-Code-Generierung-Tool (siehe oben)

#### 3.1.1 Shared Protobuf-Projekt verwenden
❓ **HINWEIS**: Protobuf-Definitions sollten in einem separaten Projekt sein (z.B. `edda-protocols`)
- [ ] Asgard als Dependency zu Protobuf-Projekt hinzufügen
- [ ] Protobuf-Definitions importieren

#### 3.1.2 Platform Capability Protocol
- [ ] `EinherjarProtocol.proto` verwenden (wenn nicht vorhanden, erstellen)
- [ ] Code-Generierung für Platform Capability Protocol konfigurieren

#### 3.1.3 Service-spezifische Protocols
- [ ] `OdinService.proto` importieren
- [ ] `HeimdallService.proto` importieren
- [ ] `BifrostService.proto` importieren
- [ ] Code-Generierung für alle Service-Protocols konfigurieren

### 3.2 gRPC Server-Implementierung

**Abhängigkeiten**: 3.1 (Protobuf Definitions)

#### 3.2.1 gRPC Server Basis
- [ ] Tests für gRPC-Server-Setup schreiben
- [ ] gRPC-Server-Setup implementieren (TDD)
  - tonic-Server konfigurieren
  - TLS-Support
  - Interceptors für Authentication
  - Health-Check-Service
- [ ] Tests ausführen und bestehen

#### 3.2.2 Platform Capability Service
- [ ] Tests für Capability-Service schreiben
- [ ] Capability-Service implementieren (TDD)
  - `GetCapabilities()` RPC
  - Capabilities von lokalen Services sammeln
  - Capabilities aggregieren
- [ ] Tests ausführen und bestehen

### 3.3 gRPC Client-Implementierung

**Abhängigkeiten**: 3.1 (Protobuf Definitions)

#### 3.3.1 Service-spezifische Clients
- [ ] Tests für `OdinClient` schreiben
- [ ] `OdinClient` implementieren (TDD)
- [ ] Tests ausführen und bestehen
- [ ] Tests für `HeimdallClient` schreiben
- [ ] `HeimdallClient` implementieren (TDD)
- [ ] Tests ausführen und bestehen
- [ ] Tests für `BifrostClient` schreiben
- [ ] `BifrostClient` implementieren (TDD)
- [ ] Tests ausführen und bestehen

---

## Phase 4: Service Discovery & Lifecycle Management

### 4.1 Service Discovery

**Abhängigkeiten**: 3.2 (gRPC Server-Implementierung)

#### 4.1.1 Service Registry
- [ ] Tests für Service-Registry schreiben
- [ ] `ServiceRegistry` implementieren (TDD)
  - Services registrieren
  - Services deregistrieren
  - Services auflisten
  - Service-Status verwalten
- [ ] Tests ausführen und bestehen

#### 4.1.2 Service Discovery Manager
- [ ] Tests für Service-Discovery schreiben
- [ ] `ServiceDiscoveryManager` implementieren (TDD)
  - Services dynamisch erkennen
  - Service-Health-Checks
  - Service-Status-Updates
- [ ] Tests ausführen und bestehen

#### 4.1.3 Capability Aggregation
- [ ] Tests für Capability-Aggregation schreiben
- [ ] `CapabilityAggregator` implementieren (TDD)
  - Capabilities von allen Services sammeln
  - Capabilities zu Odin propagieren
  - Capability-Updates verarbeiten
- [ ] Tests ausführen und bestehen

### 4.2 Service Lifecycle Management

**Abhängigkeiten**: 4.1 (Service Discovery)

#### 4.2.1 Service Lifecycle Manager
- [ ] Tests für Service-Lifecycle schreiben
- [ ] `ServiceLifecycleManager` implementieren (TDD)
  - Services starten (als separate Prozesse)
  - Services stoppen
  - Service-Restart bei Fehler
  - Service-Health-Monitoring
- [ ] Tests ausführen und bestehen

---

## Phase 5: Device Registry Service

### 5.1 Device Registration

**Abhängigkeiten**: 2.2 (Database Models), 3.2 (gRPC Server)

#### 5.1.1 Device Registration Handler
- [ ] Tests für Device-Registration schreiben
- [ ] `DeviceRegistrationHandler` implementieren (TDD)
  - Device-Registration-Requests verarbeiten
  - Device-Validation
  - Device-Duplikate behandeln (Konflikt-Auflösung)
  - Device in Database speichern
- [ ] Tests ausführen und bestehen

#### 5.1.2 Device Metadata Management
- [ ] Tests für Device-Metadata schreiben
- [ ] `DeviceMetadataManager` implementieren (TDD)
  - Device-Metadata aktualisieren
  - Device-Capabilities verwalten
  - Hardware-Specs verwalten
- [ ] Tests ausführen und bestehen

#### 5.1.3 Device Status Tracking
- [ ] Tests für Device-Status schreiben
- [ ] `DeviceStatusTracker` implementieren (TDD)
  - Device-Status aktualisieren (active, inactive, offline)
  - last_seen Timestamp aktualisieren
  - Device-Heartbeat verarbeiten
- [ ] Tests ausführen und bestehen

### 5.2 Device Discovery Support

**Abhängigkeiten**: 5.1 (Device Registration)

#### 5.2.1 Device Discovery Handler
- [ ] Tests für Device-Discovery schreiben
- [ ] `DeviceDiscoveryHandler` implementieren (TDD)
  - Discovery-Requests verarbeiten
  - Devices im Netzwerk finden
  - Discovery-Responses senden
- [ ] Tests ausführen und bestehen

### 5.3 Capability Synchronization

**Abhängigkeiten**: 5.1 (Device Registration), 4.1 (Service Discovery)

#### 5.3.1 Capability Sync Manager
- [ ] Tests für Capability-Synchronisation schreiben
- [ ] `CapabilitySyncManager` implementieren (TDD)
  - Capabilities zwischen Devices synchronisieren
  - Capability-Validation
  - Capability-Konflikte behandeln
- [ ] Tests ausführen und bestehen

### 5.4 Multi-Asgard-Server Support

**Abhängigkeiten**: 5.1 (Device Registration)

#### 5.4.1 Leading Server Election
- [ ] Tests für Leading-Server-Election schreiben
- [ ] `LeadingServerElection` implementieren (TDD)
  - Ältester Server wird leitender Server
  - Failover bei Server-Ausfall
  - Leader-Election-Protocol
- [ ] Tests ausführen und bestehen

#### 5.4.2 Server-to-Server Synchronization
- [ ] Tests für Server-Synchronisation schreiben
- [ ] `ServerSyncManager` implementieren (TDD)
  - Device-Registry zwischen Servern synchronisieren
  - Konflikt-Auflösung bei Synchronisation
- [ ] Tests ausführen und bestehen

---

## Phase 6: Network Manager Service

### 6.1 Network ID Management

**Abhängigkeiten**: 2.2 (Database Models)

#### 6.1.1 Network ID Generator
- [ ] Tests für Network-ID-Generierung schreiben
- [ ] `NetworkIdGenerator` implementieren (TDD)
  - Automatische Network-ID-Generierung (UUID)
  - Network-ID-Validation
  - Network-ID-Konflikte behandeln
- [ ] Tests ausführen und bestehen

#### 6.1.2 Network ID Assignment
- [ ] Tests für Network-ID-Assignment schreiben
- [ ] `NetworkIdAssignment` implementieren (TDD)
  - Network-ID zu Device zuweisen
  - Multi-Network-Support (Device kann Network wählen)
  - Network-Wechsel verarbeiten
- [ ] Tests ausführen und bestehen

### 6.2 Device Topology Management

**Abhängigkeiten**: 5.1 (Device Registration), 6.1 (Network ID Management)

#### 6.2.1 Topology Manager
- [ ] Tests für Topology-Manager schreiben
- [ ] `TopologyManager` implementieren (TDD)
  - Device-Topologie verwalten
  - Topologie-Updates verarbeiten
  - Topologie-Queries unterstützen
- [ ] Tests ausführen und bestehen

### 6.3 Network Health Monitoring

**Abhängigkeiten**: 5.1 (Device Registration), 6.1 (Network ID Management)

#### 6.3.1 Network Health Monitor
- [ ] Tests für Network-Health-Monitor schreiben
- [ ] `NetworkHealthMonitor` implementieren (TDD)
  - Network-Health überwachen
  - Device-Connectivity prüfen
  - Network-Performance-Metriken sammeln
- [ ] Tests ausführen und bestehen

### 6.4 Network ID Synchronization

**Abhängigkeiten**: 6.1 (Network ID Management)

#### 6.4.1 Local Network Sync (mDNS/Bonjour)
- [ ] Tests für lokale Synchronisation schreiben
- [ ] `LocalNetworkSync` implementieren (TDD)
  - mDNS/Bonjour für lokale Discovery
  - Network-ID über lokales Netzwerk synchronisieren
  - Synchronisations-Konflikte behandeln
- [ ] Tests ausführen und bestehen

#### 6.4.2 Yggdrasil Sync
- [ ] Tests für Yggdrasil-Synchronisation schreiben
- [ ] `YggdrasilSync` implementieren (TDD)
  - Network-ID über Yggdrasil synchronisieren
  - Automatische Synchronisation bei Yggdrasil-Login
  - Synchronisations-Konflikte behandeln
- [ ] Tests ausführen und bestehen

### 6.5 Network Membership Validation

**Abhängigkeiten**: 6.1 (Network ID Management)

#### 6.5.1 Network Membership Validator
- [ ] Tests für Network-Membership-Validation schreiben
- [ ] `NetworkMembershipValidator` implementieren (TDD)
  - Hybrid-Ansatz (lokal + Yggdrasil-Fallback)
  - Lokale Validation (schnell)
  - Yggdrasil-Fallback bei Unsicherheit
  - Credential-basierte Validation
- [ ] Tests ausführen und bestehen

---

## Phase 7: Routing Service

### 7.1 Message Routing

**Abhängigkeiten**: 5.1 (Device Registration), 6.1 (Network ID Management)

#### 7.1.1 Message Router
- [ ] Tests für Message-Router schreiben
- [ ] `MessageRouter` implementieren (TDD)
  - Messages zwischen Devices routen
  - Routing-Pfad auswählen (direct, relay)
  - Routing-Fehler behandeln
- [ ] Tests ausführen und bestehen

#### 7.1.2 Routing Table Management
- [ ] Tests für Routing-Table schreiben
- [ ] `RoutingTableManager` implementieren (TDD)
  - Routing-Table verwalten
  - Routing-Updates verarbeiten
  - Routing-Table-Queries
- [ ] Tests ausführen und bestehen

### 7.2 Relay Functionality

**Abhängigkeiten**: 7.1 (Message Routing)

#### 7.2.1 Relay Manager
- [ ] Tests für Relay-Manager schreiben
- [ ] `RelayManager` implementieren (TDD)
  - Relay-Verbindungen verwalten
  - Messages über Relay routen
  - Relay-Ausfälle behandeln (Fallback, Retry)
- [ ] Tests ausführen und bestehen

### 7.3 Broadcast/Multicast Support

**Abhängigkeiten**: 7.1 (Message Routing)

#### 7.3.1 Broadcast Manager
- [ ] Tests für Broadcast schreiben
- [ ] `BroadcastManager` implementieren (TDD)
  - Broadcast-Messages an alle Devices senden
  - Multicast-Messages an Gruppe senden
- [ ] Tests ausführen und bestehen

### 7.4 Load Balancing

**Abhängigkeiten**: 7.1 (Message Routing)

#### 7.4.1 Load Balancer
- [ ] Tests für Load-Balancer schreiben
- [ ] `LoadBalancer` implementieren (TDD)
  - Load-Balancing für parallele Requests
  - Round-Robin, Least-Connections, etc.
- [ ] Tests ausführen und bestehen

---

## Phase 8: Lock Management Service

### 8.1 Distributed Locking

**Abhängigkeiten**: 5.1 (Device Registration), 2.1 (Database Setup)

#### 8.1.1 Lock Manager
- [ ] Tests für Lock-Manager schreiben
- [ ] `LockManager` implementieren (TDD)
  - Distributed Locks verwalten
  - Lock-Acquisition
  - Lock-Release
  - Lock-Expiration (Timeout)
- [ ] Tests ausführen und bestehen

#### 8.1.2 Lock Registry
- [ ] Tests für Lock-Registry schreiben
- [ ] `LockRegistry` implementieren (TDD)
  - Zentrale Registry für alle aktiven Locks
  - Lock-Status verwalten
  - Lock-Queries unterstützen
- [ ] Tests ausführen und bestehen

### 8.2 Deadlock Detection

**Abhängigkeiten**: 8.1 (Distributed Locking)

#### 8.2.1 Deadlock Detector
- [ ] Tests für Deadlock-Detection schreiben
- [ ] `DeadlockDetector` implementieren (TDD)
  - Deadlocks erkennen (Wait-Graph-Analyse)
  - Deadlock-Prevention (Timeout, Priorität)
- [ ] Tests ausführen und bestehen

#### 8.2.2 Deadlock Resolver
- [ ] Tests für Deadlock-Resolution schreiben
- [ ] `DeadlockResolver` implementieren (TDD)
  - Deadlocks auflösen (Lock-Preemption)
  - Automatische Deadlock-Resolution-Mechanismen
- [ ] Tests ausführen und bestehen

### 8.3 Lock Priority Management

**Abhängigkeiten**: 8.1 (Distributed Locking)

#### 8.3.1 Priority Manager
- [ ] Tests für Priority-Management schreiben
- [ ] `PriorityManager` implementieren (TDD)
  - Lock-Requests priorisieren
  - Priority-basierte Lock-Acquisition
  - Priority-Inversion behandeln
- [ ] Tests ausführen und bestehen

### 8.4 Lock Coordination (Multi-Asgard)

**Abhängigkeiten**: 8.1 (Distributed Locking), 5.4 (Multi-Asgard Support)

#### 8.4.1 Lock Coordinator
- [ ] Tests für Lock-Coordination schreiben
- [ ] `LockCoordinator` implementieren (TDD)
  - Locks zwischen Asgard-Servern koordinieren
  - Lock-Konflikte behandeln (Priorität, Timeout)
- [ ] Tests ausführen und bestehen

---

## Phase 9: Storage Service

### 9.1 Persistent Storage

**Abhängigkeiten**: 2.1 (Database Setup)

#### 9.1.1 Storage Manager
- [ ] Tests für Storage-Manager schreiben
- [ ] `StorageManager` implementieren (TDD)
  - Device-Data persistent speichern
  - Data-Queries unterstützen
  - Data-Cleanup (alte Daten löschen)
- [ ] Tests ausführen und bestehen

### 9.2 Backup & Restore

**Abhängigkeiten**: 9.1 (Persistent Storage)

#### 9.2.1 Backup Manager
- [ ] Tests für Backup schreiben
- [ ] `BackupManager` implementieren (TDD)
  - Database-Backups erstellen
  - Backup-Schedule
  - Backup-Rotation (alte Backups löschen)
- [ ] Tests ausführen und bestehen

#### 9.2.2 Restore Manager
- [ ] Tests für Restore schreiben
- [ ] `RestoreManager` implementieren (TDD)
  - Database-Restore aus Backup
  - Restore-Validation
- [ ] Tests ausführen und bestehen

### 9.3 Data Migration

**Abhängigkeiten**: 9.1 (Persistent Storage)

#### 9.3.1 Migration Manager
- [ ] Tests für Data-Migration schreiben
- [ ] `MigrationManager` implementieren (TDD)
  - Schema-Migrations durchführen
  - Data-Migrations bei Schema-Updates
  - Rollback-Support
- [ ] Tests ausführen und bestehen

---

## Phase 10: REST API Implementation

### 10.1 API Server Setup

**Abhängigkeiten**: 1.1 (Projekt-Initialisierung)
**Erforderliche USER-Eingaben**: API-Framework-Wahl (siehe oben)

#### 10.1.1 API Server Basis
- [ ] Tests für API-Server-Setup schreiben
- [ ] API-Server-Setup implementieren (TDD)
  - Framework-Konfiguration (Axum/Actix/Warp)
  - Router-Setup
  - Middleware (Logging, CORS, etc.)
- [ ] Tests ausführen und bestehen

#### 10.1.2 Authentication Middleware
- [ ] Tests für Authentication-Middleware schreiben
- [ ] Authentication-Middleware implementieren (TDD)
  - Token-Validation (JWT oder Heimdall-Token)
  - Authorization-Header-Parsing
  - Authentication-Errors behandeln
- [ ] Tests ausführen und bestehen

#### 10.1.3 Rate Limiting Middleware
- [ ] Tests für Rate-Limiting schreiben
- [ ] Rate-Limiting-Middleware implementieren (TDD)
  - Token-basierte Rate-Limits
  - IP-basierte Rate-Limits (DDoS-Schutz)
  - Endpoint-spezifische Rate-Limits
  - Rate-Limit-Headers (`X-RateLimit-*`)
- [ ] Tests ausführen und bestehen

### 10.2 Device Management Endpoints

**Abhängigkeiten**: 10.1 (API Server Setup), 5.1 (Device Registry)

#### 10.2.1 List Devices Endpoint
- [ ] Tests für `GET /api/v1/devices` schreiben
- [ ] `GET /api/v1/devices` implementieren (TDD)
  - Devices auflisten
  - Filter unterstützen (network_id, world_type, status)
  - Pagination unterstützen
- [ ] Tests ausführen und bestehen

#### 10.2.2 Get Device Details Endpoint
- [ ] Tests für `GET /api/v1/devices/:id` schreiben
- [ ] `GET /api/v1/devices/:id` implementieren (TDD)
  - Device-Details abrufen
  - 404 bei nicht gefundenem Device
- [ ] Tests ausführen und bestehen

#### 10.2.3 Register Device Endpoint
- [ ] Tests für `POST /api/v1/devices` schreiben
- [ ] `POST /api/v1/devices` implementieren (TDD)
  - Device registrieren
  - Request-Validation
  - 201 bei Erfolg
- [ ] Tests ausführen und bestehen

#### 10.2.4 Update Device Endpoint
- [ ] Tests für `PUT /api/v1/devices/:id` schreiben
- [ ] `PUT /api/v1/devices/:id` implementieren (TDD)
  - Device aktualisieren
  - Request-Validation
  - 404 bei nicht gefundenem Device
- [ ] Tests ausführen und bestehen

#### 10.2.5 Unregister Device Endpoint
- [ ] Tests für `DELETE /api/v1/devices/:id` schreiben
- [ ] `DELETE /api/v1/devices/:id` implementieren (TDD)
  - Device deregistrieren
  - 404 bei nicht gefundenem Device
- [ ] Tests ausführen und bestehen

### 10.3 Network Management Endpoints

**Abhängigkeiten**: 10.1 (API Server Setup), 6.1 (Network Manager)

#### 10.3.1 List Networks Endpoint
- [ ] Tests für `GET /api/v1/networks` schreiben
- [ ] `GET /api/v1/networks` implementieren (TDD)
- [ ] Tests ausführen und bestehen

#### 10.3.2 Get Network Details Endpoint
- [ ] Tests für `GET /api/v1/networks/:id` schreiben
- [ ] `GET /api/v1/networks/:id` implementieren (TDD)
- [ ] Tests ausführen und bestehen

#### 10.3.3 Create Network Endpoint
- [ ] Tests für `POST /api/v1/networks` schreiben
- [ ] `POST /api/v1/networks` implementieren (TDD)
- [ ] Tests ausführen und bestehen

#### 10.3.4 Update Network Endpoint
- [ ] Tests für `PUT /api/v1/networks/:id` schreiben
- [ ] `PUT /api/v1/networks/:id` implementieren (TDD)
- [ ] Tests ausführen und bestehen

#### 10.3.5 Delete Network Endpoint
- [ ] Tests für `DELETE /api/v1/networks/:id` schreiben
- [ ] `DELETE /api/v1/networks/:id` implementieren (TDD)
- [ ] Tests ausführen und bestehen

### 10.4 Connection Management Endpoints

**Abhängigkeiten**: 10.1 (API Server Setup), 7.1 (Routing Service)

#### 10.4.1 List Connections Endpoint
- [ ] Tests für `GET /api/v1/connections` schreiben
- [ ] `GET /api/v1/connections` implementieren (TDD)
- [ ] Tests ausführen und bestehen

#### 10.4.2 Get Connection Details Endpoint
- [ ] Tests für `GET /api/v1/connections/:id` schreiben
- [ ] `GET /api/v1/connections/:id` implementieren (TDD)
- [ ] Tests ausführen und bestehen

#### 10.4.3 Create Connection Endpoint
- [ ] Tests für `POST /api/v1/connections` schreiben
- [ ] `POST /api/v1/connections` implementieren (TDD)
- [ ] Tests ausführen und bestehen

#### 10.4.4 Close Connection Endpoint
- [ ] Tests für `DELETE /api/v1/connections/:id` schreiben
- [ ] `DELETE /api/v1/connections/:id` implementieren (TDD)
- [ ] Tests ausführen und bestehen

### 10.5 Admin Endpoints (Optional)

**Abhängigkeiten**: 10.1 (API Server Setup)

#### 10.5.1 Server Statistics Endpoint
- [ ] Tests für `GET /api/v1/admin/stats` schreiben
- [ ] `GET /api/v1/admin/stats` implementieren (TDD)
  - Admin-Role-Check
  - Server-Statistics sammeln
- [ ] Tests ausführen und bestehen

#### 10.5.2 Server Logs Endpoint
- [ ] Tests für `GET /api/v1/admin/logs` schreiben
- [ ] `GET /api/v1/admin/logs` implementieren (TDD)
  - Admin-Role-Check
  - Logs abrufen (mit Filters)
  - Query-Params: level, limit, offset
- [ ] Tests ausführen und bestehen

---

## Phase 11: Enhanced Bifrost Integration

### 11.1 Network Discovery

**Abhängigkeiten**: 5.2 (Device Discovery), 3.3 (gRPC Client)

#### 11.1.1 Enhanced Discovery Manager
- [ ] Tests für Enhanced-Discovery schreiben
- [ ] `EnhancedDiscoveryManager` implementieren (TDD)
  - Erweiterte Device-Discovery über Netzwerk
  - Discovery-Optimierungen (Caching)
  - Discovery-Timeouts behandeln
- [ ] Tests ausführen und bestehen

### 11.2 Relay Routing (Bifrost)

**Abhängigkeiten**: 7.2 (Relay Functionality), 3.3 (Bifrost Client)

#### 11.2.1 Bifrost Relay Manager
- [ ] Tests für Bifrost-Relay schreiben
- [ ] `BifrostRelayManager` implementieren (TDD)
  - Relay-Routing über Asgard
  - Routing-Optimierungen
  - Relay-Ausfälle behandeln (Fallback, Retry)
- [ ] Tests ausführen und bestehen

### 11.3 Message Queuing (Offline Devices)

**Abhängigkeiten**: 7.1 (Message Routing), 9.1 (Storage Service)

#### 11.3.1 Message Queue Manager
- [ ] Tests für Message-Queuing schreiben
- [ ] `MessageQueueManager` implementieren (TDD)
  - Messages für Offline-Devices queuen
  - Queue-Size-Limits (konfigurierbar)
  - Queue-Overflow behandeln (Eviction, Notification)
  - Messages senden wenn Device online
- [ ] Tests ausführen und bestehen

### 11.4 Connection Pooling (Bifrost)

**Abhängigkeiten**: 3.3 (Bifrost Client)

#### 11.4.1 Bifrost Connection Pool
- [ ] Tests für Connection-Pooling schreiben
- [ ] `BifrostConnectionPool` implementieren (TDD)
  - WebSocket-Connections pooling
  - Connection-Reuse
  - Connection-Monitoring
- [ ] Tests ausführen und bestehen

---

## Phase 12: Odin Integration

### 12.1 Odin Process Management

**Abhängigkeiten**: 4.2 (Service Lifecycle Management)

#### 12.1.1 Odin Process Wrapper
- [ ] Tests für Odin-Wrapper schreiben
- [ ] `OdinProcessWrapper` implementieren (TDD)
  - Odin als Hauptprozess starten
  - Odin-Status überwachen
  - Odin-Restart bei Fehler
- [ ] Tests ausführen und bestehen

### 12.2 Odin Request/Response Handling

**Abhängigkeiten**: 12.1 (Odin Process Management), 3.3 (gRPC Client)

#### 12.2.1 Odin Request Handler
- [ ] Tests für Odin-Request-Handler schreiben
- [ ] `OdinRequestHandler` implementieren (TDD)
  - Requests zu Odin weiterleiten
  - Request-Queuing
  - Request-Priorisierung
- [ ] Tests ausführen und bestehen

#### 12.2.2 Odin Response Handler
- [ ] Tests für Odin-Response-Handler schreiben
- [ ] `OdinResponseHandler` implementieren (TDD)
  - Responses von Odin verarbeiten
  - Response-Routing
  - Error-Responses behandeln
- [ ] Tests ausführen und bestehen

---

## Phase 13: Heimdall Integration (Security)

### 13.1 Heimdall Client Integration

**Abhängigkeiten**: 3.3 (gRPC Client)

#### 13.1.1 Heimdall Client
- [ ] Tests für Heimdall-Client schreiben
- [ ] `HeimdallClient` implementieren (TDD)
- [ ] Tests ausführen und bestehen

### 13.2 Authentication

**Abhängigkeiten**: 13.1 (Heimdall Client)

#### 13.2.1 Authentication Manager
- [ ] Tests für Authentication schreiben
- [ ] `AuthenticationManager` implementieren (TDD)
  - Device-Authentication via Heimdall
  - Token-Generierung
  - Token-Validation
  - Token-Refresh
- [ ] Tests ausführen und bestehen

### 13.3 Authorization

**Abhängigkeiten**: 13.2 (Authentication)

#### 13.3.1 Authorization Manager
- [ ] Tests für Authorization schreiben
- [ ] `AuthorizationManager` implementieren (TDD)
  - Permission-System für Device-Zugriff
  - Role-based Access Control
  - Permission-Checks
- [ ] Tests ausführen und bestehen

### 13.4 Secure Storage

**Abhängigkeiten**: 13.1 (Heimdall Client)

#### 13.4.1 Secure Storage Manager
- [ ] Tests für Secure-Storage schreiben
- [ ] `SecureStorageManager` implementieren (TDD)
  - Credentials verschlüsselt speichern
  - Secrets verschlüsselt speichern
  - Encryption via Heimdall
- [ ] Tests ausführen und bestehen

---

## Phase 14: Web Dashboard (Optional)

**Abhängigkeiten**: 10.1 (API Server Setup)
**Erforderliche USER-Eingaben**: Web-Dashboard-Entscheidung, Framework-Wahl (siehe oben)

❓ **HINWEIS**: Diese Phase wird nur ausgeführt, wenn USER entscheidet, dass Web Dashboard in Phase 1 implementiert werden soll.

### 14.1 Frontend Setup

#### 14.1.1 Frontend-Projekt-Initialisierung
- [ ] Frontend-Verzeichnis erstellen (`src/frontend/`)
- [ ] Package Manager konfigurieren (`bun`)
- [ ] `package.json` erstellen
- [ ] Framework-Setup (React/Vue/Svelte)
- [ ] Build-Pipeline konfigurieren

#### 14.1.2 Frontend-Dependencies
- [ ] UI-Framework installieren (z.B. React)
- [ ] UI-Library installieren (z.B. Material-UI, Ant Design)
- [ ] HTTP-Client installieren (z.B. axios, fetch)
- [ ] WebSocket-Client installieren

### 14.2 Dashboard UI Components

#### 14.2.1 Layout Components
- [ ] Tests für Layout schreiben
- [ ] Main-Layout implementieren
- [ ] Navigation implementieren
- [ ] Sidebar implementieren
- [ ] Tests ausführen und bestehen

#### 14.2.2 Device Management UI
- [ ] Tests für Device-UI schreiben
- [ ] Device-List-Component implementieren
- [ ] Device-Detail-Component implementieren
- [ ] Device-Registration-Form implementieren
- [ ] Tests ausführen und bestehen

#### 14.2.3 Network Management UI
- [ ] Tests für Network-UI schreiben
- [ ] Network-List-Component implementieren
- [ ] Network-Detail-Component implementieren
- [ ] Network-Creation-Form implementieren
- [ ] Tests ausführen und bestehen

### 14.3 Real-time Updates (WebSocket)

**Abhängigkeiten**: 14.2 (Dashboard UI Components)

#### 14.3.1 WebSocket Client
- [ ] Tests für WebSocket-Client schreiben
- [ ] WebSocket-Client implementieren
  - Connection-Establishment
  - Real-time-Updates empfangen
  - Automatic-Reconnection
- [ ] Tests ausführen und bestehen

#### 14.3.2 Real-time UI Updates
- [ ] Tests für Real-time-Updates schreiben
- [ ] Real-time-Update-Logik implementieren
  - Device-Status-Updates
  - Network-Updates
  - Connection-Updates
- [ ] Tests ausführen und bestehen

### 14.4 Input Handling (Text & Voice)

**Abhängigkeiten**: 14.1 (Frontend Setup), 12.2 (Odin Integration)

#### 14.4.1 Text Input Component
- [ ] Tests für Text-Input schreiben
- [ ] Text-Input-Component implementieren
  - Text-Eingabe-Feld
  - Submit-Button
  - Command-History
- [ ] Tests ausführen und bestehen

#### 14.4.2 Voice Input Component (Optional)
❓ **FRAGE AN USER**: Soll Voice-Input im Web Dashboard unterstützt werden?
- [ ] Falls ja: Tests für Voice-Input schreiben
- [ ] Falls ja: Voice-Input-Component implementieren (via Huginn/Muninn)
- [ ] Falls ja: Mikrofon-Button UI
- [ ] Falls ja: Tests ausführen und bestehen

---

## Phase 15: Performance Optimizations

### 15.1 Database Optimizations

**Abhängigkeiten**: 2.1 (Database Setup)

#### 15.1.1 Query Optimization
- [ ] Slow-Query-Analyse durchführen
- [ ] Indizes optimieren (siehe README.md)
- [ ] Query-Performance-Tests schreiben
- [ ] Optimierte Queries implementieren
- [ ] Tests ausführen und Benchmarks erreichen

#### 15.1.2 Connection Pooling Optimization
- [ ] Connection-Pool-Settings optimieren
- [ ] Connection-Pool-Performance-Tests schreiben
- [ ] Tests ausführen und Benchmarks erreichen

### 15.2 Routing Optimizations

**Abhängigkeiten**: 7.1 (Message Routing)

#### 15.2.1 Routing-Path-Optimization
- [ ] Routing-Path-Analyse durchführen
- [ ] Routing-Performance-Tests schreiben
- [ ] Routing-Optimierungen implementieren (Caching, Prefetching)
- [ ] Tests ausführen und Benchmarks erreichen (< 10ms lokal)

### 15.3 Caching Strategy

**Abhängigkeiten**: 5.1 (Device Registry), 6.1 (Network Manager)

#### 15.3.1 Cache Manager
- [ ] Tests für Cache-Manager schreiben
- [ ] `CacheManager` implementieren (TDD)
  - Device-Registry-Caching
  - Network-Topology-Caching
  - Cache-Invalidation
  - Cache-TTL
- [ ] Tests ausführen und bestehen

---

## Phase 16: Security Hardening

### 16.1 Input Validation

**Abhängigkeiten**: 10.1 (API Server Setup)

#### 16.1.1 Input Validator
- [ ] Tests für Input-Validation schreiben
- [ ] Comprehensive Input-Validation implementieren (TDD)
  - Type-Validation
  - Range-Validation
  - Format-Validation
  - Injection-Prevention (SQL, Command, etc.)
- [ ] Tests ausführen und bestehen

### 16.2 TLS Configuration

**Abhängigkeiten**: 10.1 (API Server Setup), 3.2 (gRPC Server)

#### 16.2.1 TLS Setup
- [ ] TLS 1.3 für alle Connections konfigurieren
- [ ] Certificate-Management implementieren
- [ ] Certificate-Rotation implementieren
- [ ] TLS-Tests schreiben und ausführen

### 16.3 Audit Logging

**Abhängigkeiten**: 13.2 (Authentication)

#### 16.3.1 Audit Logger
- [ ] Tests für Audit-Logging schreiben
- [ ] `AuditLogger` implementieren (TDD)
  - Security-relevante Events loggen
  - Authentication-Events
  - Authorization-Events
  - Data-Access-Events
- [ ] Tests ausführen und bestehen

### 16.4 Security Testing

**Abhängigkeiten**: Alle vorherigen Phasen

#### 16.4.1 Security Audit
- [ ] Security-Audit durchführen
- [ ] Vulnerability-Scanning (cargo-audit)
- [ ] Penetration-Testing (optional)
- [ ] Security-Findings dokumentieren und beheben

#### 16.4.2 Unauthorized Access Prevention Tests
- [ ] Tests für API-Endpoint-Security schreiben
  - Unauthorized-Access-Tests für alle Endpoints
  - Sicherstellen, dass unauthorized Users keine Daten abrufen können
- [ ] Tests für WebSocket-Security schreiben
  - Unauthorized-Access-Tests für WebSocket-Connections
  - Sicherstellen, dass unauthorized Users keine Daten empfangen können
- [ ] Tests ausführen und bestehen (100% Coverage für Security-Tests)

---

## Phase 17: DeviceIdentity System (Phase 2)

### 17.1 DeviceIdentity Management

**Abhängigkeiten**: 5.1 (Device Registry), 13.1 (Heimdall Integration)

#### 17.1.1 DeviceIdentity Manager
- [ ] Tests für DeviceIdentity-Manager schreiben
- [ ] `DeviceIdentityManager` implementieren (TDD)
  - Device-Identity erstellen (user-assigned)
  - Device-Identity speichern (verschlüsselt)
  - Device-Metadata verwalten
- [ ] Tests ausführen und bestehen

#### 17.1.2 DeviceIdentity Validation
- [ ] Tests für DeviceIdentity-Validation schreiben
- [ ] `DeviceIdentityValidator` implementieren (TDD)
  - Identity-Validation
  - Identity-Verification
- [ ] Tests ausführen und bestehen

### 17.2 Device Discovery & Connection

**Abhängigkeiten**: 17.1 (DeviceIdentity Management), 11.1 (Network Discovery)

#### 17.2.1 Device Discovery Workflow
- [ ] Tests für Device-Discovery-Workflow schreiben
- [ ] Device-Discovery-Workflow implementieren (TDD)
  - Discovery-Request senden
  - Discovery-Response verarbeiten
  - Device-Identity austauschen
- [ ] Tests ausführen und bestehen

#### 17.2.2 Connection Establishment Workflow
- [ ] Tests für Connection-Establishment schreiben
- [ ] Connection-Establishment-Workflow implementieren (TDD)
  - Bifrost-Connection initiieren
  - Heimdall-Validation
  - TLS-Handshake
  - Connection etablieren
- [ ] Tests ausführen und bestehen

### 17.3 Cross-Device Action Execution

**Abhängigkeiten**: 17.2 (Device Discovery & Connection), 7.1 (Routing Service)

#### 17.3.1 Cross-Device Action Router
- [ ] Tests für Cross-Device-Actions schreiben
- [ ] `CrossDeviceActionRouter` implementieren (TDD)
  - ThorAction via gRPC an Remote-Device senden (über Bifrost)
  - ThorResult von Remote-Device empfangen
  - Streaming-Support für lange Actions
- [ ] Tests ausführen und bestehen

---

## Phase 18: Network Expansion (Phase 4) - WAN Connectivity

### 18.1 NAT Traversal

**Abhängigkeiten**: 11.2 (Bifrost Relay)
**Erforderliche USER-Eingaben**: NAT-Traversal-Bibliotheken (siehe oben)

#### 18.1.1 STUN Client
- [ ] Tests für STUN-Client schreiben
- [ ] `STUNClient` implementieren (TDD)
  - STUN-Protocol für NAT-Discovery
  - Public-IP ermitteln
  - NAT-Type ermitteln
- [ ] Tests ausführen und bestehen

#### 18.1.2 TURN Server (Asgard als TURN)
- [ ] Tests für TURN-Server schreiben
- [ ] `TURNServer` implementieren (TDD)
  - TURN-Server für Relay
  - Relay-Connections verwalten
- [ ] Tests ausführen und bestehen

#### 18.1.3 ICE Implementation
- [ ] Tests für ICE schreiben
- [ ] `ICEManager` implementieren (TDD)
  - ICE-Protocol für optimalen Pfad
  - Candidate-Gathering
  - Connectivity-Checks
- [ ] Tests ausführen und bestehen

#### 18.1.4 NAT-Traversal Fallback
- [ ] Tests für Fallback schreiben
- [ ] Manuelle Port-Forwarding-Konfiguration implementieren (TDD)
- [ ] UPnP/NAT-PMP für automatisches Port-Forwarding implementieren (TDD)
- [ ] Tests ausführen und bestehen

### 18.2 Dynamic IP Handling

**Abhängigkeiten**: 18.1 (NAT Traversal)

#### 18.2.1 DDNS Integration
- [ ] Tests für DDNS schreiben
- [ ] `DDNSManager` implementieren (TDD)
  - DDNS für Domain-Names (optional, falls User konfiguriert)
  - IP-Update-Service
  - Connection-Refresh bei IP-Änderung
- [ ] Tests ausführen und bestehen

#### 18.2.2 Yggdrasil Relay Fallback
- [ ] Tests für Yggdrasil-Relay schreiben
- [ ] Yggdrasil-Relay-Fallback implementieren (TDD)
  - Falls DDNS nicht konfiguriert, automatisch über Yggdrasil-Relay
- [ ] Tests ausführen und bestehen

### 18.3 WAN Connection Establishment

**Abhängigkeiten**: 18.1 (NAT Traversal), 18.2 (Dynamic IP Handling)

#### 18.3.1 WAN Connection Manager
- [ ] Tests für WAN-Connection schreiben
- [ ] `WANConnectionManager` implementieren (TDD)
  - Public-IP-Connections
  - Domain-based-Connections (nur bei expliziter Erlaubnis)
  - Relay-Connections (Hauptmethode)
- [ ] Tests ausführen und bestehen

#### 18.3.2 WAN Connection Workflow
- [ ] Tests für WAN-Connection-Workflow schreiben
- [ ] WAN-Connection-Workflow implementieren (TDD)
  - Direct-Connection-Attempt
  - Relay-Fallback bei Fehlschlag
  - TLS-Handshake
  - Device-Authentication
- [ ] Tests ausführen und bestehen

### 18.4 Enhanced Routing (WAN)

**Abhängigkeiten**: 18.3 (WAN Connection Establishment), 7.1 (Routing Service)

#### 18.4.1 WAN Routing Strategies
- [ ] Tests für WAN-Routing schreiben
- [ ] `WANRoutingManager` implementieren (TDD)
  - Direct-Routing (wenn möglich)
  - Relay-Routing (über Asgard)
  - Hybrid-Routing (Kombination)
- [ ] Tests ausführen und bestehen

#### 18.4.2 Path Optimization
- [ ] Tests für Path-Optimization schreiben
- [ ] `PathOptimizer` implementieren (TDD)
  - Routing-Pfade optimieren
  - Load-Balancing über mehrere Pfade
  - Quality-based-Routing
- [ ] Tests ausführen und bestehen

#### 18.4.3 Failover & Error Recovery
- [ ] Tests für Failover schreiben
- [ ] `FailoverManager` implementieren (TDD)
  - Automatisches Failover bei Verbindungsausfall
  - Fallback-Routing
  - Automatic-Reconnection (sofort + Exponential Backoff)
- [ ] Tests ausführen und bestehen

---

## Phase 19: Monitoring & Analytics

### 19.1 Server Monitoring

**Abhängigkeiten**: Alle vorherigen Phasen

#### 19.1.1 Metrics Collector
- [ ] Tests für Metrics-Collector schreiben
- [ ] `MetricsCollector` implementieren (TDD)
  - Performance-Metriken sammeln
  - Resource-Usage-Metriken
  - Network-Metriken
- [ ] Tests ausführen und bestehen

#### 19.1.2 Health Check Service
- [ ] Tests für Health-Check schreiben
- [ ] Health-Check-Service implementieren (TDD)
  - Service-Health-Checks
  - Database-Health-Checks
  - Network-Health-Checks
- [ ] Tests ausführen und bestehen

### 19.2 Logging

**Abhängigkeiten**: 1.1 (Projekt-Initialisierung)

#### 19.2.1 Structured Logging
- [ ] Structured-Logging konfigurieren (tracing)
- [ ] Log-Levels definieren (trace, debug, info, warn, error)
- [ ] Log-Rotation konfigurieren

#### 19.2.2 Log Aggregation (Optional)
- [ ] Log-Aggregation-Service integrieren (optional)
  - z.B. ELK-Stack, Grafana Loki
- [ ] Log-Forwarding konfigurieren

---

## Phase 20: Documentation

### 20.1 API Documentation

**Abhängigkeiten**: 10.1 (API Server Setup)

#### 20.1.1 OpenAPI Specification
- [ ] OpenAPI 3.0 Specification erstellen
  - Alle API-Endpoints dokumentieren
  - Request/Response-Schemas
  - Authentication-Dokumentation
  - Rate-Limits-Dokumentation
  - Error-Codes-Dokumentation

#### 20.1.2 API Documentation Generation
- [ ] API-Dokumentation automatisch generieren
  - Swagger UI oder ReDoc
- [ ] API-Dokumentation in CI/CD-Pipeline integrieren

### 20.2 Code Documentation

**Abhängigkeiten**: Alle vorherigen Phasen

#### 20.2.1 Rust Documentation
- [ ] Alle Public-APIs mit Rustdoc dokumentieren
- [ ] Code-Examples in Rustdoc hinzufügen
- [ ] Rustdoc generieren (`cargo doc`)

#### 20.2.2 Architecture Documentation
- [ ] Architecture-Diagramm erstellen
- [ ] Component-Diagramme erstellen
- [ ] Sequence-Diagramme für wichtige Workflows

### 20.3 User Documentation

**Abhängigkeiten**: Alle vorherigen Phasen

#### 20.3.1 Installation Guide
- [ ] Installations-Anleitung erstellen
  - Server-Setup
  - Database-Setup
  - Configuration
- [ ] Troubleshooting-Section

#### 20.3.2 Admin Guide
- [ ] Admin-Guide erstellen
  - Server-Administration
  - Monitoring
  - Backup & Restore
  - Security-Best-Practices

---

## Phase 21: Deployment & Operations

### 21.1 Docker Support

**Abhängigkeiten**: Alle vorherigen Phasen

#### 21.1.1 Dockerfile
- [ ] Production-Dockerfile erstellen
  - Multi-Stage-Build
  - Minimal Image (Alpine/Distroless)
- [ ] Docker-Compose für Production erstellen
  - Asgard-Service
  - Database-Service
  - Reverse-Proxy (optional)

#### 21.1.2 Docker Image Optimization
- [ ] Image-Size optimieren
- [ ] Build-Zeit optimieren
- [ ] Security-Scanning für Images

### 21.2 Systemd Service

**Abhängigkeiten**: Alle vorherigen Phasen

#### 21.2.1 Systemd Service File
- [ ] Systemd-Service-File erstellen
  - Auto-Start konfigurieren
  - Auto-Restart konfigurieren
- [ ] Installation-Script erstellen

### 21.3 Configuration Management

**Abhängigkeiten**: 1.3 (Projekt-Konfiguration)

#### 21.3.1 Environment-based Configuration
- [ ] Environment-Variable-Support
- [ ] Configuration-Files für verschiedene Umgebungen
  - Development
  - Staging
  - Production

### 21.4 Backup & Restore Automation

**Abhängigkeiten**: 9.2 (Backup & Restore)

#### 21.4.1 Automated Backup
- [ ] Backup-Cron-Job erstellen
- [ ] Backup-Script erstellen
- [ ] Backup-Verification implementieren

---

## Phase 22: Testing & Quality Assurance

### 22.1 Integration Testing

**Abhängigkeiten**: Alle vorherigen Phasen

#### 22.1.1 End-to-End Tests
- [ ] E2E-Tests für komplette Server-Workflows schreiben
  - Device-Registration → Device-Discovery → Connection-Establishment
  - Message-Routing (Direct, Relay)
  - Cross-Device-Actions
  - WAN-Connectivity
- [ ] E2E-Tests ausführen und bestehen

#### 22.1.2 Load Testing
- [ ] Load-Tests schreiben
  - Parallel-Connections testen
  - High-Throughput testen
  - Database-Load testen
- [ ] Load-Tests ausführen und Benchmarks erreichen

### 22.2 Performance Testing

**Abhängigkeiten**: 15.1 (Performance Optimizations)

#### 22.2.1 Performance Benchmarks
- [ ] Performance-Benchmarks definieren
  - Message-Routing-Latency (< 10ms lokal)
  - Device-Discovery-Time (< 1s lokal)
  - API-Response-Time (< 100ms)
- [ ] Performance-Tests schreiben und ausführen

### 22.3 Security Testing

**Abhängigkeiten**: 16.4 (Security Testing)

#### 22.3.1 Security Test Suite
- [ ] Comprehensive Security-Tests ausführen
  - API-Endpoint-Security (100% Coverage)
  - WebSocket-Security (100% Coverage)
  - Unauthorized-Access-Prevention (100% Coverage)
- [ ] Security-Tests bestehen

#### 22.3.2 GDPR Compliance Testing
- [ ] GDPR-Compliance-Tests schreiben
  - Data-Minimization-Tests
  - Data-Encryption-Tests
  - Access-Control-Tests
  - Audit-Logging-Tests
  - Right-to-Erasure-Tests
- [ ] GDPR-Compliance-Tests ausführen und bestehen

---

## Zusammenfassung

**Gesamtanzahl Phasen**: 22
**Gesamtanzahl Schritte**: ~400+

**Kritische Abhängigkeiten**:
1. Database-Wahl (beeinflusst Performance und Deployment)
2. API-Framework-Wahl (beeinflusst API-Implementierung)
3. Web-Dashboard-Entscheidung (beeinflusst Frontend-Implementierung)
4. NAT-Traversal-Bibliotheken (beeinflusst WAN-Connectivity)
5. Protobuf-Code-Generierung-Tool (beeinflusst Code-Generierung)

**Offene Fragen für USER**:
1. Database-Wahl (PostgreSQL, SQLite, oder beide)
2. Web-Dashboard in Phase 1? (Ja, Nein, Minimal)
3. Web-Dashboard-Framework (React, Vue, Svelte) - falls Dashboard in Phase 1
4. API-Framework (Axum, Actix-Web, Warp)
5. NAT-Traversal-Bibliotheken (webrtc-rs, separate libs, eigene Implementierung)
6. Protobuf-Code-Generierung-Tool (prost, protobuf-rust, tonic+prost)
7. Voice-Input im Web Dashboard? (Ja, Nein)

**Hinweise**:
- Alle Schritte folgen TDD (Tests zuerst, dann Implementierung)
- Alle Tests müssen in Containern laufen (keine lokalen Dependencies)
- Alle Schritte sind kleinstmöglich aufgeteilt
- Abhängigkeiten zwischen Phasen sind klar definiert
- Offene Fragen sind klar markiert (❓)
- Security ist kritisch: 100% Coverage für API/WebSocket-Security-Tests
- Performance ist wichtig: < 10ms Routing-Latency, < 1s Device-Discovery
- GDPR-Compliance ist erforderlich: Data-Minimization, Encryption, Access-Control

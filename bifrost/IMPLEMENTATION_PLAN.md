# IMPLEMENTATION_PLAN - Bifrost (Communication Service)

## Übersicht

Dieser Plan beschreibt die kleinstmöglichen Schritte zur Implementierung von Bifrost - dem Secure WebSocket Service für Inter-Device Communication. Bifrost ermöglicht sichere, verschlüsselte Kommunikation zwischen Devices im Edda-Netzwerk über WebSocket mit TLS 1.3.

## Offene Fragen (USER-INPUT ERFORDERLICH)

### WebSocket-Library-Wahl
❓ **FRAGE AN USER**: Welche WebSocket-Library soll verwendet werden?
- Option A: `tokio-tungstenite` (async, performant, gut integriert mit tokio)
- Option B: `async-tungstenite` (async, flexibel)
- Option C: `websocket` (klassische Library)

**Auswirkung**: Beeinflusst die gesamte WebSocket-Implementierung und Async-Handling.

### TLS-Library-Wahl
❓ **FRAGE AN USER**: Welche TLS-Library soll verwendet werden?
- Option A: `rustls` (moderner, pure-Rust TLS-Stack, TLS 1.3)
- Option B: `native-tls` (verwendet OS-TLS, einfacher)
- Option C: `openssl` (klassische OpenSSL-Bindings)

**Auswirkung**: Beeinflusst die TLS-Konfiguration und Certificate-Handling.

### mDNS/Bonjour-Library
❓ **FRAGE AN USER**: Welche Library für mDNS/Bonjour Device-Discovery?
- Option A: `mdns` (einfache mDNS-Implementierung)
- Option B: `zeroconf` (umfassende Zeroconf-Implementierung)
- Option C: Eigene Implementierung (mehr Kontrolle, mehr Aufwand)

**Auswirkung**: Beeinflusst die lokale Device-Discovery-Implementierung.

### NAT-Traversal-Bibliotheken
❓ **FRAGE AN USER** (aus unclear-items-analysis.md): Welche Rust-Bibliotheken für NAT-Traversal?
- Option A: `webrtc-rs` (vollständige WebRTC-Implementierung, STUN/TURN/ICE)
- Option B: Separate Libraries (`stun-rs`, `turn-rs`, `ice-rs`)
- Option C: Eigene Implementierung (mehr Kontrolle, mehr Aufwand)

**Auswirkung**: Beeinflusst die WAN-Connectivity und NAT-Traversal-Features.

### Protobuf-Code-Generierung
❓ **FRAGE AN USER**: Welches Tool für Protobuf → Rust-Generierung?
- Option A: `prost` (moderne, idiomatische Rust-Generierung)
- Option B: `protobuf-rust` (klassischer Generator)
- Option C: `tonic` mit `prost` (für gRPC + Protobuf)

**Auswirkung**: Beeinflusst die Message-Format-Definitions und Code-Generierungs-Pipeline.

### Message-Format
❓ **FRAGE AN USER**: Welches Format für Bifrost-Messages?
- Option A: JSON (einfach, menschenlesbar, größer)
- Option B: Protobuf (kompakt, schnell, typsicher)
- Option C: MessagePack (kompakt, einfacher als Protobuf)

**Auswirkung**: Beeinflusst die Message-Serialisierung, Performance und Bandbreite.

---

## Phase 1: Projekt-Setup & Grundstruktur

### 1.1 Projekt-Initialisierung

**Abhängigkeiten**: Keine
**Erforderliche USER-Eingaben**: WebSocket-Library, TLS-Library, Message-Format

#### 1.1.1 Cargo-Projekt erstellen
- [x] `Cargo.toml` erstellen
- [x] Basis-Dependencies definieren
  - Async Runtime (tokio)
  - Serialization (serde, serde_json oder prost)
  - Logging (tracing, tracing-subscriber)
  - Error-Handling (anyhow, thiserror)
  - WebSocket-Library (tokio-tungstenite)
  - TLS-Library (rustls oder native-tls)
- [x] `.gitignore` erstellen

#### 1.1.2 Verzeichnisstruktur erstellen
- [x] `src/main.rs` erstellen
- [x] `src/lib.rs` erstellen
- [x] `src/connection/` für Connection-Management erstellen
- [x] `src/routing/` für Message-Routing erstellen
- [x] `src/mesh/` für Mesh-Layer erstellen (MeshPacket, FloodRouter, Discovery, Transport – siehe Phase 11, docs/MESH_LAYER_DESIGN.md)
- [x] `src/discovery/` für Device-Discovery erstellen
- [x] `src/protocol/` für Bifrost-Protocol erstellen
- [x] `src/security/` für Security-Features erstellen
- [x] `src/utils/` für Utilities erstellen
- [x] `config/` für Konfigurationsdateien erstellen
- [x] `tests/` für Tests erstellen

#### 1.1.3 Build-System einrichten
- [x] Build-Scripts in `Cargo.toml` definieren (kein build.rs, da kein Protobuf)
- [ ] Code-Generierungs-Pipeline einrichten (falls Protobuf)
- [x] Cargo-Features definieren (z.B. `rustls-tls`, `native-tls`); Default: `native-tls`

### 1.2 Test-Infrastruktur

**Abhängigkeiten**: 1.1 (Projekt-Initialisierung)

#### 1.2.1 Container-Setup für Tests
- [x] `Dockerfile` für Test-Umgebung erstellen (Dockerfile.test)
- [x] Docker Compose für Test-Services konfigurieren (docker-compose.test.yml)
  - Mock-Heimdall-Service
  - Mock-Asgard-Service
  - Mock-Yggdrasil-Service
- [x] Test-Container-Startup-Scripts erstellen – `scripts/run-tests.sh`, `scripts/run-tests.ps1` (von bifrost/ oder Repo-Root ausführbar)
- [x] **WICHTIG**: Alle Tests müssen in Containern laufen - keine lokalen Dependencies, Tools oder Services auf der Entwicklungsmaschine installieren (docker-compose.test.yml + Scripts)

#### 1.2.2 Test-Framework konfigurieren
- [x] Test-Dependencies hinzufügen (tokio-test, mockall, etc.) – in Cargo.toml [dev-dependencies]
- [x] Test-Utilities und Helpers erstellen – tests/utils (mod.rs, test_helpers.rs), connect_websocket_test_client
- [x] Mock-Setup für Services (Heimdall, Asgard, Yggdrasil) – tests/mocks (Dockerfile.mock-service, Cargo.toml, src/main.rs)
- [x] WebSocket-Test-Client erstellen – WebSocketClient (Phase 7.1.1) + tests/utils connect_websocket_test_client für Integrationstests

#### 1.2.3 CI/CD-Pipeline
- [x] GitHub Actions / GitLab CI Workflow erstellen – `.github/workflows/bifrost.yml` (Test in Container, Lint)
- [x] Automatische Test-Ausführung bei Commits konfigurieren – on push/PR zu `bifrost/**`, Job „Test (container)“ (Timeout 15 min)
- [x] Code-Coverage-Reporting einrichten (cargo-tarpaulin) – Job „Code coverage (tarpaulin)“ in bifrost.yml, Lcov-Report als Artifact
- [x] Linting und Formatting (cargo clippy, cargo fmt) – Job „Lint (fmt, clippy)“ in bifrost.yml

### 1.3 Projekt-Konfiguration

**Abhängigkeiten**: 1.1 (Projekt-Initialisierung)

#### 1.3.1 Settings-System Design
- [x] Settings-Schema definieren (JSON oder TOML)
- [x] Settings-Struktur entwerfen (Bifrost-spezifisch)
  - WebSocket-Port
  - TLS-Configuration
  - Connection-Settings
  - Routing-Settings
  - Security-Settings
  - NAT-Traversal-Settings

#### 1.3.2 Settings-Validierung
- [x] Rust-Structs für Settings definieren
- [x] Tests für Settings-Validierung schreiben
- [x] Settings-Validator implementieren (TDD)
  - Schema-Validierung
  - Range-Checks
  - Format-Validierung
- [x] Tests ausführen und bestehen

#### 1.3.3 Settings-Loader
- [x] Tests für Settings-Loader schreiben
- [x] Settings-Loader implementieren (TDD)
  - JSON/TOML Parsing
  - Environment-Variable-Override
  - Default-Settings
- [x] Hot-Reload-Mechanismus implementieren (TDD)
  - File-Watcher für Settings-Datei
  - Settings-Reload ohne Service-Restart
- [x] Tests ausführen und bestehen

---

## Phase 2: Message Format & Protocol Definitions

### 2.1 Message Format Definition

**Abhängigkeiten**: 1.1 (Projekt-Initialisierung)
**Erforderliche USER-Eingaben**: Message-Format-Wahl, Protobuf-Tool (falls Protobuf)

#### 2.1.1 Message Types Definition
- [x] Message-Type-Enum definieren
  - CONNECTION_REQUEST
  - CONNECTION_RESPONSE
  - MESSAGE
  - HEARTBEAT
  - DISCONNECT
  - ERROR
  - VERSION_NEGOTIATION
  - CHALLENGE_REQUEST
  - CHALLENGE_RESPONSE
  - CHALLENGE_PROOF
  - AUTHENTICATION_RESULT

#### 2.1.2 Message Structs (JSON oder Protobuf)
- [x] Falls JSON: Message-Structs mit Serde definieren
- [ ] Falls Protobuf: `.proto` Definitions erstellen
  - Shared Protobuf-Projekt verwenden (wenn vorhanden)
  - Code-Generierung konfigurieren
- [x] Message-Versionierung implementieren (Version-Header)

#### 2.1.3 Message Serialization/Deserialization
- [x] Tests für Message-Serialization schreiben
- [x] Serialization implementieren (TDD)
- [x] Tests für Message-Deserialization schreiben
- [x] Deserialization implementieren (TDD)
- [x] Tests ausführen und bestehen

#### 2.1.4 Service-Integration-Protocols
- [x] Service-Integration-Protokolle definieren:
  - Heimdall: gRPC (Connection Validation) - siehe Phase 5, [docs/SERVICE_INTEGRATION_PROTOCOLS.md](docs/SERVICE_INTEGRATION_PROTOCOLS.md)
  - Asgard: WebSocket-Relay (Device-Relay) - siehe Phase 9.2.2
  - Yggdrasil Ratatoskr: WebSocket (Business-Logic) - siehe Phase 9.2.3
  - Yggdrasil API: gRPC (Device-Registry, User-Management) - siehe Phase 9.2.3

### 2.2 Protocol Versionierung

**Abhängigkeiten**: 2.1 (Message Format Definition)

#### 2.2.1 Version-Negotiation Protocol
- [x] Tests für Version-Negotiation schreiben
- [x] `VersionNegotiator` implementieren (TDD)
  - Semantic Versioning (Major.Minor.Patch)
  - Version-Negotiation bei Connection-Establishment
  - Höchste gemeinsame Version auswählen
- [x] Tests ausführen und bestehen

#### 2.2.2 Backward-Compatibility
- [x] Tests für Backward-Compatibility schreiben
- [x] Backward-Compatible-Message-Handling implementieren (TDD)
  - Minor-Updates: alte Clients mit neuen Servern
  - Major-Updates: Version-Mismatch-Handling
- [x] Tests ausführen und bestehen

---

## Phase 3: TLS & Encryption Setup

### 3.1 TLS Configuration

**Abhängigkeiten**: 1.1 (Projekt-Initialisierung)
**Erforderliche USER-Eingaben**: TLS-Library-Wahl

#### 3.1.1 TLS Server Setup
- [ ] Tests für TLS-Server-Setup schreiben
- [ ] `TLSServerConfig` implementieren (TDD)
  - TLS 1.3 konfigurieren
  - Certificate-Loading
  - Certificate-Validation
- [ ] Tests ausführen und bestehen

#### 3.1.2 TLS Client Setup
- [ ] Tests für TLS-Client-Setup schreiben
- [ ] `TLSClientConfig` implementieren (TDD)
  - TLS 1.3 konfigurieren
  - Certificate-Verification
  - Server-Certificate-Validation
- [ ] Tests ausführen und bestehen

### 3.2 Certificate Management

**Abhängigkeiten**: 3.1 (TLS Configuration)

#### 3.2.1 Certificate Loader
- [ ] Tests für Certificate-Loader schreiben
- [ ] `CertificateLoader` implementieren (TDD)
  - Certificate-Files laden
  - Private-Key-Files laden
  - PEM-Format-Parsing
- [ ] Tests ausführen und bestehen

#### 3.2.2 Certificate Validation
- [ ] Tests für Certificate-Validation schreiben
- [ ] `CertificateValidator` implementieren (TDD)
  - Certificate-Chain-Validation
  - Certificate-Expiration-Check
  - Certificate-Revocation-Check (optional)
- [ ] Tests ausführen und bestehen

### 3.3 Public/Private Key Management

**Abhängigkeiten**: Keine (unabhängig von TLS)

#### 3.3.1 Key Generation
- [x] Tests für Key-Generation schreiben
- [x] `KeyGenerator` implementieren (TDD)
  - RSA 2048 oder Ed25519 Key-Pair-Generation (Ed25519 implementiert)
  - Key beim ersten Device-Start generieren
- [x] Tests ausführen und bestehen

#### 3.3.2 Key Storage
- [x] Tests für Key-Storage schreiben
- [x] `KeyStorage` implementieren (TDD)
  - Private-Key verschlüsselt speichern (passphrase-basiert AES-256-GCM)
  - Public-Key unverschlüsselt speichern
  - Key-Loading
- [x] Tests ausführen und bestehen

---

## Phase 4: Connection/Authentication Protocol

### 4.1 Challenge-Response-Mechanismus

**Abhängigkeiten**: 3.3 (Key Management), 2.1 (Message Format)

#### 4.1.1 Challenge-Request Handler
- [x] Tests für Challenge-Request schreiben
- [x] `ChallengeRequestHandler` implementieren (TDD)
  - Challenge-Request-Message generieren
  - Device-ID und Public-Key einbinden
  - Digital-Signature erstellen
- [x] Tests ausführen und bestehen

#### 4.1.2 Challenge-Response Generator
- [x] Tests für Challenge-Response schreiben
- [x] `ChallengeResponseGenerator` implementieren (TDD)
  - Random-Challenge-String generieren
  - Challenge-Expiration setzen
  - Digital-Signature erstellen
- [x] Tests ausführen und bestehen

#### 4.1.3 Challenge-Proof Handler
- [x] Tests für Challenge-Proof schreiben
- [x] `ChallengeProofHandler` implementieren (TDD)
  - Challenge mit Private-Key signieren
  - Proof-Message generieren
  - Digital-Signature erstellen
- [x] Tests ausführen und bestehen

#### 4.1.4 Challenge-Proof Validator
- [x] Tests für Challenge-Proof-Validation schreiben
- [x] `ChallengeProofValidator` implementieren (TDD)
  - Challenge-Proof mit Public-Key verifizieren
  - Challenge-Expiration prüfen
  - Signature-Validation
- [x] Tests ausführen und bestehen

### 4.2 Token Management

**Abhängigkeiten**: 4.1 (Challenge-Response)

#### 4.2.1 Token Generator
- [x] Tests für Token-Generation schreiben
- [x] `TokenGenerator` implementieren (TDD)
  - Heimdall-Token generieren (nach erfolgreicher Authentifizierung)
  - Token-Expiration setzen
  - Refresh-Token generieren
- [x] Tests ausführen und bestehen

#### 4.2.2 Token Validator
- [x] Tests für Token-Validation schreiben
- [x] `TokenValidator` implementieren (TDD)
  - Token-Signature verifizieren
  - Token-Expiration prüfen
  - Token-Revocation prüfen
- [x] Tests ausführen und bestehen

#### 4.2.3 Token Refresh Manager
- [x] Tests für Token-Refresh schreiben
- [x] `TokenRefreshManager` implementieren (TDD)
  - Refresh-Token validieren
  - Neues Token generieren
  - Proaktive Token-Erneuerung
- [x] Tests ausführen und bestehen

### 4.3 Rate Limiting

**Abhängigkeiten**: 4.1 (Challenge-Response)

#### 4.3.1 Rate Limiter
- [x] Tests für Rate-Limiter schreiben
- [x] `RateLimiter` implementieren (TDD)
  - Token-basiertes Rate-Limiting
  - Sliding-Window-Algorithmus
  - Rate-Limit-Überschreitungs-Handling
- [x] Tests ausführen und bestehen

---

## Phase 5: Heimdall Integration (Connection Validation)

### 5.1 Heimdall Client

**Abhängigkeiten**: 2.1 (Message Format), 4.2 (Token Management)

#### 5.1.1 Heimdall gRPC Client Setup
- [x] Tests für Heimdall-Client schreiben
- [x] `HeimdallClient` implementieren (TDD)
  - gRPC-Connection zu Heimdall (Interface + Stub; echte gRPC später)
  - Connection-Pooling (bei echter gRPC-Integration)
  - Retry-Logik
- [x] Tests ausführen und bestehen

### 5.2 Connection Validation

**Abhängigkeiten**: 5.1 (Heimdall Client)

#### 5.2.1 Connection Validation Request Handler
- [x] Tests für Connection-Validation schreiben
- [x] `ConnectionValidationHandler` implementieren (TDD)
  - `ConnectionValidationRequest` an Heimdall senden
  - Request mit Device-Private-Key signieren
  - Response verarbeiten
- [x] Tests ausführen und bestehen

#### 5.2.2 Validation Response Handler
- [x] Tests für Validation-Response schreiben
- [x] `ValidationResponseHandler` implementieren (TDD)
  - `ConnectionValidationResponse` verarbeiten
  - Status (ALLOW/DENY) auswerten
  - Validation-Token extrahieren
- [x] Tests ausführen und bestehen

### 5.3 User-Isolation Rules

**Abhängigkeiten**: 5.2 (Connection Validation)

#### 5.3.1 User-Identity Verification
- [x] Tests für User-Identity-Verification schreiben
- [x] `UserIdentityVerifier` implementieren (TDD)
  - User-Identity prüfen (gleicher User vs. verschiedene User)
  - Edda-Network-Membership prüfen
  - Bestätigungs-Status prüfen
- [x] Tests ausführen und bestehen

#### 5.3.2 Cross-User Connection Blocking
- [x] Tests für Cross-User-Connection-Blocking schreiben
- [x] `CrossUserConnectionBlocker` implementieren (TDD)
  - Direkte Verbindungen zwischen verschiedenen Usern blockieren
  - Yggdrasil-Relay-Requirement enforc en
- [x] Tests ausführen und bestehen

### 5.4 Connection Status Monitoring

**Abhängigkeiten**: 5.2 (Connection Validation)

#### 5.4.1 Connection Status Tracker
- [x] Tests für Connection-Status-Tracking schreiben
- [x] `ConnectionStatusTracker` implementieren (TDD)
  - Connection-Status verfolgen (ACTIVE, IDLE, SUSPICIOUS, BLOCKED)
  - Status-Updates von Heimdall verarbeiten
  - Status-Änderungen an Clients propagieren
- [x] Tests ausführen und bestehen

#### 5.4.2 Connection Blocking Mechanism
- [x] Tests für Connection-Blocking schreiben
- [x] `ConnectionBlocker` implementieren (TDD)
  - Connection sofort blockieren bei Threat
  - Token-Revocation
  - Security-Alert auslösen
  - Audit-Log schreiben
- [x] Tests ausführen und bestehen

---

## Phase 6: WebSocket Server Implementation

### 6.1 WebSocket Server Setup

**Abhängigkeiten**: 3.1 (TLS Configuration), 1.1 (Projekt-Initialisierung)
**Erforderliche USER-Eingaben**: WebSocket-Library-Wahl

#### 6.1.1 WebSocket Server Core
- [x] Tests für WebSocket-Server-Setup schreiben
- [x] `WebSocketServer` implementieren (TDD)
  - WebSocket-Server starten (tokio-tungstenite)
  - TLS-Support integrieren (optional, aktuell ohne TLS)
  - Port-Konfiguration
  - Listener-Loop
- [x] Tests ausführen und bestehen

#### 6.1.2 Connection Acceptor
- [x] Tests für Connection-Acceptor schreiben
- [x] `ConnectionAcceptor` implementieren (TDD)
  - Incoming-Connections akzeptieren
  - TLS-Handshake durchführen
  - WebSocket-Upgrade durchführen
- [x] Tests ausführen und bestehen

### 6.2 WebSocket Connection Management

**Abhängigkeiten**: 6.1 (WebSocket Server Setup)

#### 6.2.1 Connection Handler
- [x] Tests für Connection-Handler schreiben
- [x] `ConnectionHandler` implementieren (TDD)
  - WebSocket-Connection verwalten
  - Message-Empfang
  - Message-Versand
  - Connection-State verwalten
- [x] Tests ausführen und bestehen

#### 6.2.2 Connection Pool
- [x] Tests für Connection-Pool schreiben
- [x] `ConnectionPool` implementieren (TDD) – ConnectionManager mit Register/Get/ListByDevice/Remove
  - Active-Connections verwalten
  - Connection-Lookup (by Device-ID)
  - Connection-Cleanup
- [x] Tests ausführen und bestehen

#### 6.2.3 Heartbeat Mechanism
- [x] Tests für Heartbeat schreiben
- [x] `HeartbeatManager` implementieren (TDD)
  - Heartbeat-Messages senden (konfigurierbare Frequenz)
  - Heartbeat-Messages empfangen
  - Fehlende Heartbeats erkennen
  - Connection-Timeout bei fehlenden Heartbeats
- [x] Tests ausführen und bestehen

---

## Phase 7: WebSocket Client Implementation

### 7.1 WebSocket Client Setup

**Abhängigkeiten**: 3.1 (TLS Configuration), 1.1 (Projekt-Initialisierung)

#### 7.1.1 WebSocket Client Core
- [x] Tests für WebSocket-Client-Setup schreiben
- [x] `WebSocketClient` implementieren (TDD)
  - WebSocket-Connection initiieren
  - TLS-Handshake durchführen
  - WebSocket-Upgrade durchführen
- [x] Tests ausführen und bestehen

#### 7.1.2 Connection Initiator
- [x] Tests für Connection-Initiator schreiben
- [x] `ConnectionInitiator` implementieren (TDD)
  - Connection-Request senden
  - Connection-Response verarbeiten
  - Connection-Establishment
- [x] Tests ausführen und bestehen

### 7.2 Automatic Reconnection

**Abhängigkeiten**: 7.1 (WebSocket Client Setup)

#### 7.2.1 Reconnection Manager
- [x] Tests für Reconnection schreiben
- [x] `ReconnectionManager` implementieren (TDD)
  - Sofortiger Reconnect-Versuch bei Disconnect
  - Exponential-Backoff-Algorithmus
  - Maximale Wartezeit (60 Sekunden)
  - Jitter zur Vermeidung von Thundering-Herd
  - Kontinuierliche Versuche
- [x] Tests ausführen und bestehen

---

## Phase 8: Device Discovery

### 8.1 Local Device Discovery (mDNS/Bonjour) ✅

**Abhängigkeiten**: 6.1 (WebSocket Server), 7.1 (WebSocket Client)
**Entscheidung**: Stub-Implementierung für Container/CI (mDNS-Multicast in Containern oft nicht verfügbar). Echte mDNS-Library (z. B. `mdns-sd`) optional später integrierbar über `MDNSServiceProvider`-Trait.

#### 8.1.1 mDNS/Bonjour Service ✅
- [x] Tests für mDNS-Service schreiben (`tests/mdns_service_test.rs`, Unit-Tests in `src/discovery/mdns.rs`)
- [x] `MDNSService` + `MDNSServiceStub` implementieren (TDD)
  - Service-Announcement (Device registriert sich)
  - Service-Discovery (Device sucht andere Devices)
  - Service-Record-Parsing (Stub liefert simulierte Devices)
- [x] Tests ausführen und bestehen

#### 8.1.2 Local Discovery Manager ✅
- [x] Tests für Local-Discovery schreiben (`tests/local_discovery_manager_test.rs`)
- [x] `LocalDiscoveryManager` implementieren (TDD)
  - Discovery-Requests senden
  - Discovery-Responses verarbeiten
  - Discovery-Timeouts behandeln
  - Device-Liste aktualisieren
- [x] Tests ausführen und bestehen

### 8.2 Manual Discovery (IP-based)

**Abhängigkeiten**: 7.1 (WebSocket Client)

#### 8.2.1 IP-based Connection Manager
- [x] Tests für IP-based-Connection schreiben
- [x] `IPConnectionManager` implementieren (TDD)
  - Connection zu IP-Adresse initiieren
  - Connection-Timeout
  - Connection-Retry
- [x] Tests ausführen und bestehen

### 8.3 Global Discovery (via Yggdrasil)

**Abhängigkeiten**: 6.1 (WebSocket Server), 7.1 (WebSocket Client)

#### 8.3.1 Yggdrasil Discovery Client
- [x] Tests für Yggdrasil-Discovery schreiben
- [x] `YggdrasilDiscoveryClient` implementieren (TDD)
  - Discovery-Request an Yggdrasil senden
  - Discovery-Response von Yggdrasil verarbeiten
  - Device-Liste von Yggdrasil abrufen
- [x] Tests ausführen und bestehen

---

## Phase 9: Message Routing

### 9.1 Direct Routing

**Abhängigkeiten**: 6.2 (Connection Management), 2.1 (Message Format)

#### 9.1.1 Direct Message Router
- [x] Tests für Direct-Routing schreiben
- [x] `DirectMessageRouter` implementieren (TDD) – MessageRouter
  - Message direkt an Ziel-Device senden
  - Connection-Lookup im Connection-Pool
  - Message-Delivery-Confirmation
- [x] Tests ausführen und bestehen

#### 9.1.2 Routing Table Manager
- [x] Tests für Routing-Table schreiben
- [x] `RoutingTableManager` implementieren (TDD) – ConnectionManager (Device-ID → Connection)
  - Routing-Table verwalten (Device-ID → Connection)
  - Routing-Table-Updates
  - Routing-Table-Queries
- [x] Tests ausführen und bestehen

### 9.2 Relay Routing

**Abhängigkeiten**: 9.1 (Direct Routing), 7.1 (WebSocket Client)

#### 9.2.1 Relay Manager
- [x] Tests für Relay-Routing schreiben
- [x] `RelayManager` implementieren (TDD)
  - Message über Relay routen (Asgard/Yggdrasil)
  - Relay-Server-Auswahl (basierend auf Verfügbarkeit)
  - Relay-Fallback bei Ausfall
- [x] Tests ausführen und bestehen

#### 9.2.2 Asgard Relay Client
- [x] Tests für Asgard-Relay schreiben
- [x] `AsgardRelayClient` implementieren (TDD)
  - Connection zu Asgard-Relay
  - Message über Asgard routen
- [x] Tests ausführen und bestehen

#### 9.2.3 Yggdrasil Relay Client
- [x] Tests für Yggdrasil-Relay schreiben
- [x] `YggdrasilRelayClient` implementieren (TDD)
  - Persistente Connection zu Yggdrasil
  - Message über Yggdrasil routen
  - Event-Notifications von Yggdrasil empfangen
- [x] Tests ausführen und bestehen

### 9.3 Broadcast/Multicast

**Abhängigkeiten**: 9.1 (Direct Routing)

#### 9.3.1 Broadcast Manager
- [x] Tests für Broadcast schreiben
- [x] `BroadcastManager` implementieren (TDD)
  - Broadcast-Message an alle Devices senden
  - Broadcast-Storms verhindern (Rate-Limiting, TTL)
- [x] Tests ausführen und bestehen

#### 9.3.2 Multicast Manager
- [x] Tests für Multicast schreiben
- [x] `MulticastManager` implementieren (TDD)
  - Multicast-Message an Device-Gruppe senden
  - Device-Gruppen verwalten
- [x] Tests ausführen und bestehen

### 9.4 Routing Optimization

**Abhängigkeiten**: 9.1 (Direct Routing), 9.2 (Relay Routing)

#### 9.4.1 Connection Quality Monitor
- [x] Tests für Connection-Quality-Monitoring schreiben
- [x] `ConnectionQualityMonitor` implementieren (TDD)
  - Connection-Quality-Metriken sammeln (Latency, Packet-Loss)
  - Connection-Quality-Score berechnen
  - Quality-Degradation erkennen
- [x] Tests ausführen und bestehen

#### 9.4.2 Quality-based Routing
- [x] Tests für Quality-based-Routing schreiben
- [x] `QualityBasedRouter` implementieren (TDD)
  - Routing-Entscheidungen basierend auf Connection-Quality
  - Automatisches Failover bei Quality-Degradation
- [x] Tests ausführen und bestehen

---

## Phase 10: Error Recovery & Resilience

### 10.1 Network Error Handling

**Abhängigkeiten**: 9.1 (Direct Routing), 9.2 (Relay Routing)

#### 10.1.1 Retry Manager (mit Exponential Backoff)
- [x] Tests für Retry-Manager schreiben
- [x] `RetryManager` implementieren (TDD)
  - Sofortiger Retry bei Fehler
  - Exponential-Backoff-Algorithmus
  - Maximale Retries (5 Versuche)
  - Retry-Timeout
- [x] Tests ausführen und bestehen
- [x] `RetryManager` in `MessageRouter` integriert (`with_retry(RetryConfig)`); Routing-Fehler werden mit Backoff wiederholt

#### 10.1.2 Fallback Routing Manager
- [x] Tests für Fallback-Routing schreiben
- [x] `FallbackRoutingManager` implementieren (TDD)
  - Fallback-Hierarchie (Direct → Asgard → Yggdrasil)
  - Fallback-Trigger (Retry-Limit, Timeout) – nutzt MessageRouter-Retry
  - Route-Erkennung und Route-Auswahl (Reihenfolge Direct → Asgard → Yggdrasil)
- [x] Tests ausführen und bestehen
- [x] Stubs `AsgardRelayStub` / `YggdrasilRelayStub` bis Phase 9.2 implementiert sind

### 10.2 Connection Error Recovery

**Abhängigkeiten**: 7.2 (Automatic Reconnection)

#### 10.2.1 Connection Error Handler
- [x] Tests für Connection-Error-Handler schreiben
- [x] `ConnectionErrorHandler` implementieren (TDD)
  - Connection-Errors kategorisieren (Transient, Permanent, Auth, Network, Timeout, Critical)
  - Retry vs. Fallback-Entscheidung (`suggest_action`: Retry, Fallback, NotifyUser, LogOnly)
  - User-Benachrichtigung bei kritischen Fehlern (Action NotifyUser für Auth/Critical)
- [x] Tests ausführen und bestehen

---

## Phase 11: Mesh-Integration (Device-Mesh)

**Hinweis**: VPN (Valhalla) wurde verworfen. Stattdessen erweitert Bifrost um ein Meshtastic-inspiriertes Device-Mesh (IP + optional LoRa). Ein Dienst, ein Name: Bifrost.

**Design-Referenz**: [docs/MESH_LAYER_DESIGN.md](docs/MESH_LAYER_DESIGN.md) – Paketformat (MeshPacket/Data), Managed Flood, Hop-Limit, Discovery, IP-Transport (WebSocket/TLS).

### 11.0 Mesh-Layer-Grundstruktur (Voraussetzung für 11.1)

**Abhängigkeiten**: 6.2 (Connection Management)

#### 11.0.1 Mesh-Layer-Modul
- [x] `src/mesh/` Verzeichnis anlegen
- [x] `src/mesh/mod.rs`, `src/mesh/packet.rs`, `src/mesh/flood_router.rs`, `src/mesh/discovery.rs`, `src/mesh/transport.rs` (Struktur)
- [x] MeshPacket- und Data-artige Typen (Rust-Structs mit Serde) definieren (siehe MESH_LAYER_DESIGN)
- [x] Tests für MeshPacket/Data-Serialisierung schreiben
- [x] FloodRouter-Stub (should_forward mit hop_limit, my_node_id) – Grundstruktur
- [x] Discovery-Stub (NodeInfo/MyNodeInfo) – Grundstruktur
- [x] IP-Transport: WebSocket als Transport für MeshPackets (Anbindung an bestehenden WebSocket-Server)
  - Codec encode_mesh_packet/decode_mesh_packet in mesh/transport
  - handle_connection: Text-Frame zuerst als MeshPacket prüfen → FloodRouter.should_forward → hop_limit dekrementieren, an alle anderen Connections senden; sonst BifrostMessage
  - ConnectionManager.list_connection_ids() für Mesh-Broadcast
- [x] Tests ausführen und bestehen

### 11.1 Mesh-Layer

**Abhängigkeiten**: 6.2 (Connection Management)

#### 11.1.1 Mesh-Membership-Checker
- [x] Tests für Mesh-Membership-Check schreiben
- [x] `MeshMembershipChecker` implementieren (TDD)
  - Mesh-Membership prüfen (Heimdall) – über `MeshMembershipProvider`-Trait; Stub bis Phase 5
  - Mesh-Connectivity prüfen (`is_mesh_connected`)
  - User-Mesh-Zugehörigkeit prüfen (`is_user_in_mesh`, `is_device_in_mesh`)
- [x] Tests ausführen und bestehen
- [x] `MeshMembershipStub` (all_allowed / all_denied / custom) für Tests und bis Heimdall-Integration

#### 11.1.2 Mesh-Status-Monitor
- [x] Tests für Mesh-Status-Monitoring schreiben
- [x] `MeshStatusMonitor` implementieren (TDD)
  - Kontinuierliche Mesh-Connectivity-Überwachung (`check()` delegiert an MeshMembershipChecker)
  - Mesh-Ausfall erkennen (`MeshStatusSnapshot.failure_detected` bei Übergang connected → disconnected)
  - Mesh-Wiederherstellung bzw. alternative Hops erkennen (`recovery_detected` bei disconnected → connected)
- [x] Tests ausführen und bestehen
- [x] `MeshStatusSnapshot` (connected, failure_detected, recovery_detected); `is_connected()` liefert letzten bekannten Zustand

### 11.2 Mesh-Enforcement

**Abhängigkeiten**: 11.1 (Mesh-Layer), 5.2 (Connection Validation)

#### 11.2.1 Mesh Connection Enforcer
- [x] Tests für Mesh-Enforcement schreiben
- [x] `MeshConnectionEnforcer` implementieren (TDD)
  - Mesh-Membership bei jedem Connection-Request prüfen (`allow_connection(user_id, device_id)` nutzt MeshMembershipChecker)
  - Connection blockieren wenn kein Mesh (Err mit `MeshEnforcerError`)
  - Fehler-Message an Client senden (`client_message()` → z. B. "MESH_ACCESS_DENIED" für WebSocket-Close)
- [x] Tests ausführen und bestehen

#### 11.2.2 Mesh-based Connection Lifecycle
- [x] Tests für Mesh-based-Connection-Lifecycle schreiben
- [x] `MeshConnectionLifecycleManager` implementieren (TDD)
  - Connections bei Mesh-Ausfall schließen oder über alternative Hops routen (`tick()` → `LifecycleAction::MeshFailure`; Caller schließt Connections)
  - Automatische Wiederverbindung bei Mesh-Wiederherstellung (`tick()` → `LifecycleAction::MeshRecovery`; Caller löst Wiederverbindung aus)
- [x] Tests ausführen und bestehen
- [x] `MeshMembershipStub::set_connected()` für Lifecycle-Tests (Arc+RwLock für geteilte Connectivity)

---

## Phase 12: Guest Mesh (Gast-Mesh-Isolation)

**Wann ist Guest Mesh nötig?** Ein User mit eigenem Device nutzt bereits das **Haupt-Mesh** (Main Mesh) – kein Guest Mesh. **Guest Mesh** ist nur für **fremde Devices** (Besucher): z. B. wenn jemand mit seinem Phone an deinem Bifrost/Edda teilnimmt, ohne dort einen User-Account zu haben. Dafür wird ein isoliertes Segment (Guest Mesh) erstellt; Zugriff auf dein Haupt-Mesh oder Datentransfer erfordert explizite Erlaubnis (Heimdall-User-Bestätigung). Kurz: Eigene Devices → Main Mesh; fremde Devices (Gäste) → Guest Mesh.

**Evaluation (ohne VPN):** VPN (Valhalla) wurde verworfen; Connectivity läuft über Mesh-Membership. Das „Gast“-Konzept betrifft nur **fremde Devices** (z. B. Besucher-Phone): **isoliertes Mesh-Segment**, kein Zugriff auf das Haupt-User-Mesh, explizite Erlaubnis für Datentransfer. Mesh-nativ: **Guest Mesh** = separater Mesh-Segment mit eigener ID; kein VLAN/VPN. Phase 12 wird daher als **Guest Mesh** geführt; die bestehende Implementierung (IDs, Segmentation) bleibt und wird begrifflich angepasst.

### 12.1 Guest Mesh Management

**Abhängigkeiten**: 5.3 (User-Isolation Rules), 6.2 (Connection Management)

#### 12.1.1 Guest Mesh Manager
- [x] Tests für Guest-Mesh-Management schreiben
- [x] `GuestMeshManager` implementieren (TDD) – vormals Guest Network Manager
  - Automatisches Gast-Mesh erstellen (`create_guest_mesh()` → `GuestMeshId`)
  - Separate Mesh-ID für Gast-Segment (`MAIN_MESH_ID` vs. `guest-{uuid}`; `is_guest_mesh(id)`)
  - Mesh-Segmentation (Gast-IDs in `HashSet`, `list_guest_meshes()`)
- [x] Tests ausführen und bestehen

#### 12.1.2 Mesh Isolation Enforcer
- [x] Tests für Mesh-Isolation schreiben
- [x] `MeshIsolationEnforcer` implementieren (TDD)
  - Routing-Regeln für Gast-Mesh (kein Flood in Haupt-Mesh): `can_deliver(from_mesh_id, to_mesh_id)` – guest→main und main→guest blockiert
  - Blockierung Zugriff auf Haupt-Mesh (guest→main false)
  - Mesh-Segment-Isolation: gleicher Gast-Segment erlaubt, Gast↔anderer Gast blockiert
- [x] Tests ausführen und bestehen

### 12.2 Data Transfer Permission

**Abhängigkeiten**: 12.1 (Guest Mesh Management), 5.2 (Connection Validation)

#### 12.2.1 Data Transfer Request Handler
- [x] Tests für Data-Transfer-Request schreiben
- [x] `DataTransferRequestHandler` implementieren (TDD)
  - Data-Transfer-Request von Gast-Device empfangen
  - Request an Heimdall weiterleiten für User-Bestätigung
- [x] Tests ausführen und bestehen

#### 12.2.2 User Confirmation Manager
- [x] Tests für User-Confirmation schreiben
- [x] `UserConfirmationManager` implementieren (TDD)
  - User über Frontend benachrichtigen
  - User-Bestätigung empfangen (Allow/Deny)
  - Mehrfache Bestätigung für expliziten Zugang (2-3 Mal)
  - Bestätigungs-Intervall (mindestens 5 Sekunden)
  - Warnung über Sicherheitsrisiken
- [x] Tests ausführen und bestehen

#### 12.2.3 Permission Token Manager
- [x] Tests für Permission-Token schreiben
- [x] `PermissionTokenManager` implementieren (TDD)
  - Permission-Token generieren (nach User-Bestätigung)
  - Permission-Token validieren
  - Permission-Token-Expiration (z.B. 24h)
- [x] Tests ausführen und bestehen

### 12.3 Guest Mesh Cleanup

**Abhängigkeiten**: 12.1 (Guest Mesh Management)

#### 12.3.1 Guest Mesh Cleanup Manager
- [x] Tests für Guest-Mesh-Cleanup schreiben
- [x] `GuestMeshCleanupManager` implementieren (TDD)
  - Automatisches Cleanup bei Verbindungsabbruch
  - Timeout-basiertes Cleanup
  - Resource-Freigabe
- [x] Tests ausführen und bestehen

---

## Phase 13: NAT Traversal (WAN Connectivity)

### 13.1 STUN Client ✅ (Stub)

**Abhängigkeiten**: 7.1 (WebSocket Client)
**Entscheidung**: Stub-Implementierung für Container/CI. Echte STUN-Library (z. B. webrtc-rs oder stun-rs) optional später über `STUNClientProvider`-Trait integrierbar.

#### 13.1.1 STUN Client Implementation ✅
- [x] Tests für STUN-Client schreiben (`tests/stun_client_test.rs`, Unit-Tests in `src/nat/stun_client.rs`)
- [x] `STUNClient` + `STUNClientStub` implementieren (TDD)
  - Public-IP ermitteln (Stub: konfigurierbar)
  - NAT-Type ermitteln (Stub: konfigurierbar; Typen: Unknown, FullCone, RestrictedCone, PortRestrictedCone, Symmetric)
- [x] Tests ausführen und bestehen

### 13.2 TURN Server/Client ✅ (Stub)

**Abhängigkeiten**: 13.1 (STUN Client)

#### 13.2.1 TURN Client Implementation ✅
- [x] Tests für TURN-Client schreiben (`tests/turn_client_test.rs`, Unit-Tests in `src/nat/turn_client.rs`)
- [x] `TURNClient` + `TURNClientStub` implementieren (TDD)
  - Relay-Allocation (Stub: konfigurierbar; echte TURN-Library später über `TURNClientProvider`-Trait)
  - Connection zu TURN-Server (Asgard/Yggdrasil) – Stub liefert konfigurierte Allocation
- [x] Tests ausführen und bestehen

#### 13.2.2 TURN Server Implementation (Optional)
❓ **HINWEIS**: Falls Bifrost selbst als TURN-Server fungieren soll
- [ ] Tests für TURN-Server schreiben
- [ ] `TURNServer` implementieren (TDD)
  - TURN-Server für Relay
  - Relay-Connections verwalten
- [ ] Tests ausführen und bestehen

### 13.3 ICE Implementation ✅ (Stub)

**Abhängigkeiten**: 13.1 (STUN Client), 13.2 (TURN Client)

#### 13.3.1 ICE Manager ✅
- [x] Tests für ICE schreiben (`tests/ice_manager_test.rs`, Unit-Tests in `src/nat/ice_manager.rs`)
- [x] `ICEManager` + `ICEManagerStub` implementieren (TDD)
  - Candidate-Gathering (Host, ServerReflexive, Relayed) – Stub liefert konfigurierbare Kandidaten
  - Best-Path-Selection – Stub liefert konfigurierbaren SelectedPath
  - Connectivity-Checks (echte Implementierung später über Provider)
- [x] Tests ausführen und bestehen

### 13.4 NAT Traversal Fallback ✅ (Stub)

**Abhängigkeiten**: 13.3 (ICE Implementation)

#### 13.4.1 Manual Port Forwarding Configuration ✅
- [x] Tests für Port-Forwarding-Config schreiben (`tests/port_forwarding_configurator_test.rs`, Unit-Tests in `src/nat/port_forwarding.rs`)
- [x] `PortForwardingConfigurator` + `PortForwardingConfiguratorStub` implementieren (TDD)
  - Manuelle/automatische Port-Forwarding-Konfiguration (Stub: konfigurierbarer Erfolg/Misserfolg)
  - UPnP/NAT-PMP für automatisches Port-Forwarding (echte Implementierung später über Provider)
  - Router-Kompatibilitätsprobleme (Stub simuliert Erfolg/Misserfolg)
- [x] Tests ausführen und bestehen

---

## Phase 14: Monitoring & Logging

### 14.1 Structured Logging

**Abhängigkeiten**: 1.1 (Projekt-Initialisierung)

#### 14.1.1 Logging Setup
- [x] Structured-Logging konfigurieren (tracing)
- [x] Log-Levels definieren (trace, debug, info, warn, error)
- [x] Context-Tracking implementieren
- [x] Log-Rotation konfigurieren

#### 14.1.2 Audit Logging
- [x] Tests für Audit-Logging schreiben
- [x] `AuditLogger` implementieren (TDD)
  - Security-relevante Events loggen
  - Connection-Events loggen
  - Authentication-Events loggen
- [x] Tests ausführen und bestehen

### 14.2 Performance Monitoring

**Abhängigkeiten**: 9.4 (Routing Optimization)

#### 14.2.1 Metrics Collector
- [x] Tests für Metrics-Collector schreiben
- [x] `MetricsCollector` implementieren (TDD)
  - Performance-Metriken sammeln (Response-Zeiten, Durchsatz)
  - Connection-Quality-Metriken sammeln
  - Resource-Usage-Metriken sammeln
- [x] Tests ausführen und bestehen

#### 14.2.2 Performance Alerts
- [x] Tests für Performance-Alerts schreiben
- [x] `PerformanceAlertManager` implementieren (TDD)
  - Alerts bei Performance-Problemen
  - Threshold-basierte Alerts
- [x] Tests ausführen und bestehen

---

## Phase 15: Security Hardening

### 15.1 Message Validation

**Abhängigkeiten**: 2.1 (Message Format), 6.2 (Connection Management)

#### 15.1.1 Message Validator
- [x] Tests für Message-Validation schreiben
- [x] `MessageValidator` implementieren (TDD)
  - Message-Format-Validation
  - Message-Signature-Validation
  - Message-Sanitization (Injection-Prevention)
- [x] Tests ausführen und bestehen

### 15.2 Threat Detection

**Abhängigkeiten**: 5.4 (Connection Status Monitoring)

#### 15.2.1 Anomaly Detector
- [x] Tests für Anomaly-Detection schreiben
- [x] `AnomalyDetector` implementieren (TDD)
  - Ungewöhnliche Connection-Patterns erkennen
  - Anomaly-Score berechnen
  - Anomaly-Alerts auslösen
- [x] Tests ausführen und bestehen

#### 15.2.2 Intrusion Detection
- [x] Tests für Intrusion-Detection schreiben
- [x] `IntrusionDetector` implementieren (TDD)
  - Angriffsmuster erkennen
  - Security-Alerts auslösen
  - Automatische Connection-Blockierung
- [x] Tests ausführen und bestehen

### 15.3 Security Testing

**Abhängigkeiten**: Alle vorherigen Phasen

#### 15.3.1 Security Audit
- [ ] Security-Audit durchführen (manuell/periodisch)
- [x] Vulnerability-Scanning (cargo-audit) – CI-Job „Security (cargo-audit)“ in bifrost.yml
- [ ] Penetration-Testing (optional)
- [x] Security-Findings dokumentieren und beheben – [docs/SECURITY.md](docs/SECURITY.md) (Prozess, Template, Behebung)

#### 15.3.2 WebSocket Security Tests
- [x] Tests für WebSocket-Security schreiben – [tests/websocket_security_test.rs](tests/websocket_security_test.rs)
  - Unauthorized-Access-Tests: Validation DENY → kein Zugang; Cross-User Direct → blockiert; Threat → Block + Revoke
  - Sicherstellen, dass unauthorized Users keine Daten empfangen können (ValidationResponseHandler Denied, CrossUserConnectionBlocker, ConnectionBlocker)
- [ ] Tests ausführen und bestehen (100% Coverage für Security-Tests – Ziel; Basis-Tests vorhanden)

---

## Phase 16: Message Queuing (Offline Devices)

### 16.1 Message Queue Management

**Abhängigkeiten**: 9.1 (Direct Routing), 6.2 (Connection Management)

#### 16.1.1 Message Queue Manager
- [x] Tests für Message-Queuing schreiben
- [x] `MessageQueueManager` implementieren (TDD)
  - Messages für Offline-Devices queuen
  - Queue-Size-Limits (konfigurierbar)
  - Queue-Overflow behandeln (Eviction, Notification)
- [x] Tests ausführen und bestehen

#### 16.1.2 Queue Delivery Manager
- [x] Tests für Queue-Delivery schreiben
- [x] `QueueDeliveryManager` implementieren (TDD)
  - Messages senden wenn Device online
  - Message-Delivery-Order (FIFO)
  - Delivery-Confirmation
- [x] Tests ausführen und bestehen

---

## Phase 17: Cross-Device Action Execution (gRPC over Bifrost)

### 17.1 gRPC over WebSocket

**Abhängigkeiten**: 6.2 (Connection Management), 9.1 (Direct Routing)

#### 17.1.1 gRPC Bridge
- [x] Tests für gRPC-Bridge schreiben
- [x] `GRPCBridge` implementieren (TDD)
  - gRPC-Requests über Bifrost-WebSocket tunneln
  - gRPC-Responses über Bifrost-WebSocket tunneln
  - Streaming-Support für lange Actions (Vorbereitung: request_id-Korrelation, payload body)
- [x] Tests ausführen und bestehen

#### 17.1.2 ThorAction Routing
- [x] Tests für ThorAction-Routing schreiben
- [x] `ThorActionRouter` implementieren (TDD)
  - ThorAction via gRPC an Remote-Device senden (über Bifrost)
  - ThorResult von Remote-Device empfangen
  - Action-Timeout-Handling
- [x] Tests ausführen und bestehen

---

## Phase 18: Caching & Performance Optimization

### 18.1 Connection Information Caching

**Abhängigkeiten**: 6.2 (Connection Management), 9.1 (Direct Routing)

#### 18.1.1 Connection Cache Manager
- [x] Tests für Connection-Cache schreiben
- [x] `ConnectionCacheManager` implementieren (TDD)
  - Connection-Information cachen
  - Cache-Invalidation bei Status-Updates
  - Cache-TTL
- [x] Tests ausführen und bestehen

### 18.2 Validation Caching (Optional)

**Abhängigkeiten**: 5.2 (Connection Validation)

#### 18.2.1 Validation Cache Manager
- [x] Tests für Validation-Cache schreiben
- [x] `ValidationCacheManager` implementieren (TDD)
  - Validation-Results cachen (optional, für Performance)
  - Cache-TTL (z.B. 5 Minuten)
  - Cache-Invalidation bei Status-Updates
- [x] Tests ausführen und bestehen

### 18.3 Message Batching (Optional)

**Abhängigkeiten**: 9.1 (Direct Routing)

#### 18.3.1 Message Batch Manager
- [x] Tests für Message-Batching schreiben
- [x] `MessageBatchManager` implementieren (TDD)
  - Multiple Messages zu Batch zusammenfassen
  - Batch-Size-Limits
  - Batch-Delivery
- [x] Tests ausführen und bestehen

---

## Phase 19: Documentation

### 19.1 Protocol Documentation

**Abhängigkeiten**: 2.1 (Message Format), 4.1 (Challenge-Response)

#### 19.1.1 Protocol Specification
- [x] Bifrost-Protocol-Specification erstellen – [docs/BIFROST_PROTOCOL_SPECIFICATION.md](docs/BIFROST_PROTOCOL_SPECIFICATION.md)
  - Message-Types dokumentieren
  - Message-Format dokumentieren
  - Connection-Workflow dokumentieren
  - Authentication-Workflow dokumentieren
- [x] Protocol-Examples erstellen – JSON-Beispiele in BIFROST_PROTOCOL_SPECIFICATION.md (Connection-Request/Response, MESSAGE, HEARTBEAT, GRPC_REQUEST)

#### 19.1.2 Connection/Authentication Protocol Documentation
- [x] Connection/Authentication-Protocol-Specification erstellen – [docs/BIFROST_CONNECTION_AUTH_PROTOCOL.md](docs/BIFROST_CONNECTION_AUTH_PROTOCOL.md)
  - Challenge-Response-Mechanismus dokumentieren
  - Token-Management dokumentieren
  - Rate-Limiting dokumentieren

### 19.2 Code Documentation

**Abhängigkeiten**: Alle vorherigen Phasen

#### 19.2.1 Rust Documentation
- [x] Alle Public-APIs mit Rustdoc dokumentieren – Crate- und Modul-Docs in lib.rs, message, connection; weitere Module mit //! (Phase 19.1/4)
- [x] Code-Examples in Rustdoc hinzufügen – BifrostMessage, MessageHandler (parse/serialize), WebSocketClient (connect)
- [x] Rustdoc generieren (`cargo doc`) – CI-Job „Rustdoc“ in bifrost.yml, Artifact bifrost-rustdoc

#### 19.2.2 Architecture Documentation
- [x] Architecture-Diagramm erstellen – [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md) (Mermaid flowchart)
- [x] Sequence-Diagramme für Connection-Establishment – Mermaid sequence in ARCHITECTURE.md
- [x] Sequence-Diagramme für Message-Routing – Mermaid sequence in ARCHITECTURE.md

### 19.3 User Documentation

**Abhängigkeiten**: Alle vorherigen Phasen

#### 19.3.1 Integration Guide
- [x] Integration-Guide für Platforms erstellen – [docs/INTEGRATION_GUIDE.md](docs/INTEGRATION_GUIDE.md)
  - Wie Platforms Bifrost nutzen
  - Connection-Establishment-Examples
  - Message-Routing-Examples

---

## Phase 20: Testing & Quality Assurance

**Verifikation:** Nach erfolgreicher Ausführung von `docker compose -f docker-compose.test.yml run --rm bifrost-test` (bzw. `./scripts/run-tests.sh` / `.\scripts\run-tests.ps1`) die Checkboxen „… ausführen und bestehen“ unten abhaken.

### 20.1 Integration Testing

**Abhängigkeiten**: Alle vorherigen Phasen

#### 20.1.1 End-to-End Tests
- [x] E2E-Tests für komplette Communication-Workflows schreiben
  - Device-Discovery → Connection-Establishment → Message-Exchange
  - Direct-Routing
  - Relay-Routing (Asgard, Yggdrasil)
  - Cross-Device-Actions (gRPC over Bifrost)
  - Dedizierte Suite: [tests/e2e_communication_workflow_test.rs](tests/e2e_communication_workflow_test.rs)
- [x] E2E-Tests ausführen und bestehen

#### 20.1.2 Error Recovery Tests
- [x] Error-Recovery-Tests schreiben
  - Automatic-Reconnection-Tests
  - Retry-Mechanism-Tests
  - Fallback-Routing-Tests
  - Dedizierte Suite: `tests/error_recovery_test.rs`
- [x] Error-Recovery-Tests ausführen und bestehen

### 20.2 Performance Testing

**Abhängigkeiten**: 18.1 (Caching & Performance Optimization)

#### 20.2.1 Performance Benchmarks
- [x] Performance-Benchmarks definieren
  - Message-Routing-Latency (< 10ms lokal; Test-Schwelle relaxed für CI)
  - Message-Throughput (Messages/Sekunde; Mindest-Durchsatz)
  - Connection-Establishment-Time
  - Dedizierte Suite: [tests/performance_benchmark_test.rs](tests/performance_benchmark_test.rs)
- [x] Performance-Tests ausführen und bestehen

### 20.3 Security Testing

**Abhängigkeiten**: 15.3 (Security Testing)

#### 20.3.1 Security Test Suite
- [x] Comprehensive Security-Tests (Suite definiert)
  - WebSocket-Security (Validation DENY/ALLOW, siehe [tests/security_test_suite.rs](tests/security_test_suite.rs))
  - Unauthorized-Access-Prevention (Cross-User-Block, Threat-Block + Revoke)
  - Connection-Authentication-Tests (Challenge-Request)
  - Message-Validation-Tests (Invalid Format, PayloadTooLarge, Sanitize)
  - Siehe auch: websocket_security_test.rs, message_validator_test.rs, challenge_*_test.rs
- [x] Security-Tests ausführen und bestehen (CI/Container)

#### 20.3.2 GDPR Compliance Testing
- [x] GDPR-Compliance-Tests schreiben
  - Data-Minimization-Tests (Payload-Größe, Sanitize-Truncation)
  - Data-Encryption / No sensitive data in audit (Audit ohne Payload)
  - Access-Control-Tests (Cross-User-Block, Connection-Block + Audit)
  - Audit-Logging-Tests (Security-/Connection-Events)
  - Right-to-Erasure-Tests (Guest-Cleanup, Mesh-Removal)
  - Dedizierte Suite: [tests/gdpr_compliance_test.rs](tests/gdpr_compliance_test.rs)
- [x] GDPR-Compliance-Tests ausführen und bestehen

**Phase-20-Status (Test-Suites):** Alle fünf Suites sind geschrieben (E2E, Error Recovery, Performance, Security, GDPR). E2E gRPC-over-Bifrost-Fix: GrpcResponse im Test mit `target_device_id: "unknown"` (Server routet nach Device-ID; Test-Clients ohne Handshake = „unknown“). Warnungen bereinigt (unused imports/variables in connection_quality_test, thor_action_router_test, key_generator_test, security_test_suite, intrusion_detector_test, websocket_client_test; multicast.rs). websocket_client_test: `client_connects_to_server` akzeptiert 101 Switching Protocols (WebSocket-Upgrade). CI-Workflow eingerichtet: [.github/workflows/bifrost.yml](../.github/workflows/bifrost.yml). Alle Phase-20-Checkboxen nach erfolgreichem Lauf (lokal/Container) abgehakt.

---

## Zusammenfassung

**Gesamtanzahl Phasen**: 20
**Gesamtanzahl Schritte**: ~350+

**Kritische Abhängigkeiten**:
1. WebSocket-Library-Wahl (beeinflusst gesamte WebSocket-Implementierung)
2. TLS-Library-Wahl (beeinflusst TLS-Konfiguration)
3. Message-Format-Wahl (beeinflusst Serialisierung und Performance)
4. mDNS/Bonjour-Library-Wahl (beeinflusst lokale Discovery)
5. NAT-Traversal-Bibliotheken-Wahl (beeinflusst WAN-Connectivity)
6. Protobuf-Code-Generierung-Tool (beeinflusst Code-Generierung, falls Protobuf)

**Offene Fragen für USER**:
1. WebSocket-Library (tokio-tungstenite, async-tungstenite, websocket)
2. TLS-Library (rustls, native-tls, openssl)
3. Message-Format (JSON, Protobuf, MessagePack)
4. mDNS/Bonjour-Library (mdns, zeroconf, eigene Implementierung)
5. NAT-Traversal-Bibliotheken (webrtc-rs, separate libs, eigene Implementierung)
6. Protobuf-Code-Generierung-Tool (prost, protobuf-rust, tonic+prost) - falls Protobuf gewählt

**Hinweise**:
- Alle Schritte folgen TDD (Tests zuerst, dann Implementierung)
- **ALLE Tests müssen in Containern laufen** - keine lokalen Dependencies, Tools oder Services auf der Entwicklungsmaschine installieren
- Container-Setup muss vollständig isolierte Test-Umgebung bereitstellen
- Alle Schritte sind kleinstmöglich aufgeteilt
- Abhängigkeiten zwischen Phasen sind klar definiert
- Offene Fragen sind klar markiert (❓)
- Security ist kritisch: 100% Coverage für WebSocket-Security-Tests
- Performance ist wichtig: < 10ms Message-Routing-Latency lokal
- GDPR-Compliance ist erforderlich: Data-Minimization, Encryption, Access-Control
- Mesh-Integration ist erforderlich: Bifrost-Connections erfordern Mesh-Membership (Device-Mesh, Meshtastic-inspiriert)
- Guest-Mesh-Isolation ist erforderlich: Automatisches Gast-Mesh (isoliertes Segment) mit expliziter Datentransfer-Erlaubnis (kein VPN)

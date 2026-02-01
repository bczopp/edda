# Rust Services - Übersicht

## Was wird in Rust implementiert?

### Core Services (alle in Rust)

1. **Odin** - Main Process Service & Vision-Model Orchestrator
   - Zentraler Orchestrator
   - Command Processing
   - Action Planning
   - Device State Management
   - Service-Koordination
   - Vision-Model-Interpretation (nutzt Geri für Bild/Video-Analyse)
   - Verhaltensmuster-Erkennung (mit User-Zustimmung)
   - Einherjar Protocol (Funktions-Entdeckung aller Götter)
   - Responsibility Service (Zuständigkeits-Management)
   - Dynamische Zuständigkeits-Weiterleitung

2. **Thor** - Action Executor & Tool-Calling-Agent
   - Action Execution
   - Tool-Calling (erkennt Actions aus Agent-Ergebnissen)
   - Resource Management
   - Task Scheduling
   - Conflict Resolution

3. **Freki** - RAG Service
   - Vector Database Management
   - Embedding-Generierung
   - Context-Enrichment
   - Semantic Search

4. **Geri** - LLM Service & Vision-Model Support
   - LLM-Integration (lokale und Cloud-Models)
   - Prompt-Processing
   - Response-Generierung
   - Model-Management
   - Vision-Model-Support (Bild/Video-Interpretation für Odin)
   - Video-Stream-Verarbeitung

5. **Huginn & Muninn** - STT/TTS Service & Data Forwarding
   - Speech-to-Text (STT) - bestehend
   - Text-to-Speech (TTS) - bestehend
   - Text-Input - bestehend
   - Bild/Video/Video-Stream-Empfang - neu
   - Daten-Weiterleitung an Odin (keine Interpretation)
   - Mythologie-Integration: "Odin sieht mit den Augen des Raben"

6. **Bifrost** - Communication Service
   - Device-to-Device Communication
   - WebSocket-basierte Verbindungen
   - Message Routing
   - Connection Management
   - NAT Traversal

7. **Heimdall** - Security Service
   - Authentication
   - Authorization
   - Token-Management
   - Permission-Checking
   - Security-Auditing

8. **Skuld** - LLM-Selection Service
   - **Muss auf allen Devices installiert werden**: Midgard, Alfheim, Asgard, Ragnarok
   - LLM-Auswahl basierend auf Netzwerkplan
   - Effektivster Weg + effektivstes Model + User-Vorgaben
   - Verfügbar lokal und bei Yggdrasil
   - Odin benötigt diesen Service

9. **Vedrfolnir** - Connection Builder Client
   - Client-Service auf User-Device-Seite
   - Baut Verbindungen zu Yggdrasil auf (über Ratatoskr-Protocol)
   - Odin nutzt Vedrfolnir für Yggdrasil-Kommunikation

### Plugins (auch in Rust)

10. **Brünnhilde (Valkyries)** - Coding Agent
   - Task Decomposition
   - Sub-Agent Orchestration
   - Quality Assurance
   - Workflow Management

11. **Frigg** - Healthcare Plugin
   - Healthcare-Task-Verarbeitung
   - Treatment-Planning
   - Health-Monitoring
   - Ausschließlich für persönliche und Gesundheitsfragen
   - Zuständigkeits-Rückgabe und Rückweisungs-Mechanismus
   - Einherjar Protocol-Implementierung
   - Persönlichkeiten

### Platformen (TypeScript für Frontend, Rust für Platform-Logik und Services)

**Hinweis**: Midgard, Alfheim, Asgard und Ragnarok sind Platformen, keine Services. Sie haben TypeScript-Frontends (GUI/TUI) und Platform-Logik in Rust, die mit Rust-Services via gRPC kommunizieren. Alle Backend-Logik ist in Rust implementiert.

12. **Ragnarok** - Terminal Platform
    - TUI-Frontend (statt GUI) - TypeScript
    - Platform-Logik (Rust)
    - Service-Integration via gRPC
    - Action-Handling
    - Device-Control

13. **Jotunheim** - IoT Platform
    - Platform für IoT-Devices (ESP32, ESP8266, Raspberry Pi Pico, etc.)
    - Platformspezifische Implementierung
    - Connections, Konvertierung zu Anfragen an Services
    - Ruft Services (Odin, Loki) via gRPC auf

14. **Loki** - Script Execution Service
    - Unabhängiger Service für Script-Execution
    - User-generierte Scripte per gRPC zugänglich machen
    - Jedes Script wird zu einer aufrufbaren gRPC-Funktion
    - Direkte Ausführung von Scripts auf Device
    - Koordination der 3 Kinder: Fenrir, Jörmungandr, Hel
    - **Fenrir**: Aggressive Tasks, Hardware-Control
    - **Jörmungandr**: Network/Communication
    - **Hel**: Data/Storage

### Yggdrasil Rust-Microservices

15. **Mimir (Mímisbrunnr)** - Privacy Database Service
    - **Programmiersprache**: Rust
    - **Aufgabe**: Privacy Database Service für personenbezogene Daten
    - **Der Brunnen**: Die Datenbank selbst
    - **Features**: Isolierte Datenbank, Verschlüsselung, Access Control, Audit-Logging, GDPR-Compliance

16. **Nornen (Urd, Verdandi)** - Decision Service
    - **Programmiersprache**: Rust
    - **Aufgabe**: Entscheidungen über Requests, Provider-Registrierung, User-Konfiguration, Admin API
    - **Urd**: Vergangenheit (Historie, Request-History, historische Statistiken)
    - **Verdandi**: Gegenwart (Aktuelle Statistiken, Real-time Analytics, Live-Metriken)
    - **Hinweis**: Skuld ist ein separater Service, der auf allen Devices installiert werden muss
    - **Features**: Request-Entscheidungen, Provider-Registrierung, User-Konfiguration, Admin API, Analytics

17. **Nidhöggr** - Connection Endpoint & Message Receiver
    - **Programmiersprache**: Rust
    - **Aufgabe**: Server-Side Connection Endpoint bei Yggdrasil
    - **Features**: Empfängt Verbindungen (von Vedrfolnir), empfängt Nachrichten über Ratatoskr-Protocol, leitet Nachrichten direkt weiter an Nornen

18. **Njörðr** - Marketplace Service
    - **Programmiersprache**: Rust
    - **Aufgabe**: Marketplace-Management, Provider-Management, Request-Routing, Transaction-Management
    - **Features**: Provider-Registration, Request-Routing, Fair-Distribution-Algorithm, Transaction-Management

19. **Heidrun** - Token & Pricing Service
    - **Programmiersprache**: Rust
    - **Aufgabe**: Token-Berechnungen, Pricing, Settlement, Pre-Authorization
    - **Features**: Token-Counting, Cost-Calculation, Pricing-Model, Pre-Authorization

20. **Eikthyrnir** - Quality Assessment Service
    - **Programmiersprache**: Rust
    - **Aufgabe**: Quality Assessment, Aggregation, Quality-Metriken
    - **Features**: Quality-Messung, Gewichteter Durchschnitt, Quality-Aggregation, Quality-Updates

21. **Læraðr** (Dáinn, Dvalinn, Duneyrr, Duraþrór) - Data Management Service
    - **Programmiersprache**: Rust
    - **Aufgabe**: Data Management (Indexing, Validation, Aggregation, Retention)
    - **Dáinn**: Data Indexing
    - **Dvalinn**: Data Validation
    - **Duneyrr**: Data Aggregation
    - **Duraþrór**: Data Retention
    - **Features**: Ordnung der Daten, Data Integrity, Data Lifecycle, Data Cleanup

### Server-Services (in Elixir)

22. **Yggdrasil** - Cloud-Server (Main Process)
    - **Programmiersprache**: Elixir (Erlang VM/BEAM)
    - **Warum Elixir**: Millionen von gleichzeitigen Bifrost-Verbindungen
    - **Eigenständiger Server**: Kein eigener Odin, User-Devices kommunizieren direkt mit Yggdrasil
    - **Ratatoskr-Protocol**: Business-Protocol für Yggdrasil-Kommunikation (zusätzlich zu Bifrost)
    - **Koordiniert**: Alle Rust-Microservices (Mimir, Nornen, Nidhöggr, Heidrun, Eikthyrnir, Læraðr)
    - Multi-Tenant-Architektur
    - Device Registry
    - User Management
    - Marketplace-Infrastruktur
    - Provider-Management
    - Connection Management (Millionen Verbindungen)

## Was wird NICHT in Rust implementiert?

### TypeScript (nur Frontends)
- **Midgard Frontend**: GUI-Frontend (React/TypeScript)
- **Alfheim Frontend**: Mobile GUI-Frontend (React Native/TypeScript)
- **Asgard Frontend**: Server GUI-Frontend (React/TypeScript)

### Elixir
- **Yggdrasil**: Cloud-Server (Elixir für Millionen Verbindungen und Bifrost-Relay)

### Andere Sprachen
- **Separate Projekte**: Wenn gemeinsame Komponenten benötigt werden, sollten separate Projekte erstellt werden (WICHTIG: Keine Edda Core Library)

## Rust-Projektstruktur

Jeder Service ist ein eigenes Rust-Projekt:

```
odin/
├── Cargo.toml
├── src/
│   ├── main.rs
│   ├── lib.rs
│   ├── orchestrator/
│   ├── state/
│   └── ...
└── tests/

thor/
├── Cargo.toml
├── src/
│   ├── main.rs
│   ├── lib.rs
│   ├── executor/
│   ├── toolcalling/
│   └── ...
└── tests/

freki/
├── Cargo.toml
├── src/
│   ├── main.rs
│   ├── lib.rs
│   ├── rag/
│   ├── vector/
│   └── ...
└── tests/

# ... etc. für alle Services
```

## Platform-Konzept

**Alle Orte (außer Yggdrasil) sind Platformen:**
- **Midgard**: Desktop-Platform
- **Alfheim**: Mobile-Platform
- **Asgard**: Homeserver-Platform
- **Ragnarok**: Terminal-Platform
- **Jotunheim**: IoT-Platform

**Platform-Rolle:**
- **Connections**: Platformen kümmern sich um Connections (Netzwerk, UI, etc.)
- **Konvertierung**: Platformen konvertieren Connections zu Anfragen an Services
- **Platformspezifisch**: Komplett platformspezifische Implementierung
- **Service-Aufrufe**: Rufen Services (Odin, Loki, etc.) via gRPC auf

**Service-Unabhängigkeit:**
- **Services in Rust**: Alle Services (Odin, Thor, Freki, Geri, Loki, etc.) sind unabhängig von Platformen
- **gRPC-Kommunikation**: Platformen kommunizieren mit Services via gRPC
- **Wiederverwendbar**: Services können von verschiedenen Platformen genutzt werden

## Kommunikation zwischen Services

- **gRPC**: Für Service-zu-Service-Kommunikation
  - **Loki Function Calls**: gRPC für IoT-Device Toolcalling (via Loki Service für Jotunheim-Devices)
  - **Loki Script Execution**: gRPC für Loki-Service (Script-Execution)
  - **Cross-Device Actions**: gRPC für ThorAction/ThorResult zwischen Devices
  - **On-Device Services**: gRPC für Odin ↔ Thor, Freki, Geri, Skuld
  - **Plugin Communication**: gRPC für Odin ↔ Plugins
  - **Yggdrasil Microservices**: gRPC für Yggdrasil ↔ Rust-Microservices
  - **Einherjar Protocol**: gRPC für Funktions-Entdeckung und Capability-Offenlegung (alle Götter)
  - **Responsibility Service**: gRPC für Zuständigkeits-Management (alle Götter)
  - **Vision Service**: gRPC für Bild/Video-Analyse (Odin ↔ Geri)
  - **Huginn Data Service**: gRPC für Daten-Weiterleitung (Text, Bilder, Videos, Video-Streams)
- **Protobuf**: Für Message-Definitionen (gRPC Services)
- **WebSocket**: Für Bifrost (Device-to-Device Messaging, Events, Connection Establishment)
- **Ratatoskr Protocol**: WebSocket für Yggdrasil Business-Kommunikation

## Vorteile von Rust für Services

- **Performance**: Maximale Performance, nahe an C/C++
- **Memory-Safety**: Memory-Safety ohne Garbage Collector
- **Concurrency**: Native Async/Await für parallele Verarbeitung
- **Cross-Platform**: Läuft auf allen Plattformen (inkl. ESP32, etc.)
- **Moderne Tooling**: Cargo, rustfmt, clippy
- **Gute Libraries**: Umfangreiches Ecosystem (tokio, axum, etc.)
- **Deployment**: Einfaches Deployment (single binary)
- **Zero-Cost Abstractions**: Abstraktionen ohne Runtime-Overhead

## Yggdrasil-Architektur

**Elixir (Yggdrasil Core):**
- Millionen Bifrost-Verbindungen
- Ratatoskr-Protocol-Verbindungen (via Nidhöggr)
- Message-Routing
- Connection Management
- Event-Notifications
- Koordiniert alle Rust-Microservices

**Rust-Microservices:**
- **Mimir**: Privacy Database Service
- **Nornen (Urd, Verdandi)**: Decision Service
- **Nidhöggr**: Connection Endpoint & Message Receiver
- **Heidrun**: Token & Pricing
- **Eikthyrnir**: Quality Assessment
- **Læraðr**: Data Management

**Kommunikations-Protokolle mit Yggdrasil:**
- **Ratatoskr-Protocol**: WebSocket-basiert für persistente Business-Verbindungen (Marketplace, Payments, Provider-Registrierung)
- **gRPC**: Für Request/Response-Patterns und effiziente API-Calls (Device-Registry, User-Management, etc.)
- **Bifrost**: WebSocket-basiert für Device-zu-Device-Relay und Event-Notifications

**Kommunikation:**
- User-Devices (Vedrfolnir) ↔ Yggdrasil (Nidhöggr): 
  - Ratatoskr-Protocol (WebSocket) für persistente Business-Verbindungen
  - gRPC für Request/Response-Patterns und einzelne API-Calls
- Yggdrasil (Elixir) ↔ Rust-Microservices: gRPC
- Nidhöggr → Nornen/andere Services: gRPC (direkte Weiterleitung)
- Asynchron: Yggdrasil sendet Requests, Microservices antworten asynchron
- Caching: Ergebnisse können gecacht werden (Redis)

## CI/CD (Rust-Services)

Services mit GitHub Actions CI (Test im Container, Lint bei Push/PR): **Bifrost** ([.github/workflows/bifrost.yml](../.github/workflows/bifrost.yml)), **Heimdall** ([.github/workflows/heimdall.yml](../.github/workflows/heimdall.yml)), **Thor** ([.github/workflows/thor.yml](../.github/workflows/thor.yml)), **Odin** ([.github/workflows/odin.yml](../.github/workflows/odin.yml)), **Loki** ([.github/workflows/loki.yml](../.github/workflows/loki.yml)), **Freki** ([.github/workflows/freki.yml](../.github/workflows/freki.yml)), **Geri** ([.github/workflows/geri.yml](../.github/workflows/geri.yml)), **Skuld** ([.github/workflows/skuld.yml](../.github/workflows/skuld.yml)), **Mimir** ([.github/workflows/mimir.yml](../.github/workflows/mimir.yml)), **Huginn-Muninn** ([.github/workflows/huginn-muninn.yml](../.github/workflows/huginn-muninn.yml)), **Nornen** ([.github/workflows/nornen.yml](../.github/workflows/nornen.yml)), **Vedrfolnir** ([.github/workflows/vedrfolnir.yml](../.github/workflows/vedrfolnir.yml)), **Valkyries** ([.github/workflows/valkyries.yml](../.github/workflows/valkyries.yml)), **Nidhöggr** ([.github/workflows/nidhoggr.yml](../.github/workflows/nidhoggr.yml)), **Frigg** ([.github/workflows/frigg.yml](../.github/workflows/frigg.yml)), **Ragnarok** ([.github/workflows/ragnarok.yml](../.github/workflows/ragnarok.yml)), **Njörðr** ([.github/workflows/njordr.yml](../.github/workflows/njordr.yml)), **Heidrun** ([.github/workflows/heidrun.yml](../.github/workflows/heidrun.yml)), **Ratatoskr** ([.github/workflows/ratatoskr.yml](../.github/workflows/ratatoskr.yml)), **Eikthyrnir** ([.github/workflows/eikthyrnir.yml](../.github/workflows/eikthyrnir.yml)), **Læraðr** ([.github/workflows/laeradr.yml](../.github/workflows/laeradr.yml)), **Jotunheim** ([.github/workflows/jotunheim.yml](../.github/workflows/jotunheim.yml)), **Asgard** ([.github/workflows/asgard.yml](../.github/workflows/asgard.yml)), **Forseti** ([.github/workflows/forseti.yml](../.github/workflows/forseti.yml)), **Midgard** ([.github/workflows/midgard.yml](../.github/workflows/midgard.yml)), **Hirtir** ([.github/workflows/hirtir.yml](../.github/workflows/hirtir.yml)), **Gladsheim** ([.github/workflows/gladsheim.yml](../.github/workflows/gladsheim.yml)). Zusätzlich CI nur Test im Container: **Alfheim** (Bun), **Yggdrasil** (Elixir). Siehe [README – Für Entwickler](../README.md#für-entwickler) und [Test-Infrastructure-Template](test-infrastructure-template.md#cicd).

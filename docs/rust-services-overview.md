# Rust Services - Übersicht

## Was wird in Rust implementiert?

### Core Services (alle in Rust)

1. **Odin** - Main Process Service
   - Zentraler Orchestrator
   - Command Processing
   - Action Planning
   - Device State Management
   - Service-Koordination

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

4. **Geri** - LLM Service
   - LLM-Integration (lokale und Cloud-Models)
   - Prompt-Processing
   - Response-Generierung
   - Model-Management

5. **Huginn & Muninn** - STT/TTS Service
   - Speech-to-Text (STT)
   - Text-to-Speech (TTS)
   - Voice-Input-Verarbeitung
   - Audio-Processing

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

### Backend-Services (in Rust)

12. **Midgard Backend** - Desktop/Laptop Backend
    - Service-Integration
    - Action-Handling
    - Device-Control

13. **Alfheim Backend** - Mobile Backend
    - Service-Integration
    - Mobile-optimierte Actions
    - Device-Control

14. **Asgard Backend** - Homeserver Backend
    - Service-Integration
    - Server-Funktionalität
    - Device-Control
    - Network-Management

15. **Ragnarok** - Terminal Agent
    - TUI-Frontend (statt GUI)
    - Service-Integration
    - Action-Handling
    - Device-Control

16. **Jötnar** - IoT-Devices Client
    - Lightweight Implementation
    - Bifrost-Verbindungen
    - Toolcalling Protocol
    - Remote Control

### Yggdrasil Rust-Microservices

17. **Mimir (Mímisbrunnr)** - Privacy Database Service
    - **Programmiersprache**: Rust
    - **Aufgabe**: Privacy Database Service für personenbezogene Daten
    - **Der Brunnen**: Die Datenbank selbst
    - **Features**: Isolierte Datenbank, Verschlüsselung, Access Control, Audit-Logging, GDPR-Compliance

18. **Nornen (Urd, Verdandi)** - Decision Service
    - **Programmiersprache**: Rust
    - **Aufgabe**: Entscheidungen über Requests, Provider-Registrierung, User-Konfiguration, Admin API
    - **Urd**: Vergangenheit (Historie, Request-History, historische Statistiken)
    - **Verdandi**: Gegenwart (Aktuelle Statistiken, Real-time Analytics, Live-Metriken)
    - **Hinweis**: Skuld ist ein separater Service, der auf allen Devices installiert werden muss
    - **Features**: Request-Entscheidungen, Provider-Registrierung, User-Konfiguration, Admin API, Analytics

19. **Nidhöggr** - Connection Endpoint & Message Receiver
    - **Programmiersprache**: Rust
    - **Aufgabe**: Server-Side Connection Endpoint bei Yggdrasil
    - **Features**: Empfängt Verbindungen (von Vedrfolnir), empfängt Nachrichten über Ratatoskr-Protocol, leitet Nachrichten direkt weiter an Nornen

20. **Heidrun** - Token & Pricing Service
    - **Programmiersprache**: Rust
    - **Aufgabe**: Token-Berechnungen, Pricing, Settlement, Pre-Authorization
    - **Features**: Token-Counting, Cost-Calculation, Pricing-Model, Pre-Authorization

21. **Eikthyrnir** - Quality Assessment Service
    - **Programmiersprache**: Rust
    - **Aufgabe**: Quality Assessment, Aggregation, Quality-Metriken
    - **Features**: Quality-Messung, Gewichteter Durchschnitt, Quality-Aggregation, Quality-Updates

22. **Die vier Hirsche** (Dáinn, Dvalinn, Duneyrr, Duraþrór) - Data Management Service
    - **Programmiersprache**: Rust
    - **Aufgabe**: Data Management (Indexing, Validation, Aggregation, Retention)
    - **Dáinn**: Data Indexing
    - **Dvalinn**: Data Validation
    - **Duneyrr**: Data Aggregation
    - **Duraþrór**: Data Retention
    - **Features**: Ordnung der Daten, Data Integrity, Data Lifecycle, Data Cleanup

### Server-Services (in Elixir)

23. **Yggdrasil** - Cloud-Server (Main Process)
    - **Programmiersprache**: Elixir (Erlang VM/BEAM)
    - **Warum Elixir**: Millionen von gleichzeitigen Bifrost-Verbindungen
    - **Eigenständiger Server**: Kein eigener Odin, User-Devices kommunizieren direkt mit Yggdrasil
    - **Ratatoskr-Protocol**: Business-Protocol für Yggdrasil-Kommunikation (zusätzlich zu Bifrost)
    - **Koordiniert**: Alle Rust-Microservices (Mimir, Nornen, Nidhöggr, Heidrun, Eikthyrnir, Hirsche)
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
- **Edda Core Library**: Multi-Sprache (Rust, Elixir, TypeScript)

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

## Kommunikation zwischen Services

- **gRPC**: Für Service-zu-Service-Kommunikation
- **Protobuf**: Für Message-Definitionen
- **WebSocket**: Für Bifrost (Device-to-Device)
- **MessagePack**: Für Jötnar-Devices (token-effizient)

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
- **Die vier Hirsche**: Data Management

**Ratatoskr-Protocol:**
- Business-Protocol für Yggdrasil-Kommunikation
- WebSocket-basiert mit extra Security-Features
- Für Marketplace, Payments, Provider-Registrierung

**Kommunikation:**
- User-Devices (Vedrfolnir) ↔ Yggdrasil (Nidhöggr): Ratatoskr-Protocol (WebSocket)
- Yggdrasil (Elixir) ↔ Rust-Microservices: gRPC
- Nidhöggr → Nornen/andere Services: gRPC (direkte Weiterleitung)
- Asynchron: Yggdrasil sendet Requests, Microservices antworten asynchron
- Caching: Ergebnisse können gecacht werden (Redis)


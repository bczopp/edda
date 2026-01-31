# Edda

**Edda** ist ein dezentrales, privacy-fokussiertes AI-Assistant-System mit Microservices-Architektur. Das System ermöglicht es Nutzern, ihre Geräte per Voice oder Text zu steuern, diese untereinander zu vernetzen und mit KI-Assistenten zu interagieren - alles mit Fokus auf Datenschutz, lokale Verarbeitung und dezentrale Architektur.

## Was ist Edda?

Edda ist ein verteiltes AI-System, das nach dem Prinzip der nordischen Mythologie strukturiert ist. Jede Komponente trägt den Namen einer mythologischen Figur und erfüllt eine spezifische Rolle im Gesamtsystem.

### Kernkonzept

- **Dezentral**: Keine Vendor Lock-in, Nutzer behalten die volle Kontrolle über ihre Daten und Geräte
- **Privacy-First**: Lokale Verarbeitung wo möglich, keine unnötige Datenweitergabe
- **Local-First**: Funktioniert auch ohne Internetverbindung, lokale LLMs als Standard
- **Multi-Device**: Nahtlose Kommunikation zwischen Desktop, Mobile, Server, Terminal und IoT-Geräten
- **Erweiterbar**: Plugin-System für spezialisierte Funktionen (Coding Agent, Healthcare, etc.)

### Architektur

Edda folgt einer Microservices-Architektur, bei der jeder Service unabhängig und wiederverwendbar ist:

- **Platformen**: Midgard (Desktop), Alfheim (Mobile), Asgard (Homeserver), Ragnarok (Terminal), Jotunheim (IoT)
- **Core Services**: Odin (Orchestrator), Thor (Actions), Freki (RAG), Geri (LLM), Huginn/Muninn (STT/TTS), Bifrost (Communication), Heimdall (Security), Skuld (LLM Selection), Loki (Script Execution)
- **Plugins**: Valkyries (Coding Agent), Frigg (Healthcare)
- **Infrastructure**: Yggdrasil (Cloud Server), Mimir (Privacy Database), Nornen (Decision Service), und weitere unterstützende Services

## Was kann Edda?

### Voice-Assistant
- **Speech-to-Text (STT)**: Natürliche Sprachinteraktion über Huginn
- **Text-to-Speech (TTS)**: Sprachausgabe über Muninn
- **Multi-Language Support**: Unterstützung für mehrere Sprachen
- **Wake Word Detection**: Optionale Wake-Word-Erkennung

### Multi-Device-Steuerung
- **Geräteübergreifende Kommunikation**: Nahtlose Kommunikation zwischen allen Geräten über Bifrost
- **Device-Control**: Steuerung von Geräten und Aktionen über Thor
- **Cross-Device Actions**: Aktionen können auf verschiedenen Geräten ausgeführt werden
- **State-Synchronisation**: Automatische Synchronisation des Gerätestatus zwischen allen Geräten

### AI-Funktionalitäten
- **RAG (Retrieval-Augmented Generation)**: Kontextbewusste Antworten durch Freki
- **LLM-Integration**: Lokale und Cloud-LLMs über Geri
- **Vision-Model Support**: Bild- und Video-Analyse über Geri
- **Intelligente LLM-Auswahl**: Automatische Auswahl des besten verfügbaren LLMs über Skuld

### Erweiterte Features
- **Plugin-System**: Erweiterbar durch Plugins (Coding Agent, Healthcare, etc.)
- **Coding Agent (Valkyries)**: Vollständiger Coding-Agent mit 13 spezialisierten Sub-Agents
- **Healthcare Plugin (Frigg)**: Spezialisiertes Plugin für Gesundheitsfragen
- **Script Execution (Loki)**: Ausführung von benutzerdefinierten Scripts
- **Marketplace (optional)**: Token-basierter Marktplatz für Compute-Ressourcen

## Wofür ist Edda gedacht?

### Use Cases

**Voice-gesteuerte Gerätesteuerung**
- Steuerung von Geräten per Voice-Commands
- Automatisierung von wiederkehrenden Aufgaben
- Smart Home-Integration

**Multi-Device-AI-Assistant**
- Konsistente Erfahrung über alle Geräte hinweg
- Nahtlose Übergabe zwischen Desktop, Mobile und Server
- Zentrale Verwaltung aller Geräte

**Privacy-fokussierte AI-Interaktion**
- Lokale Verarbeitung von sensiblen Daten
- Keine Datenweitergabe an Dritte ohne Zustimmung
- Vollständige Kontrolle über eigene Daten

**Entwickler-Tools**
- Coding-Agent für Software-Entwicklung
- Automatisierte Code-Generierung und -Optimierung
- Test-Generierung und Code-Review

**Healthcare-Anwendungen**
- Zertifizierte Gesundheitskurse
- Integration mit Krankenkassen
- Persönliche Gesundheitsbetreuung

**Dezentrale Compute-Ressourcen-Nutzung**
- Optional: Nutzung von Compute-Ressourcen anderer Nutzer
- Optional: Monetarisierung eigener Hardware-Ressourcen
- Fairer, transparenter Marktplatz

### Zielgruppen

**End-Nutzer**
- Privacy-bewusste Nutzer, die Kontrolle über ihre Daten behalten möchten
- Nutzer, die lokale AI-Verarbeitung bevorzugen
- Nutzer mit mehreren Geräten, die nahtlos zusammenarbeiten sollen

**Entwickler**
- Entwickler, die Plugins erstellen möchten
- Entwickler, die Edda in eigene Projekte integrieren möchten
- Entwickler, die den Coding-Agent nutzen möchten

**Provider (optional)**
- Nutzer, die ihre Hardware-Ressourcen monetarisieren möchten
- Unternehmen, die Compute-Ressourcen anbieten möchten

## Architektur-Übersicht

### Platformen

Edda unterstützt verschiedene Gerätetypen, die als "Platformen" bezeichnet werden:

- **Midgard**: Desktop/Laptop-Platform (Windows, macOS, Linux)
- **Alfheim**: Mobile-Platform (iOS, Android)
- **Asgard**: Homeserver-Platform (Linux, Docker, Cloud)
- **Ragnarok**: Terminal-Platform (TUI statt GUI)
- **Jotunheim**: IoT-Platform (ESP32, ESP8266, Raspberry Pi Pico)

### Core Services

**Odin** - Main Orchestrator
- Zentraler Koordinator auf jedem Gerät
- Verarbeitet User-Commands (Text, Voice, Bild, Video)
- Orchestriert alle anderen Services
- Plugin-Management und Zuständigkeits-Verwaltung

**Thor** - Action Executor
- Führt Aktionen aus (Datei-Operationen, System-Commands, etc.)
- Tool-Calling-Agent
- Resource Management und Task Scheduling

**Freki** - RAG Service
- Retrieval-Augmented Generation
- Vector Database Management
- Context Enrichment für LLM-Prompts

**Geri** - LLM Service
- LLM-Integration (lokal und Cloud)
- Vision-Model Support
- Model Management und Load Balancing

**Huginn & Muninn** - STT/TTS Service
- Speech-to-Text (Huginn)
- Text-to-Speech (Muninn)
- Daten-Weiterleitung (Bild, Video, Video-Streams)

**Bifrost** - Communication Service
- Secure WebSocket-basierte Device-to-Device-Kommunikation
- Message Routing
- Connection Management

**Heimdall** - Security Service
- Authentication und Authorization
- Token Management
- Connection Validation

**Skuld** - LLM Selection Service
- Intelligente LLM-Auswahl basierend auf Netzwerkplan
- Effizienz-basierte Provider-Auswahl

**Loki** - Script Execution Service
- Ausführung von benutzerdefinierten Scripts
- Tool-Calling für IoT-Devices

### Plugins

**Valkyries** - Coding Agent
- 13 spezialisierte Sub-Agents für verschiedene Coding-Aufgaben
- Task Decomposition und Quality Assurance
- Vollständige Software-Entwicklung-Unterstützung

**Frigg** - Healthcare Plugin
- Spezialisiert für Gesundheitsfragen
- Zertifizierte Kurse
- Integration mit Krankenkassen

### Infrastructure

**Yggdrasil** - Cloud Server
- Globale Device-Registry
- User Management und Subscriptions
- Marketplace-Infrastruktur
- Bifrost-Relay für globale Kommunikation

**Mimir** - Privacy Database Service
- Verwaltung personenbezogener Daten
- GDPR-Compliance
- Verschlüsselte Datenspeicherung

**Nornen** - Decision Service
- Entscheidungen über Requests
- Provider-Registrierung
- Analytics und Monitoring

### Kommunikation

- **gRPC**: Service-to-Service-Kommunikation (on-device und cross-device)
- **Bifrost Protocol**: WebSocket-basierte Device-to-Device-Kommunikation
- **Ratatoskr Protocol**: WebSocket-basierte Business-Logic-Kommunikation mit Yggdrasil

## Technologie-Stack

- **Rust**: Alle Services und Backend-Logik (Performance, Memory-Safety, Cross-Platform)
- **TypeScript**: Frontend-Anwendungen (nur UI, keine Backend-Logik)
- **Elixir**: Yggdrasil Cloud-Server (massive Concurrency, Fault Tolerance)
- **gRPC**: Service-Kommunikation (Protobuf)
- **WebSocket**: Device-Kommunikation (Bifrost, Ratatoskr)

## Projektstruktur

Edda besteht aus mehreren eigenständigen Projekten, die selektiv von Platformen eingebunden werden können:

### Platformen
- [Midgard](midgard/README.md) - Desktop Platform
- [Alfheim](alfheim/README.md) - Mobile Platform
- [Asgard](asgard/README.md) - Homeserver Platform
- [Ragnarok](ragnarok/README.md) - Terminal Platform
- [Jotunheim](jotunheim/README.md) - IoT Platform

### Core Services
- [Odin](odin/README.md) - Main Orchestrator
- [Thor](thor/README.md) - Action Executor
- [Freki](freki/README.md) - RAG Service
- [Geri](geri/README.md) - LLM Service
- [Huginn & Muninn](huginn-muninn/README.md) - STT/TTS Service
- [Bifrost](bifrost/README.md) - Communication Service
- [Heimdall](heimdall/README.md) - Security Service
- [Skuld](skuld/README.md) - LLM Selection Service
- [Loki](loki/README.md) - Script Execution Service

### Plugins
- [Valkyries](valkyries/README.md) - Coding Agent
- [Frigg](frigg/README.md) - Healthcare Plugin

### Infrastructure
- [Yggdrasil](yggdrasil/README.md) - Cloud Server
- [Mimir](mimir/README.md) - Privacy Database Service
- [Nornen](nornen/README.md) - Decision Service
- [Nidhöggr](nidhoggr/README.md) - Connection Endpoint
- [Njörðr](njordr/README.md) - Marketplace Service
- [Heidrun](heidrun/README.md) - Token & Pricing Service
- [Eikthyrnir](eikthyrnir/README.md) - Quality Assessment Service
- [Læraðr](laeradr/README.md) - Data Management Service
- [Vedrfolnir](vedrfolnir/README.md) - Connection Builder Client

### Weitere Services
- [Ratatoskr](ratatoskr/README.md) - Business Protocol
- Valhalla (VPN) **entfallen** – ersetzt durch [Bifrost Device-Mesh](bifrost/README.md) (Meshtastic-inspiriert, IP + optional LoRa)
- [Forseti](forseti/README.md) - ML/DL/RL Service

**Hinweis**: Jedes Projekt hat eine eigene README mit detaillierten Informationen. Das `edda` Verzeichnis dient nur als Metadaten-Sammlung für die Grundstruktur und Zusammenhänge der Projekte.

## Getting Started

### Für End-Nutzer

**Desktop-Nutzung**: Siehe [Midgard/README.md](midgard/README.md)
**Mobile-Nutzung**: Siehe [Alfheim/README.md](alfheim/README.md)
**Server-Nutzung**: Siehe [Asgard/README.md](asgard/README.md)
**Terminal-Nutzung**: Siehe [Ragnarok/README.md](ragnarok/README.md)
**IoT-Devices**: Siehe [Jotunheim/README.md](jotunheim/README.md)

### Für Entwickler

**Development Guidelines**: Siehe [AGENTS.md](AGENTS.md)
**Technology Decisions**: Siehe [docs/TECHNOLOGY_DECISIONS.md](docs/TECHNOLOGY_DECISIONS.md)
**Rust Services Overview**: Siehe [docs/rust-services-overview.md](docs/rust-services-overview.md)

## Dokumentation

- [AGENTS.md](AGENTS.md) - Development Guidelines für alle Projekte
- [docs/rust-services-overview.md](docs/rust-services-overview.md) - Übersicht über alle Rust Services
- [docs/TECHNOLOGY_DECISIONS.md](docs/TECHNOLOGY_DECISIONS.md) - Technologie-Entscheidungen
- [docs/business-plan.md](docs/business-plan.md) - Business Plan (intern)

## Lizenz & Status

**Status**: In Entwicklung

**Lizenz**: Proprietär (Freeware - kostenloser Download, Installation und Nutzung, aber kein Open Source)

## Weitere Informationen

Edda ist ein komplexes, verteiltes System mit vielen Komponenten. Für detaillierte Informationen zu einzelnen Komponenten siehe die jeweiligen Projekt-READMEs.

**Wichtig**: Das `edda` Verzeichnis ist **KEIN PROJEKT**. Es dient nur als Metadaten-Sammlung für die Grundstruktur und Zusammenhänge der anderen Projekte. Alle Services sind eigenständige Projekte und können selektiv von Platformen eingebunden werden.

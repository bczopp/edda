# Edda - Core Project

## Übersicht

Das Edda-Projekt enthält die Kern-Bibliothek mit allen gemeinsamen DTOs, Protocols und Utilities, die von allen anderen Projekten verwendet werden.

## Projektstruktur

```
edda/
├── dtos/              # Data Transfer Objects
│   ├── device_identity.ts
│   ├── raven_message.ts
│   ├── wolf_request.ts
│   ├── thor_action.ts
│   ├── heimdall_token.ts
│   ├── marketplace.ts
│   ├── valkyrie_task.ts
│   ├── healthcare.ts
│   ├── bifrost_connection.ts
│   └── index.ts
├── protocols/         # Communication Protocols
│   ├── bifrost/
│   │   ├── protocol.ts
│   │   ├── message.ts
│   │   └── connection.ts
│   ├── jotnar/
│   │   ├── toolcalling.ts
│   │   ├── messagepack.ts
│   │   └── streaming.ts
│   └── index.ts
├── utils/             # Shared Utilities
│   ├── logger.ts
│   ├── crypto.ts
│   ├── validation.ts
│   ├── serialization.ts
│   └── index.ts
└── services/           # Service Interfaces (optional)
    ├── odin.ts
    ├── thor.ts
    └── index.ts
```

## Komponenten

### DTOs (Data Transfer Objects)

Alle DTOs sind bereits im Plan definiert:
- **DeviceIdentity & DeviceCapabilities**: Device-Identifikation und Capabilities
- **RavenMessage**: Messages zwischen Huginn/Muninn und Odin
- **WolfRequest & WolfResponse**: Requests/Responses für Freki (RAG) und Geri (LLM)
- **ThorAction & ThorResult**: Actions und Results für Thor
- **HeimdallToken & HeimdallPermission**: Security Tokens und Permissions
- **ComputeRequest, ProviderOffer, ComputeTransaction**: Marketplace DTOs
- **ValkyrieTask & ValkyrieResult**: Coding Agent Tasks und Results
- **HealthcareCourse & CourseProgress**: Healthcare Plugin DTOs
- **BifrostConnection**: Connection-Informationen für Bifrost

### Protocols

#### Bifrost Protocol
- Secure WebSocket-basiert
- TLS Encryption
- Message Routing
- Connection Management
- Device Discovery

#### Jötnar Toolcalling Protocol
- MessagePack-basiert (Binary)
- Token-effizienter als MCP
- Streaming Support
- Capability Negotiation

### Utilities

#### Logger
- Structured Logging
- Log Levels
- Context Tracking
- Log Rotation

#### Crypto
- Encryption/Decryption
- Key Management
- Digital Signatures
- Hash Functions

#### Validation
- DTO Validation
- Input Sanitization
- Type Checking
- Schema Validation

#### Serialization
- JSON Serialization
- MessagePack Serialization
- Binary Format Support
- Version Compatibility

## Technologie-Stack

### Programmiersprache
- **TypeScript/Node.js**: Alle Projekte werden in TypeScript/Node.js geschrieben
- **Gemeinsame Basis**: Alle Projekte nutzen die gleiche Programmiersprache für Konsistenz und Wiederverwendbarkeit
- **Native Bindings**: Für Performance-kritische Teile (z.B. llama.cpp) werden native Bindings verwendet

### Gemeinsame Pakete
- **Edda Core Library**: Alle Projekte nutzen die Edda Core Library (DTOs, Protocols, Utils)
- **Gemeinsame Dependencies**: Projekte sollten gemeinsame Pakete nutzen, wo möglich
- **Konsistenz**: Gemeinsame Pakete sorgen für Konsistenz und einfachere Wartung
- **Ragnarok**: Nutzt ebenfalls die gleichen Pakete wie andere Projekte

## Abhängigkeiten

- Keine externen Abhängigkeiten (außer Standard-Libraries)
- Sollte möglichst lightweight sein
- Muss von allen anderen Projekten verwendet werden können
- **TypeScript/Node.js**: Basis für alle Projekte

## Integration

Edda wird von allen anderen Projekten als Core Library verwendet:
- **Midgard, Alfheim, Asgard**: Verwenden DTOs und Protocols
- **Jötnar**: Verwendet Jötnar Protocol
- **Alle Services**: Verwenden DTOs für Kommunikation
- **Yggdrasil**: Verwendet DTOs für API-Kommunikation

## Performance

### Optimierungen
- **Lightweight Design**: Minimale Abhängigkeiten für schnelle Ladezeiten
- **Effiziente Serialisierung**: Optimierte JSON/MessagePack Serialization
- **Caching**: Intelligentes Caching für häufig verwendete DTOs und Validierungen
- **Lazy Loading**: Lazy Loading für große Datenstrukturen
- **Memory Management**: Effizientes Memory-Management für minimale Footprint

### Performance-Metriken
- Schnelle DTO-Validierung (< 1ms für Standard-DTOs)
- Effiziente Crypto-Operationen (asynchron, non-blocking)
- Optimierte Protocol-Implementierungen (minimaler Overhead)

## Datenschutz

### Datenschutz-Features
- **Minimale Datensammlung**: Nur notwendige Daten werden verarbeitet
- **Lokale Verarbeitung**: Daten werden lokal verarbeitet, wo möglich
- **Keine Tracking-Daten**: Keine User-Tracking oder Analytics-Daten
- **Privacy-by-Design**: Datenschutz ist von Anfang an integriert

### Compliance
- **GDPR-konform**: Unterstützung für GDPR-Anforderungen
- **Data Minimization**: Nur notwendige Daten werden gespeichert/übertragen
- **User Control**: User hat Kontrolle über seine Daten

## Sicherheit

### Security-Features
- **Crypto-Utilities**: Sichere Verschlüsselung und Hash-Funktionen
- **Key Management**: Sicheres Key-Management für Cryptographic Keys
- **Digital Signatures**: Unterstützung für digitale Signaturen
- **Input Validation**: Umfassende Input-Validierung und Sanitization
- **Secure Protocols**: Sichere Protocol-Implementierungen (TLS, etc.)

### Security-Best-Practices
- **Keine Hardcoded Secrets**: Keine Hardcoded Passwords oder Keys
- **Secure Defaults**: Sichere Standard-Konfigurationen
- **Regular Updates**: Regelmäßige Security-Updates für Dependencies
- **Vulnerability Scanning**: Automatisches Scanning für bekannte Vulnerabilities

## Implementierungs-Notizen

- Sollte als Library/Package exportiert werden
- Muss TypeScript-Types haben
- Sollte Versionierung unterstützen
- Muss Backward-Compatibility beachten
- Sollte gut dokumentiert sein
- Muss Tests haben
- **Performance**: Muss optimiert sein für minimale Latenz und Footprint
- **Datenschutz**: Muss Privacy-by-Design implementieren
- **Sicherheit**: Muss Security-Best-Practices folgen


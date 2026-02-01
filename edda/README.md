# Edda - Metadaten-Sammlung

## ⚠️ WICHTIG: KEIN PROJEKT!

**Das `edda` Verzeichnis ist KEIN Projekt!** Es wird niemals ein eigenständiges Projekt sein und ist auch nicht geplant.

Dieser Ordner dient **ausschließlich** als **Metadaten-Sammlung**:
- Grundstruktur der anderen Projekte
- Zusammenhänge zwischen Projekten
- Architektur-Übersicht
- Protokoll-Dokumentation

**Alle Services sind eigenständige Projekte** und können selektiv von Platformen eingebunden werden. Wenn gemeinsame Komponenten (DTOs, Protocols, Utils) benötigt werden, sollten separate Projekte erstellt werden.

## Zweck dieses Verzeichnisses

**Nur Metadaten-Sammlung**: Dieser Ordner ist KEIN Projekt. Er dient nur zur Dokumentation der Grundstruktur und Zusammenhänge. Wenn gemeinsame Komponenten benötigt werden, sollten separate Projekte erstellt werden:

- **Separate Projekte für DTOs**: Wenn mehrere Projekte die gleichen DTOs benötigen, sollte ein separates Projekt erstellt werden (z.B. `edda-dtos-bifrost`, `edda-dtos-thor`)
- **Separate Projekte für Protocols**: Jedes Protocol sollte ein eigenes Projekt sein (z.B. `bifrost-protocol`, `ratatoskr-protocol`)
- **Separate Projekte für Utils**: Wenn Utils von mehreren Projekten benötigt werden, sollte ein separates Projekt erstellt werden

**Vorteil**: Platformen können selektiv Services und deren Dependencies einbinden. Beispiel: Alfheim (Mobile) kann ohne Valkyries laufen, wenn das Smartphone nicht stark genug ist oder keine Verbindung zu Yggdrasil besteht. Midgard (Desktop) kann das volle Paket erhalten.

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

**Neue DTOs (Einherjar Protocol, Vision, Responsibility):**
- **CapabilityRequest & CapabilityResponse**: Einherjar Protocol für Funktions-Offenlegung
- **FunctionDefinition & ParameterDefinition**: Funktionen-Definitionen für Einherjar Protocol
- **ImageData, VideoData, VideoStreamChunk**: Bild/Video-Daten von Huginn an Odin
- **ImageAnalysisRequest & ImageAnalysisResponse**: Bild-Analyse via Vision-Model
- **VideoAnalysisRequest & VideoAnalysisResponse**: Video-Analyse via Vision-Model
- **BehaviorPattern, BehaviorPatternRequest & BehaviorPatternResponse**: Verhaltensmuster-Erkennung
- **ResponsibilityRequest & ResponsibilityResponse**: Zuständigkeits-Übernahme
- **ResponsibilityReturn & ResponsibilityAcknowledgment**: Zuständigkeits-Rückgabe
- **ResponsibilityRejection**: Zuständigkeits-Rückweisung

### Protocols

#### Bifrost Protocol
- Secure WebSocket-basiert
- TLS Encryption
- Message Routing
- Connection Management
- Device Discovery
- Für Device-zu-Device-Kommunikation (lokal und global)

#### Ratatoskr Protocol
- Secure WebSocket-basiert (zusätzlich zu Bifrost)
- TLS 1.3 Encryption
- Message-Signierung
- Audit-Logging
- Rate-Limiting
- Request-Validation
- Für Yggdrasil Business-Logik (Marketplace, Payments, Provider-Registrierung)
- Persistente Verbindungen für kontinuierliche Business-Kommunikation
- Nicht direkt nach außen (sicherer als Bifrost für lokale Nutzung)

#### Yggdrasil gRPC Protocol
- **gRPC-basiert**: Für Request/Response-Patterns mit Yggdrasil
- **Type-safe**: Protobuf für alle Service-Interfaces
- **Effizient**: HTTP/2, Binary-Format, weniger Overhead als WebSocket für einzelne Requests
- **Bessere Performance**: Schnellere Serialisierung, HTTP/2 Multiplexing
- **Streaming**: Built-in Streaming für große Responses
- **Error-Handling**: Besseres Error-Handling mit Status-Codes
- **Wann verwenden**: Für einzelne API-Calls (Device-Registry, User-Management, etc.)
- **Alternative zu Ratatoskr**: Wenn keine persistente Verbindung nötig ist

#### Huginn Media Protocol
- **gRPC-basiert**: Unified media/media stream transport protocol
- **Kombiniert Huginn Data Service und Vision Service**: Ein einheitliches Protokoll für Media-Transport und Vision-Analyse
- **Media-Transport**: Text, Bilder, Videos, Video-Streams von Huginn/Muninn an Odin
- **Vision-Analyse**: Bild/Video-Analyse via Vision-Model (Odin → Geri)
- **Streams über Huginn/Muninn**: Da alle Media-Streams von Huginn/Muninn kommen, macht es Sinn, alles in einem Protocol zu kombinieren
- **DTOs**: ImageData, VideoData, VideoStreamChunk, ImageAnalysisRequest/Response, VideoAnalysisRequest/Response

#### Platform Capability Protocol (für alle Platformen)
- **Einheitliches Protocol**: Alle Platformen (Midgard, Alfheim, Asgard, Ragnarok, Jotunheim) nutzen das gleiche Protocol
- **Einherjar Protocol**: Platformen rufen `EinherjarProtocol.GetCapabilities()` für alle Services auf der Platform auf
- **Service-Discovery**: Platform propagiert alle Methoden, die Odin als public ermittelt von allen Göttern, die auf der Platform vorhanden sind
- **Jotunheim**: Nutzt das gleiche Protocol wie andere Platformen für Capability-Exposure
- **HINWEIS**: "Loki-Toolcalling-Protocol" existiert nicht als separates Protocol - Jotunheim nutzt Loki Service Protocol via gRPC für Toolcalling

#### Loki Service Protocol
- **gRPC-basiert**: Type-safe, effizient, HTTP/2
- **Protobuf**: Binary, kompakt, automatische Code-Generierung
- **Dynamische Script-Funktionen**: Jedes User-Script wird zu einer aufrufbaren gRPC-Funktion
- **Script-Execution**: Direkte Ausführung von Scripts auf Device
- **Verschlüsselte Streams**: TLS-Verschlüsselung für gRPC-Streams
- **Bifrost optional**: Nur bedingt nötig, wenn gRPC-Streams verschlüsselt sind

#### Einherjar Protocol (Service-Discovery & Responsibility)
- **gRPC-basiert**: Für Funktions-Offenlegung und Zuständigkeits-Definition
- **Kombiniert Service-Discovery und Responsibility**: Einherjar Protocol enthält `responsibility_domains` und `responsibility_keywords` in `CapabilityResponse`
- **Einherjar = Krieger Odins**: Die Services (Krieger) stehen Odin zur Verfügung und definieren ihre Aufgaben im Protocol
- **Zusammen mit Responsibility Service**: Einherjar definiert WAS ein Service kann, Responsibility Service verwaltet WER zuständig ist
- **Für alle Götter**: Alle Services und Plugins müssen Einherjar Protocol implementieren
- **gRPC-basiert**: Type-safe, effizient, HTTP/2
- **Protobuf**: Binary, kompakt, automatische Code-Generierung
- **Standard für alle Götter**: Jeder Gott (Service/Plugin) muss dieses Protocol implementieren
- **Funktions-Offenlegung**: Alle gRPC-Funktionen inkl. Parameter werden offengelegt
- **Zweck-Erkennung**: Odin erkennt anhand der Funktionen, welcher Gott für welche Art von Anfragen zuständig ist
- **Zuständigkeits-Domains**: Jeder Gott definiert seine Zuständigkeits-Domains
- **Responsibility-Keywords**: Keywords, die auf Zuständigkeit hinweisen
- **Automatische Erkennung**: Odin kann automatisch alle verfügbaren Funktionen entdecken

**Einherjar Protocol gRPC Definition:**
```protobuf
service EinherjarProtocol {
  rpc GetCapabilities(CapabilityRequest) returns (CapabilityResponse);
}

message CapabilityRequest {
  // Leer, oder optional Filter-Parameter
}

message CapabilityResponse {
  string god_name = 1;  // z.B. "Thor", "Frigg", "Valkyries"
  string purpose = 2;   // Zweck des Gottes (z.B. "Action Execution", "Healthcare", "Coding")
  repeated FunctionDefinition functions = 3;
  repeated string responsibility_domains = 4;  // Für welche Art von Anfragen ist dieser Gott zuständig
}

message FunctionDefinition {
  string name = 1;
  string description = 2;
  repeated ParameterDefinition parameters = 3;
  string return_type = 4;
  repeated string capabilities = 5;
  repeated string responsibility_keywords = 6;  // Keywords, die auf Zuständigkeit hinweisen
}

message ParameterDefinition {
  string name = 1;
  string type = 2;
  string description = 3;
  bool required = 4;
  string default_value = 5;
}
```

**Responsibility Service gRPC Definition:**
```protobuf
service ResponsibilityService {
  rpc TakeResponsibility(ResponsibilityRequest) returns (ResponsibilityResponse);
  rpc ReturnResponsibility(ResponsibilityReturn) returns (ResponsibilityAcknowledgment);
  rpc RejectResponsibility(ResponsibilityRejection) returns (ResponsibilityAcknowledgment);
}

message ResponsibilityRequest {
  string request_id = 1;
  string request_type = 2;
  string request_content = 3;
  // Weitere Request-Details
}

message ResponsibilityResponse {
  bool accepted = 1;
  string reason = 2;
}

message ResponsibilityReturn {
  string request_id = 1;
  string reason = 2;  // "Unterhaltung ist nicht mehr in meinem Bereich"
}

message ResponsibilityRejection {
  string request_id = 1;
  string reason = 2;  // "Das ist nicht mein Bereich"
  string suggested_god = 3;  // Hinweis auf besseren Gott
}

message ResponsibilityAcknowledgment {
  bool acknowledged = 1;
}
```

**LokiService gRPC Definition:**
```protobuf
service LokiService {
  // Statische Methoden
  rpc GetCapabilities(CapabilityRequest) returns (CapabilityResponse);
  rpc GetChildrenStatus(StatusRequest) returns (StatusResponse);
  rpc ListScripts(ListScriptsRequest) returns (ListScriptsResponse);
  rpc RegisterScript(RegisterScriptRequest) returns (RegisterScriptResponse);
  
  // Dynamische Script-Funktionen (zur Laufzeit generiert)
  // Jedes User-Script wird zu einer gRPC-Funktion:
  // rpc Script_<script_name>(ScriptInput) returns (ScriptOutput);
  // rpc StreamScript_<script_name>(stream ScriptChunk) returns (stream ScriptResult);
}
```

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
- Protobuf Serialization (für gRPC)
- Protobuf-Lite (für ESP32, weniger Memory)
- Binary Format Support
- Version Compatibility

## Technologie-Stack

### Programmiersprache

**Sprachverteilung:**
- **TypeScript**: Nur in GUI-Frontends (Midgard/Alfheim Frontend-Komponenten)
- **Rust**: Für alle Services (Odin, Thor, Freki, Geri, Huginn, Muninn, Bifrost, Heimdall, Ragnarok, Jotunheim, Skuld, Vedrfolnir)
  - **Warum Rust**: Maximale Performance, Memory-Safety ohne GC, moderne Tooling, Cross-compilation
- **Elixir**: Für Yggdrasil (Millionen Verbindungen, Bifrost-Relay, Erlang VM)
- **Native Bindings**: Für Performance-kritische Teile (z.B. llama.cpp) werden native Bindings verwendet

**Gemeinsame Basis:**
- **Separate Projekte**: DTOs, Protocols, Utils werden als separate Projekte verwaltet, wenn sie von mehreren Projekten benötigt werden
- **Protobuf/MessagePack**: Für plattformübergreifende Kommunikation
- **JSON**: Für Konfiguration und einfache Datenstrukturen

### Code-Sharing Strategie
- **Separate Projekte**: Wenn gemeinsame Komponenten benötigt werden, werden separate Projekte erstellt
- **Selektive Nutzung**: Platformen können selektiv Services und deren Dependencies einbinden
- **Beispiel**: Alfheim kann ohne Valkyries laufen, Midgard kann das volle Paket erhalten
- **Konsistenz**: Durch separate Projekte wird Konsistenz gewährleistet, ohne dass alle Projekte alle Dependencies haben müssen

## Abhängigkeiten

**KEIN PROJEKT**: Dieser Ordner ist KEIN Projekt und hat keine Abhängigkeiten. Er dient nur als Metadaten-Sammlung.

## Integration

**Eigenständige Services**: Alle Services sind eigenständige Projekte:
- **Midgard, Alfheim, Asgard**: Platformen, die Services selektiv einbinden können
- **Jotunheim**: IoT-Platform, die Services selektiv einbinden kann
- **Alle Services**: Eigenständige Projekte (Odin, Thor, Freki, Geri, etc.)
- **Yggdrasil**: Cloud-Server, eigenständiges Projekt

**Selektive Integration**: Platformen können Services basierend auf ihren Anforderungen einbinden:
- **Alfheim (Mobile)**: Kann ohne Valkyries laufen, wenn das Smartphone nicht stark genug ist
- **Midgard (Desktop)**: Kann das volle Paket erhalten, da PCs/Laptops besser ausgestattet sind
- **Asgard (Homeserver)**: Kann alle Services einbinden, da Server mehr Ressourcen haben

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

## Protokoll-Übersicht

### gRPC (Service-to-Service Communication)

**gRPC wird verwendet für:**
1. **Loki Function Calls** - IoT-Device Toolcalling (via Loki Service für Jotunheim-Devices)
2. **Cross-Device Action Execution** - ThorAction/ThorResult zwischen Devices
3. **On-Device Services** - Odin ↔ Thor, Freki, Geri, Skuld
4. **Plugin Communication** - Odin ↔ Plugins (Valkyries, Frigg)
5. **Yggdrasil Microservices** - Yggdrasil ↔ Rust-Microservices
6. **Yggdrasil API** - Request/Response-Patterns mit Yggdrasil (Device-Registry, User-Management, etc.)

**Vorteile:**
- ✅ **Type-Safe**: Protobuf mit automatischer Code-Generierung
- ✅ **Effizienter**: HTTP/2, Binary-Format, weniger Overhead
- ✅ **Bessere Performance**: Bei Request/Response-Patterns
- ✅ **Built-in Streaming**: Server-Streaming, Client-Streaming, Bidirectional-Streaming
- ✅ **Besseres Error-Handling**: Status-Codes, Error-Details
- ✅ **RPC-Pattern**: Perfekt für Remote Procedure Calls

### WebSocket (Device-to-Device Messaging)

**Bifrost Protocol:**
- Device-to-Device-Kommunikation (Messaging, Events)
- Connection Establishment für gRPC-Verbindungen
- Real-time Messaging
- Event-Streaming
- Secure WebSocket-basiert mit TLS Encryption
- Message Routing und Connection Management
- Device Discovery

**Ratatoskr Protocol:**
- Yggdrasil Business-Kommunikation
- Marketplace, Payments, Provider-Registrierung
- Exklusiv für Yggdrasil-Kommunikation
- Secure WebSocket-basiert (zusätzlich zu Bifrost)
- TLS 1.3 Encryption
- Message-Signierung
- Audit-Logging
- Rate-Limiting
- Request-Validation
- Nonce-basierte Authentifizierung

**Warum WebSocket:**
- ✅ **Persistente Verbindungen**: Für kontinuierliche Device-Kommunikation
- ✅ **Event-Streaming**: Für kontinuierliche Events
- ✅ **Real-time Messaging**: Für Echtzeit-Kommunikation
- ✅ **Connection Management**: Für Device-Discovery und Connection-Establishment

### Protokoll-Entscheidungsmatrix

| Use Case | Protokoll | Grund |
|----------|-----------|-------|
| Jotunheim Function Calls | **gRPC** | Request/Response, Type-Safe, effizient |
| Cross-Device Actions | **gRPC** | Request/Response, Type-Safe, bessere Performance |
| On-Device Services | **gRPC** | Microservices-Architektur, Type-Safe |
| Plugin Communication | **gRPC** | Plugin-Isolation, Type-Safe |
| Device Messaging | **WebSocket (Bifrost)** | Persistente Verbindung, Events |
| Yggdrasil Business | **WebSocket (Ratatoskr)** | Persistente Verbindung, Business-Events |
| Connection Establishment | **WebSocket (Bifrost)** | Device-Discovery, Verbindungsaufbau |
| Yggdrasil API | **gRPC** | Request/Response, Type-Safe, bessere Performance |

### Cross-Device gRPC

**Architektur:**
1. Bifrost: Connection-Establishment
2. gRPC: Action-Execution nach Verbindung
3. Fallback: WebSocket wenn gRPC nicht möglich

**Connection Flow:**
1. Device A verbindet sich mit Device B über Bifrost
2. gRPC-Verbindung wird über Bifrost etabliert
3. Actions werden über gRPC ausgeführt
4. gRPC-Verbindung wird wiederverwendet

### gRPC Security

**Use-Case-abhängige Security:**
- **Lokale Verbindungen**: gRPC mit TLS-Verschlüsselung oder in abgesichertem Netzwerk
- **WAN-Verbindungen**: gRPC mit TLS 1.3-Verschlüsselung erforderlich
- **Kein extra Protocol nötig**: gRPC mit TLS ist sicher genug, kein zusätzliches Security-Protocol erforderlich
- **TLS-Verschlüsselung**: Alle gRPC-Verbindungen sollten TLS-verschlüsselt sein (besonders für WAN)
- **Abgesichertes Netzwerk**: Für lokale Verbindungen im abgesicherten Netzwerk kann TLS optional sein

**Security-Best-Practices:**
- **TLS 1.3**: Für WAN-Verbindungen TLS 1.3 verwenden
- **Certificate Validation**: Certificate Validation für WAN-Verbindungen
- **Connection-Security**: gRPC-Streams können verschlüsselt (TLS) oder in abgesichertem Netzwerk verwendet werden

## Implementierungs-Status

**Projekt-Status** liegt in den jeweiligen Projekten: Jedes Projekt hat einen `IMPLEMENTATION_PLAN` (bzw. `IMPLEMENTATION_STATUS`) und ein `README` mit aktueller Funktionalität und Tests.

**Aktueller Gesamtfortschritt und kritischer Pfad:** [IMPLEMENTATION_PLAN.md](../IMPLEMENTATION_PLAN.md) (Master-Plan im Repo-Root). Stand: Odin, Ratatoskr, Nornen ✅; Bifrost ~85%, Heimdall ~90%, Nidhoggr ~85%; Mimir ~90%; Thor ~55%; Ragnarok ~35%; Geri ~20%; Skuld ~18%; Freki ~8% (Phase 1.1 Struktur ✅); in Arbeit.

## Implementierungs-Notizen

**WICHTIG**: Dieser Ordner ist **KEIN PROJEKT**. Er dient nur als Metadaten-Sammlung für die Grundstruktur und Zusammenhänge der anderen Projekte.

Wenn gemeinsame Komponenten benötigt werden:
- **Separate Projekte erstellen**: Für DTOs, Protocols, Utils, die von mehreren Projekten benötigt werden
- **Selektive Nutzung**: Platformen können selektiv Services und deren Dependencies einbinden
- **Versionierung**: Separate Projekte können ihre eigene Versionierung haben
- **Backward-Compatibility**: Jedes separate Projekt muss Backward-Compatibility beachten
- **Dokumentation**: Jedes separate Projekt sollte gut dokumentiert sein
- **Tests**: Jedes separate Projekt muss Tests haben
- **Performance**: Jedes separate Projekt muss optimiert sein für minimale Latenz und Footprint
- **Datenschutz**: Jedes separate Projekt muss Privacy-by-Design implementieren
- **Sicherheit**: Jedes separate Projekt muss Security-Best-Practices folgen


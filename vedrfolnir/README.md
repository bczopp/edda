# Vedrfolnir - Connection Builder Client

## Übersicht

**Tests ausführen:** Von `vedrfolnir/`: `docker compose -f docker-compose.test.yml run --rm vedrfolnir-test` oder `./scripts/run-tests.sh` (bzw. `.\scripts\run-tests.ps1` unter Windows). **CI:** Bei Push/PR auf `vedrfolnir/**` läuft die Pipeline [.github/workflows/vedrfolnir.yml](../.github/workflows/vedrfolnir.yml) (Test im Container, Lint).

Vedrfolnir ist der Client-Service auf User-Device-Seite, der Verbindungen zu Yggdrasil aufbaut. Odin nutzt Vedrfolnir für die Kommunikation mit Yggdrasil.

**Mythologische Bedeutung**: Vedrfolnir ist der Habicht, der zwischen den Augen des Adlers sitzt. Er vermittelt zwischen User-Device und Yggdrasil (wie der Habicht zwischen den Welten vermittelt).

**Programmiersprache**: Rust

**Kommunikations-Protokolle:**
- **Ratatoskr-Protocol**: WebSocket-basiert für persistente Business-Verbindungen (Marketplace, Payments, Provider-Registrierung)
- **gRPC**: Für Request/Response-Patterns und effiziente API-Calls (Device-Registry, User-Management, etc.)
- **Protokoll-Auswahl**: gRPC für einzelne API-Calls, Ratatoskr für persistente Business-Verbindungen

## Verantwortlichkeiten

### 1. Connection Management
- **Verbindungsaufbau**: Baut Verbindungen zu Yggdrasil auf (Ratatoskr-Protocol oder gRPC)
  - **Connection-Strategien**: Verschiedene Connection-Strategien je nach Use-Case
  - **Connection-Fehler**: Robustes Error-Handling bei Connection-Fehlern (Retry, Fallback)
- **Connection-Pooling**: Verwaltet Verbindungen effizient (für beide Protokolle)
- **Automatic Reconnection**: Automatische Wiederverbindung bei Verbindungsabbruch
- **Connection-Monitoring**: Überwacht Verbindungsstatus
  - **Monitoring-Metriken**: Detaillierte Monitoring-Metriken für Verbindungsstatus
  - **Monitoring-Fehler**: Robustes Error-Handling bei Monitoring-Fehlern
- **Protokoll-Auswahl**: Wählt automatisch das beste Protokoll basierend auf Use-Case
  - **Auswahl-Strategien**: Intelligente Auswahl-Strategien für Protokoll-Auswahl
  - **Auswahl-Konflikte**: Behandlung von Auswahl-Konflikten

### 2. Message-Handling
- **Message-Sending**: Sendet Messages von Odin an Yggdrasil
  - **Message-Queuing**: Message-Queuing für bessere Performance
  - **Message-Fehler**: Robustes Error-Handling bei Message-Fehlern (Retry, Fallback)
- **Message-Receiving**: Empfängt Responses von Yggdrasil
  - **Response-Validierung**: Validierung von Responses (Signatur, Nonce, Format)
  - **Response-Fehler**: Robustes Error-Handling bei Response-Fehlern
- **Message-Queueing**: Queueing von Messages bei Verbindungsproblemen
  - **Queue-Size-Limits**: Konfigurierbare Queue-Size-Limits
  - **Queue-Overflow**: Behandlung von Queue-Overflow (Eviction, Notification)
- **Message-Retry**: Automatischer Retry bei Fehlern
- **Protokoll-spezifisch**: Unterschiedliche Handling für Ratatoskr (WebSocket) und gRPC
  - **Protokoll-spezifische Optimierungen**: Optimierungen für verschiedene Protokolle
  - **Protokoll-Kompatibilität**: Behandlung von Protokoll-Kompatibilitätsproblemen

### 3. Protocol-Handling
- **Ratatoskr-Protocol**: Implementiert Ratatoskr-Protocol (WebSocket) für persistente Verbindungen
  - **Protocol-Versionierung**: Versionierung des Ratatoskr-Protocols
  - **Protocol-Updates**: Behandlung von Protocol-Updates (Kompatibilität, Migration)
- **gRPC-Protocol**: Implementiert gRPC für Request/Response-Patterns
  - **Protocol-Versionierung**: Versionierung des gRPC-Protocols
  - **Protocol-Updates**: Behandlung von Protocol-Updates (Kompatibilität, Migration)
- **TLS-Handshake**: Handhabt TLS-Verschlüsselung (für beide Protokolle)
- **Message-Signierung**: Signiert alle ausgehenden Messages (Ratatoskr)
  - **Signierungs-Strategien**: Verschiedene Signierungs-Strategien
  - **Signierungs-Fehler**: Robustes Error-Handling bei Signierungs-Fehlern
- **Nonce-Management**: Verwaltet Nonces für Replay-Schutz (Ratatoskr)
  - **Nonce-Strategien**: Verschiedene Nonce-Strategien
  - **Nonce-Fehler**: Robustes Error-Handling bei Nonce-Fehlern
- **Protobuf-Serialization**: Protobuf für gRPC-Messages
  - **Serialization-Optimierungen**: Optimierungen für Protobuf-Serialization
  - **Serialization-Fehler**: Robustes Error-Handling bei Serialization-Fehlern

## Service-Interfaces

### Inputs
- `YggdrasilRequest` (von Odin) - Requests an Yggdrasil
  - **Ratatoskr-Requests**: Business-Requests (Marketplace, Payments, Provider-Registrierung)
  - **gRPC-Requests**: API-Requests (Device-Registry, User-Management, etc.)
  - Protokoll-Auswahl basierend auf Request-Type

### Outputs
- `YggdrasilResponse` (an Odin) - Responses von Yggdrasil
  - **Ratatoskr-Responses**: Business-Responses (Marketplace, Payments, etc.)
  - **gRPC-Responses**: API-Responses (Device-Registry, User-Management, etc.)
  - Protokoll-spezifische Responses

## Workflow

### Connection Establishment

**Ratatoskr (WebSocket) - Persistente Verbindung:**

1. **Odin initiiert Yggdrasil-Kommunikation (Ratatoskr)**
   - Odin sendet Request an Vedrfolnir (für persistente Business-Verbindung)
   - Vedrfolnir prüft, ob Ratatoskr-Verbindung zu Yggdrasil besteht

2. **Verbindungsaufbau (falls nötig)**
   - Vedrfolnir sendet `CONNECTION_REQUEST` an Yggdrasil (Nidhöggr)
   - TLS-Handshake wird durchgeführt
   - Authentication-Token wird übermittelt
   - Nidhöggr validiert Request und antwortet mit `CONNECTION_RESPONSE`

3. **Connection Established**
   - Verbindung ist aktiv
   - Heartbeat wird regelmäßig gesendet
   - Odin kann nun Requests senden

**gRPC - Request/Response:**

1. **Odin initiiert Yggdrasil-Kommunikation (gRPC)**
   - Odin sendet Request an Vedrfolnir (für einzelne API-Calls)
   - Vedrfolnir prüft, ob gRPC-Verbindung zu Yggdrasil besteht

2. **Verbindungsaufbau (falls nötig)**
   - Vedrfolnir baut gRPC-Verbindung zu Yggdrasil auf
   - TLS-Handshake wird durchgeführt
   - Authentication wird übermittelt

3. **Connection Established**
   - gRPC-Verbindung ist aktiv
   - Request kann gesendet werden

### Message Flow

**Ratatoskr (WebSocket):**

1. **Odin sendet Request**
   - Odin erstellt `RatatoskrRequest`
   - Odin sendet Request an Vedrfolnir

2. **Vedrfolnir verarbeitet Request**
   - Request wird validiert
   - Message wird signiert
   - Nonce wird hinzugefügt
   - Request wird über Ratatoskr-Protocol an Yggdrasil gesendet

3. **Response empfangen**
   - Vedrfolnir empfängt Response von Yggdrasil
   - Response wird validiert (Signatur, Nonce)
   - Response wird an Odin weitergegeben

**gRPC:**

1. **Odin sendet Request**
   - Odin erstellt `gRPCRequest`
   - Odin sendet Request an Vedrfolnir

2. **Vedrfolnir verarbeitet Request**
   - Request wird validiert
   - Request wird als Protobuf serialisiert
   - Request wird über gRPC an Yggdrasil gesendet

3. **Response empfangen**
   - Vedrfolnir empfängt Response von Yggdrasil
   - Response wird deserialisiert
   - Response wird an Odin weitergegeben

4. **Error-Handling (beide Protokolle)**
   - Bei Verbindungsfehler: Automatischer Retry
   - Bei Rate-Limiting: Request wird in Queue gelegt
   - Bei Authentication-Fehler: Odin wird informiert

## Abhängigkeiten

### Keine Core Library

- **WICHTIG**: Es gibt keine Edda Core Library
- **Separate Projekte**: Wenn gemeinsame Komponenten benötigt werden (DTOs, Protocols wie Ratatoskr Protocol, Utils), sollte ein separates Projekt erstellt werden
- **Selektive Nutzung**: Dies hält Apps klein, da genau gewählt werden kann, was benötigt wird
- **Keine Abhängigkeit**: Vedrfolnir sollte nicht auf Dateien/Protocols/Utils aus dem `edda` Verzeichnis verweisen (KEIN PROJEKT - nur Metadaten-Sammlung)

### Technische Abhängigkeiten

- **Heimdall**: Für Authentication-Token-Management
- **Network Stack**: Für WebSocket und TLS
- **Security Libraries**: Für Verschlüsselung und Signierung

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

### Vedrfolnir-spezifische Settings

**Settings-Inhalt (wird während Implementierung definiert)**
- Connection-Management-Einstellungen
- Protocol-Konfiguration
- Error-Recovery-Einstellungen

## Integration

- **Odin**: Nutzt Vedrfolnir für Yggdrasil-Kommunikation
  - **Odin-Integration**: Integration-Workflows für Odin
  - **Odin-Ausfälle**: Robustes Error-Handling bei Odin-Ausfällen
- **Yggdrasil (Nidhöggr)**: Empfängt Verbindungen von Vedrfolnir
  - **Yggdrasil-Integration**: Kommunikation zwischen Vedrfolnir und Yggdrasil (Nidhöggr)
  - **Server-Implementierungen**: Server-seitige Implementierungen für Yggdrasil
  - **Yggdrasil-Ausfälle**: Robustes Error-Handling bei Yggdrasil-Ausfällen
- **Ratatoskr-Protocol**: WebSocket-basiertes Kommunikationsprotokoll für persistente Business-Verbindungen
- **gRPC**: Request/Response-basiertes Kommunikationsprotokoll für effiziente API-Calls
- **Protokoll-Auswahl**: Vedrfolnir wählt automatisch das beste Protokoll basierend auf Request-Type

## Error Recovery

### Automatische Wiederverbindung
- **Sofortiger Versuch**: Bei Verbindungsabbruch wird sofort versucht, Verbindung wiederherzustellen
- **Exponential Backoff**: Nach erstem Fehler beginnt Exponential Backoff
- **Kontinuierliche Versuche**: System versucht kontinuierlich, Verbindung wiederherzustellen
- **Reconnection-Strategien**: Verschiedene Reconnection-Strategien je nach Use-Case
- **Persistente Connection-Probleme**: Behandlung von persistenten Connection-Problemen (Fallback, Notification)

### Message-Queueing
- **Queue bei Verbindungsproblemen**: Messages werden in Queue gelegt, wenn Verbindung nicht verfügbar ist
- **Automatisches Senden**: Messages werden automatisch gesendet, sobald Verbindung wiederhergestellt ist
- **Queue-Limits**: Queue hat Limits, um Memory-Overflow zu verhindern
- **Queue-Management**: Intelligentes Queue-Management für bessere Performance
- **Queue-Overflow**: Behandlung von Queue-Overflow (Eviction, Notification)

## Performance

### Connection-Performance
- **Connection-Performance-Optimierungen**: Optimierungen für Connection-Performance
- **Connection-Pooling**: Wiederverwendung von Verbindungen
- **Connection-Load**: Effiziente Behandlung von hohem Connection-Load

### Message-Performance
- **Message-Performance-Optimierungen**: Optimierungen für Message-Performance
- **Message-Batching**: Batching von Messages für bessere Performance
- **Message-Load**: Effiziente Behandlung von hohem Message-Load

### Serialization-Performance
- **Serialization-Performance-Optimierungen**: Optimierungen für Serialization-Performance
- **Serialization-Optimierungen**: Verschiedene Serialization-Optimierungen
- **Serialization-Load**: Effiziente Behandlung von hohem Serialization-Load

### Performance-Optimierungen
- **Connection-Pooling**: Wiederverwendung von Verbindungen
- **Message-Batching**: Batching von Messages für bessere Performance
- **Async Processing**: Asynchrone Verarbeitung von Messages
- **Efficient Serialization**: Effiziente Serialisierung von Messages

### Performance-Metriken
- Schnelle Connection-Establishment (< 500ms)
- Niedrige Message-Latenz (< 100ms für Standard-Requests)
- Effiziente Message-Übertragung (minimaler Overhead)

## Sicherheit

### TLS-Encryption
- **TLS 1.3 Encryption**: Implementierung von TLS 1.3 Encryption
- **Certificate-Validierung**: Validierung von TLS-Zertifikaten
- **Encryption-Fehler**: Robustes Error-Handling bei Encryption-Fehlern

### Message-Security
- **Message-Security-Implementierung**: Umfassende Message-Security-Implementierung
- **Security-Validierung**: Validierung von Message-Security (Signatur, Nonce, etc.)
- **Security-Threats**: Erkennung und Behandlung von Security-Threats

### Token-Management
- **Token-Management-Implementierung**: Sichere Verwaltung von Authentication-Tokens
- **Token-Refresh-Mechanismen**: Automatische Token-Refresh-Mechanismen
- **Token-Fehler**: Robustes Error-Handling bei Token-Fehlern (Refresh, Re-Authentication)

### Security-Features
- **TLS 1.3 Encryption**: Verschlüsselte Verbindungen
- **Message-Signierung**: Digitale Signaturen für alle Messages
- **Nonce-basierte Authentifizierung**: Schutz vor Replay-Angriffen
- **Token-Management**: Sichere Verwaltung von Authentication-Tokens
- **Connection-Validation**: Validierung aller Verbindungen

## Data Protection

### Message-Privacy
- **Message-Privacy**: Gewährleistung von Message-Privacy
- **Privacy-by-Design**: Privacy-by-Design-Prinzipien für alle Messages
- **Message-Schutz**: Schutz von Messages vor unbefugtem Zugriff

## Implementierungs-Notizen

- **Programmiersprache**: Rust
- **Async Runtime**: tokio für asynchrone Verarbeitung
- **WebSocket Library**: tokio-tungstenite oder ähnlich (für Ratatoskr)
- **gRPC Library**: tonic oder ähnlich (für gRPC)
- **TLS Library**: rustls oder native-tls (für beide Protokolle)
- **Message-Serialization**: 
  - Protobuf für gRPC
  - Protobuf oder MessagePack für Ratatoskr
- **Connection-Pooling**: Effizientes Connection-Pooling für beide Protokolle
- **Error-Handling**: Robustes Error-Handling mit Retry-Mechanismen
- **Performance**: Optimiert für schnelle Message-Übertragung
- **Protokoll-Auswahl**: Intelligente Auswahl zwischen Ratatoskr und gRPC basierend auf Use-Case


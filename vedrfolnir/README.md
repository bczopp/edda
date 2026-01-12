# Vedrfolnir - Connection Builder Client

## Übersicht

Vedrfolnir ist der Client-Service auf User-Device-Seite, der Verbindungen zu Yggdrasil über das Ratatoskr-Protocol aufbaut. Odin nutzt Vedrfolnir für die Kommunikation mit Yggdrasil.

**Mythologische Bedeutung**: Vedrfolnir ist der Habicht, der zwischen den Augen des Adlers sitzt. Er vermittelt zwischen User-Device und Yggdrasil (wie der Habicht zwischen den Welten vermittelt).

**Programmiersprache**: Rust

## Verantwortlichkeiten

### 1. Connection Management
- **Verbindungsaufbau**: Baut Verbindungen zu Yggdrasil auf (über Ratatoskr-Protocol)
- **Connection-Pooling**: Verwaltet Verbindungen effizient
- **Automatic Reconnection**: Automatische Wiederverbindung bei Verbindungsabbruch
- **Connection-Monitoring**: Überwacht Verbindungsstatus

### 2. Message-Handling
- **Message-Sending**: Sendet Messages von Odin an Yggdrasil
- **Message-Receiving**: Empfängt Responses von Yggdrasil
- **Message-Queueing**: Queueing von Messages bei Verbindungsproblemen
- **Message-Retry**: Automatischer Retry bei Fehlern

### 3. Protocol-Handling
- **Ratatoskr-Protocol**: Implementiert Ratatoskr-Protocol
- **TLS-Handshake**: Handhabt TLS-Verschlüsselung
- **Message-Signierung**: Signiert alle ausgehenden Messages
- **Nonce-Management**: Verwaltet Nonces für Replay-Schutz

## Service-Interfaces

### Inputs
- `RatatoskrRequest` (von Odin) - Business-Requests an Yggdrasil
  - Marketplace-Requests
  - Payment-Requests
  - Provider-Registration-Requests
  - Business-Requests

### Outputs
- `RatatoskrResponse` (an Odin) - Responses von Yggdrasil
  - Marketplace-Responses
  - Payment-Responses
  - Provider-Registration-Responses
  - Business-Responses

## Workflow

### Connection Establishment

1. **Odin initiiert Yggdrasil-Kommunikation**
   - Odin sendet Request an Vedrfolnir
   - Vedrfolnir prüft, ob Verbindung zu Yggdrasil besteht

2. **Verbindungsaufbau (falls nötig)**
   - Vedrfolnir sendet `CONNECTION_REQUEST` an Yggdrasil (Nidhöggr)
   - TLS-Handshake wird durchgeführt
   - Authentication-Token wird übermittelt
   - Nidhöggr validiert Request und antwortet mit `CONNECTION_RESPONSE`

3. **Connection Established**
   - Verbindung ist aktiv
   - Heartbeat wird regelmäßig gesendet
   - Odin kann nun Requests senden

### Message Flow

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

4. **Error-Handling**
   - Bei Verbindungsfehler: Automatischer Retry
   - Bei Rate-Limiting: Request wird in Queue gelegt
   - Bei Authentication-Fehler: Odin wird informiert

## Abhängigkeiten

- **Edda Core Library**: DTOs, Protocols (Ratatoskr Protocol)
- **Heimdall**: Für Authentication-Token-Management
- **Network Stack**: Für WebSocket und TLS
- **Security Libraries**: Für Verschlüsselung und Signierung

## Integration

- **Odin**: Nutzt Vedrfolnir für Yggdrasil-Kommunikation
- **Yggdrasil (Nidhöggr)**: Empfängt Verbindungen von Vedrfolnir
- **Ratatoskr-Protocol**: Kommunikationsprotokoll zwischen Vedrfolnir und Nidhöggr

## Error Recovery

### Automatische Wiederverbindung
- **Sofortiger Versuch**: Bei Verbindungsabbruch wird sofort versucht, Verbindung wiederherzustellen
- **Exponential Backoff**: Nach erstem Fehler beginnt Exponential Backoff
- **Kontinuierliche Versuche**: System versucht kontinuierlich, Verbindung wiederherzustellen

### Message-Queueing
- **Queue bei Verbindungsproblemen**: Messages werden in Queue gelegt, wenn Verbindung nicht verfügbar ist
- **Automatisches Senden**: Messages werden automatisch gesendet, sobald Verbindung wiederhergestellt ist
- **Queue-Limits**: Queue hat Limits, um Memory-Overflow zu verhindern

## Performance

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

### Security-Features
- **TLS 1.3 Encryption**: Verschlüsselte Verbindungen
- **Message-Signierung**: Digitale Signaturen für alle Messages
- **Nonce-basierte Authentifizierung**: Schutz vor Replay-Angriffen
- **Token-Management**: Sichere Verwaltung von Authentication-Tokens
- **Connection-Validation**: Validierung aller Verbindungen

## Implementierungs-Notizen

- **Programmiersprache**: Rust
- **Async Runtime**: tokio für asynchrone Verarbeitung
- **WebSocket Library**: tokio-tungstenite oder ähnlich
- **TLS Library**: rustls oder native-tls
- **Message-Serialization**: Protobuf oder MessagePack
- **Connection-Pooling**: Effizientes Connection-Pooling
- **Error-Handling**: Robustes Error-Handling mit Retry-Mechanismen
- **Performance**: Optimiert für schnelle Message-Übertragung


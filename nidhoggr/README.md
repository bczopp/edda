# Nidhöggr - Connection Endpoint & Message Receiver

## Übersicht

Nidhöggr ist der Server-Side Connection Endpoint bei Yggdrasil. Er empfängt Verbindungen von Vedrfolnir-Clients (User-Devices) über das Ratatoskr-Protocol und leitet Nachrichten direkt an entsprechende Services weiter.

**Mythologische Bedeutung**: Nidhöggr ist der Drache, der an den Wurzeln des Weltenbaums nagt. Repräsentiert User Requests (Root/Wurzeln), die von den User-Devices kommen.

**Programmiersprache**: Rust

## Verantwortlichkeiten

### 1. Connection Management
- **Empfängt Verbindungen**: Von Vedrfolnir-Clients (User-Devices)
- **Connection-Validation**: Validiert alle eingehenden Verbindungen
- **Connection-Monitoring**: Überwacht alle aktiven Verbindungen
- **Connection-Termination**: Kann Verbindungen trennen (bei bestimmten Umständen)

### 2. Message-Receiving
- **Empfängt Nachrichten**: Über Ratatoskr-Protocol
- **Message-Validation**: Validiert alle eingehenden Messages (Signatur, Nonce, etc.)
- **Rate-Limiting**: Prüft Rate-Limits pro Device/User
- **Audit-Logging**: Erstellt Audit-Logs für alle Business-Transaktionen

### 3. Message-Routing
- **Direkte Weiterleitung**: Leitet Nachrichten direkt weiter an entsprechende Services
- **Service-Discovery**: Bestimmt, welcher Service für welche Message-Type zuständig ist
- **Load-Balancing**: Lastverteilung bei mehreren Service-Instanzen
- **Error-Handling**: Fehlerbehandlung bei Service-Ausfällen

## Service-Interfaces

### Inputs
- `RatatoskrRequest` (von Vedrfolnir) - Business-Requests von User-Devices
  - Marketplace-Requests → Nornen
  - Payment-Requests → Heidrun
  - Provider-Registration-Requests → Nornen
  - Business-Requests → Nornen

### Outputs
- `RatatoskrResponse` (an Vedrfolnir) - Responses an User-Devices
- `ServiceRequest` (an Nornen/Mimir/Heidrun/etc.) - Weitergeleitete Requests

## Workflow

### Connection Establishment

1. **Vedrfolnir sendet Connection-Request**
   - Vedrfolnir sendet `CONNECTION_REQUEST` an Yggdrasil
   - Enthält: Device-Identity, Authentication-Token

2. **Nidhöggr validiert Request**
   - Heimdall validiert Device-Identity
   - Authentication-Token wird geprüft
   - Rate-Limiting wird geprüft

3. **TLS Handshake**
   - TLS 1.3 Encryption wird etabliert
   - Keys werden ausgetauscht
   - Verbindung ist verschlüsselt

4. **Connection Established**
   - Nidhöggr sendet `CONNECTION_RESPONSE`
   - WebSocket-Verbindung ist aktiv
   - Heartbeat wird regelmäßig gesendet

### Message Flow

1. **Vedrfolnir sendet Message**
   - Vedrfolnir sendet Message über Ratatoskr-Protocol
   - Nidhöggr empfängt Message

2. **Message-Validation**
   - Message-Signatur wird geprüft
   - Nonce wird validiert
   - Rate-Limiting wird geprüft
   - Audit-Log wird erstellt

3. **Message-Routing**
   - Nidhöggr bestimmt Ziel-Service basierend auf Message-Type
   - Message wird an entsprechenden Service weitergeleitet:
     - `MARKETPLACE_REQUEST` → Nornen
     - `PAYMENT_REQUEST` → Heidrun
     - `PROVIDER_REGISTRATION` → Nornen
     - `BUSINESS_REQUEST` → Nornen

4. **Response-Handling**
   - Service antwortet an Nidhöggr
   - Nidhöggr sendet Response über Ratatoskr-Protocol zurück
   - Vedrfolnir empfängt Response

### Connection Termination

Nidhöggr kann Verbindungen trennen bei:
- **Authentication-Fehler**: Ungültige Authentication-Tokens
- **Rate-Limiting-Verstoß**: Zu viele Requests in kurzer Zeit
- **Security-Verstoß**: Verdächtige Aktivitäten
- **Graceful Shutdown**: Geplantes Herunterfahren
- **Timeout**: Inaktive Verbindungen

## Service-Routing

### Message-Type → Service Mapping

- **MARKETPLACE_REQUEST** → Nornen (Urd/Verdandi)
  - Provider-Registrierung
  - Provider-Abfrage
  - Request-Routing

- **PAYMENT_REQUEST** → Heidrun
  - Payment-Processing
  - Pre-Authorization
  - Refund-Requests

- **PROVIDER_REGISTRATION** → Nornen (Urd/Verdandi)
  - Registration-Request
  - Registration-Response
  - Registration-Update

- **BUSINESS_REQUEST** → Nornen (Urd/Verdandi)
  - Allgemeine Business-Transaktionen
  - User-Konfiguration
  - Admin-Requests

## Abhängigkeiten

- **Edda Core Library**: DTOs, Protocols (Ratatoskr Protocol)
- **Heimdall**: Für Connection Validation und Security
- **Network Stack**: Für WebSocket und TLS
- **Security Libraries**: Für Verschlüsselung und Signierung
- **Yggdrasil Services**: Nornen, Mimir, Heidrun, Eikthyrnir, etc.

## Integration

- **Vedrfolnir**: Empfängt Verbindungen von Vedrfolnir-Clients
- **Yggdrasil (Elixir)**: Hauptprozess, koordiniert alle Services
- **Nornen**: Entscheidungen über Requests, Provider-Registrierung
- **Heidrun**: Payment-Processing, Token-Berechnungen
- **Mimir**: Database-Queries (falls nötig)
- **Eikthyrnir**: Quality-Assessment (falls nötig)

## Error Handling

### Connection Errors
- **Invalid Authentication**: Verbindung wird abgelehnt
- **Rate-Limiting**: Verbindung wird temporär blockiert
- **Security-Violation**: Verbindung wird sofort getrennt

### Message Errors
- **Invalid Signature**: Message wird abgelehnt
- **Invalid Nonce**: Message wird abgelehnt
- **Service Unavailable**: Message wird in Queue gelegt oder Fehler-Response gesendet

## Performance

### Performance-Optimierungen
- **Connection-Pooling**: Effizientes Connection-Pooling
- **Message-Batching**: Batching von Messages für bessere Performance
- **Async Processing**: Asynchrone Verarbeitung von Messages
- **Load-Balancing**: Lastverteilung bei mehreren Service-Instanzen

### Performance-Metriken
- Niedrige Connection-Latenz (< 100ms für Connection-Establishment)
- Schnelle Message-Routing (< 10ms für Message-Routing)
- Hoher Durchsatz (1000+ Messages/Sekunde pro Instanz)

## Sicherheit

### Security-Features
- **TLS 1.3 Encryption**: Verschlüsselte Verbindungen
- **Message-Validation**: Validierung aller Messages (Signatur, Nonce)
- **Rate-Limiting**: Rate-Limiting pro Device/User
- **Audit-Logging**: Vollständiges Audit-Logging
- **Connection-Monitoring**: Überwachung aller Verbindungen auf verdächtige Aktivitäten

## Implementierungs-Notizen

- **Programmiersprache**: Rust
- **Async Runtime**: tokio für asynchrone Verarbeitung
- **WebSocket Library**: tokio-tungstenite oder ähnlich
- **TLS Library**: rustls oder native-tls
- **Message-Serialization**: Protobuf oder MessagePack
- **gRPC**: Für Kommunikation mit anderen Yggdrasil-Services
- **Connection-Pooling**: Effizientes Connection-Pooling
- **Error-Handling**: Robustes Error-Handling
- **Performance**: Optimiert für hohen Durchsatz


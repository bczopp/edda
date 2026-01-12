# Ratatoskr - Business Protocol

## Übersicht

Ratatoskr ist ein WebSocket-basiertes Business-Protocol für die Kommunikation zwischen User-Devices und Yggdrasil. Es ist zusätzlich zu Bifrost implementiert und speziell für Yggdrasil Business-Logik (Marketplace, Payments, Provider-Registrierung) designed.

**Mythologische Bedeutung**: Ratatoskr ist das Eichhörnchen, das Nachrichten zwischen Nidhöggr (an den Wurzeln) und Vedrfolnir (dem Habicht zwischen den Augen des Adlers) transportiert. Nidhöggr repräsentiert User Requests (Root/Wurzeln), Vedrfolnir repräsentiert den Client auf User-Device-Seite.

## Unterschied zu Bifrost

**Bifrost:**
- Device-zu-Device-Kommunikation (lokal und global)
- Für allgemeine Inter-Device-Kommunikation
- Kann auch lokal genutzt werden

**Ratatoskr:**
- Business-Logik-Kommunikation (nur Yggdrasil)
- Speziell für Marketplace, Payments, Provider-Registrierung
- Extra abgesichert (nicht direkt nach außen)
- Nicht für lokale Device-zu-Device-Kommunikation

## Protocol-Features

### Basis-Protokoll
- **WebSocket-basiert**: Wie Bifrost, aber mit zusätzlichen Security-Features
- **TLS 1.3 Encryption**: Verschlüsselte Verbindungen
- **Binary Message Format**: Effiziente binäre Nachrichten

### Zusätzliche Security-Layer
- **Message-Signierung**: Alle Messages sind digital signiert
- **Audit-Logging**: Vollständiges Audit-Logging aller Business-Transaktionen
- **Rate-Limiting**: Rate-Limiting für Business-Requests
- **Request-Validation**: Umfassende Validierung aller Requests
- **Nonce-basierte Authentifizierung**: Schutz vor Replay-Angriffen

## Message-Types

### Connection Management
- `CONNECTION_REQUEST`: Anfrage für neue Verbindung
- `CONNECTION_RESPONSE`: Antwort auf Verbindungsanfrage
- `HEARTBEAT`: Keep-Alive für Verbindungen
- `DISCONNECT`: Verbindung beenden
- `ERROR`: Fehler-Message

### Business-Transaktionen
- `BUSINESS_REQUEST`: Allgemeine Business-Transaktionen
- `MARKETPLACE_REQUEST`: Marketplace-Operationen
  - Provider-Registrierung
  - Provider-Abfrage
  - Request-Routing
- `PAYMENT_REQUEST`: Payment-Operationen
  - Payment-Processing
  - Pre-Authorization
  - Refund-Requests
- `PROVIDER_REGISTRATION`: Provider-Registrierung
  - Registration-Request
  - Registration-Response
  - Registration-Update

## Workflow

### Connection Establishment

1. **Vedrfolnir (Client) initiiert Verbindung**
   - Client sendet `CONNECTION_REQUEST` an Yggdrasil
   - Enthält: Device-Identity, Authentication-Token

2. **Nidhöggr (Server) validiert Request**
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

1. **Odin (User-Device) sendet Request**
   - Odin nutzt Vedrfolnir-Service
   - Vedrfolnir sendet Message über Ratatoskr-Protocol

2. **Nidhöggr empfängt Message**
   - Message wird validiert (Signatur, Nonce, etc.)
   - Audit-Log wird erstellt
   - Rate-Limiting wird geprüft

3. **Message-Routing**
   - Nidhöggr leitet Message weiter an entsprechende Services
   - Je nach Message-Type: Nornen, Mimir, Heidrun, etc.

4. **Response**
   - Service antwortet an Nidhöggr
   - Nidhöggr sendet Response über Ratatoskr-Protocol zurück
   - Vedrfolnir empfängt Response und gibt sie an Odin weiter

## Security

### Verschlüsselung
- **TLS 1.3**: Transport-Layer-Verschlüsselung
- **Message-Signierung**: Alle Messages sind digital signiert
- **Nonce-basierte Authentifizierung**: Schutz vor Replay-Angriffen

### Audit-Logging
- **Vollständiges Logging**: Alle Business-Transaktionen werden geloggt
- **Immutable Logs**: Logs können nicht verändert werden
- **Compliance**: Erfüllt Compliance-Anforderungen (GDPR, PCI-DSS, etc.)

### Rate-Limiting
- **Per-Device Rate-Limiting**: Rate-Limiting pro Device
- **Per-User Rate-Limiting**: Rate-Limiting pro User
- **Per-Request-Type Rate-Limiting**: Unterschiedliche Limits für verschiedene Request-Types

## Abhängigkeiten

- **Edda Core Library**: DTOs, Protocols (Ratatoskr Protocol)
- **Heimdall**: Für Connection Validation und Security
- **Network Stack**: Für WebSocket und TLS
- **Security Libraries**: Für Verschlüsselung und Signierung

## Integration

- **Vedrfolnir**: Client-Service auf User-Device-Seite nutzt Ratatoskr-Protocol
- **Nidhöggr**: Server-Side Connection Endpoint bei Yggdrasil
- **Odin**: Nutzt Vedrfolnir für Yggdrasil-Kommunikation
- **Yggdrasil Services**: Nornen, Mimir, Heidrun, etc. empfangen Requests über Nidhöggr

## Implementierungs-Notizen

- **WebSocket-basiert**: Implementierung als WebSocket-Protokoll
- **Binary Format**: Effizientes binäres Nachrichtenformat
- **Message-Signierung**: Digitale Signaturen für alle Messages
- **Audit-Logging**: Vollständiges Audit-Logging
- **Rate-Limiting**: Intelligentes Rate-Limiting
- **Error-Handling**: Robustes Error-Handling
- **Connection-Pooling**: Effizientes Connection-Pooling auf Server-Seite


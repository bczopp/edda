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

### Keine Core Library

- **WICHTIG**: Es gibt keine Edda Core Library
- **Separate Projekte**: Wenn gemeinsame Komponenten benötigt werden (DTOs, Protocols wie Ratatoskr Protocol, Utils), sollte ein separates Projekt erstellt werden
- **Selektive Nutzung**: Dies hält Apps klein, da genau gewählt werden kann, was benötigt wird
- **Keine Abhängigkeit**: Ratatoskr sollte nicht auf Dateien/Protocols/Utils aus dem `edda` Verzeichnis verweisen (KEIN PROJEKT - nur Metadaten-Sammlung)

### Technische Abhängigkeiten

- **Heimdall**: Für Connection Validation und Security
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

### Ratatoskr-spezifische Settings

**Settings-Inhalt (wird während Implementierung definiert)**
- WebSocket-Konfiguration
- Message-Signierung-Einstellungen
- Rate-Limiting-Einstellungen

## Integration

- **Vedrfolnir**: Client-Service auf User-Device-Seite nutzt Ratatoskr-Protocol
- **Nidhöggr**: Server-Side Connection Endpoint bei Yggdrasil
- **Odin**: Nutzt Vedrfolnir für Yggdrasil-Kommunikation
- **Yggdrasil Services**: Nornen, Mimir, Heidrun, etc. empfangen Requests über Nidhöggr

## Implementierungs-Status

✅ **Vollständig implementiert** - Alle Phasen des IMPLEMENTATION_PLANs sind abgeschlossen:

- ✅ Phase 1: Projekt-Setup (Dependencies, Verzeichnisstruktur, Test-Infrastruktur)
- ✅ Phase 2: Protocol-Definition (Message-Definitions: RatatoskrRequest, RatatoskrResponse)
- ✅ Phase 3: Message-Serialization (MessageSerializer mit Protobuf)
- ✅ Phase 4: Message-Validation (MessageValidator: Schema, Nonce, Signature, Timestamp)
- ✅ Phase 5: Connection-Protocol (CONNECTION_REQUEST/RESPONSE, Handshake)
- ✅ Phase 6: Security-Features (Message-Signature mit Ed25519, Nonce-Management)
- ✅ Phase 7: Documentation & Examples
- ✅ Phase 8: Testing (Unit-Tests, Integration-Tests)

## Struktur

```
ratatoskr/
├── src/
│   ├── proto/              # Protobuf-Definitionen (generiert)
│   ├── messages/           # Request/Response Helper
│   │   ├── request.rs      # RatatoskrRequest Helper
│   │   └── response.rs     # RatatoskrResponse Helper
│   └── protocol/           # Protocol-Implementation
│       ├── serializer.rs   # Message Serialization/Deserialization
│       ├── validator.rs    # Message Validation
│       ├── security.rs     # Security Features (Signing, Nonce)
│       └── connection.rs   # Connection Protocol
├── proto/
│   └── ratatoskr.proto     # Protobuf Definition
├── tests/                  # Unit- und Integration-Tests
└── example/                # Beispiel-Implementierung
```

## Verwendung

### Als Dependency

Füge Ratatoskr zu deinem `Cargo.toml` hinzu:

```toml
[dependencies]
ratatoskr = { path = "../ratatoskr" }
```

### Beispiel: Connection Request erstellen

```rust
use ratatoskr::messages::*;
use ratatoskr::protocol::*;
use ratatoskr::proto::ratatoskr::*;

// Connection Request erstellen
let request = RatatoskrRequest::new_connection_request(
    "req-123".to_string(),
    "device-456".to_string(),
    "user-789".to_string(),
    "device-identity".to_string(),
    "auth-token".to_string(),
    "1.0.0".to_string(),
);

// Nonce generieren und setzen
let nonce_manager = NonceManager::new();
request.nonce = nonce_manager.generate_nonce();

// Message signieren
let signing_key = SigningKey::generate(&mut OsRng);
let signer = MessageSigner::new(signing_key);
signer.sign_request(&mut request)?;

// Message serialisieren
let serializer = MessageSerializer::new();
let serialized = serializer.serialize_request(&request)?;
```

### Beispiel: Message validieren

```rust
use ratatoskr::protocol::*;

// Message deserialisieren
let serializer = MessageSerializer::new();
let request = serializer.deserialize_request(&data)?;

// Message validieren
let validator = MessageValidator::new();
validator.validate_request(&request)?;

// Nonce prüfen (Replay-Schutz)
let nonce_manager = NonceManager::new();
nonce_manager.validate_and_record_nonce(&request.nonce)?;

// Signatur verifizieren
let verifying_key = signer.verifying_key();
signer.verify_request(&request, &verifying_key)?;
```

## Testing

Alle Tests laufen in Containern:

```bash
# Tests ausführen
docker compose -f docker-compose.test.yml run --rm ratatoskr-test
# Alternativ: ./scripts/run-tests.sh (bzw. .\scripts\run-tests.ps1 unter Windows)
```

**CI:** Bei Push/PR auf `ratatoskr/**` läuft die Pipeline [.github/workflows/ratatoskr.yml](../.github/workflows/ratatoskr.yml) (Test im Container, Lint).

## Implementierungs-Notizen

- **WebSocket-basiert**: Implementierung als WebSocket-Protokoll (für Nidhöggr/Vedrfolnir)
- **Protobuf Format**: Effizientes binäres Nachrichtenformat mit Protobuf
- **Message-Signierung**: Digitale Signaturen für alle Messages (Ed25519)
- **Nonce-Management**: Replay-Schutz durch Nonce-Validierung
- **Error-Handling**: Robustes Error-Handling mit thiserror
- **TDD**: Alle Features wurden mit Test-Driven Development implementiert


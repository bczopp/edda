# Bifrost - Communication Service

## Übersicht

Bifrost ist der Secure WebSocket Service für Inter-Device Communication. Er ermöglicht sichere, verschlüsselte Kommunikation zwischen Devices im Edda-Netzwerk.

**Tests ausführen:** Von `bifrost/`: `docker compose -f docker-compose.test.yml run --rm bifrost-test` oder `./scripts/run-tests.sh` (bzw. `.\scripts\run-tests.ps1` unter Windows). Siehe [Phase 20 Test Suites](#phase-20-test-suites-implementation_plan) für die Test-Dateien. **CI:** Bei Push/PR auf `bifrost/**` läuft die Pipeline [.github/workflows/bifrost.yml](../.github/workflows/bifrost.yml) (Test im Container, Lint, Coverage, Rustdoc, cargo-audit).

**Mesh-Layer-Design**: Siehe [docs/MESH_LAYER_DESIGN.md](docs/MESH_LAYER_DESIGN.md) für Paketformat (MeshPacket/Data), Managed Flood, Hop-Limit, Discovery und IP-Transport-Entscheid.

## Features

- **Secure WebSocket Communication**: WebSocket-basierte Kommunikation mit TLS-Verschlüsselung
- **TLS Encryption**: Alle Verbindungen sind verschlüsselt
- **Device-Mesh (Meshtastic-inspiriert)**: Bifrost nutzt ein Multi-Hop-Mesh zwischen den Devices eines Users; Verbindungen erfordern Mesh-Membership
- **Message Routing**: Routing von Messages zwischen Devices (direkt oder über mehrere Hop)
- **Device Discovery**: Automatische Device-Discovery im Netzwerk
- **Connection Management**: Verwaltung von Device-Verbindungen
- **Error Recovery**: Automatische Wiederverbindung (sofort + Exponential Backoff)
- **Network Error Handling**: Retry → Fallback → Fehler
- **Resilience**: Robustes Error-Handling für Netzwerk-Fehler

## Error Recovery

### Automatische Wiederverbindung
- **Sofortiger Versuch**: Bei Verbindungsabbruch wird sofort versucht, Verbindung wiederherzustellen
- **Exponential Backoff**: Nach erstem Fehler beginnt Exponential Backoff
- **Kontinuierliche Versuche**: System versucht kontinuierlich, Verbindung wiederherzustellen

### Netzwerk-Fehlerbehandlung
- **Retry mit Exponential Backoff**: Automatischer Retry bei Netzwerk-Fehlern
- **Fallback zu alternativer Route**: Falls Retry fehlschlägt, Fallback zu alternativer Route
- **Relay-Fallback**: Falls direkte Verbindung fehlschlägt, Fallback zu Relay (Asgard/Yggdrasil)

## Protocol Features

### Bifrost Protocol
- Secure WebSocket-basiert
- TLS Encryption
- Message Routing
- Connection Management
- Device Discovery

### Message Types
- **CONNECTION_REQUEST**: Anfrage für neue Verbindung
- **CONNECTION_RESPONSE**: Antwort auf Verbindungsanfrage
- **MESSAGE**: Standard-Message zwischen Devices
- **HEARTBEAT**: Keep-Alive für Verbindungen
- **DISCONNECT**: Verbindung beenden
- **ERROR**: Fehler-Message

## Workflow

### Connection Establishment

**Mesh-Membership (Device-Mesh):**
- **WICHTIG**: Bifrost-Verbindungen erfordern Mesh-Membership; Devices müssen im gleichen User-Mesh sein
- Bifrost enthält einen Mesh-Layer (Meshtastic-inspiriert): Multi-Hop-Routing (Managed Flood), Discovery, Transports (IP, optional LoRa)
- Schichten:
  - **Mesh-Layer**: Membership, Discovery, Multi-Hop-Routing, Transports (IP/LoRa)
  - **Bifrost-Protokoll (Layer 7)**: WebSocket-Kommunikation ÜBER den Mesh-Layer
- Ohne Mesh-Membership: Bifrost-Verbindung wird blockiert
- Mesh-Check: Bei jedem Connection-Request wird Mesh-Membership geprüft (Heimdall)

**Connection-Workflow (mit Mesh-Check):**

1. **Device A möchte mit Device B kommunizieren**
   - Device A prüft Mesh-Membership (über Heimdall)
   - Device A prüft ob Device B im gleichen User-Mesh ist
   - **Wenn kein Mesh**: Connection wird blockiert, Fehler wird zurückgegeben

2. **Mesh-Membership aktiv → Bifrost-Verbindung kann aufgebaut werden**
   - Device A sendet Connection-Request an Device B (über Mesh-Pfad, ggf. Multi-Hop)
   - Request wird an Heimdall weitergeleitet für Validation
   - Heimdall prüft:
     - Ist Device A authentifiziert? (Heimdall-Token)
     - Hat Device A Permission für Device B?
     - Sind beide Devices im gleichen User-Mesh?
     - Mesh-Membership-Validation

3. **Heimdall-Validation erfolgreich → Connection wird zugelassen**
   - Device B erhält Connection-Request (direkt oder über Mesh-Relays)
   - Device B akzeptiert Verbindung
   - WebSocket-Verbindung wird etabliert (über Mesh-Layer)

4. **Bidirektionale Kommunikation**
   - Messages werden über WebSocket gesendet (Application Layer)
   - WebSocket läuft über Mesh-Layer (Multi-Hop bei Bedarf)
   - Defense in Depth: Mesh-Verschlüsselung + TLS + Heimdall-Validation

5. **Mesh-Monitoring**
   - Bifrost überwacht kontinuierlich Mesh-Connectivity
   - Bei Mesh-Ausfall: Bifrost-Verbindungen werden automatisch geschlossen oder über alternative Hops geroutet
   - Bei Mesh-Wiederherstellung: Automatische Wiederverbindung

**User-Isolation und Verbindungsregeln:**

**1. Devices eines Users (gleicher User)**
- **Direkte Verbindung möglich**: Devices desselben Users können sich direkt verbinden
- **Yggdrasil kann Verbindung herstellen**: Yggdrasil kann Verbindungen zwischen Devices desselben Users automatisch herstellen
- **Keine Bestätigung nötig**: Da es eigene Devices sind, ist keine explizite Bestätigung erforderlich
- **Direkt oder über Yggdrasil**: Verbindung kann direkt (lokal) oder über Yggdrasil-Relay erfolgen

**2. Devices unterschiedlicher User (verschiedene User)**
- **NICHT direkt verbindbar**: Devices unterschiedlicher User dürfen sich NICHT direkt verbinden
- **Immer über Yggdrasil**: Alle Verbindungen zwischen verschiedenen Usern müssen über Yggdrasil erfolgen
- **Yggdrasil-Relay als Sicherheitsfeature**: Yggdrasil fungiert als Relay zwischen User-Meshes
  - Verhindert direkte Verbindungen zwischen verschiedenen Usern
  - Erzwingt zentrale Kontrolle über Cross-User-Kommunikation
  - Ermöglicht Security-Monitoring und Access-Control
- **Sicherheit**: Verhindert, dass Devices fremdgesteuert werden, wenn es nicht gewollt ist
- **Bezahlmaßnahmen**: Yggdrasil verwaltet auch Bezahlmaßnahmen für Cross-User-Verbindungen

**3. Ausnahme: Gleiches Edda-Netzwerk**
- **Bestätigung erforderlich**: Wenn beide User im gleichen Edda-Netzwerk sind, können sie sich verbinden, ABER es muss eine explizite Bestätigung geben, bevor die Devices sich verbinden dürfen
- **User-Bestätigung**: User muss explizit bestätigen, dass Verbindung erlaubt ist
- **Sicherheitsmaßnahme**: Verhindert ungewollte Verbindungen auch im gleichen Netzwerk

**Connection Establishment Workflow:**

1. **Device Discovery**
   - Device sucht nach anderen Devices im Netzwerk
   - Lokale Discovery (mDNS/Bonjour) - nur für Devices desselben Users
   - Globale Discovery über Yggdrasil - für alle Devices

2. **User-Verification (Heimdall/Yggdrasil)**
   - Heimdall prüft, ob beide Devices demselben User gehören
   - Yggdrasil prüft User-Identität für Cross-User-Verbindungen
   - Bei verschiedenen Usern: Verbindung muss über Yggdrasil erfolgen

3. **Connection-Initiation**
   - **Gleicher User**: Direkte Verbindung möglich, oder Yggdrasil kann Verbindung herstellen
   - **Verschiedene User**: Verbindung muss über Yggdrasil erfolgen
     - Device A sendet Bifrost-Message an Yggdrasil: "Möchte mich mit Device B verbinden"
     - Yggdrasil sendet Bifrost-Message an Device B: "Device A möchte sich verbinden"
     - Device B antwortet über Bifrost (Allow/Deny)
     - Bei Allow: Yggdrasil informiert Device A über Bifrost
   - **Gleiches Edda-Netzwerk**: Bestätigung erforderlich, dann Verbindung möglich

4. **Bifrost Connection Request**
   - Device A initiiert Bifrost-WebSocket-Verbindung (direkt oder über Yggdrasil-Relay)
   - Heimdall validiert Request (User-Identität, Permissions, Security-Policies)
   - Bei Allow: Verbindung wird etabliert

5. **TLS Handshake**
   - TLS-Verschlüsselung wird etabliert
   - Keys werden ausgetauscht
   - Verbindung ist verschlüsselt

6. **Connection Established**
   - WebSocket-Verbindung ist aktiv
   - Messages können gesendet werden
   - Heartbeat wird regelmäßig gesendet
   - Yggdrasil sendet Bifrost-Message: "Connection Established" (bei Yggdrasil-Relay)

### Message Routing

1. **Message empfangen**
   - Device empfängt Message
   - Message wird validiert
   - Routing-Entscheidung wird getroffen

2. **Routing**
   - Direkte Verbindung: Message wird direkt gesendet
   - Relay: Message wird über Asgard/Yggdrasil geroutet
     - **Yggdrasil als Relay**: Yggdrasil hält Bifrost-Verbindungen zu Devices und routet Messages
     - **Persistente Verbindungen**: Yggdrasil hat persistente Bifrost-WebSocket-Verbindungen zu allen registrierten Devices
   - Broadcast: Message wird an alle Devices gesendet

3. **Message Delivery**
   - Message wird an Ziel-Device gesendet
   - Bestätigung wird erwartet
   - Bei Fehler: Retry oder Fallback

## Abhängigkeiten

### Keine Core Library

- **WICHTIG**: Es gibt keine Edda Core Library
- **Separate Projekte**: Wenn gemeinsame Komponenten benötigt werden (DTOs, Protocols wie Bifrost Protocol, Utils), sollte ein separates Projekt erstellt werden
- **Selektive Nutzung**: Dies hält Apps klein, da genau gewählt werden kann, was benötigt wird
- **Keine Abhängigkeit**: Bifrost sollte nicht auf Dateien/Protocols/Utils aus dem `edda` Verzeichnis verweisen (KEIN PROJEKT - nur Metadaten-Sammlung)

### Technische Abhängigkeiten

- **Heimdall**: Für Connection Validation und Security
- **Network Stack**: Für WebSocket und TLS
- **Security Libraries**: Für Verschlüsselung
- **WebSocket Library**: `tokio-tungstenite` für WebSocket-Implementierung
- **TLS Library**: `rustls` oder `native-tls` für TLS 1.3

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

### Bifrost-spezifische Settings

**Settings-Inhalt (wird während Implementierung definiert)**
- Connection-Einstellungen
- Message-Routing-Einstellungen
- Security-Einstellungen

## Integration

- **Odin**: Koordiniert Bifrost für Inter-Device Communication
- **Heimdall**: Validiert alle Connections über Bifrost
  - **Connection-Validation**: Heimdall validiert alle Bifrost-Verbindungen (siehe unten "Connection-Validation-Workflow")
  - **Validation-Caching**: Optional für Performance (z.B. 5 Minuten)
  - **Heimdall-Ausfälle**: Fail-Safe: Verbindungen blockieren wenn Heimdall nicht verfügbar
- **Asgard**: Kann als Relay fungieren
- **Yggdrasil**: 
  - **Bifrost-Relay**: Yggdrasil baut Bifrost-WebSocket-Verbindungen zu Devices auf
  - **Persistente Verbindungen**: Yggdrasil hat persistente Bifrost-Verbindungen zu allen registrierten Devices
  - **Message-Routing**: Yggdrasil routet Messages zwischen Devices über Bifrost
  - **Globale Discovery**: Yggdrasil unterstützt globale Device-Discovery
  - **Event-Notifications**: Alle Events werden über Bifrost-Messages gesendet
  - **Yggdrasil-Ausfälle**: Fallback zu alternativer Route (Asgard) oder lokale Verbindungen
- **Alle Devices**: Midgard, Alfheim, Asgard verwenden Bifrost für Kommunikation

## Datenschutz

### Datenschutz-Features
- **End-to-End Encryption**: Optional End-to-End Encryption für Messages
- **Encryption-Keys**: Encryption-Keys werden über sichere Key-Exchange-Protokolle verwaltet (siehe heimdall/README.md Encryption)
- **Minimale Datensammlung**: Nur notwendige Daten werden übertragen
- **Keine Tracking-Daten**: Keine User-Tracking oder Analytics-Daten
- **Message-Privacy**: Messages werden verschlüsselt übertragen
- **User Control**: User hat Kontrolle über Message-Übertragung

### Compliance
- **GDPR-konform**: Unterstützung für GDPR-Anforderungen
- **Data Minimization**: Nur notwendige Daten werden übertragen
- **Right to Deletion**: User kann alle Message-Daten löschen
- **Transparency**: User wird über Message-Verarbeitung informiert

## Sicherheit

### Security-Features
- **TLS 1.3 Encryption**: Verschlüsselte Verbindungen für alle Connections
- **Connection Authentication**: Authentifizierung aller Connections über Heimdall
- **Message Validation**: Validierung aller Messages
- **Threat Detection**: Erkennung von verdächtigen Aktivitäten
  - **Anomaly Detection**: Heimdall erkennt verdächtige Aktivitäten (Anomaly Detection, Pattern Recognition)
  - **Rate-Limiting**: Rate-Limiting wird von Heimdall bereitgestellt
  - **Intrusion Detection**: Intrusion Detection durch Heimdall
- **Connection Blocking**: Blockierung von nicht-autorisierten Verbindungen
- **Secure Key Exchange**: Sichere Key-Exchange-Protokolle
- **Audit Logging**: Logging aller Connection-Events für Security-Audits

### Security-Best-Practices
- **Keine Hardcoded Secrets**: Keine Hardcoded Passwords oder Keys
- **Secure Defaults**: Sichere Standard-Konfigurationen
- **Regular Updates**: Automatische Security-Updates
- **Vulnerability Scanning**: Regelmäßiges Scanning für bekannte Vulnerabilities
- **Certificate Validation**: Validierung von TLS-Zertifikaten für alle WAN-Connections
- **Firewall Integration**: Integration mit Firewall-Regeln für WAN-Connections
- **Intrusion Detection**: Erkennung von Angriffen und verdächtigen Aktivitäten
- **Message Sanitization**: Sanitization von Messages zum Schutz vor Injection-Angriffen

## Protocol Design

### Connection/Authentication Protocol

**Basis**
- **Transport**: TLS 1.3 (sicherer Transport)
- **Message Format**: JSON mit digitalen Signaturen
- **Architecture**: Challenge-Response, Token-basiert
- **Zweck**: Sichere Device-Registrierung und Authentifizierung

**Protokoll-Details:**

**1. Challenge-Response-Mechanismus**

**Challenge-Request (Device → Heimdall):**
```json
{
  "type": "CHALLENGE_REQUEST",
  "device_id": "device-uuid",
  "public_key": "base64-encoded-public-key",
  "timestamp": 1234567890,
  "signature": "digital-signature-of-request"
}
```

**Challenge-Response (Heimdall → Device):**
```json
{
  "type": "CHALLENGE_RESPONSE",
  "challenge": "random-challenge-string",
  "timestamp": 1234567890,
  "expires_in": 300,
  "signature": "digital-signature-of-response"
}
```

**Challenge-Proof (Device → Heimdall):**
```json
{
  "type": "CHALLENGE_PROOF",
  "device_id": "device-uuid",
  "challenge": "challenge-from-response",
  "proof": "signed-challenge-with-private-key",
  "timestamp": 1234567890,
  "signature": "digital-signature-of-proof"
}
```

**Authentication-Result (Heimdall → Device):**
```json
{
  "type": "AUTHENTICATION_RESULT",
  "status": "SUCCESS",
  "token": "heimdall-token",
  "expires_in": 86400,
  "refresh_token": "refresh-token",
  "timestamp": 1234567890,
  "signature": "digital-signature-of-result"
}
```

**2. Public/Private Key-Generierung und -Speicherung**

- **Key-Generierung**: Beim ersten Start des Devices werden Public/Private Key-Pairs generiert (RSA 2048 oder Ed25519)
- **Key-Speicherung**: 
  - Private Keys werden verschlüsselt im Device-Secure-Storage gespeichert (OS-spezifisch: Keychain, Credential Manager, etc.)
  - Public Keys werden unverschlüsselt gespeichert und mit anderen Devices geteilt
- **Key-Rotation**: Wird von Heimdall verwaltet (siehe heimdall/README.md)
- **Key-Sharing**: Keys werden zwischen Devices über sichere Key-Exchange-Protokolle geteilt (Diffie-Hellman oder ähnlich)

**3. Gast-Netzwerk-Isolation (Technische Umsetzung)**

- **Separate Network-ID**: Jedes Gast-Netzwerk erhält eine eindeutige Network-ID
- **Network-Segmentation**: 
  - Gast-Devices werden in separate Network-Segments isoliert
  - Routing-Regeln verhindern direkten Zugriff auf Hauptnetzwerk
- **Firewall-Regeln**: 
  - Standard: Alle Verbindungen zum Hauptnetzwerk blockiert
  - Explizite Erlaubnis: Nur bei expliziter User-Bestätigung werden Firewall-Regeln geändert
- **VLAN-ähnliche Isolation**: Gast-Devices können nur mit anderen Gast-Devices und eigenen Geräten kommunizieren

**4. Datentransfer-Erlaubnis-Mechanismus**

**Erlaubnis-Anfrage (Gast-Device → Heimdall):**
```json
{
  "type": "DATA_TRANSFER_REQUEST",
  "guest_device_id": "guest-device-uuid",
  "target_resource": "resource-path",
  "transfer_type": "READ|WRITE|EXECUTE",
  "timestamp": 1234567890,
  "signature": "digital-signature"
}
```

**User-Bestätigung (User → Heimdall):**
- User wird über Frontend benachrichtigt
- User kann Erlaubnis erteilen oder verweigern
- Erlaubnis kann pro Device, pro Session oder pro Resource erteilt werden

**Erlaubnis-Response (Heimdall → Gast-Device):**
```json
{
  "type": "DATA_TRANSFER_RESPONSE",
  "status": "ALLOWED|DENIED",
  "permission_token": "permission-token",
  "expires_in": 3600,
  "scope": "resource-path",
  "timestamp": 1234567890,
  "signature": "digital-signature"
}
```

**5. Mehrfache Bestätigung für expliziten Zugang**

- **Bestätigungs-Count**: User muss 2-3 Mal bestätigen (konfigurierbar)
- **Bestätigungs-Intervall**: Zwischen Bestätigungen muss mindestens 5 Sekunden vergehen
- **Warnung**: Klare Warnung über Sicherheitsrisiken wird angezeigt
- **Audit-Log**: Alle Bestätigungen werden protokolliert
- **Timeout**: Zugang läuft automatisch ab (z.B. nach 24h)

**Design Goals**
- **Sicherheit**: Verhindert unbefugte Device-Registrierung
- **Authentifizierung**: Verifizierung von Device-Identities
- **Autorisierung**: Prüfung, ob Device sich verbinden darf
- **Token-basiert**: Nach Authentifizierung werden Tokens für weitere Kommunikation verwendet

**Initial Setup & First-Time Registration**

**Device kann autonom starten**
- **Autonome Funktionalität**: Jedes Device (außer Jotunheim) kann ohne Yggdrasil-Account starten und autonom funktionieren
- **Download erfordert Anmeldung**: Um Software herunterzuladen, muss User sich auf Website anmelden
- **Key-Generierung**: Public/Private Keys werden auf dem Device selbst beim ersten Start generiert
- **Key-Rotation**: Key Rotation wird von Heimdall verwaltet (siehe heimdall/README.md Encryption-Section)
- **Key-Sharing**: Keys werden zwischen Devices über sichere Key-Exchange-Protokolle geteilt (siehe heimdall/README.md Encryption-Section)
- **Lokale Registrierung**: Device kann sich bei anderen Devices oder Servern (Asgard) registrieren, Yggdrasil ist nicht immer beteiligt

**Automatische Registrierung im eigenen Netzwerk**
- **User-Credentials**: Wenn User-Credentials übereinstimmen, kann neues Device automatisch im eigenen Netzwerk registriert werden
- **Keine manuelle Bestätigung**: Bei gleichen Credentials ist keine manuelle Bestätigung erforderlich
- **User-Credentials-Validierung**: Über Heimdall (siehe Connection Establishment Workflow)
- **Sicherheits-Mechanismen**: Heimdall validiert alle Connections und prüft User-Identität, Permissions und Security-Policies
- **Token-Speicherung**: Device speichert Token, muss sich nur außerhalb des eigenen Netzwerks wieder anmelden
- **Token-Management**: Token-basierte Authentication über Heimdall. Proaktive Token-Erneuerung wird von Heimdall verwaltet

**Gast-Netzwerk für fremde Devices**
- **Automatisches Gast-Netzwerk**: Für fremde Devices wird automatisch ein Gast-Netzwerk erstellt
- **Eigene Credentials**: Gast-Device nutzt seine eigenen User-Credentials
- **Kommunikation**: Gast kann von seinem Gast-Netzwerk mit seinen eigenen Geräten im Heimnetz kommunizieren
- **Datentransfer**: Explizite Erlaubnis erforderlich für Datentransfer zwischen Gast- und Hauptnetzwerk (Default)
- **Expliziter Zugang**: Mehrfache Bestätigung erforderlich für expliziten Zugang zum eigenen Netzwerk (hohes Sicherheitsrisiko)

### Bifrost Protocol

**Basis**
- **Transport**: Secure WebSockets mit TLS 1.3
- **Message Format**: JSON mit Encryption
  - **Message-Versionierung**: Implementiert für Backward-Compatibility (siehe unten)
  - **Chunking**: Für große Messages sollte Chunking implementiert werden
- **Architecture**: Client-Server, Peer-to-Peer Support
- **Voraussetzung**: Connection/Authentication Protocol muss erfolgreich abgeschlossen sein

**Protocol-Versionierung:**

**Version-Header in Messages:**
```json
{
  "version": "1.0",
  "type": "MESSAGE",
  "payload": { ... }
}
```

**Versionierung-Strategie:**
- **Semantic Versioning**: Major.Minor.Patch (z.B. "1.0.0")
- **Major-Version**: Breaking Changes (nicht kompatibel)
- **Minor-Version**: Neue Features (rückwärtskompatibel)
- **Patch-Version**: Bug-Fixes (rückwärtskompatibel)

**Backward-Compatibility:**
- **Minor-Updates**: Alte Clients können mit neuen Servern kommunizieren (neue Felder werden ignoriert)
- **Major-Updates**: Alte Clients müssen aktualisiert werden oder Fallback auf ältere Version
- **Version-Negotiation**: Client und Server handeln kompatible Version aus (höchste gemeinsame Version)

**Version-Negotiation (bei Connection-Establishment):**
```json
{
  "type": "VERSION_NEGOTIATION",
  "client_versions": ["1.0.0", "0.9.0"],
  "timestamp": 1234567890
}
```

**Response:**
```json
{
  "type": "VERSION_NEGOTIATION_RESPONSE",
  "selected_version": "1.0.0",
  "supported_versions": ["1.0.0", "0.9.0"],
  "timestamp": 1234567890
}
```

**Verwendung**
- **Für**: Midgard (Desktop), Alfheim (Mobile), Asgard (Homeserver)
- **Nicht für**: Jotunheim (IoT-Devices) - diese verwenden Loki Toolcalling Protocol (via Loki Service)

**Features**

**1. Device Discovery**
- **mDNS/Bonjour**: Automatische Device-Erkennung im lokalen Netzwerk via mDNS/Bonjour
- **Manual Discovery**: IP-basierte Verbindung
- **Service Registry**: Zentralisierte Device-Registry (optional)
- **Discovery-Konflikte**: Device-ID-basierte Auflösung
- **Discovery-Timeouts**: Timeout-Mechanismus für Discovery-Requests

**2. Connection Management**
- **Handshake**: Authentifizierung und Verschlüsselung
- **Heartbeat**: Keep-Alive Mechanism
  - **Heartbeat-Frequenz**: Konfigurierbar (typisch: 30-60 Sekunden)
  - **Fehlende Heartbeats**: Connection wird als getrennt betrachtet, automatische Wiederverbindung wird initiiert
  - **Adaptive Intervalle**: Optional basierend auf Connection-Quality
- **Reconnection**: Automatische Wiederverbindung bei Verbindungsabbruch
- **Connection Pooling**: Mehrere Verbindungen pro Device
- **Connection-Termination**: 
  - **Saubere Beendigung**: DISCONNECT Message-Type für graceful shutdown
  - **Hängende Connections**: Timeout-Mechanismus und automatische Bereinigung bei fehlenden Heartbeats

**3. Message Routing**
- **Direct Routing**: Direkte Device-to-Device Kommunikation wenn möglich
  - **Routing-Optimierung**: Connection-Quality-Monitoring für optimale Routing-Entscheidungen
  - **Routing-Fehler**: Retry mit Exponential Backoff, dann Fallback zu Relay
- **Relay Routing**: Über Server (Asgard/Yggdrasil)
  - **Relay-Auswahl**: Basierend auf Connection-Quality und Verfügbarkeit
  - **Relay-Ausfälle**: Fallback zu alternativer Route
- **Broadcast**: Broadcast-Nachrichten an alle Devices im Netzwerk
- **Multicast**: Multicast an Device-Gruppen
- **Broadcast-Storms**: Rate-Limiting und TTL-Mechanismen verhindern Broadcast-Storms

**4. Security**
- **TLS Encryption**: End-to-End Verschlüsselung
- **Authentication**: Device-Identity-Verification
- **Authorization**: Permission-basierte Zugriffskontrolle
- **Message Signing**: Digitale Signaturen für Message-Integrität
  - **Signatur-Validierung**: Alle Messages werden validiert
  - **Signatur-Fehler**: Messages werden verworfen, Security-Alert wird ausgelöst

## Connection-Validation-Workflow (Bifrost ↔ Heimdall)

### Detaillierter Validierungs-Workflow

**1. Connection-Request (Bifrost → Heimdall)**
- Bifrost sendet `ConnectionValidationRequest` an Heimdall
- Request enthält: `source_device_id`, `target_device_id`, `connection_type`, `timestamp`
- Request wird mit Device-Private-Key signiert

**2. Heimdall Validierung**
- **Device-Identity-Validierung**: 
  - Heimdall prüft Device-Identities beider Devices
  - Public-Key-Validierung für beide Devices
  - Device-Status-Prüfung (aktiv, gesperrt, etc.)
- **User-Identity-Prüfung**:
  - Heimdall prüft User-Identität beider Devices
  - **Gleicher User**: Direkte Verbindung möglich
  - **Verschiedene User**: Direkte Verbindung blockiert, muss über Yggdrasil
  - **Gleiches Edda-Netzwerk**: Prüft Netzwerk-Mitgliedschaft und Bestätigungs-Status
- **Permission-Check**:
  - Heimdall prüft Permissions für Connection-Typ
  - Prüft Security-Policies
  - Prüft User-Isolation-Regeln
- **Threat-Assessment**:
  - Prüft auf verdächtige Patterns
  - Prüft Rate-Limiting
  - Prüft Blacklist/Whitelist

**3. Validation-Response (Heimdall → Bifrost)**
- Heimdall sendet `ConnectionValidationResponse` zurück
- Response enthält: `status` (ALLOW/DENY), `reason`, `validation_token`, `expires_at`
- Response wird mit Heimdall-Private-Key signiert

**4. Connection-Establishment (bei ALLOW)**
- Bifrost verwendet `validation_token` für Connection
- Token wird bei jeder Message validiert (optional, für Performance kann gecacht werden)
- Connection wird etabliert

**5. Connection-Blocking (bei DENY)**
- Bifrost blockiert Connection sofort
- Security-Alert wird ausgelöst
- User wird benachrichtigt (optional)

### Connection-Status-Überwachung

**Kontinuierliche Überwachung:**
- **Heartbeat-Validierung**: Heimdall validiert regelmäßig Connection-Status via Heartbeats
- **Message-Monitoring**: Alle Messages werden auf verdächtige Aktivitäten geprüft
- **Anomalie-Erkennung**: Ungewöhnliche Patterns werden erkannt
- **Rate-Limiting-Monitoring**: Rate-Limiting-Verstöße werden erkannt

**Status-Updates:**
- **Connection-Status**: Heimdall trackt Connection-Status (ACTIVE, IDLE, SUSPICIOUS, BLOCKED)
- **Status-Änderungen**: Bei Status-Änderungen wird Bifrost benachrichtigt
- **Automatische Blockierung**: Bei erkannten Threats wird Connection automatisch blockiert

**Connection-Status-Types:**
- `ACTIVE`: Verbindung ist aktiv und validiert
- `IDLE`: Verbindung ist inaktiv (keine Messages für X Minuten)
- `SUSPICIOUS`: Verdächtige Aktivitäten erkannt, wird überwacht
- `BLOCKED`: Verbindung wurde blockiert (Security-Threat)

### Connection-Blocking-Mechanismus

**Blocking-Trigger:**
- **Security-Threat**: Erkannte Security-Threats (Brute-Force, Intrusion, etc.)
- **Permission-Verletzung**: Verletzung von Permissions oder Security-Policies
- **Rate-Limiting-Verstoß**: Überschreitung von Rate-Limits
- **Token-Invalidität**: Ungültiger oder abgelaufener Token
- **User-Isolation-Verletzung**: Verletzung von User-Isolation-Regeln

**Blocking-Prozess:**
1. **Sofortige Blockierung**: Connection wird sofort blockiert
2. **Token-Revocation**: Validation-Token wird widerrufen
3. **Security-Alert**: Security-Alert wird ausgelöst
4. **User-Benachrichtigung**: User wird benachrichtigt (optional)
5. **Audit-Log**: Blockierung wird in Audit-Log protokolliert

**Blocking-Dauer:**
- **Temporär**: Blockierung für X Minuten (z.B. 15 Minuten bei Rate-Limiting)
- **Permanent**: Blockierung bis manuelle Freigabe (bei schwerwiegenden Threats)
- **Automatische Freigabe**: Nach Timeout wird Blockierung automatisch aufgehoben (nur bei temporären Blockierungen)

**Unblocking:**
- **Automatisch**: Nach Timeout (nur bei temporären Blockierungen)
- **Manuell**: User kann Blockierung manuell aufheben (bei permanenten Blockierungen)
- **Re-Validation**: Nach Unblocking muss Connection neu validiert werden

## Error Recovery und Resilience

### Automatische Wiederverbindung

**Kombination: Sofortiger Versuch, dann Exponential Backoff**

**Sofortiger Reconnect-Versuch**
- **Sofort**: Bei Verbindungsabbruch wird sofort versucht, Verbindung wiederherzustellen
- **Erster Versuch**: Keine Wartezeit beim ersten Versuch
- **Schnelle Wiederherstellung**: Minimiert Unterbrechung

**Exponential Backoff**

**Backoff-Formel:**
```
wait_time = base_delay * (2 ^ retry_count) + jitter
```

- **Base-Delay**: 1 Sekunde
- **Retry-Count**: Anzahl der bisherigen Retry-Versuche
- **Jitter**: Zufälliger Wert (0-500ms) zur Vermeidung von Thundering-Herd-Problem
- **Max-Wait-Time**: 60 Sekunden

**Retry-Limits:**
- **Max-Retries**: Unbegrenzt (kontinuierliche Versuche)
- **Max-Wait-Time**: 60 Sekunden zwischen Versuchen
- **Gesamt-Timeout**: Kein Gesamt-Timeout (System versucht kontinuierlich)

**Retry-Strategie:**
- **Nach erstem Fehler**: Nach erstem fehlgeschlagenen Versuch beginnt Exponential Backoff
- **Wartezeit erhöht sich**: Wartezeit zwischen Versuchen erhöht sich exponentiell
- **Maximale Wartezeit**: Maximale Wartezeit (60 Sekunden)
- **Kontinuierliche Versuche**: System versucht kontinuierlich, Verbindung wiederherzustellen

**Retry-Beispiel:**
- Versuch 1: Sofort (0ms)
- Versuch 2: Nach 1s + jitter
- Versuch 3: Nach 2s + jitter
- Versuch 4: Nach 4s + jitter
- Versuch 5: Nach 8s + jitter
- Versuch 6+: Nach max. 60s + jitter (kappt bei 60s)

### Netzwerk-Fehlerbehandlung

**Kombination: Retry → Fallback → Fehler**

**1. Automatischer Retry mit Exponential Backoff**
- **Erster Versuch**: Sofortiger Retry bei Netzwerk-Fehler
- **Exponential Backoff**: Bei wiederholten Fehlern wird Wartezeit exponentiell erhöht
- **Maximale Retries**: Maximale Anzahl von Retries (z.B. 3-5 Versuche)
- **Timeout**: Retry-Versuche haben Timeout

**2. Sofortiger Fallback (nur platformübergreifend)**

**Fallback-Routing-Mechanismus:**

**Fallback-Hierarchie:**
1. **Direkte Verbindung**: Zuerst direkte Device-to-Device-Verbindung versuchen
2. **Asgard-Relay**: Falls direkte Verbindung fehlschlägt, Fallback zu Asgard-Relay
3. **Yggdrasil-Relay**: Falls Asgard-Relay fehlschlägt, Fallback zu Yggdrasil-Relay
4. **Fehlermeldung**: Falls alle Routen fehlschlagen, Fehlermeldung an User

**Fallback-Trigger:**
- **Retry-Limit erreicht**: Nach max. 5 Retry-Versuchen
- **Timeout**: Nach Gesamt-Timeout (5 Minuten)
- **Connection-Fehler**: Bei bestimmten Connection-Fehlern (z.B. Connection-Refused)

**Fallback-Routing-Prozess:**
1. **Route-Erkennung**: System erkennt, welche alternativen Routen verfügbar sind
2. **Route-Auswahl**: Beste verfügbare Route wird ausgewählt (basierend auf Latency, Verfügbarkeit)
3. **Connection-Establishment**: Verbindung wird über alternative Route etabliert
4. **Message-Routing**: Messages werden über alternative Route geroutet

**Netzwerkplan für Service-Discovery:**
- **Netzwerkplan-Erstellung**: Odin erstellt Netzwerkplan (on the fly, mit Cache)
- **Service-Discovery**: Netzwerkplan wird für Service-Discovery verwendet
- **Yggdrasil-Übertragung**: Falls mit Yggdrasil verbunden: Netzwerkplan an Yggdrasil übertragen
- **WICHTIG**: Netzwerkplan darf unter keinen Umständen anderen Usern zugänglich gemacht werden
- **Asgard**: Asgard fungiert wie eine weitere Platform (Server-optimiert), ähnlich wie Midgard (Desktop-optimiert) und Alfheim (Mobile-optimiert)

**3. Fehlermeldung**
- **User-Benachrichtigung**: Falls alle Versuche fehlschlagen, Fehlermeldung an User
- **Error-Logging**: Alle Fehler werden geloggt für Debugging
- **Retry-Later**: User kann später erneut versuchen

## Monitoring & Logging

### Strukturiertes Logging

**Strukturiertes Logging:**
- Structured Logging mit strukturierten Daten
- Log Levels: Verschiedene Log-Level (DEBUG, INFO, WARN, ERROR, etc.)
- Context Tracking: Context wird mitgeloggt
- Log Rotation: Automatische Log-Rotation
- Umfassendes Logging für Debugging und Monitoring

### Performance-Monitoring

**Performance-Monitoring:**
- Performance-Metriken: Response-Zeiten, Durchsatz, Connection-Quality
- Connection-Quality-Monitoring: Überwachung der Connection-Quality für optimale Routing-Entscheidungen
- Quality-basiertes Routing: Quality-basierte Routing-Entscheidungen werden verwendet
- Performance-Tracking für alle Connections
- Kontinuierliche Überwachung und Performance-Optimierung
- Alerts bei Performance-Problemen

## Testing Requirements

### Device Discovery Tests
- Device Discovery im lokalen Netzwerk (mDNS/Bonjour)
- Globale Device Discovery über Yggdrasil
- Device Identity Validation
- Discovery Timeout Handling

### Connection Establishment Tests
- Lokale Verbindungsaufbau (ohne Yggdrasil)
- Globale Verbindungsaufbau (über Yggdrasil)
- TLS Handshake Tests
- Connection Authentication Tests
- Connection Rejection Tests

### Cross-Device Action Tests
- gRPC-Verbindung über Bifrost
- ThorAction Routing Tests
- ThorResult Response Tests
- Streaming Tests für lange Actions
- Connection Reuse Tests

### Security Tests
- Device Identity Verification
- Token Validation Tests
- Permission Checking Tests
- Message Signing Tests
- Access Control Tests

### Error Handling Tests
- Network Error Recovery
- Connection Timeout Tests
- Retry Mechanism Tests
- Fallback Routing Tests
- Error Message Validation

### Phase 20 Test Suites (IMPLEMENTATION_PLAN)
Dedizierte Test-Dateien für Integration, Performance, Security und GDPR:

| Suite | Datei | Inhalt |
|-------|--------|--------|
| E2E Communication Workflows | `tests/e2e_communication_workflow_test.rs` | Discovery→Connection→Message-Exchange, Direct/Relay-Routing, gRPC over Bifrost |
| Error Recovery | `tests/error_recovery_test.rs` | Retry, Reconnection, Fallback-Routing |
| Performance Benchmarks | `tests/performance_benchmark_test.rs` | Routing-Latency, Throughput, Connection-Establishment |
| Security Test Suite | `tests/security_test_suite.rs` | WebSocket-Security, Unauthorized-Access-Prevention, Authentication, Message-Validation |
| GDPR Compliance | `tests/gdpr_compliance_test.rs` | Data-Minimization, Access-Control, Audit-Logging, Right-to-Erasure |

**Ausführen:** Alle Tests im Container: `docker compose -f docker-compose.test.yml run --rm bifrost-test` (führt `cargo test --release` aus). Einzeltest: `docker compose -f docker-compose.test.yml run --rm bifrost-test cargo test <test_name>`. Von `bifrost/` oder mit Pfad. Siehe auch `scripts/run-tests.sh` / `scripts/run-tests.ps1`. Beim ersten Lauf kann der Docker-Build länger dauern. **Verifikation:** Nach erfolgreicher Ausführung die Phase-20-Checkboxen „… ausführen und bestehen“ in [IMPLEMENTATION_PLAN](IMPLEMENTATION_PLAN.md) abhaken.

## NAT Traversal

### STUN/TURN/ICE
- **STUN/TURN/ICE**: Unterstützung für automatisches NAT-Traversal
- **Automatische NAT-Discovery**: Automatisches NAT-Traversal wird stark bevorzugt
- **TURN-Server**: Yggdrasil/Asgard können als TURN-Server fungieren
- **Fehlerbehandlung**: Fallback auf manuelle Port-Forwarding-Konfiguration wenn automatisch nicht möglich

### Port-Forwarding
- **Automatische Konfiguration**: Automatisches NAT-Traversal wird bevorzugt
- **Manuelle Konfiguration**: Als Fallback wenn automatisch nicht möglich
- **Router-Kompatibilität**: UPnP/IGD-Protokoll für automatische Port-Forwarding

## Implementierungs-Notizen

- **WebSocket Library**: `tokio-tungstenite` für WebSocket-Implementierung
- **TLS Library**: `rustls` oder `native-tls` für TLS 1.3
- **WebSocket-Compression**: Optional implementierbar
- Muss Secure WebSocket mit TLS unterstützen
- Muss automatische Wiederverbindung haben
- Sollte Relay-Funktionalität unterstützen
- Muss robustes Error-Handling haben
- Sollte Device-Discovery unterstützen
- Muss Message-Routing unterstützen
- Sollte Heartbeat-Mechanismus haben
- Muss Performance-optimiert sein
- Sollte Monitoring für Connection-Status haben
- **Muss Connection/Authentication Protocol implementieren**: Vor Bifrost-Protokoll
- **Muss Public/Private Key Management haben**: Für Device-Authentifizierung
- **Muss Token-Management haben**: Erstellung, Validierung, Erneuerung
- **Muss Rate-Limiting haben**: Verhindert Brute-Force-Attacks
- **Muss Retry-Mechanismus mit Exponential Backoff haben**: Für Netzwerk-Fehler
- **Muss Fallback-Mechanismen haben**: Alternative Routen bei Fehlern
- **Sollte mit lokalen Devices beginnen**: Kein Internet nötig für lokale Verbindungen
- **Sollte User-Feedback für Connection-Status haben**: User sollte Connection-Status sehen können
- **Muss Security von Anfang an implementieren**: Security ist nicht optional
- **Sollte Logging für Debugging haben**: Umfassendes Logging für Debugging
- **Muss robustes Error-Handling für Netzwerk-Fehler haben**: Umfassendes Error-Handling für alle Netzwerk-Szenarien (WAN und LAN)
- **Sollte verschiedene NAT-Traversal-Strategien unterstützen**: STUN, TURN, ICE für WAN-Connections
- **Muss Connection-Quality-Monitoring haben**: Überwachung der Connection-Quality für optimale Routing-Entscheidungen
- **Sollte automatisches Failover haben**: Automatisches Failover bei Verbindungsausfall
- **Muss Security-Best-Practices für WAN-Connections folgen**: TLS, Certificate Validation, Firewall Integration, Intrusion Detection
- **Performance**: Muss optimiert sein für schnelle Message-Übertragung und niedrige Latenz
- **Message-Batching**: Optional für Performance-Optimierung
- **Quality-Degradation**: Automatisches Failover zu besserer Route bei Quality-Degradation
- **Datenschutz**: Muss Privacy-by-Design implementieren und Message-Privacy gewährleisten
- **Sicherheit**: Muss robuste Security-Mechanismen haben, insbesondere für verschlüsselte Kommunikation
- **Heimdall-Ausfälle**: Fail-Safe: Verbindungen blockieren wenn Heimdall nicht verfügbar
- **Gast-Netzwerk-Cleanup**: Automatisches Cleanup nach Verbindungsabbruch oder Timeout


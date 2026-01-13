# Heimdall - Security Service

## Übersicht

Heimdall ist der Security Service, der den Transport über Bifrost-Verbindungen überwacht und blockiert, falls nicht erlaubt. Er ist verantwortlich für Authentication, Authorization und Security-Monitoring.

## Verantwortlichkeiten

### 1. Authentication
- **Device Authentication**: Verifizierung von Device-Identities
- **Token Management**: Erstellung und Validierung von Tokens (Heimdall-Tokens und Session-Tokens)
- **Session Management**: Verwaltung von Authentifizierungs-Sessions
- **Key Management**: Verwaltung von Cryptographic Keys
- **Automatisch**: Alles erfolgt automatisch, User wird so wenig wie möglich gestört

### 2. Authorization
- **Permission Checking**: Prüfung von Permissions für Actions
- **Access Control**: Zugriffskontrolle basierend auf Permissions
- **Resource Protection**: Schutz von Device-Ressourcen
- **Policy Enforcement**: Durchsetzung von Security-Policies

### 3. Bifrost Connection Validation
- **Connection Authentication**: Validierung von Bifrost-Verbindungen
- **Message Validation**: Validierung von Messages
- **Threat Detection**: Erkennung von verdächtigen Aktivitäten
- **Connection Blocking**: Blockierung von nicht-autorisierten Verbindungen

### 4. Security Monitoring
- **Audit Logging**: Logging aller Security-relevanten Events
- **Threat Detection**: Erkennung von Security-Threats
- **Incident Response**: Automatische Response auf Security-Incidents
  - **Automatische Response**: Bei Security-Incidents wird automatisch gehandelt (z.B. Connection-Blockierung, Token-Revocation)
  - **Response-Eskalation**: Bei schwerwiegenden Incidents wird Eskalation durchgeführt (z.B. User-Benachrichtigung, Admin-Alert)
  - **User-Kommunikation**: Incidents werden dem User transparent kommuniziert
- **Security Analytics**: Analyse von Security-Events
  - **Event-Analyse**: Security-Events werden analysiert für Trend-Erkennung
  - **Trend-Analyse**: Langfristige Trends werden identifiziert
  - **Metriken-Sammlung**: Security-Metriken werden kontinuierlich gesammelt (z.B. Anzahl von Angriffen, erfolgreiche Authentifizierungen)

## Service-Interfaces

### Inputs
- Authentication Requests
- Authorization Requests
- Connection Validation Requests
- Token Validation Requests

### Outputs
- Authentication Tokens
- Authorization Decisions
- Security Alerts
- Audit Logs

## Workflow

### Device Authentication

**Device-Identity-Verifizierung:**
- **Public/Private Key Pairs**: Jedes Device hat ein Public/Private Key Pair für Device-Identity
- **Device-Certificates**: Optional Device-Certificates für zusätzliche Sicherheit
- **Identity-Sharing**: Device-Identities werden zwischen Devices über sichere Key-Exchange-Protokolle geteilt
- **Identity-Verifizierung**: Heimdall verifiziert Device-Identity über Public-Key-Validierung

**Challenge-Response-Protokoll (Detailliert):**

1. **Device sendet Challenge-Request**
   - Device-Identity wird übermittelt (Public Key oder Certificate)
   - Request wird mit Private Key signiert
   - Request enthält: device_id, public_key, timestamp, signature

2. **Heimdall generiert Challenge**
   - Heimdall validiert Request-Signatur
   - Heimdall generiert zufälligen Challenge-String
   - Challenge wird mit Heimdall-Private-Key signiert
   - Challenge-Response enthält: challenge, timestamp, expires_in, signature

3. **Device beweist Identität (Challenge-Proof)**
   - Device signiert Challenge mit Private Key
   - Proof-Request enthält: device_id, challenge, proof (signed challenge), timestamp, signature
   - Device sendet Proof an Heimdall

4. **Heimdall validiert Proof und generiert Token**
   - Heimdall validiert Proof-Signatur mit Device-Public-Key
   - Heimdall prüft Challenge-Expiration
   - Heimdall prüft Device-Status und Permissions
   - Bei Erfolg: Heimdall erstellt `HeimdallToken`
   - **Token-Signing**: Token wird mit Heimdall-Private-Key signiert
   - **Token-Payload-Struktur**: Enthält Device-ID, User-ID, Expiration, Permissions, Signatur
   - Token wird an Device zurückgegeben

5. **Token Validation**
   - Device verwendet Token für Requests
   - Heimdall validiert Token bei jedem Request (Signatur, Expiration, Revocation-Liste)
   - Token-Expiration wird geprüft

**Token-Struktur:**
```json
{
  "device_id": "device-uuid",
  "user_id": "user-uuid",
  "issued_at": 1234567890,
  "expires_at": 1234654290,
  "permissions": ["read", "write"],
  "signature": "token-signature"
}
```

### Session-Management

**Session-Verwaltung:**
- **Session-Tracking**: Heimdall trackt aktive Sessions pro Device
- **Session-Timeouts**: Sessions haben Timeouts (z.B. 1 Stunde Inaktivität)
- **Session-Hijacking-Schutz**: 
  - **Device-Tracking**: Sessions sind an Device-Identity gebunden
  - **Token-Binding**: Tokens sind an Device gebunden
  - **Anomalie-Erkennung**: Ungewöhnliche Session-Aktivitäten werden erkannt
  - **Automatische Revocation**: Bei erkanntem Hijacking wird Session automatisch beendet

### Authorization

**Permission-System:**
- **Granulare Permissions**: Permission-System unterstützt granulare Permissions (z.B. read, write, execute pro Resource)
- **Permission-Struktur**: Hierarchische Permission-Struktur (z.B. device.*, device.file.read)
- **Permission-Synchronisation**: Permissions werden zwischen Devices über Yggdrasil oder lokale Synchronisation geteilt

**Role-Based Access Control (RBAC):**
- **Vordefinierte Rollen**: Basis-Rollen (Admin, User, Guest) + Custom-Rollen
- **Rollen-Synchronisation**: Rollen werden zwischen Devices synchronisiert (über Yggdrasil oder lokal)
- **Rollen-Hierarchie**: Rollen können hierarchisch strukturiert sein

**Resource-Protection:**
- **Resource-basierte Permissions**: Permissions können pro Resource definiert werden
- **Resource-Konflikte**: Bei Resource-Konflikten wird Priorität + Locking verwendet (siehe Thor)

1. **Action Request kommt an**
   - Device sendet Action-Request mit Token
   - Heimdall extrahiert Token

2. **Permission Check**
   - Heimdall prüft Token (Validität, Expiration, Revocation)
   - Heimdall prüft Permissions für Resource (granulare Permissions)
   - Heimdall prüft Rollen (RBAC)
   - Heimdall prüft Conditions (z.B. Zeit-basierte Bedingungen)

3. **Authorization Decision**
   - Heimdall entscheidet: Allow oder Deny
   - Bei Allow: Request wird weitergeleitet
   - Bei Deny: Request wird blockiert, Error wird zurückgegeben

### Bifrost Connection Validation

**User-Isolation und Verbindungsregeln:**

**1. Devices eines Users (gleicher User)**
- **Direkte Verbindung erlaubt**: Heimdall erlaubt direkte Verbindungen zwischen Devices desselben Users
- **Automatische Validierung**: Heimdall validiert, dass beide Devices demselben User gehören
- **Keine explizite Bestätigung**: Da es eigene Devices sind, ist keine explizite Bestätigung erforderlich

**2. Devices unterschiedlicher User (verschiedene User)**
- **NICHT direkt verbindbar**: Heimdall blockiert direkte Verbindungen zwischen Devices unterschiedlicher User
- **Immer über Yggdrasil**: Alle Verbindungen zwischen verschiedenen Usern müssen über Yggdrasil erfolgen
- **Sicherheit**: Verhindert, dass Devices fremdgesteuert werden, wenn es nicht gewollt ist
- **User-Verification**: Heimdall prüft User-Identität für alle Verbindungsanfragen

**3. Ausnahme: Gleiches Edda-Netzwerk**
- **Bestätigung erforderlich**: Wenn beide User im gleichen Edda-Netzwerk sind, kann Heimdall Verbindung erlauben, ABER nur nach expliziter User-Bestätigung
- **User-Bestätigung**: User muss explizit bestätigen, dass Verbindung erlaubt ist
- **Sicherheitsmaßnahme**: Verhindert ungewollte Verbindungen auch im gleichen Netzwerk

**Connection Validation Workflow:**

1. **Connection Request**
   - Device A möchte sich mit Device B verbinden
   - Bifrost sendet Connection-Request an Heimdall

2. **User-Verification**
   - Heimdall prüft User-Identität beider Devices
   - **Gleicher User**: Verbindung kann direkt erfolgen (nach Device-Identity-Validierung)
   - **Verschiedene User**: Verbindung muss über Yggdrasil erfolgen - Heimdall blockiert direkte Verbindung
   - **Gleiches Edda-Netzwerk**: Prüft, ob beide User im gleichen Netzwerk sind und ob Bestätigung vorliegt

3. **Connection Validation**
   - Heimdall prüft Device-Identities
   - Heimdall prüft Permissions
   - Heimdall prüft Security-Policies
   - Heimdall prüft User-Isolation-Regeln

4. **Connection Decision**
   - Bei Allow: Connection wird erlaubt
   - Bei Deny: Connection wird blockiert (z.B. bei direkter Verbindung zwischen verschiedenen Usern)
   - Heimdall überwacht Connection

5. **Ongoing Monitoring**
   - Heimdall überwacht Connection kontinuierlich
   - Verdächtige Aktivitäten werden erkannt (Anomalie-Erkennung, Pattern-Recognition)
   - Connection kann bei Bedarf beendet werden

**Detaillierter Workflow (siehe auch `bifrost/README.md` Connection-Validation-Workflow):**

**Connection-Validation-Request (von Bifrost):**
```json
{
  "type": "CONNECTION_VALIDATION_REQUEST",
  "source_device_id": "device-a-uuid",
  "target_device_id": "device-b-uuid",
  "connection_type": "DIRECT|RELAY",
  "timestamp": 1234567890,
  "signature": "request-signature"
}
```

**Connection-Validation-Response (an Bifrost):**
```json
{
  "type": "CONNECTION_VALIDATION_RESPONSE",
  "status": "ALLOW|DENY",
  "reason": "reason-string",
  "validation_token": "validation-token",
  "expires_at": 1234654290,
  "timestamp": 1234567890,
  "signature": "response-signature"
}
```

**Connection-Status-Überwachung:**
- **Status-Tracking**: Heimdall trackt Connection-Status (ACTIVE, IDLE, SUSPICIOUS, BLOCKED)
- **Heartbeat-Validierung**: Regelmäßige Validierung via Heartbeats
- **Message-Monitoring**: Kontinuierliche Überwachung aller Messages
- **Status-Updates**: Bifrost wird bei Status-Änderungen benachrichtigt

**Connection-Blocking:**
- **Blocking-Trigger**: Security-Threats, Permission-Verletzungen, Rate-Limiting-Verstöße
- **Sofortige Blockierung**: Connection wird sofort blockiert, Token wird widerrufen
- **Blocking-Dauer**: Temporär (X Minuten) oder permanent (bis manuelle Freigabe)
- **Unblocking**: Automatisch nach Timeout oder manuell durch User

**Connection-Validation-Details:**
- **Validation-Caching**: Connection-Validierungen werden gecacht für bessere Performance (z.B. 5 Minuten)
- **Cache-Invalidierung**: Cache wird bei wichtigen Änderungen (z.B. Permission-Änderungen) invalidiert
- **Validation-Fehler**: Bei Validation-Fehlern wird Connection blockiert, Security-Alert wird ausgelöst

**Message-Validation:**
- **Message-Signatur-Prüfung**: Alle Messages werden auf Signatur geprüft
- **Ungültige Messages**: Messages mit ungültiger Signatur werden verworfen, Security-Alert wird ausgelöst

**Threat-Detection:**
- **Pattern-Recognition**: Heimdall erkennt verdächtige Patterns (z.B. Brute-Force-Attacks, Rate-Limiting-Verstöße)
- **Bei erkannten Threats**: 
  - Connection wird blockiert
  - Device wird temporär gesperrt
  - Security-Alert wird ausgelöst
  - User wird benachrichtigt

**Connection-Validation-Details:**
- **Validation-Caching**: Connection-Validierungen werden gecacht für bessere Performance (z.B. 5 Minuten)
- **Cache-Invalidierung**: Cache wird bei wichtigen Änderungen (z.B. Permission-Änderungen) invalidiert
- **Validation-Fehler**: Bei Validation-Fehlern wird Connection blockiert, Security-Alert wird ausgelöst

**Message-Validation:**
- **Message-Signatur-Prüfung**: Alle Messages werden auf Signatur geprüft
- **Ungültige Messages**: Messages mit ungültiger Signatur werden verworfen, Security-Alert wird ausgelöst

**Threat-Detection:**
- **Pattern-Recognition**: Heimdall erkennt verdächtige Patterns (z.B. Brute-Force-Attacks, Rate-Limiting-Verstöße)
- **Bei erkannten Threats**: 
  - Connection wird blockiert
  - Device wird temporär gesperrt
  - Security-Alert wird ausgelöst
  - User wird benachrichtigt

## Security Features

### Authentication
- Public/Private Key Pairs
- Digital Signatures
- Token-based Authentication
- Multi-factor Authentication (optional)

### Authorization

**Role-Based Access Control (RBAC):**

**Rollen-Struktur:**
- **Basis-Rollen**: 
  - `admin`: Vollzugriff auf alle Ressourcen
  - `user`: Standard-User-Zugriff
  - `guest`: Eingeschränkter Zugriff (nur Lesen)
- **Custom-Rollen**: User können eigene Rollen definieren
- **Rollen-Hierarchie**: Rollen können hierarchisch strukturiert sein (z.B. `admin` > `user` > `guest`)
- **Rollen-Vererbung**: Untergeordnete Rollen erben Permissions von übergeordneten Rollen

**RBAC-Workflow:**
1. **Rollen-Zuweisung**: User/Device wird einer oder mehreren Rollen zugewiesen
2. **Permission-Mapping**: Rollen haben zugewiesene Permissions
3. **Authorization-Check**: Bei Request wird geprüft:
   - Welche Rollen hat User/Device?
   - Welche Permissions haben diese Rollen?
   - Reicht das für die angeforderte Action?
4. **Decision**: Allow oder Deny basierend auf Rollen-Permissions

**Permission-based Access Control:**

**Granulare Permissions:**
- **Permission-Struktur**: Hierarchische Permission-Struktur (z.B. `device.*`, `device.file.read`, `device.file.write`)
- **Resource-basierte Permissions**: Permissions können pro Resource definiert werden
- **Action-basierte Permissions**: Permissions können pro Action definiert werden (read, write, execute, delete)
- **Wildcard-Permissions**: Wildcard-Permissions für flexible Zugriffskontrolle (z.B. `device.file.*`)

**Permission-Check-Workflow:**
1. **Request-Analyse**: Heimdall analysiert Request (Resource, Action, Context)
2. **Permission-Lookup**: Heimdall sucht relevante Permissions für Resource/Action
3. **Permission-Validierung**: Heimdall prüft, ob User/Device die benötigten Permissions hat
4. **Decision**: Allow oder Deny basierend auf Permissions

**Conditional Permissions:**
- **Zeit-basierte Bedingungen**: Permissions können zeit-basiert sein (z.B. nur während Arbeitszeiten)
- **Context-basierte Bedingungen**: Permissions können context-basiert sein (z.B. nur auf bestimmten Devices)
- **IP-basierte Bedingungen**: Permissions können IP-basiert sein (z.B. nur von bestimmten IPs)
- **Bedingungs-Evaluierung**: Bedingungen werden bei jedem Permission-Check evaluiert

**OAuth-Integration (für Yggdrasil):**

**OAuth 2.0 Flow:**
- **Authorization Code Flow**: Standard OAuth 2.0 Authorization Code Flow
- **OAuth-Provider**: Unterstützung für gängige OAuth-Provider (Google, GitHub, etc.)
- **Token-Exchange**: OAuth-Token wird gegen Heimdall-Token getauscht
- **Token-Validierung**: OAuth-Token wird validiert (Signatur, Expiration, etc.)

**OAuth-Workflow:**
1. **User startet OAuth-Login**: User wird zu OAuth-Provider weitergeleitet
2. **OAuth-Authorization**: User autorisiert App bei OAuth-Provider
3. **Authorization Code**: OAuth-Provider gibt Authorization Code zurück
4. **Token-Exchange**: Heimdall tauscht Authorization Code gegen Access Token
5. **User-Info-Abfrage**: Heimdall fragt User-Info von OAuth-Provider ab
6. **Heimdall-Token-Generierung**: Heimdall generiert Heimdall-Token basierend auf OAuth-User-Info
7. **Token-Rückgabe**: Heimdall-Token wird an Device zurückgegeben

**Email/Code-Verifizierung:**

**Email-Verifizierung-Workflow:**
1. **Email-Eingabe**: User gibt Email-Adresse ein
2. **Verifizierungs-Code-Generierung**: Heimdall generiert zufälligen 6-stelligen Code
3. **Code-Versand**: Code wird per Email an User gesendet
4. **Code-Eingabe**: User gibt Code ein
5. **Code-Validierung**: Heimdall validiert Code (Code, Expiration, Rate-Limiting)
6. **Email-Verifizierung**: Bei erfolgreicher Validierung wird Email als verifiziert markiert
7. **Token-Generierung**: Heimdall generiert Token für verifizierten User

**Code-Sicherheit:**
- **Code-Generierung**: Kryptographisch sicherer Zufallszahlengenerator (CSPRNG)
- **Code-Expiration**: Codes laufen nach 10 Minuten ab
- **Rate-Limiting**: Max. 3 Versuche pro Email pro Stunde
- **Code-Invalidierung**: Codes werden nach Verwendung oder Expiration invalidiert
- **Brute-Force-Schutz**: Nach mehreren fehlgeschlagenen Versuchen wird Email temporär gesperrt

### Encryption

**Verschlüsselungsalgorithmen:**

**Asymmetrische Verschlüsselung (Public/Private Keys):**
- **Algorithmus**: Ed25519 (bevorzugt) oder RSA 2048 (Fallback)
- **Key-Generierung**: 
  - Ed25519: 256-bit Private Key, 256-bit Public Key
  - RSA: 2048-bit Key-Pair
- **Key-Format**: PEM-Format für Kompatibilität
- **Key-Storage**: Private Keys werden verschlüsselt im OS-spezifischen Secure Storage gespeichert (Keychain, Credential Manager)

**Symmetrische Verschlüsselung (für End-to-End Encryption):**
- **Algorithmus**: AES-256-GCM (Galois/Counter Mode)
- **Key-Derivation**: PBKDF2 mit SHA-256 (100.000 Iterationen) oder Argon2id
- **IV-Generierung**: Kryptographisch sicherer Zufallszahlengenerator (CSPRNG)

**TLS-Verschlüsselung:**
- **TLS-Version**: TLS 1.3 (mindestens)
- **Cipher Suites**: 
  - TLS_AES_256_GCM_SHA384 (bevorzugt)
  - TLS_CHACHA20_POLY1305_SHA256 (Fallback)
- **Certificate Validation**: Striktes Certificate-Validation (keine Self-Signed Certificates ohne explizite Konfiguration)

**Key-Generierung:**
- **Device-Keys**: Beim ersten Start des Devices werden Public/Private Key-Pairs generiert
- **Key-Generierung**: Kryptographisch sicherer Zufallszahlengenerator (CSPRNG)
- **Key-Validierung**: Keys werden auf Validität geprüft (Format, Größe, etc.)
- **Key-Backup**: Keys können optional gesichert werden (verschlüsselt)

**Key-Speicherung:**
- **Private Keys**: 
  - Verschlüsselt im OS-spezifischen Secure Storage (Keychain, Credential Manager, etc.)
  - Niemals im Klartext gespeichert
  - Zugriff nur für Heimdall-Service
- **Public Keys**: 
  - Unverschlüsselt gespeichert (können öffentlich sein)
  - Werden zwischen Devices geteilt

**Key-Rotation:**
- **Automatische Rotation**: Keys werden automatisch rotiert (z.B. alle 90 Tage)
- **Rotation-Trigger**: 
  - Zeit-basiert (alle X Tage)
  - Event-basiert (bei Security-Incidents, bei Key-Leak-Erkennung)
  - Manuell (User kann Rotation auslösen)
- **Rotation-Prozess**:
  1. Neues Key-Pair wird generiert
  2. Neuer Public Key wird an alle verbundenen Devices gesendet
  3. Alte Keys werden als "deprecated" markiert
  4. Nach Grace-Period (z.B. 7 Tage) werden alte Keys entfernt
- **Rollover-Mechanismus**: Während Grace-Period werden beide Keys akzeptiert (für nahtlose Rotation)

**Key-Exchange-Protokolle:**
- **Diffie-Hellman Key Exchange**: Für sicheren Key-Austausch zwischen Devices
- **ECDH (Elliptic Curve Diffie-Hellman)**: Mit Ed25519-Curve
- **Perfect Forward Secrecy**: Jede Session hat eigenen Session-Key

**End-to-End Encryption:**
- **Optional**: End-to-End Encryption für Messages (User kann aktivieren)
- **Session-Keys**: Jede Session hat eigenen Session-Key (Perfect Forward Secrecy)
- **Key-Austausch**: Session-Keys werden über sicheren Key-Exchange ausgetauscht
- **Message-Encryption**: Messages werden mit AES-256-GCM verschlüsselt

### Threat Detection
- Anomaly Detection
- Pattern Recognition
- Rate Limiting
- Intrusion Detection

## Abhängigkeiten

### Keine Core Library

- **WICHTIG**: Es gibt keine Edda Core Library
- **Separate Projekte**: Wenn gemeinsame Komponenten benötigt werden (DTOs wie HeimdallToken/HeimdallPermission, Protocols, Utils), sollte ein separates Projekt erstellt werden
- **Selektive Nutzung**: Dies hält Apps klein, da genau gewählt werden kann, was benötigt wird
- **Keine Abhängigkeit**: Heimdall sollte nicht auf Dateien/Protocols/Utils aus dem `edda` Verzeichnis verweisen (KEIN PROJEKT - nur Metadaten-Sammlung)

### Service-Abhängigkeiten

- **Odin**: Für Coordination
- **Bifrost**: Für Connection Validation
- **Device Registry**: Für Device-Information

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

### Heimdall-spezifische Settings

**Settings-Inhalt (wird während Implementierung definiert)**
- Security-Policy-Einstellungen
- Token-Konfiguration
- Permission-System-Einstellungen
- Session-Management-Einstellungen

## Integration

**Odin-Integration:**
- **Security-Checks**: Odin koordiniert Security-Checks über Heimdall
- **Caching**: Optionales Caching für Security-Checks für bessere Performance
- **Odin-Ausfälle**: Bei Odin-Ausfall können Devices direkt mit Heimdall kommunizieren (für kritische Security-Checks)

**Bifrost-Integration:**
- **Connection-Validation**: Bifrost validiert alle Connections über Heimdall
- **Validation-Streaming**: Validation-Requests können gestreamt werden für bessere Performance
- **Bifrost-Ausfälle**: Bei Bifrost-Ausfall werden lokale Security-Checks durchgeführt, WAN-Connections werden blockiert

**Weitere Integrationen:**
- **Thor**: Prüft Permissions für Actions über Heimdall
- **Alle Devices**: Verwenden Heimdall für Authentication/Authorization
- **Yggdrasil**: Zentrale Token-Verwaltung und User-Management

## Monitoring & Logging

### Strukturiertes Logging

**Strukturiertes Logging:**
- Structured Logging mit strukturierten Daten
- Log Levels: Verschiedene Log-Level (DEBUG, INFO, WARN, ERROR, etc.)
- Context Tracking: Context wird mitgeloggt
- Log Rotation: Automatische Log-Rotation
- Umfassendes Logging für Debugging und Monitoring
- Audit-Logs für Security-Audits

### Performance-Monitoring

**Performance-Monitoring:**
- Performance-Metriken: Response-Zeiten, Durchsatz, Resource-Usage
- Performance-Tracking für alle Authentication/Authorization-Requests
- Kontinuierliche Überwachung und Performance-Optimierung
- Alerts bei Performance-Problemen

## Performance

### Performance-Optimierungen
- **Schnelle Token-Validierung**: Optimierte Token-Validierung für minimale Latenz
- **Caching**: Intelligentes Caching für Token-Validierungen und Permissions
  - **Token-Validierung-Caching**: Token-Validierungen werden gecacht (z.B. 5 Minuten)
  - **Cache-Invalidierung**: Cache wird bei Revocation oder wichtigen Änderungen invalidiert
  - **Validation-Load**: Bei hoher Last wird Caching verstärkt
- **Permission-Check-Caching**: 
  - **Permission-Caching**: Permission-Checks werden gecacht für bessere Performance
  - **Cache-Invalidierung**: Cache wird bei Permission-Änderungen invalidiert
  - **Permission-Check-Load**: Bei hoher Last wird Caching verstärkt
- **Parallel Processing**: Parallele Verarbeitung von mehreren Security-Checks
- **Connection Pooling**: Effizientes Connection-Pooling für Database-Zugriffe
- **Optimierte Algorithms**: Optimierte Security-Algorithmen für schnelle Checks

### Performance-Metriken
- Schnelle Token-Validierung (< 10ms für Standard-Tokens)
- Niedrige Latenz für Permission-Checks (< 5ms)
- Hoher Durchsatz für parallele Security-Checks

## Datenschutz

### Datenschutz-Features
- **Minimale Datensammlung**: Nur notwendige Daten werden für Authentication/Authorization gespeichert
- **Keine Tracking-Daten**: Keine User-Tracking oder Analytics-Daten
- **Token-Privacy**: Tokens enthalten nur notwendige Informationen
- **User Control**: User hat Kontrolle über Authentication-Daten
- **Data Minimization**: Nur notwendige Daten werden verarbeitet

### Compliance
- **GDPR-konform**: Vollständige Einhaltung der GDPR-Anforderungen
- **Data Minimization**: Nur notwendige Daten werden gespeichert
- **Right to Deletion**: User kann alle Authentication-Daten löschen
- **Transparency**: User wird über Security-Verarbeitung informiert
- **Audit Logging**: Vollständiges Audit-Logging für Compliance

## Sicherheit

### Security-Features
- **Public/Private Key Pairs**: Sichere Key-Pairs für Authentication
- **Digital Signatures**: Digitale Signaturen für Message-Validierung
- **Token-based Authentication**: Sichere Token-basierte Authentication
- **Multi-factor Authentication**: Optional Multi-Factor Authentication
- **Role-Based Access Control**: Granulares RBAC-System
- **TLS 1.3 Encryption**: Verschlüsselte Verbindungen
- **End-to-End Encryption**: Optional End-to-End Encryption
- **Key Rotation**: Automatische Key-Rotation
- **Threat Detection**: Anomaly Detection und Pattern Recognition
- **Rate Limiting**: Rate Limiting zum Schutz vor Brute-Force-Angriffen
- **Intrusion Detection**: Erkennung von Intrusion-Versuchen

### Security-Best-Practices
- **Keine Hardcoded Secrets**: Keine Hardcoded Passwords oder Keys
- **Secure Defaults**: Sichere Standard-Konfigurationen
- **Regular Updates**: Automatische Security-Updates
- **Vulnerability Scanning**: Kontinuierliches Scanning für bekannte Vulnerabilities
- **Penetration Testing**: Regelmäßige Penetration Tests
- **Incident Response**: Automatische Response auf Security-Incidents
- **Fail-Safe**: Bei Fehlern wird sicherheitshalber verweigert (Deny statt Allow)

## Guest Network Functionality

### Zwei Modi für fremde Devices

**1. Gast-Netzwerk (Default)**
- **Automatisch erstellt**: Für fremde Devices wird ein separates Gast-Netzwerk erstellt
- **Isoliert**: Gast-Netzwerk ist vom Hauptnetzwerk isoliert
- **Eigene Credentials**: Gast-Device nutzt seine eigenen User-Credentials
- **Kommunikation**: Gast kann von seinem Gast-Netzwerk mit seinen eigenen Geräten im Heimnetz kommunizieren
- **Datentransfer**: Explizite Erlaubnis erforderlich für Datentransfer zwischen Gast-Netzwerk und Hauptnetzwerk
- **Standard**: Dies ist die Default-Wahl für fremde Devices

**2. Expliziter Zugang zum eigenen Netzwerk**
- **Mehrfache Bestätigung erforderlich**: User muss explizit mehrfach bestätigen
- **Vollzugriff**: Gast-Device erhält vollständigen Zugang zum Hauptnetzwerk
- **Sicherheitsrisiko**: Alle Türen stehen offen, System kann manipuliert werden
- **Nur in speziellen Szenarien**: Sollte nur verwendet werden, wenn es keinen anderen Weg gibt

### Gast-Netzwerk Features

**Isolation**
- **Separates Network ID**: Gast-Netzwerk hat eigene Network ID
- **Isolierte Kommunikation**: Gast-Devices können nur mit anderen Gast-Devices und eigenen Geräten kommunizieren
- **Kein Zugriff auf Hauptnetzwerk**: Standardmäßig kein Zugriff auf Hauptnetzwerk-Devices

**Datentransfer-Erlaubnis**
- **Explizite Erlaubnis erforderlich**: User muss explizit erlauben
- **Granulare Kontrolle**: Erlaubnis kann pro Device oder pro Session erteilt werden
- **Revocation**: Erlaubnis kann jederzeit widerrufen werden

**Kommunikation mit eigenen Geräten**
- **Eigene Geräte im Heimnetz**: Gast kann mit seinen eigenen Geräten kommunizieren
- **Über Gast-Netzwerk**: Kommunikation läuft über Gast-Netzwerk
- **Transparent**: Für Gast transparent, sieht seine eigenen Geräte

### Expliziter Zugang zum Hauptnetzwerk

**Sicherheitsmaßnahmen**
- **Mehrfache Bestätigung**: User muss 2-3 Mal bestätigen
- **Warnung**: Klare Warnung über Sicherheitsrisiken
- **Audit Log**: Alle Zugriffe werden protokolliert
- **Timeout**: Zugang kann automatisch ablaufen (z.B. nach 24h)

## Security Token Management

### Token-Arten

**Heimdall-Tokens (Authentication/Authorization)**
- **Zweck**: Device-Authentifizierung und Autorisierung
- **Lebensdauer**: Längerlebig (z.B. 24 Stunden)
- **Verwendung**: Für Connection/Authentication Protocol

**Session-Tokens**
- **Zweck**: Für Bifrost/Jotunheim-Verbindungen
- **Lebensdauer**: Kurzlebig (z.B. 1 Stunde)
- **Verwendung**: Für aktive Sessions

### Token-Erneuerung ohne Unterbrechung

**Kombination: Refresh-Token + proaktive Erneuerung**

**Refresh-Token**
- **Längerlebiges Token**: Refresh-Token ist längerlebig (z.B. 30 Tage)
- **Für Erneuerung**: Wird verwendet, um neue Access-Tokens zu erhalten
- **Automatisch**: Erneuerung erfolgt automatisch ohne User-Intervention

**Proaktive Erneuerung**
- **Vor Ablauf**: Token wird erneuert, bevor es abläuft (z.B. 5 Minuten vor Ablauf)
- **Keine Unterbrechung**: User bemerkt keine Unterbrechung
- **Automatisch**: Erneuerung erfolgt automatisch im Hintergrund
- **Transparent**: Für User vollständig transparent

### Abgelaufene Tokens

**Kombination: Automatische Erneuerung wenn möglich, sonst Re-Auth**

**Automatische Erneuerung (wenn möglich)**
- **Refresh-Token vorhanden**: Falls Refresh-Token vorhanden und gültig, automatische Erneuerung
- **Im Hintergrund**: Erneuerung erfolgt im Hintergrund
- **Keine Störung**: User wird nicht gestört

**Re-Authentifizierung (falls nötig)**
- **Nur wenn nötig**: Nur wenn automatische Erneuerung nicht möglich ist
- **Minimal störend**: Re-Auth sollte so wenig störend wie möglich sein
- **Automatisch wenn möglich**: Falls Credentials gespeichert sind, automatische Re-Auth

### Token-Revocation

**Kombination: Sofortige Revocation + Timeout als Fallback**

**Sofortige Revocation**
- **Sofort ungültig**: Token wird sofort ungültig gemacht
- **Revocation-Liste**: Token wird in Revocation-Liste aufgenommen
- **Sofortige Wirkung**: Alle nachfolgenden Requests mit diesem Token werden abgelehnt

**Timeout als Fallback**
- **Falls Revocation fehlschlägt**: Falls sofortige Revocation nicht möglich ist, Timeout als Fallback
- **Kürzere Lebensdauer**: Token läuft nach Revocation nach kurzer Zeit ab (z.B. 5 Minuten)
- **Sicherheit**: Garantiert, dass Token nicht lange gültig bleibt

### Token-Leak Detection

**Kombination: Anomalie-Erkennung + Device-Tracking**

**Anomalie-Erkennung**
- **Ungewöhnliche Nutzung**: System erkennt ungewöhnliche Token-Nutzung
- **Pattern-Analyse**: Analysiert Nutzungsmuster
- **Alerts**: Sendet Alerts bei verdächtigen Aktivitäten
- **Automatisch**: Erkennung erfolgt automatisch

**Device-Tracking**
- **Token-Nutzung pro Device**: Token-Nutzung wird pro Device getrackt
- **Device-Identifikation**: Jede Token-Nutzung wird mit Device-ID verknüpft
- **Verdächtige Aktivitäten**: Erkennt, wenn Token von unbekanntem Device verwendet wird
- **Automatisch**: Tracking erfolgt automatisch

### Token-Rotation

**Kombination: Regelmäßig + bei Events**

**Regelmäßige Rotation**
- **Periodisch**: Token wird regelmäßig rotiert (z.B. täglich/wöchentlich)
- **Automatisch**: Rotation erfolgt automatisch
- **Transparent**: User bemerkt Rotation nicht

**Event-basierte Rotation**
- **Bei bestimmten Events**: Token wird bei bestimmten Events rotiert
- **Events**: Z.B. nach Security-Incident, nach längerer Inaktivität, nach Device-Änderung
- **Sicherheit**: Erhöht Sicherheit bei verdächtigen Aktivitäten

### User-Experience: Minimale Störung

**Automatisierung**
- **Alles automatisch**: Alle Token-Operationen erfolgen automatisch
- **Keine User-Intervention**: User muss nicht aktiv werden
- **Transparent**: User bemerkt Token-Management nicht

**Benachrichtigungen**
- **Nur bei wichtigen Events**: User wird nur bei wichtigen Events benachrichtigt
- **Nicht störend**: Benachrichtigungen sind nicht störend (z.B. Notification, nicht Popup)
- **Optional**: User kann Benachrichtigungen deaktivieren

## Implementierungs-Notizen

- Sollte als zentraler Security-Service fungieren
- Muss verschiedene Authentication-Methoden unterstützen
- Sollte extensible Permission-System haben
- Muss Threat-Detection-Mechanismen haben
- Sollte Audit-Logging für Compliance haben
- Muss Performance-optimiert sein (nicht zu langsam)
- Sollte Fail-Safe sein (bei Fehler: Deny statt Allow)
- Muss automatisch funktionieren (minimaler User-Eingriff)
- **Muss robuste Isolation zwischen Gast- und Hauptnetzwerk haben**: Für Gast-Netzwerk-Funktionalität
- **Muss mehrfache Bestätigung für expliziten Zugang implementieren**: Sicherheitsmaßnahme
- **Muss Audit-Logging für alle Gast-Zugriffe haben**: Für Security-Compliance
- **Muss automatische Cleanup-Mechanismen haben**: Für Gast-Devices
- **Muss User-Feedback für Erlaubnis-Anfragen haben**: Für Datentransfer-Erlaubnis
- **Muss Refresh-Token + proaktive Erneuerung haben**: Für Token-Management
- **Muss automatische Erneuerung haben**: Wenn möglich, ohne User-Intervention
- **Muss sofortige Revocation + Timeout als Fallback haben**: Für Token-Sicherheit
- **Muss Anomalie-Erkennung + Device-Tracking haben**: Für Token-Leak Detection
- **Muss regelmäßige + Event-basierte Rotation haben**: Für Token-Sicherheit
- **Muss User so wenig wie möglich stören**: Alles automatisch, transparent
- **Sollte mit lokalen Devices beginnen**: Kein Internet nötig für lokale Device-Authentifizierung
- **Muss Security von Anfang an implementieren**: Security ist nicht optional, muss von Anfang an vorhanden sein
- **Sollte Logging für Debugging haben**: Umfassendes Logging für Security-Debugging
- **Performance**: Muss optimiert sein für schnelle Security-Checks ohne Latenz-Overhead
- **Datenschutz**: Muss Privacy-by-Design implementieren und minimale Datensammlung gewährleisten
- **Sicherheit**: Muss Enterprise-Grade Security haben mit kontinuierlichem Monitoring


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
- **Security Analytics**: Analyse von Security-Events

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

1. **Device sendet Authentication Request**
   - Device-Identity wird übermittelt
   - Credentials werden übermittelt

2. **Heimdall validiert Identity**
   - Device-Identity wird verifiziert
   - Credentials werden geprüft
   - Device-Status wird überprüft

3. **Token Generation**
   - Heimdall erstellt `HeimdallToken`
   - Token wird signiert
   - Token wird an Device zurückgegeben

4. **Token Validation**
   - Device verwendet Token für Requests
   - Heimdall validiert Token bei jedem Request
   - Token-Expiration wird geprüft

### Authorization

1. **Action Request kommt an**
   - Device sendet Action-Request mit Token
   - Heimdall extrahiert Token

2. **Permission Check**
   - Heimdall prüft Token
   - Heimdall prüft Permissions für Resource
   - Heimdall prüft Conditions

3. **Authorization Decision**
   - Heimdall entscheidet: Allow oder Deny
   - Bei Allow: Request wird weitergeleitet
   - Bei Deny: Request wird blockiert, Error wird zurückgegeben

### Bifrost Connection Validation

1. **Connection Request**
   - Device A möchte sich mit Device B verbinden
   - Bifrost sendet Connection-Request an Heimdall

2. **Connection Validation**
   - Heimdall prüft Device-Identities
   - Heimdall prüft Permissions
   - Heimdall prüft Security-Policies

3. **Connection Decision**
   - Bei Allow: Connection wird erlaubt
   - Bei Deny: Connection wird blockiert
   - Heimdall überwacht Connection

4. **Ongoing Monitoring**
   - Heimdall überwacht Connection
   - Verdächtige Aktivitäten werden erkannt
   - Connection kann bei Bedarf beendet werden

## Security Features

### Authentication
- Public/Private Key Pairs
- Digital Signatures
- Token-based Authentication
- Multi-factor Authentication (optional)

### Authorization
- Role-Based Access Control (RBAC)
- Permission-based Access Control
- Resource-based Permissions
- Conditional Permissions

### Encryption
- TLS 1.3 für Connections
- End-to-End Encryption
- Key Exchange Protocols
- Key Rotation

### Threat Detection
- Anomaly Detection
- Pattern Recognition
- Rate Limiting
- Intrusion Detection

## Abhängigkeiten

- **Odin**: Für Coordination
- **Bifrost**: Für Connection Validation
- **Device Registry**: Für Device-Information
- **Edda Core Library**: DTOs (HeimdallToken, HeimdallPermission)

## Integration

- **Odin**: Koordiniert Security-Checks
- **Bifrost**: Validiert Connections über Heimdall
- **Thor**: Prüft Permissions für Actions
- **Alle Devices**: Verwenden Heimdall für Authentication/Authorization
- **Yggdrasil**: Zentrale Token-Verwaltung und User-Management

## Performance

### Performance-Optimierungen
- **Schnelle Token-Validierung**: Optimierte Token-Validierung für minimale Latenz
- **Caching**: Intelligentes Caching für Token-Validierungen und Permissions
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
- **Zweck**: Für Bifrost/Jötnar-Verbindungen
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
- **Performance**: Muss optimiert sein für schnelle Security-Checks ohne Latenz-Overhead
- **Datenschutz**: Muss Privacy-by-Design implementieren und minimale Datensammlung gewährleisten
- **Sicherheit**: Muss Enterprise-Grade Security haben mit kontinuierlichem Monitoring


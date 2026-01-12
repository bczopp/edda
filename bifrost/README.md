# Bifrost - Communication Service

## Übersicht

Bifrost ist der Secure WebSocket Service für Inter-Device Communication. Er ermöglicht sichere, verschlüsselte Kommunikation zwischen Devices im Edda-Netzwerk.

## Features

- **Secure WebSocket Communication**: WebSocket-basierte Kommunikation mit TLS-Verschlüsselung
- **TLS Encryption**: Alle Verbindungen sind verschlüsselt
- **Message Routing**: Routing von Messages zwischen Devices
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

**Connection Establishment über Bifrost**

1. **Device Discovery**
   - Device sucht nach anderen Devices im Netzwerk
   - Lokale Discovery (mDNS/Bonjour)
   - Globale Discovery über Yggdrasil

2. **Connection-Initiation über Bifrost (wenn über Yggdrasil)**
   - Device A möchte sich mit Device B verbinden
   - Device A sendet Bifrost-Message an Yggdrasil: "Möchte mich mit Device B verbinden"
   - Yggdrasil sendet Bifrost-Message an Device B: "Device A möchte sich verbinden"
   - Device B antwortet über Bifrost (Allow/Deny)
   - Bei Allow: Yggdrasil informiert Device A über Bifrost

3. **Bifrost Connection Request**
   - Device A initiiert Bifrost-WebSocket-Verbindung (direkt oder über Yggdrasil-Relay)
   - Heimdall validiert Request
   - Bei Allow: Verbindung wird etabliert

4. **TLS Handshake**
   - TLS-Verschlüsselung wird etabliert
   - Keys werden ausgetauscht
   - Verbindung ist verschlüsselt

5. **Connection Established**
   - WebSocket-Verbindung ist aktiv
   - Messages können gesendet werden
   - Heartbeat wird regelmäßig gesendet
   - Yggdrasil sendet Bifrost-Message: "Connection Established"

**Alternative: Direkte Verbindung (ohne Yggdrasil)**
- Devices im lokalen Netzwerk können sich direkt verbinden
- Direkter Bifrost-WebSocket-Aufbau

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

- **Edda Core Library**: DTOs, Protocols (Bifrost Protocol)
- **Heimdall**: Für Connection Validation und Security
- **Network Stack**: Für WebSocket und TLS
- **Security Libraries**: Für Verschlüsselung

## Integration

- **Odin**: Koordiniert Bifrost für Inter-Device Communication
- **Heimdall**: Validiert alle Connections über Bifrost
- **Asgard**: Kann als Relay fungieren
- **Yggdrasil**: 
  - **Bifrost-Relay**: Yggdrasil baut Bifrost-WebSocket-Verbindungen zu Devices auf
  - **Persistente Verbindungen**: Yggdrasil hält persistente Bifrost-Verbindungen zu allen registrierten Devices
  - **Message-Routing**: Yggdrasil routet Messages zwischen Devices über Bifrost
  - **Globale Discovery**: Yggdrasil unterstützt globale Device-Discovery
  - **Event-Notifications**: Alle Events werden über Bifrost-Messages gesendet
- **Alle Devices**: Midgard, Alfheim, Asgard verwenden Bifrost für Kommunikation

## Datenschutz

### Datenschutz-Features
- **End-to-End Encryption**: Optional End-to-End Encryption für Messages
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
- **Connection Blocking**: Blockierung von nicht-autorisierten Verbindungen
- **Secure Key Exchange**: Sichere Key-Exchange-Protokolle
- **Audit Logging**: Logging aller Connection-Events für Security-Audits

### Security-Best-Practices
- **Keine Hardcoded Secrets**: Keine Hardcoded Passwords oder Keys
- **Secure Defaults**: Sichere Standard-Konfigurationen
- **Regular Updates**: Automatische Security-Updates
- **Vulnerability Scanning**: Regelmäßiges Scanning für bekannte Vulnerabilities
- **Certificate Validation**: Validierung von TLS-Zertifikaten
- **Message Sanitization**: Sanitization von Messages zum Schutz vor Injection-Angriffen

## Protocol Design

### Connection/Authentication Protocol

**Basis**
- **Transport**: TLS 1.3 (sicherer Transport)
- **Message Format**: JSON mit digitalen Signaturen
- **Architecture**: Challenge-Response, Token-basiert
- **Zweck**: Sichere Device-Registrierung und Authentifizierung

**Design Goals**
- **Sicherheit**: Verhindert unbefugte Device-Registrierung
- **Authentifizierung**: Verifizierung von Device-Identities
- **Autorisierung**: Prüfung, ob Device sich verbinden darf
- **Token-basiert**: Nach Authentifizierung werden Tokens für weitere Kommunikation verwendet

**Initial Setup & First-Time Registration**

**Device kann autonom starten**
- **Autonome Funktionalität**: Jedes Device (außer Jötnar) kann ohne Yggdrasil-Account starten und autonom funktionieren
- **Download erfordert Anmeldung**: Um Software herunterzuladen, muss User sich auf Website anmelden
- **Key-Generierung**: Public/Private Keys werden auf dem Device selbst beim ersten Start generiert
- **Lokale Registrierung**: Device kann sich bei anderen Devices oder Servern (Asgard) registrieren, Yggdrasil ist nicht immer beteiligt

**Automatische Registrierung im eigenen Netzwerk**
- **User-Credentials**: Wenn User-Credentials übereinstimmen, kann neues Device automatisch im eigenen Netzwerk registriert werden
- **Keine manuelle Bestätigung**: Bei gleichen Credentials ist keine manuelle Bestätigung erforderlich
- **Token-Speicherung**: Device speichert Token, muss sich nur außerhalb des eigenen Netzwerks wieder anmelden

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
- **Architecture**: Client-Server, Peer-to-Peer Support
- **Voraussetzung**: Connection/Authentication Protocol muss erfolgreich abgeschlossen sein

**Verwendung**
- **Für**: Midgard (Desktop), Alfheim (Mobile), Asgard (Homeserver)
- **Nicht für**: Jötnar (IoT-Devices) - diese verwenden Jötnar Toolcalling Protocol

**Features**

**1. Device Discovery**
- **mDNS/Bonjour**: Automatische Device-Erkennung im lokalen Netzwerk
- **Manual Discovery**: IP-basierte Verbindung
- **Service Registry**: Zentralisierte Device-Registry (optional)

**2. Connection Management**
- **Handshake**: Authentifizierung und Verschlüsselung
- **Heartbeat**: Keep-Alive Mechanism
- **Reconnection**: Automatische Wiederverbindung bei Verbindungsabbruch
- **Connection Pooling**: Mehrere Verbindungen pro Device

**3. Message Routing**
- **Direct Routing**: Direkte Device-to-Device Kommunikation
- **Relay Routing**: Über Server (Asgard/Yggdrasil)
- **Broadcast**: Broadcast-Nachrichten an alle Devices im Netzwerk
- **Multicast**: Multicast an Device-Gruppen

**4. Security**
- **TLS Encryption**: End-to-End Verschlüsselung
- **Authentication**: Device-Identity-Verification
- **Authorization**: Permission-basierte Zugriffskontrolle
- **Message Signing**: Digitale Signaturen für Message-Integrität

## Error Recovery und Resilience

### Automatische Wiederverbindung

**Kombination: Sofortiger Versuch, dann Exponential Backoff**

**Sofortiger Reconnect-Versuch**
- **Sofort**: Bei Verbindungsabbruch wird sofort versucht, Verbindung wiederherzustellen
- **Erster Versuch**: Keine Wartezeit beim ersten Versuch
- **Schnelle Wiederherstellung**: Minimiert Unterbrechung

**Exponential Backoff**
- **Nach erstem Fehler**: Nach erstem fehlgeschlagenen Versuch beginnt Exponential Backoff
- **Wartezeit erhöht sich**: Wartezeit zwischen Versuchen erhöht sich exponentiell
- **Maximale Wartezeit**: Maximale Wartezeit (z.B. 60 Sekunden)
- **Kontinuierliche Versuche**: System versucht kontinuierlich, Verbindung wiederherzustellen

### Netzwerk-Fehlerbehandlung

**Kombination: Retry → Fallback → Fehler**

**1. Automatischer Retry mit Exponential Backoff**
- **Erster Versuch**: Sofortiger Retry bei Netzwerk-Fehler
- **Exponential Backoff**: Bei wiederholten Fehlern wird Wartezeit exponentiell erhöht
- **Maximale Retries**: Maximale Anzahl von Retries (z.B. 3-5 Versuche)
- **Timeout**: Retry-Versuche haben Timeout

**2. Sofortiger Fallback**
- **Alternative Route**: Falls Retry fehlschlägt, sofortiger Fallback zu alternativer Route
- **Relay-Fallback**: Falls direkte Verbindung fehlschlägt, Fallback zu Relay (Asgard/Yggdrasil)
- **Alternative Provider**: Falls Provider-Fehler, Fallback zu alternativem Provider

**3. Fehlermeldung**
- **User-Benachrichtigung**: Falls alle Versuche fehlschlagen, Fehlermeldung an User
- **Error-Logging**: Alle Fehler werden geloggt für Debugging
- **Retry-Later**: User kann später erneut versuchen

## Implementierungs-Notizen

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
- **Performance**: Muss optimiert sein für schnelle Message-Übertragung und niedrige Latenz
- **Datenschutz**: Muss Privacy-by-Design implementieren und Message-Privacy gewährleisten
- **Sicherheit**: Muss robuste Security-Mechanismen haben, insbesondere für verschlüsselte Kommunikation


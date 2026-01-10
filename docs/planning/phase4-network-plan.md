# Phase 4: Network Expansion Plan

## Übersicht
Phase 4 erweitert die Netzwerk-Funktionalität um WAN-Connectivity. Devices können sich nun auch über das Internet verbinden, nicht nur im lokalen Netzwerk.

## Komponenten

### 1. WAN Connectivity

#### IP-based Connections
- **Public IP Support**: Devices können über öffentliche IPs verbunden werden
- **Dynamic IP Handling**: Umgang mit dynamischen IP-Adressen
- **NAT Traversal**: Unterstützung für NAT-Netzwerke
- **Port Forwarding**: Automatische oder manuelle Port-Forwarding-Konfiguration

#### Connection Types
- **Direct IP**: Direkte Verbindung über IP-Adresse (nur bei expliziter Erlaubnis bei Asgard)
- **Domain-based**: Verbindung über Domain-Name (nur bei expliziter Erlaubnis bei Asgard)
- **Relay through Server**: Verbindung über Relay-Server (Asgard/Yggdrasil) - Hauptmethode
- **Yggdrasil als Registry**: Hauptsächlich über Yggdrasil als zentrale Registry

### 2. Enhanced Routing

#### Routing Strategies
- **Direct Routing**: Direkte Device-to-Device Verbindung wenn möglich
- **Relay Routing**: Routing über Server wenn direkte Verbindung nicht möglich
- **Hybrid Routing**: Kombination aus Direct und Relay

#### Routing Features
- **Path Optimization**: Optimierung der Routing-Pfade
- **Load Balancing**: Lastverteilung über mehrere Pfade
- **Failover**: Automatisches Failover bei Verbindungsausfall
- **Quality-based Routing**: Routing basierend auf Connection-Quality

### 3. Connection Management

#### Connection Types
- **Local Connections**: Verbindungen im lokalen Netzwerk
- **WAN Connections**: Verbindungen über das Internet
- **Hybrid Connections**: Kombination aus Local und WAN

#### Connection Features
- **Connection Pooling**: Pool von Verbindungen
- **Connection Reuse**: Wiederverwendung von Verbindungen
- **Connection Monitoring**: Überwachung von Verbindungen
- **Automatic Reconnection**: Automatische Wiederverbindung (sofort + Exponential Backoff)
- **Error Recovery**: Robustes Error-Handling für Verbindungsfehler
- **Fallback-Routing**: Fallback zu alternativen Routen bei Fehlern

## Configuration

### IP Configuration
- **Static IP**: Feste IP-Adresse konfigurieren
- **Dynamic IP**: Dynamische IP mit Update-Mechanismus
- **Domain Name**: Domain-Name für Device
- **Port Configuration**: Port-Konfiguration für Services

### Network Configuration
- **Network ID**: Network ID für Software-Netzwerk
- **Server Configuration**: Konfiguration von Asgard/Yggdrasil-Servern
- **Relay Configuration**: Konfiguration von Relay-Servern
- **Firewall Rules**: Firewall-Regeln für Verbindungen

## Workflow

### WAN Connection Establishment

1. **Device A möchte sich mit Device B verbinden (WAN)**
   - Device A hat IP/Domain von Device B
   - Device A initiiert WAN-Connection

2. **Connection Attempt**
   - Device A versucht direkte Verbindung
   - Falls nicht möglich: Relay über Server

3. **Connection Establishment**
   - TLS Handshake
   - Device Authentication
   - Connection wird etabliert

4. **Ongoing Communication**
   - Messages werden über WAN-Connection geroutet
   - Connection wird überwacht
   - Bei Ausfall: Automatische Wiederverbindung

### Enhanced Routing

1. **Message muss geroutet werden**
   - Source und Target Device werden identifiziert
   - Routing-Strategie wird gewählt

2. **Path Selection**
   - Verfügbare Pfade werden evaluiert
   - Optimaler Pfad wird gewählt
   - Fallback-Pfade werden vorbereitet

3. **Message Routing**
   - Message wird über gewählten Pfad geroutet
   - Bei Fehler: Fallback-Pfad wird verwendet
   - Routing-Status wird überwacht

### Relay-Funktionalität

#### Automatisch bevorzugt
- **Automatisch versuchen**: System versucht automatisch direkte Verbindung
- **Relay bei Bedarf**: Falls direkte Verbindung nicht möglich, automatisch über Relay
- **User kann erzwingen**: User kann Relay-Modus explizit erzwingen
- **Kombination**: Automatisch versuchen, User kann erzwingen

#### Relay über Asgard/Yggdrasil
- **Asgard als Relay**: Asgard kann als Relay-Server fungieren
- **Yggdrasil als Relay**: Yggdrasil fungiert als zentraler Relay-Server
- **Automatische Auswahl**: System wählt automatisch besten Relay-Server

## Technical Details

### NAT Traversal
- **Automatisch bevorzugt**: Automatisches NAT-Traversal wird stark bevorzugt
- **STUN**: STUN-Protokoll für NAT-Discovery
- **TURN**: TURN-Server für Relay wenn NAT-Traversal nicht möglich (Yggdrasil/Asgard als TURN-Server)
- **ICE**: ICE-Protokoll für optimalen Pfad
- **Asgard braucht NAT-Traversal**: Asgard sollte auch NAT-Traversal-Funktionalität haben
- **Fallback auf manuelle Konfiguration**: Falls automatisch nicht möglich, Fallback auf manuelle Port-Forwarding-Konfiguration

### Dynamic IP Handling
- **Kombination**: DDNS wenn konfiguriert, sonst Relay über Yggdrasil
- **DDNS**: Dynamic DNS für Domain-Names (wenn User konfiguriert)
- **IP Update Service**: Service für IP-Updates
- **Connection Refresh**: Automatische Connection-Refresh bei IP-Änderung
- **Yggdrasil-Relay**: Falls DDNS nicht konfiguriert, automatisch über Yggdrasil-Relay
- **Sicherheit**: Muss sicher sein und nicht zu kompliziert

### Security
- **TLS Encryption**: TLS für alle WAN-Connections
- **Certificate Validation**: Validierung von TLS-Zertifikaten
- **Firewall Integration**: Integration mit Firewall-Regeln
- **Intrusion Detection**: Erkennung von Angriffen

## Abhängigkeiten
- Phase 3 Components (Asgard, Enhanced Bifrost)
- Network Stack
- DNS Services
- NAT Traversal Libraries

## Implementierungs-Notizen
- Muss robustes Error-Handling für Netzwerk-Fehler haben
- Sollte verschiedene NAT-Traversal-Strategien unterstützen
- Muss Connection-Quality-Monitoring haben
- Sollte automatisches Failover haben
- Muss Security-Best-Practices für WAN-Connections folgen
- Sollte User-Feedback für Connection-Status haben


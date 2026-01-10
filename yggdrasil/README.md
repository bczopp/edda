# Yggdrasil - Main Server

## Übersicht

Yggdrasil ist der Main Server, der von der Company kontrolliert wird. Er erweitert Asgard um globale Features und stellt die zentrale Infrastruktur für das globale Edda-Netzwerk bereit. Yggdrasil ist nicht von Usern selbst hostbar.

**Wichtig**: Yggdrasil basiert auf Asgard und erweitert es um:
- Globale Device-Registry (weltweit)
- User-Management und Subscriptions
- Payment-Integration
- Marketplace-Infrastruktur
- Strikte Netzwerk-Isolation zwischen Usern und Netzwerken

**Netzwerk-Sicherheit ist oberste Priorität**: User dürfen nicht in andere Netze eindringen können, nicht einmal wissen, dass andere Netze existieren oder wie diese heißen.

## Projektstruktur

```
yggdrasil/
├── src/
│   ├── server/          # Server Core
│   │   ├── main.ts
│   │   ├── config/
│   │   └── platform/
│   ├── services/        # Server Services
│   │   ├── device-registry/
│   │   ├── user-management/
│   │   ├── subscription/
│   │   ├── payment/
│   │   ├── marketplace/
│   │   └── analytics/
│   ├── api/            # API Endpoints
│   │   ├── v1/
│   │   │   ├── devices/
│   │   │   ├── users/
│   │   │   ├── subscriptions/
│   │   │   ├── payments/
│   │   │   └── marketplace/
│   │   └── admin/
│   ├── database/       # Database Layer
│   │   ├── models/
│   │   ├── migrations/
│   │   └── queries/
│   └── utils/
├── config/
├── deployments/       # Deployment Configs
│   ├── kubernetes/
│   ├── docker/
│   └── cloud/
└── tests/
```

## Features

### 1. Global Device Registry
- **Device Registration**: Registrierung aller Devices weltweit
- **Device Discovery**: Globale Device-Discovery
- **Device Status**: Tracking von Device-Status
- **Device Metadata**: Verwaltung von Device-Metadaten

### 2. User Management
- **User Accounts**: User-Account-Verwaltung
- **Authentication**: User-Authentifizierung
  - **OAuth bevorzugt**: OAuth (Google, Apple, etc.) ist die bevorzugte Methode
  - **Email/Code als Fallback**: Falls OAuth nicht möglich, Email-Registrierung mit Code-Versand
  - **Code-Verifizierung**: Code wird an Email geschickt, User bestätigt damit Registrierung/Anmeldung
  - **Token-Management**: Device speichert Token, außerhalb des eigenen Netzwerks muss man sich wieder anmelden
- **Authorization**: User-Autorisierung
- **Profile Management**: User-Profile-Verwaltung
- **Network ID Synchronisation**: Wenn User außerhalb des Heimnetzes ist und sich bei Yggdrasil anmeldet, wird Network ID an neues Device übergeben

### 3. Subscription System
- **Subscription Tiers**: Verschiedene Subscription-Levels
  - Free Tier (Kostenlos - alle Core-Features)
  - Premium Tier (€30/Monat - 400K Tokens Cloud-LLM)
  - Pro Tier (€120/Monat - 1.7M Tokens Cloud-LLM)
  - Enterprise Tier (€250/Monat - 3.5M Tokens Cloud-LLM)
  - Pay-per-Token Tier (Flexibel - keine monatliche Gebühr)
- **Subscription Management**: Verwaltung von Subscriptions
- **Billing**: Abrechnung von Subscriptions
- **Feature Access**: Feature-Zugriff basierend auf Tier

### 4. Payment Integration
- **Payment Providers**: Integration mit Payment-Providern
  - Stripe
  - PayPal
  - Andere Payment-Provider
- **Payment Processing**: Verarbeitung von Zahlungen
- **Invoice Management**: Rechnungsverwaltung
- **Refund Handling**: Rückerstattungen

### 5. Marketplace Management (Phase 7)

Der Marketplace ermöglicht es Usern, ihre Hardware als Compute-Provider anzubieten und andere User können diese für LLM-Requests nutzen.

#### Provider Registration

**Provider Setup**
- **Device Registration**: User kann Device als Provider registrieren
- **Capability Declaration**: Device teilt Capabilities mit
  - Verfügbare Models
  - Hardware Specs (GPU, RAM, etc.)
  - Pricing Configuration
- **Availability Settings**: Verfügbarkeit konfigurieren
- **Sharing Settings**: Welche Devices zum Sharing freigegeben werden

**Provider Configuration**
- **Pricing**: Preis pro 1000 Tokens (in Cents, ganzzahlig)
- **Model Selection**: Welche Models angeboten werden
- **Capacity Limits**: Maximale Kapazität
- **Quality Settings**: Quality-Settings für Requests
- **Zahlungsmethode erforderlich**: Gültige Zahlungsmethode muss hinterlegt sein für Auszahlung von Earnings

**Provider Registration Workflow**
1. User registriert Device als Provider
   - Device-Capabilities werden übermittelt
   - Pricing wird konfiguriert
   - Availability wird gesetzt
   - Zahlungsmethode wird hinterlegt und verifiziert
2. Yggdrasil testet Connection
   - Test-Request wird gesendet
   - Connection Quality wird gemessen
   - Latency wird gemessen
3. Provider wird registriert
   - Provider wird in Marketplace aufgenommen
   - Provider ist verfügbar für Requests
   - Nur wenn Zahlungsmethode verifiziert ist

#### Request Routing Engine

**Request Processing**
- **Request Reception**: Empfang von Compute-Requests
- **Provider Matching**: Matching von Requests mit Providern
- **Quality Assessment**: Bewertung der Provider-Quality
  - Nach jedem Request: Quality wird nach jedem Request gemessen (automatisch + optionales User-Feedback)
  - Periodische Tests: Regelmäßige Tests ergänzen kontinuierliche Bewertung
  - Gewichteter Durchschnitt: Quality-Metriken werden aggregiert (neuere Requests haben höheres Gewicht)
  - Sofort + Batch: Sofortige Updates für wichtige Änderungen, Batch-Aggregation für Effizienz
- **Route Selection**: Auswahl des optimalen Providers

**Routing Algorithm**
- **Fair Distribution**: Fair Distribution Algorithm
  - Round-Robin bei gleichen Bedingungen
  - Berücksichtigung von:
    - Preis pro Token
    - Verfügbare Kapazität
    - Connection Quality
    - Estimated Latency
    - Provider History (Fairness-Score)
- **Load Balancing**: Lastverteilung über Provider
- **Failover**: Automatisches Failover bei Provider-Ausfall

#### Fair Distribution Algorithm

**Algorithm Details**
- **Fairness Score**: Score basierend auf bisheriger Nutzung
- **Round-Robin**: Rotation bei gleichen Bedingungen
- **Quality Weighting**: Gewichtung nach Quality-Metriken
- **Cost Optimization**: Optimierung nach Kosten

**Scoring**
- **Preis** (30% Gewichtung): Niedriger Preis = höherer Score
- **Qualität** (25% Gewichtung): Höhere Qualität = höherer Score
- **Latency** (20% Gewichtung): Niedrigere Latency = höherer Score
- **Verfügbarkeit** (15% Gewichtung): Höhere Verfügbarkeit = höherer Score
- **Fairness** (10% Gewichtung): Fair Distribution Score

#### Transaction System

**Transaction Lifecycle**
- **PENDING**: Transaction erstellt
- **PROCESSING**: Request wird verarbeitet
- **COMPLETED**: Transaction abgeschlossen
- **FAILED**: Transaction fehlgeschlagen
- **CANCELLED**: Transaction abgebrochen
- **REFUNDED**: Rückerstattung

**Transaction Management**
- **Transaction Tracking**: Tracking aller Transactions
- **Settlement**: Abrechnung zwischen Requester, Provider und Company
- **Refund Handling**: Rückerstattungen
- **Dispute Resolution**: Streitbeilegung

**Payment Structure**
- **Requester**: Zahlt totalCost (muss gültige Zahlungsmethode haben)
- **Provider**: Erhält providerEarnings (totalCost - companyFee, muss gültige Zahlungsmethode haben)
- **Company**: Erhält companyFee (10-15% Commission)

**Pricing Model**
- **Token Pricing**: Cent-Berechnung pro 1000 Tokens (ganzzahlig, keine Kommastellen)
- **Berechnung**: (tokens / 1000) * pricePerToken (aufgerundet)
- **Commission**: 10-15% des Token-Preises

#### Request Processing Workflow

1. **Requester sendet Compute-Request**
   - Request mit Model-Requirements und Prompt
   - Max Cost wird angegeben
   - Zahlungsmethode wird geprüft (gültige Zahlungsmethode muss vorhanden sein)
   - Pre-Authorization für geschätzte Kosten

2. **Yggdrasil findet passende Provider**
   - Provider-Matching basierend auf Requirements
   - Quality-Assessment
   - Fair Distribution Algorithm
   - Provider-Zahlungsmethode wird geprüft (gültige Zahlungsmethode für Auszahlung muss vorhanden sein)

3. **Request wird geroutet**
   - Request wird an gewählten Provider gesendet
   - Provider verarbeitet Request
   - Response wird zurückgesendet

4. **Transaction wird abgeschlossen**
   - Tokens werden gezählt
   - Kosten werden berechnet
   - Payment wird verarbeitet (von Requester)
   - Earnings werden gutgeschrieben (an Provider)
   - Transaction wird abgeschlossen

#### Analytics Dashboard

**Provider Analytics**
- **Request Statistics**: Anzahl von Requests
- **Earnings**: Verdienst-Statistiken
- **Quality Metrics**: Quality-Metriken
- **Usage Patterns**: Nutzungsmuster

**Requester Analytics**
- **Request History**: Request-Historie
- **Cost Analysis**: Kosten-Analyse
- **Quality Metrics**: Quality-Metriken der verwendeten Provider

### 6. Global Lock Management
- **Distributed Locking**: Verwaltet Locks für globale Resources
- **Lock-Registry**: Zentrale Registry für alle aktiven Locks
- **Lock-Expiration**: Verwaltet Lock-Expiration (Timeout)
- **Deadlock-Detection**: Erkennt Deadlocks und löst sie auf
- **Priority-Management**: Verwaltet Prioritäten für Lock-Requests

## Database Schema

### Users Table
- user_id (PK)
- email
- password_hash
- created_at
- updated_at
- subscription_tier
- subscription_status

### Devices Table
- device_id (PK)
- user_id (FK)
- network_id
- world_type
- capabilities (JSON)
- registered_at
- last_seen
- status

### Subscriptions Table
- subscription_id (PK)
- user_id (FK)
- tier
- status
- started_at
- expires_at
- payment_method_id

### Providers Table (Phase 7)
- provider_id (PK)
- user_id (FK)
- device_id (FK)
- pricing_per_1000_tokens (integer, cents)
- available_models (JSON)
- hardware_specs (JSON)
- capacity_limits (JSON)
- availability_settings (JSON)
- quality_settings (JSON)
- status (active/inactive)
- registered_at
- last_seen

### Transactions Table (Phase 7)
- transaction_id (PK)
- request_id
- provider_device_id
- requester_device_id
- tokens_used
- cost_per_token
- total_cost
- company_fee
- provider_earnings
- status
- completed_at

## API Endpoints

### User Management
- `POST /api/v1/users/register` - Register new user
- `POST /api/v1/users/login` - User login
- `GET /api/v1/users/me` - Get current user
- `PUT /api/v1/users/me` - Update user profile
- `DELETE /api/v1/users/me` - Delete user account

### Device Management
- `GET /api/v1/devices` - List user devices
- `POST /api/v1/devices` - Register device
- `GET /api/v1/devices/:id` - Get device details
- `PUT /api/v1/devices/:id` - Update device
- `DELETE /api/v1/devices/:id` - Unregister device

### Subscription Management
- `GET /api/v1/subscriptions` - Get user subscription
- `POST /api/v1/subscriptions` - Create subscription
- `PUT /api/v1/subscriptions` - Update subscription
- `DELETE /api/v1/subscriptions` - Cancel subscription

### Payment Management
- `POST /api/v1/payments/methods` - Add payment method
- `GET /api/v1/payments/methods` - List payment methods
- `DELETE /api/v1/payments/methods/:id` - Remove payment method
- `POST /api/v1/payments/process` - Process payment

### Marketplace (Phase 7)
- `POST /api/v1/marketplace/providers/register` - Register device as provider
- `GET /api/v1/marketplace/providers` - List available providers
- `GET /api/v1/marketplace/providers/:id` - Get provider details
- `PUT /api/v1/marketplace/providers/:id` - Update provider configuration
- `DELETE /api/v1/marketplace/providers/:id` - Unregister as provider
- `POST /api/v1/marketplace/requests` - Create compute request
- `GET /api/v1/marketplace/requests/:id` - Get request status
- `GET /api/v1/marketplace/transactions` - List transactions
- `GET /api/v1/marketplace/transactions/:id` - Get transaction details
- `GET /api/v1/marketplace/analytics/provider` - Get provider analytics
- `GET /api/v1/marketplace/analytics/requester` - Get requester analytics

## Infrastructure

### Deployment
- **Kubernetes**: Container-Orchestrierung
- **Docker**: Containerisierung
- **Cloud Providers**: AWS, GCP, Azure
- **Multi-Region**: Deployment in mehreren Regionen

### Scalability
- **Horizontal Scaling**: Skalierung über mehrere Instanzen
- **Load Balancing**: Lastverteilung
- **Database Sharding**: Datenbank-Sharding
- **Caching**: Redis/Memcached für Caching

### Monitoring
- **Application Monitoring**: Application Performance Monitoring
- **Infrastructure Monitoring**: Infrastructure Monitoring
- **Logging**: Centralized Logging
- **Alerting**: Alert-System

## Security

### Security Measures
- **TLS Encryption**: TLS für alle Connections
- **Authentication**: JWT-based Authentication
- **Authorization**: Role-based Authorization
- **Rate Limiting**: Rate Limiting für API
- **DDoS Protection**: DDoS-Schutz
- **Data Encryption**: Verschlüsselung von sensiblen Daten

### Compliance
- **GDPR**: GDPR-Compliance
- **Data Privacy**: Datenschutz-Best-Practices
- **Audit Logging**: Audit-Logging für Compliance

## Abhängigkeiten

- **Edda Core Library**: DTOs, Protocols, Utils
- Database (PostgreSQL)
- Payment Providers (Stripe, PayPal, etc.)
- Cloud Infrastructure
- Monitoring Tools

## Service-Integration

Yggdrasil stellt alle Services von Asgard bereit, zusätzlich zu den globalen Features:

### Verfügbare Services
- **Odin**: Hauptprozess für Orchestrierung (wie auf Asgard)
- **Thor**: Action Executor (wie auf Asgard)
- **Huginn & Muninn**: STT/TTS Service (wie auf Asgard)
- **Freki**: RAG Service (wie auf Asgard)
- **Geri**: LLM Service (wie auf Asgard)
- **Heimdall**: Security Service (wie auf Asgard)
- **Bifrost**: Communication Service (wie auf Asgard)
- **Frigg**: Healthcare Plugin (automatisch vorhanden)
- **Valkyries**: Coding Agent (automatisch vorhanden)

### Multi-Tenant-Architektur
- **Strikte Netzwerk-Isolation**: Jeder User und jedes Netzwerk ist vollständig isoliert
- **Keine Sichtbarkeit**: User können andere Netze nicht sehen oder auf sie zugreifen
- **Automatische Model-Auswahl**: Yggdrasil koordiniert automatische Model-Auswahl über alle User
- **Provider-Registrierung**: User können sich als Compute-Provider registrieren

## Integration

- **Alle Devices**: Midgard, Alfheim, Asgard verbinden sich mit Yggdrasil
- **Device Registry**: Zentrale Registry für alle Devices weltweit
- **User Management**: Zentrale User-Verwaltung
- **Marketplace**: Zentrale Marketplace-Infrastruktur
- **Frigg**: Healthcare Plugin ist automatisch bei Yggdrasil vorhanden
- **Valkyries**: Coding Agent ist automatisch bei Yggdrasil vorhanden
- **Alle Services**: Odin, Thor, Huginn-Muninn, Freki, Geri, Heimdall, Bifrost sind verfügbar

## Performance

### Performance-Optimierungen
- **Horizontal Scaling**: Skalierung über mehrere Instanzen für hohen Durchsatz
- **Load Balancing**: Intelligente Lastverteilung für optimale Performance
- **Database Sharding**: Datenbank-Sharding für bessere Performance bei großen Datenmengen
- **Caching**: Redis/Memcached für schnellen Zugriff auf häufig verwendete Daten
- **CDN Integration**: CDN für statische Assets und API-Responses
- **Database Optimization**: Optimierte Datenbankabfragen mit Indizes und Query-Optimierung
- **Async Processing**: Asynchrone Verarbeitung für bessere Response-Zeiten

### Performance-Metriken
- Niedrige API-Latenz (< 100ms für Standard-Requests)
- Hoher Durchsatz (1000+ Requests/Sekunde pro Instanz)
- Schnelle Database-Queries (< 50ms für Standard-Queries)
- Effiziente Skalierung (linear mit zusätzlichen Instanzen)

## Datenschutz

### Datenschutz-Features
- **Datenverschlüsselung**: Alle sensiblen Daten werden verschlüsselt gespeichert
- **Minimale Datensammlung**: Nur notwendige Daten werden gespeichert
- **User Control**: User hat volle Kontrolle über seine Daten
- **Right to Deletion**: User kann alle Daten löschen (GDPR-konform)
- **Data Minimization**: Nur notwendige Daten werden verarbeitet
- **Transparency**: User wird über Datenverarbeitung informiert

### Compliance
- **GDPR-konform**: Vollständige Einhaltung der GDPR-Anforderungen
- **Data Privacy**: Datenschutz-Best-Practices werden befolgt
- **Audit Logging**: Vollständiges Audit-Logging für Compliance
- **Data Retention**: Automatische Löschung nach Retention-Policy
- **User Rights**: Unterstützung für alle User-Rechte (Access, Portability, Deletion)

## Sicherheit

### Netzwerk-Isolation (Oberste Priorität)

**Netzwerk-Sicherheit ist die oberste Priorität von Yggdrasil**. User dürfen nicht in andere Netze eindringen können, nicht einmal wissen, dass andere Netze existieren.

#### Isolation-Mechanismen
- **Vollständige Netzwerk-Isolation**: Jeder User und jedes Netzwerk ist vollständig isoliert
- **Keine Sichtbarkeit**: User können andere Netze nicht sehen, nicht auf sie zugreifen, nicht einmal deren Namen kennen
- **Network Segmentation**: Separate Netzwerksegmente für jeden User/Network
- **VPC-Isolation**: Virtual Private Cloud Isolation für jeden Tenant
- **Kubernetes Network Policies**: Strikte Network Policies verhindern Cross-Network-Kommunikation
- **RBAC**: Role-Based Access Control stellt sicher, dass User nur auf ihre eigenen Ressourcen zugreifen können
- **Resource Quotas**: Jeder Tenant hat eigene Resource-Limits

#### Security-Features
- **TLS Encryption**: TLS 1.3 für alle Connections
- **Authentication**: JWT-based Authentication mit sicheren Tokens
- **Authorization**: Role-based Authorization (RBAC) mit strikter Netzwerk-Isolation
- **Rate Limiting**: Rate Limiting für API zum Schutz vor DDoS
- **DDoS Protection**: DDoS-Schutz auf Infrastructure-Level
- **Data Encryption**: Verschlüsselung von sensiblen Daten (at rest und in transit)
- **Security Monitoring**: Kontinuierliches Security-Monitoring und Threat Detection
- **Network Monitoring**: Kontinuierliche Überwachung auf unautorisierte Cross-Network-Zugriffe

### Security-Best-Practices
- **Keine Hardcoded Secrets**: Keine Hardcoded Passwords oder Keys
- **Secure Defaults**: Sichere Standard-Konfigurationen
- **Regular Updates**: Automatische Security-Updates
- **Vulnerability Scanning**: Kontinuierliches Scanning für bekannte Vulnerabilities
- **Penetration Testing**: Regelmäßige Penetration Tests
- **Incident Response**: Automatische Response auf Security-Incidents
- **Security Training**: Regelmäßige Security-Trainings für Team

## Marketplace Payment Requirements

### Anforderungen

**Für Consumer (Requester)**

**Zahlungsmethode erforderlich**
- **Gültige Zahlungsmethode muss hinterlegt sein**:
  - Kreditkarte (Visa, Mastercard, etc.)
  - Debitkarte
  - PayPal
  - Bank Account (SEPA)
  - Weitere Zahlungsmethoden je nach Region

**Verwendungszweck**
- **Bezahlung von Compute-Requests**:
  - Automatische Abrechnung pro Request
  - Pre-Authorization für geschätzte Kosten
  - Post-Payment nach tatsächlicher Token-Nutzung

**Verifizierung**
- **Zahlungsmethode muss verifiziert sein**:
  - Kreditkarte: CVV-Check, Adress-Verifizierung
  - Bank Account: SEPA-Verifizierung (kleine Test-Transaktion)
  - PayPal: PayPal-Verifizierung

**Limits & Controls**
- **Spending Limits**: Optional max. Ausgaben pro Monat
- **Budget Alerts**: Warnung bei bestimmten Ausgaben
- **Auto-Top-up**: Automatisches Aufladen bei niedrigem Guthaben (optional)

**Für Provider**

**Zahlungsmethode erforderlich**
- **Gültige Zahlungsmethode muss hinterlegt sein**:
  - Bank Account (SEPA, ACH, etc.)
  - PayPal
  - Weitere Auszahlungsmethoden je nach Region

**Verwendungszweck**
- **Auszahlung von Earnings**:
  - Automatische Auszahlung von verdienten Beträgen
  - Auszahlungsfrequenz konfigurierbar (täglich/wöchentlich/monatlich)
  - Minimum-Auszahlungsbetrag (z.B. €10)

**Verifizierung**
- **Zahlungsmethode muss verifiziert sein**:
  - Bank Account: SEPA-Verifizierung (kleine Test-Transaktion)
  - PayPal: PayPal-Verifizierung
  - Steuerliche Dokumentation (je nach Region)

**Payout Settings**
- **Auszahlungsfrequenz**: Täglich, wöchentlich, monatlich
- **Minimum Payout**: Mindestbetrag für Auszahlung
- **Auto-Payout**: Automatische Auszahlung bei Erreichen des Minimums

### Beide Rollen gleichzeitig

**User als Consumer UND Provider**
- **Separate Zahlungsmethoden möglich**:
  - Eine für Requests (Consumer)
  - Eine für Earnings (Provider)
  - Oder: Gleiche Zahlungsmethode für beide

**Netting (Optional)**
- **Earnings gegen Costs verrechnen**:
  - Wenn User sowohl Consumer als auch Provider ist
  - Earnings werden gegen Costs verrechnet
  - Nur Differenz wird abgerechnet/ausgezahlt
  - Reduziert Transaktionskosten

### Payment Provider Integration

- **Stripe**: Für Kreditkarten, SEPA
- **PayPal**: Für PayPal-Zahlungen
- **Weitere Provider**: Je nach Region

### Security & Compliance

**Security**
- **PCI-DSS Compliance**: Für Kreditkarten-Daten
- **Encryption**: Verschlüsselung aller Zahlungsdaten
- **Tokenization**: Kreditkarten-Daten werden tokenisiert gespeichert
- **3D Secure**: Für zusätzliche Sicherheit

**Compliance**
- **KYC (Know Your Customer)**: Für Provider (je nach Region)
- **Steuerliche Dokumentation**: Für Auszahlungen
- **GDPR**: Datenschutz für Zahlungsdaten
- **Regional Compliance**: Je nach Region (EU, US, etc.)

## Device Interconnection (Phase 2)

### DeviceIdentity System

**DeviceIdentity Management**
- **Device Registration**: Jedes Device erhält eine eindeutige ID (user-assigned)
- **Identity Storage**: Device-Identity wird lokal gespeichert
- **Identity Validation**: Validierung von Device-Identities
- **Identity Sharing**: Devices teilen ihre Identity mit anderen Devices

**Features**
- User-assigned Device IDs
- Device Metadata (Name, Type, Capabilities)
- Identity Persistence
- Identity Verification

**Data Structure**
- Device ID (user-assigned, unique)
- Device Name
- World Type (Midgard, Asgard, Alfheim, Jötnar)
- Capabilities
- Hardware Specs
- Registration Timestamp

**Storage**
- Local SQLite Database
- Encrypted Storage
- Backup & Restore

### Device Discovery & Connection

**Workflow**
1. **Device A möchte sich mit Device B verbinden**
   - Device A sendet Discovery-Request
   - Device B antwortet mit Device-Identity

2. **Connection Establishment**
   - Device A initiiert Bifrost-Connection
   - Heimdall validiert beide Device-Identities
   - TLS-Handshake wird durchgeführt
   - Connection wird etabliert

3. **Device Communication**
   - Device A kann Messages an Device B senden
   - Device B kann Messages an Device A senden
   - Messages werden über Bifrost geroutet

### Cross-Device Action Execution

**Workflow**
1. **User gibt Command auf Device A**
   - Odin auf Device A verarbeitet Command
   - Odin entscheidet, dass Action auf Device B ausgeführt werden soll

2. **Action Routing**
   - Device A sendet `ThorAction` über Bifrost an Device B
   - Heimdall auf Device B prüft Permissions

3. **Action Execution**
   - Thor auf Device B führt Action aus
   - `ThorResult` wird zurück an Device A gesendet

4. **Response**
   - Device A empfängt Result
   - User erhält Response

## Network Expansion (Phase 4)

### WAN Connectivity

**IP-based Connections**
- **Public IP Support**: Devices können über öffentliche IPs verbunden werden
- **Dynamic IP Handling**: Umgang mit dynamischen IP-Adressen
- **NAT Traversal**: Unterstützung für NAT-Netzwerke
- **Port Forwarding**: Automatische oder manuelle Port-Forwarding-Konfiguration

**Connection Types**
- **Direct IP**: Direkte Verbindung über IP-Adresse (nur bei expliziter Erlaubnis)
- **Domain-based**: Verbindung über Domain-Name (nur bei expliziter Erlaubnis)
- **Relay through Server**: Verbindung über Relay-Server (Yggdrasil) - Hauptmethode
- **Yggdrasil als Registry**: Hauptsächlich über Yggdrasil als zentrale Registry

### Enhanced Routing

**Routing Strategies**
- **Direct Routing**: Direkte Device-to-Device Verbindung wenn möglich
- **Relay Routing**: Routing über Yggdrasil wenn direkte Verbindung nicht möglich
- **Hybrid Routing**: Kombination aus Direct und Relay

**Routing Features**
- **Path Optimization**: Optimierung der Routing-Pfade
- **Load Balancing**: Lastverteilung über mehrere Pfade
- **Failover**: Automatisches Failover bei Verbindungsausfall
- **Quality-based Routing**: Routing basierend auf Connection-Quality

### Connection Management

**Connection Types**
- **Local Connections**: Verbindungen im lokalen Netzwerk
- **WAN Connections**: Verbindungen über das Internet
- **Hybrid Connections**: Kombination aus Local und WAN

**Connection Features**
- **Connection Pooling**: Pool von Verbindungen
- **Connection Reuse**: Wiederverwendung von Verbindungen
- **Connection Monitoring**: Überwachung von Verbindungen
- **Automatic Reconnection**: Automatische Wiederverbindung (sofort + Exponential Backoff)
- **Error Recovery**: Robustes Error-Handling für Verbindungsfehler
- **Fallback-Routing**: Fallback zu alternativen Routen bei Fehlern

### Relay-Funktionalität

**Yggdrasil als Relay**
- **Yggdrasil als Relay**: Yggdrasil fungiert als zentraler Relay-Server
- **Automatische Auswahl**: System wählt automatisch besten Relay-Server
- **Automatisch bevorzugt**: Automatisch versuchen, Relay bei Bedarf

**Relay-Workflow**
- **Automatisch versuchen**: System versucht automatisch direkte Verbindung
- **Relay bei Bedarf**: Falls direkte Verbindung nicht möglich, automatisch über Relay
- **User kann erzwingen**: User kann Relay-Modus explizit erzwingen

### NAT Traversal

**Automatisches NAT-Traversal**
- **Automatisch bevorzugt**: Automatisches NAT-Traversal wird stark bevorzugt
- **STUN**: STUN-Protokoll für NAT-Discovery
- **TURN**: TURN-Server für Relay wenn NAT-Traversal nicht möglich (Yggdrasil als TURN-Server)
- **ICE**: ICE-Protokoll für optimalen Pfad
- **Fallback auf manuelle Konfiguration**: Falls automatisch nicht möglich, Fallback auf manuelle Port-Forwarding-Konfiguration

### Dynamic IP Handling

**Kombination: DDNS wenn konfiguriert, sonst Relay über Yggdrasil**
- **DDNS**: Dynamic DNS für Domain-Names (wenn User konfiguriert)
- **IP Update Service**: Service für IP-Updates
- **Connection Refresh**: Automatische Connection-Refresh bei IP-Änderung
- **Yggdrasil-Relay**: Falls DDNS nicht konfiguriert, automatisch über Yggdrasil-Relay
- **Sicherheit**: Muss sicher sein und nicht zu kompliziert

## Implementierungs-Notizen

- **Basiert auf Asgard**: Yggdrasil erweitert Asgard um globale Features
- **Alle Services verfügbar**: Odin, Thor, Huginn-Muninn, Freki, Geri, Heimdall, Bifrost sind verfügbar
- **Multi-Tenant-Architektur**: Strikte Isolation zwischen Usern und Netzwerken
- **Netzwerk-Isolation oberste Priorität**: User dürfen nicht in andere Netze eindringen können
- Muss hochverfügbar sein (99.9%+ Uptime)
- Sollte skalierbar sein (horizontal scaling)
- Muss robustes Error-Handling haben
- Sollte Monitoring und Alerting haben
- Muss Security-Best-Practices folgen
- Sollte Backup & Disaster Recovery haben
- Muss Compliance-Anforderungen erfüllen
- Sollte API-Dokumentation haben
- **Muss Payment Methods Management haben**: Add, Verify, Set Default, Remove
- **Muss Pre-Authorization haben**: Für geschätzte Kosten bei Requests
- **Muss Payout Processing haben**: Automatische und manuelle Auszahlungen
- **Muss Netting unterstützen**: Optional Earnings gegen Costs verrechnen
- **Muss PCI-DSS Compliance haben**: Für Kreditkarten-Daten
- **Muss KYC haben**: Für Provider (je nach Region)
- **Muss Provider Registration haben**: User können Devices als Provider registrieren
- **Muss Request Routing Engine haben**: Matching und Routing von Compute-Requests
- **Muss Fair Distribution Algorithm haben**: Für faire Provider-Auswahl
- **Muss Transaction System haben**: Verwaltung von Transactions und Settlement
- **Muss Provider Analytics haben**: Statistiken für Provider
- **Muss Requester Analytics haben**: Statistiken für Requester
- **Muss Quality Assessment haben**: Bewertung der Provider-Quality
- **Muss Load Balancing haben**: Lastverteilung über Provider
- **Muss Failover haben**: Automatisches Failover bei Provider-Ausfall
- **Performance**: Muss optimiert sein für Cloud-Infrastructure und hohen Durchsatz
- **Datenschutz**: Muss Privacy-by-Design implementieren und GDPR vollständig erfüllen
- **Sicherheit**: Muss Enterprise-Grade Security haben mit kontinuierlichem Monitoring und strikter Netzwerk-Isolation


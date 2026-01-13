# Yggdrasil - Main Server

## Übersicht

Yggdrasil ist der Main Server, der von der Company kontrolliert wird. Er erweitert Asgard um globale Features und stellt die zentrale Infrastruktur für das globale Edda-Netzwerk bereit. Yggdrasil ist nicht von Usern selbst hostbar.

**Programmiersprache: Elixir (Erlang VM/BEAM)**
- **Warum Elixir**: Yggdrasil muss Millionen von gleichzeitigen Bifrost-Verbindungen koordinieren
- **Erlang VM**: BEAM ist speziell für massive Concurrency und Fault Tolerance designed
- **Phoenix Channels**: Elixir/Phoenix ist perfekt für WebSocket-Verbindungen (Bifrost)
- **Skalierbarkeit**: Kann Millionen von Devices gleichzeitig über Bifrost verbinden
- **Fault Tolerance**: Eingebaute Fault Tolerance durch Erlang VM

**Rust-Microservices für CPU-intensive Tasks:**
- **Mimir (Mímisbrunnr)**: Privacy Database Service (Rust)
- **Nornen (Urd, Verdandi)**: Decision Service (Rust)
- **Nidhöggr**: Connection Endpoint & Message Receiver (Rust)
- **Njörðr**: Marketplace Service (Rust)
- **Heidrun**: Token & Pricing (Rust)
- **Eikthyrnir**: Quality Assessment (Rust)
- **Læraðr**: Data Management (Rust)

**Kommunikations-Protokolle:**
- **Bifrost**: WebSocket-basiert für Device-zu-Device-Relay und persistente Verbindungen
- **Ratatoskr**: WebSocket-basiert für Business-Logik (Marketplace, Payments, Provider-Registrierung)
- **gRPC**: Für Request/Response-Patterns, Service-zu-Service-Kommunikation und effiziente API-Calls

**Wichtig**: Yggdrasil ist ein eigenständiger Server ohne eigenen Odin. User-Devices (Midgard/Alfheim/Asgard) haben eigene Odin-Instanzen, die direkt mit Yggdrasil kommunizieren.

Yggdrasil erweitert Asgard um:
- Globale Device-Registry (weltweit)
- User-Management und Subscriptions
- Payment-Integration
- Marketplace-Infrastruktur
- Strikte Netzwerk-Isolation zwischen Usern und Netzwerken

**Netzwerk-Sicherheit ist oberste Priorität**: User dürfen nicht in andere Netze eindringen können, nicht einmal wissen, dass andere Netze existieren oder wie diese heißen.

## Projektstruktur

```
yggdrasil/
├── config/
│   ├── config.exs
│   ├── dev.exs
│   ├── prod.exs
│   └── test.exs
├── lib/
│   ├── yggdrasil/
│   │   ├── application.ex      # Application Entry Point
│   │   ├── endpoint.ex         # Phoenix Endpoint
│   │   └── repo.ex             # Ecto Repo
│   ├── yggdrasil_web/
│   │   ├── router.ex           # Phoenix Router
│   │   ├── controllers/        # API Controllers
│   │   │   ├── device_controller.ex
│   │   │   ├── user_controller.ex
│   │   │   ├── subscription_controller.ex
│   │   │   ├── payment_controller.ex
│   │   │   └── marketplace_controller.ex
│   │   └── channels/           # Phoenix Channels (Bifrost & Ratatoskr WebSockets)
│   │       ├── device_channel.ex    # Bifrost-Verbindungen zu Devices
│   │       ├── relay_channel.ex    # Relay-Funktionalität
│   │       └── ratatoskr_channel.ex # Ratatoskr-Protocol Verbindungen
│   ├── yggdrasil/
│   │   ├── device_registry/    # Device Registry Service
│   │   ├── user_management/    # User Management Service
│   │   ├── subscription/       # Subscription Service
│   │   ├── payment/            # Payment Service
│   │   ├── marketplace/        # Marketplace Service
│   │   ├── bifrost/           # Bifrost-Relay Service
│   │   │   ├── connection_manager.ex  # Bifrost-Verbindungen verwalten
│   │   │   ├── message_router.ex     # Message-Routing über Bifrost
│   │   │   └── relay.ex              # Relay-Funktionalität
│   │   ├── ratatoskr/         # Ratatoskr-Protocol Service
│   │   │   └── nidhoggr.ex          # Nidhöggr Connection Endpoint
│   │   └── connection_manager/ # Connection Management (Millionen Verbindungen)
│   └── yggdrasil/
│       ├── schemas/            # Ecto Schemas
│       └── migrations/         # Database Migrations
├── test/
├── mix.exs
└── mix.lock
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

**Token-Limit-Enforcement:**

**Token-Tracking:**
- **Monatliches Token-Budget**: Jeder Subscription-Tier hat monatliches Token-Budget
- **Token-Usage-Tracking**: Verwendete Tokens werden pro User getrackt
- **Real-time Updates**: Token-Usage wird in Echtzeit aktualisiert

**Enforcement-Mechanismus:**
1. **Request kommt an**: User sendet Cloud-LLM-Request
2. **Token-Check**: System prüft, ob User noch Token-Budget hat
3. **Budget verfügbar**: 
   - Request wird verarbeitet
   - Tokens werden vom Budget abgezogen
   - Token-Usage wird aktualisiert
4. **Budget erschöpft**:
   - Request wird blockiert
   - User wird benachrichtigt
   - Automatischer Fallback zu lokalem LLM (siehe unten)

**Token-Usage-Berechnung:**
- **Input-Tokens**: Anzahl der Input-Tokens (Prompt, Context)
- **Output-Tokens**: Anzahl der Output-Tokens (Response)
- **Total-Tokens**: `input_tokens + output_tokens`
- **Budget-Abzug**: Total-Tokens werden vom monatlichen Budget abgezogen

**Monatliches Reset:**

**Reset-Mechanismus:**
- **Reset-Datum**: Jeden Monat am gleichen Tag (basierend auf Subscription-Start-Datum)
- **Automatisches Reset**: Token-Budget wird automatisch zurückgesetzt
- **Reset-Prozess**:
  1. Token-Usage wird auf 0 gesetzt
  2. Token-Budget wird auf Tier-Limit zurückgesetzt
  3. Reset wird in Datenbank protokolliert
  4. User wird benachrichtigt (optional)

**Reset-Zeitpunkt:**
- **UTC-Midnight**: Reset erfolgt um UTC-Midnight am Reset-Datum
- **Timezone-Handling**: Reset erfolgt basierend auf UTC, unabhängig von User-Timezone
- **Grace-Period**: Optional Grace-Period (z.B. 1 Stunde) für laufende Requests

**Automatischer Fallback:**

**Fallback-Trigger:**
- **Token-Budget erschöpft**: Wenn monatliches Token-Budget aufgebraucht ist
- **Subscription-Ausfall**: Wenn Subscription abgelaufen ist (nur für Pay-per-Token)
- **Payment-Fehler**: Wenn Zahlung fehlschlägt

**Fallback-Workflow:**
1. **Token-Budget-Check**: System prüft Token-Budget
2. **Budget erschöpft**: System erkennt, dass Budget erschöpft ist
3. **Fallback-Aktivierung**: Automatischer Fallback zu lokalem LLM wird aktiviert
4. **LLM-Auswahl**: System wählt bestes verfügbares lokales LLM (via Skuld)
5. **User-Benachrichtigung**: User wird via TTS benachrichtigt ("Ich nutze jetzt lokales Modell, da Cloud-Limit erreicht")
6. **Request-Verarbeitung**: Request wird mit lokalem LLM verarbeitet

**Fallback-Hierarchie:**
1. **Lokales Device-LLM**: Zuerst lokales LLM auf eigenem Device
2. **Netzwerk-LLM**: Falls lokales LLM nicht verfügbar, LLM im Netzwerk (via Einherjar Protocol)
3. **Fehlermeldung**: Falls auch kein Netzwerk-LLM verfügbar, Fehlermeldung an User

**Subscription-Status-Tracking:**
- **Active**: Subscription ist aktiv, Token-Budget verfügbar
- **Expired**: Subscription ist abgelaufen, nur lokale LLMs verfügbar
- **Suspended**: Subscription ist suspendiert (z.B. bei Payment-Fehler)
- **Cancelled**: Subscription wurde gekündigt, läuft bis zum Ende des bezahlten Zeitraums

### 4. Payment Integration
- **Payment Providers**: Integration mit Payment-Providern
  - Stripe
  - PayPal
  - Andere Payment-Provider
- **Payment Processing**: Verarbeitung von Zahlungen
- **Invoice Management**: Rechnungsverwaltung
- **Refund Handling**: Rückerstattungen

### 5. Bifrost-Relay-System & User-Isolation

**User-Isolation und Verbindungsregeln:**

**1. Devices eines Users (gleicher User)**
- **Direkte Verbindung möglich**: Yggdrasil erlaubt direkte Verbindungen zwischen Devices desselben Users
- **Yggdrasil kann Verbindung herstellen**: Yggdrasil kann Verbindungen zwischen Devices desselben Users automatisch herstellen und koordinieren
- **Keine Bestätigung nötig**: Da es eigene Devices sind, ist keine explizite Bestätigung erforderlich
- **Direkt oder über Yggdrasil**: Verbindung kann direkt (lokal) oder über Yggdrasil-Relay erfolgen

**2. Devices unterschiedlicher User (verschiedene User)**
- **NICHT direkt verbindbar**: Devices unterschiedlicher User dürfen sich NICHT direkt verbinden
- **Immer über Yggdrasil**: Alle Verbindungen zwischen verschiedenen Usern müssen über Yggdrasil erfolgen
- **Sicherheit**: Verhindert, dass Devices fremdgesteuert werden, wenn es nicht gewollt ist
- **Bezahlmaßnahmen**: Yggdrasil verwaltet auch Bezahlmaßnahmen für Cross-User-Verbindungen (falls implementiert)
- **Strikte Isolation**: Yggdrasil stellt sicher, dass keine direkten Verbindungen zwischen verschiedenen Usern möglich sind

**3. Ausnahme: Gleiches Edda-Netzwerk**
- **Bestätigung erforderlich**: Wenn beide User im gleichen Edda-Netzwerk sind, können sie sich verbinden, ABER es muss eine explizite Bestätigung geben, bevor die Devices sich verbinden dürfen
- **User-Bestätigung**: User muss explizit bestätigen, dass Verbindung erlaubt ist
- **Sicherheitsmaßnahme**: Verhindert ungewollte Verbindungen auch im gleichen Netzwerk
- **Yggdrasil koordiniert**: Yggdrasil koordiniert die Bestätigung und stellt Verbindung her

**Bifrost-Relay-Funktionalität:**
- **Persistente Verbindungen**: Yggdrasil hält persistente Bifrost-WebSocket-Verbindungen zu allen registrierten Devices
- **Message-Routing**: Yggdrasil routet Messages zwischen Devices (nur für Devices desselben Users oder nach Bestätigung)
- **Connection-Management**: Yggdrasil verwaltet alle Bifrost-Verbindungen und stellt sicher, dass User-Isolation eingehalten wird
- **Automatische Verbindungsherstellung**: Yggdrasil kann Verbindungen zwischen Devices desselben Users automatisch herstellen

### 5. Bifrost-Relay-System (Details)

**Bifrost-Funktionalität:**
- **Yggdrasil baut Bifrost-Verbindungen auf**: Yggdrasil baut Bifrost-WebSocket-Verbindungen zu allen registrierten Devices auf
- **Persistente Verbindungen**: Yggdrasil hält persistente Bifrost-Verbindungen zu allen registrierten Devices
- **Connection-Initiation über Bifrost**: Devices verbinden sich über Bifrost, nicht über Webhooks
- **Message-Routing**: Yggdrasil routet Messages zwischen Devices über Bifrost
- **Relay-Funktion**: Wenn direkte Device-zu-Device-Verbindung nicht möglich, routet Yggdrasil Messages über Bifrost
- **Event-Notifications**: Events werden über Bifrost-Messages gesendet (nicht über Webhooks)

**Bifrost für Device-zu-Device-Verbindungen:**
- **Workflow**: 
  1. Device A registriert sich bei Yggdrasil (Bifrost-Verbindung wird etabliert)
  2. Device A möchte sich mit Device B verbinden
  3. **User-Verification**: Yggdrasil prüft, ob beide Devices demselben User gehören
     - **Gleicher User**: Verbindung kann direkt erfolgen, Yggdrasil kann Verbindung automatisch herstellen
     - **Verschiedene User**: Verbindung muss über Yggdrasil erfolgen, direkte Verbindung wird blockiert
  4. Device A sendet Bifrost-Message an Yggdrasil: "Möchte mich mit Device B verbinden"
  5. **User-Isolation-Prüfung**: Yggdrasil prüft User-Identität beider Devices
     - **Gleicher User**: Yggdrasil kann Verbindung automatisch herstellen oder Device B informieren
     - **Verschiedene User**: Yggdrasil sendet Bifrost-Message an Device B: "Device A möchte sich verbinden"
     - **Gleiches Edda-Netzwerk**: Yggdrasil prüft, ob beide User im gleichen Netzwerk sind und ob Bestätigung vorliegt
  6. Device B antwortet über Bifrost (Allow/Deny) - nur bei verschiedenen Usern oder gleichem Netzwerk
  7. Bei Allow: Bifrost-Verbindung zwischen Device A und Device B wird etabliert (direkt oder über Relay)
  8. Persistente Kommunikation über Bifrost (WebSocket)

**Yggdrasil als Bifrost-Relay:**
- **Yggdrasil baut Bifrost-Verbindungen auf**: Yggdrasil kann Bifrost-WebSocket-Verbindungen zu Devices aufbauen
- **Relay-Funktion**: Wenn direkte Device-zu-Device-Verbindung nicht möglich, routet Yggdrasil Messages über Bifrost
- **Persistente Verbindungen**: Yggdrasil hält persistente Bifrost-Verbindungen zu allen registrierten Devices
- **Message-Routing**: Yggdrasil kann Messages zwischen Devices über Bifrost routen
- **Event-Notifications**: Alle Events werden über Bifrost-Messages gesendet

### 6. Marketplace Management (Phase 7)

Der Marketplace ist ein umfassendes System für den Verkauf und die Verteilung von:
- **Compute-Provider**: User können ihre Hardware als Compute-Provider anbieten
- **Plugins**: Vollständige Plugin-Implementierungen für Odin
- **Skills**: Vordefinierte Funktionssammlungen
- **Rules**: Regel-basierte Konfigurationen
- **Commands**: Vordefinierte Command-Sets
- **Snippets für AGENTS.md**: Code-Snippets für Agent-Konfigurationen

#### Compute-Provider Marketplace

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

**Fairness Score-Berechnung:**
```
fairness_score = 1.0 - (provider_usage_count / max_usage_count)
```

- `provider_usage_count`: Anzahl der Requests für diesen Provider (letzte X Stunden, z.B. 24 Stunden)
- `max_usage_count`: Höchste Usage-Count aller Provider
- Niedrigere Usage = höherer Score (Fair Distribution)

**Round-Robin-Mechanismus:**
- **Bei gleichen Scores**: Wenn mehrere Provider den gleichen Gesamt-Score haben, wird Round-Robin verwendet
- **Rotation**: Provider werden rotiert, um faire Verteilung zu gewährleisten
- **Usage-Tracking**: Usage wird pro Provider getrackt (letzte 24 Stunden)

**Quality Weighting:**
- **Gewichteter Durchschnitt**: Quality-Metriken werden als gewichteter Durchschnitt verwendet
- **Time-Decay**: Neuere Requests haben höheres Gewicht
- **Aggregation**: Quality wird aus `provider_quality_aggregates` Tabelle gelesen

**Cost Optimization:**
- **Preis-Filter**: Provider über Max Cost werden ausgeschlossen
- **Preis-Gewichtung**: Niedrigere Preise erhalten höheren Score
- **Balance**: Balance zwischen Preis und Qualität

**Scoring-Formel:**
```
total_score = (price_score * 0.30) + 
              (quality_score * 0.25) + 
              (latency_score * 0.20) + 
              (availability_score * 0.15) + 
              (fairness_score * 0.10)
```

**Provider-Auswahl:**
1. **Filter**: Provider werden gefiltert (Requirements, Max Cost, Availability)
2. **Scoring**: Alle verbleibenden Provider werden gescort
3. **Sortierung**: Provider werden nach Score sortiert
4. **Auswahl**: Bester Provider wird ausgewählt (mit Round-Robin bei gleichen Scores)
5. **Fairness-Update**: Usage-Count wird nach Auswahl aktualisiert

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

**Transaction-Settlement-Workflow:**

**1. Transaction-Erstellung (PENDING)**
- Transaction wird erstellt mit Status `PENDING`
- Pre-Authorization wird durchgeführt (geschätzte Kosten)
- Transaction wird in Datenbank gespeichert

**2. Request-Verarbeitung (PROCESSING)**
- Status wird auf `PROCESSING` gesetzt
- Request wird an Provider gesendet
- Provider verarbeitet Request
- Response wird zurückgesendet

**3. Transaction-Abschluss (COMPLETED)**
- Tokens werden gezählt
- Kosten werden berechnet:
  - `total_cost = (tokens / 1000) * price_per_token` (aufgerundet)
  - `company_fee = total_cost * commission_rate` (10-15%)
  - `provider_earnings = total_cost - company_fee`
- **Payment-Verarbeitung**:
  - Requester wird belastet (`total_cost`)
  - Provider erhält Gutschrift (`provider_earnings`)
  - Company erhält Commission (`company_fee`)
- Status wird auf `COMPLETED` gesetzt
- Transaction wird in Datenbank aktualisiert

**4. Fehlerbehandlung (FAILED/REFUNDED)**
- Bei Provider-Ausfall: Status wird auf `FAILED` gesetzt
- Pre-Authorization wird zurückerstattet (Refund)
- Requester wird nicht belastet
- Provider erhält keine Earnings

**Settlement-Mechanismus:**
- **Sofortiges Settlement**: Settlement erfolgt sofort nach Transaction-Abschluss
- **Batch-Settlement**: Optional Batch-Settlement für Provider-Earnings (täglich/wöchentlich)
- **Netting**: Optional Netting (Earnings gegen Costs) für User, die sowohl Consumer als auch Provider sind

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
   - **Pre-Authorization für geschätzte Kosten** (siehe unten)

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

#### Plugin Marketplace

Der Plugin Marketplace ermöglicht es Usern, ihre eigenen Plugins zu entwickeln und zu verkaufen.

**Plugin-Entwicklung**
- **Interface-basiert**: Alle Plugins müssen das `OdinPlugin`-Interface implementieren
- **Function Call Protocol**: Plugins müssen ihre Funktionen über das Function Call Protocol beschreiben
- **Schnelle Erkennung**: Odin kann Plugins sofort verstehen, ohne Code-Analyse

**Plugin-Publishing**
- **Plugin-Registrierung**: Developer können Plugins registrieren
- **Plugin-Beschreibung**: Titel, Beschreibung, Funktionen werden über Interface bereitgestellt
- **Pricing**: Developer können Preise für ihre Plugins festlegen
- **Versioning**: Plugin-Versionen werden verwaltet

**Plugin-Kategorien**
- **Action-Execution**: Plugins für Action-Execution (z.B. Thor)
- **Coding**: Plugins für Coding-Aufgaben (z.B. Valkyries)
- **Healthcare**: Plugins für Gesundheitsfragen (z.B. Frigg)
- **Custom**: Benutzerdefinierte Plugins

**Plugin-Installation**
- **Marketplace-Installation**: User können Plugins direkt aus dem Marketplace installieren
- **Lokale Installation**: User können Plugins lokal installieren
- **Plugin-Management**: User können installierte Plugins verwalten

#### Skills, Rules, Commands & Snippets Marketplace

Der Marketplace unterstützt auch den Verkauf von:
- **Skills**: Vordefinierte Funktionssammlungen für spezifische Aufgaben
- **Rules**: Regel-basierte Konfigurationen für Agent-Verhalten
- **Commands**: Vordefinierte Command-Sets für häufige Aufgaben
- **Snippets für AGENTS.md**: Code-Snippets für Agent-Konfigurationen, die in AGENTS.md-Dateien verwendet werden können

**Publishing**
- **Content-Registrierung**: Developer können Skills, Rules, Commands und Snippets registrieren
- **Pricing**: Developer können Preise für ihre Inhalte festlegen
- **Kategorisierung**: Inhalte werden kategorisiert für bessere Auffindbarkeit

**Installation & Verwendung**
- **Marketplace-Installation**: User können Inhalte direkt aus dem Marketplace installieren
- **Integration**: Inhalte werden automatisch in die entsprechende Konfiguration integriert
- **Verwaltung**: User können installierte Inhalte verwalten und aktualisieren

### 6. Kommunikations-Protokolle

Yggdrasil unterstützt mehrere Kommunikations-Protokolle, je nach Use-Case:

#### Bifrost Protocol
- **WebSocket-basiert**: Für persistente Verbindungen
- **Device-zu-Device-Relay**: Routing von Messages zwischen Devices
- **Event-Notifications**: Kontinuierliche Event-Streams
- **Connection Management**: Persistente Verbindungen zu allen Devices

#### Ratatoskr Protocol
- **WebSocket-basiert**: Für Business-Logik-Kommunikation
- **Speziell für**: Marketplace, Payments, Provider-Registrierung
- **Extra Absicherung**: Zusätzliche Verschlüsselung, Audit-Logging, Rate-Limiting
- **Persistente Verbindungen**: Für kontinuierliche Business-Kommunikation

**Protocol-Features:**
- WebSocket-basiert
- TLS 1.3 Encryption
- Zusätzliche Security-Layer:
  - Message-Signierung
  - Audit-Logging
  - Rate-Limiting
  - Request-Validation

**Message-Types:**
- `BUSINESS_REQUEST`: Business-Transaktionen
- `MARKETPLACE_REQUEST`: Marketplace-Operationen
- `PAYMENT_REQUEST`: Payment-Operationen
- `PROVIDER_REGISTRATION`: Provider-Registrierung
- `HEARTBEAT`: Keep-Alive
- `DISCONNECT`: Verbindung beenden

#### gRPC Protocol
- **gRPC-basiert**: Für Request/Response-Patterns
- **Type-safe**: Protobuf für alle Service-Interfaces
- **Effizient**: HTTP/2, Binary-Format, weniger Overhead als WebSocket für einzelne Requests
- **Bessere Performance**: Schnellere Serialisierung, HTTP/2 Multiplexing
- **Streaming**: Built-in Streaming für große Responses
- **Error-Handling**: Besseres Error-Handling mit Status-Codes

**Wann gRPC verwenden:**
- **Request/Response-Patterns**: Für einzelne API-Calls (Device-Registry, User-Management, etc.)
- **Service-zu-Service**: Für Kommunikation mit Rust-Microservices
- **Effiziente API-Calls**: Wenn keine persistente Verbindung nötig ist
- **Type-Safe Communication**: Für strukturierte Daten mit Protobuf

**Wann Ratatoskr verwenden:**
- **Persistente Business-Verbindungen**: Für kontinuierliche Business-Kommunikation
- **Event-Streaming**: Für kontinuierliche Business-Events
- **Real-time Messaging**: Für Echtzeit-Business-Kommunikation
- **Audit-Logging**: Wenn vollständiges Audit-Logging erforderlich ist

**Unterschied zu Bifrost:**
- Bifrost: Device-zu-Device-Kommunikation (lokal und global)
- Ratatoskr: Business-Logik-Kommunikation (nur Yggdrasil, extra abgesichert)
- gRPC: Request/Response-Patterns und Service-zu-Service-Kommunikation

### 7. Rust-Microservices (Wesen am Weltenbaum)

Yggdrasil koordiniert mehrere Rust-Microservices für CPU-intensive Berechnungen. Diese Services kommunizieren über gRPC mit Yggdrasil.

#### Mimir (Mímisbrunnr) - Privacy Database Service

**Mythologische Bedeutung**: Mimir ist der Wächter des Brunnens Mímisbrunnr (Brunnen der Weisheit). Der Brunnen selbst ist die Datenbank.

**Programmiersprache**: Rust

**Aufgaben:**
- **Privacy Database Service**: Eigene, isolierte Datenbank für personenbezogene Daten
- **Extra Sicherheitsschicht**: Verschlüsselung, Access Control, Audit-Logging
- **GDPR-Compliance**: Vollständige Einhaltung der GDPR-Anforderungen
- **Query-Optimierung**: Optimierte Datenbankabfragen
- **Connection-Pooling**: Effizientes Connection-Pooling
- **Transaction-Management**: Verwaltung von Datenbank-Transactions
- **Database-Sharding**: Unterstützung für Database-Sharding
- **Caching-Integration**: Integration mit Redis/Memcached

**Kommunikation:**
- Yggdrasil (Elixir) ↔ Mimir (Rust): gRPC
- Asynchron: Yggdrasil sendet Query-Requests, Mimir antwortet asynchron

#### Nidhöggr - Connection Endpoint & Message Receiver

**Mythologische Bedeutung**: Nidhöggr ist der Drache, der an den Wurzeln des Weltenbaums nagt. Repräsentiert User Requests (Root/Wurzeln).

**Programmiersprache**: Rust

**Aufgaben:**
- **Connection Endpoint**: Server-Seite bei Yggdrasil
- **Empfängt Verbindungen**: Von Vedrfolnir-Clients (User-Devices)
- **Empfängt Nachrichten**: Über Ratatoskr-Protocol
- **Message-Routing**: Leitet Nachrichten direkt weiter an Nornen (und andere Services je nach Message-Type)
- **Connection Management**: Verwaltet Verbindungen zu User-Devices
- **Connection Termination**: Kann Verbindungen trennen (bei bestimmten Umständen)

**Kommunikation:**
- User-Devices (Vedrfolnir) ↔ Nidhöggr: Ratatoskr-Protocol (WebSocket) oder gRPC (je nach Use-Case)
- Nidhöggr ↔ Nornen/andere Services: gRPC
- Asynchron: Nidhöggr leitet Nachrichten weiter an entsprechende Services
- **gRPC für Request/Response**: Direkte gRPC-Verbindungen für einzelne API-Calls (Device-Registry, User-Management, etc.)

#### Heidrun - Token & Pricing Service

**Mythologische Bedeutung**: Heidrun ist die Ziege, die Met produziert (Wert/Flüssigkeit).

**Programmiersprache**: Rust

**Aufgaben:**
- **Token-Berechnungen**: Token-Counting nach Request-Verarbeitung
- **Pricing**: Berechnung von Kosten basierend auf Token-Verbrauch
- **Pricing-Model**: Cent-Berechnung pro 1000 Tokens (ganzzahlig, keine Kommastellen)
- **Settlement**: Berechnung von Provider-Earnings und Company-Fee
- **Pre-Authorization**: Pre-Authorization für geschätzte Kosten vor Request
- **Commission-Berechnung**: Berechnung der Company-Commission (10-15%)

**Berechnungsformel:**
- `(tokens / 1000) * pricePerToken` (aufgerundet)
- `providerEarnings = totalCost - companyFee`
- `companyFee = totalCost * commissionRate`

**Kommunikation:**
- Yggdrasil (Elixir) ↔ Heidrun (Rust): gRPC
- Asynchron: Yggdrasil sendet Token-Requests, Heidrun antwortet mit Berechnungen

#### Nornen (Urd, Verdandi) - Decision Service

**Mythologische Bedeutung**: Die Nornen sind die Schicksalsgöttinnen. Urd (Vergangenheit), Verdandi (Gegenwart). **Hinweis**: Skuld ist ein separater Service, der auf allen Devices installiert werden muss.

**Programmiersprache**: Rust

**Aufgaben:**
- **Urd (Vergangenheit)**: Historie, Request-History, historische Statistiken
- **Verdandi (Gegenwart)**: Aktuelle Statistiken, Real-time Analytics, Live-Metriken
- **Entscheidungen über Requests**: Entscheidungen über eingehende/ausgehende Requests
- **Provider-Registrierung**: Genehmigung und Verwaltung von Provider-Registrierungen
- **User-Konfiguration**: Speichern der User-Konfiguration für Marketplace
- **Admin API**: Health Check, Dashboard, Monitoring, Admin-Informationen
- **Der Brunnen (Mímisbrunnr)**: Datenbank (von Mimir verwaltet)

**Analytics-Features:**
- **Provider Analytics**: Request Statistics, Earnings, Quality Metrics, Usage Patterns
- **Requester Analytics**: Request History, Cost Analysis, Quality Metrics der verwendeten Provider
- **Aggregation**: Aggregation von Daten über Zeiträume
- **Trend-Analyse**: Erkennung von Trends und Mustern

**Kommunikation:**
- Yggdrasil (Elixir) ↔ Nornen (Rust): gRPC
- Nidhöggr → Nornen: gRPC (Nachrichten-Weiterleitung)
- Asynchron: Yggdrasil/Nidhöggr sendet Requests, Nornen antworten mit Entscheidungen/Statistiken

#### Eikthyrnir - Quality Assessment Service

**Mythologische Bedeutung**: Eikthyrnir ist der Hirsch, der aus dem Brunnen trinkt. Die Tropfen werden zu Flüssen (Qualität fließt weiter).

**Programmiersprache**: Rust

**Aufgaben:**
- **Quality Assessment**: Bewertung der Provider-Quality nach jedem Request
- **Quality-Aggregation**: Gewichteter Durchschnitt von Quality-Metriken
- **Quality-Updates**: Sofortige Updates für wichtige Änderungen, Batch-Aggregation für Effizienz
- **Quality-Metriken**: Messung von Response-Quality, Latency, Availability
- **Quality-Weighting**: Neuere Requests haben höheres Gewicht

**Quality-Messung:**
- Nach jedem Request: Automatische Quality-Messung
- Periodische Tests: Regelmäßige Tests ergänzen kontinuierliche Bewertung
- Gewichteter Durchschnitt: Quality-Metriken werden aggregiert
- Sofort + Batch: Sofortige Updates für wichtige Änderungen, Batch-Aggregation für Effizienz

**Kommunikation:**
- Yggdrasil (Elixir) ↔ Eikthyrnir (Rust): gRPC
- Asynchron: Yggdrasil sendet Quality-Assessment-Requests, Eikthyrnir antwortet mit Quality-Metriken

#### Læraðr (Dáinn, Dvalinn, Duneyrr, Duraþrór) - Data Management Service

**Mythologische Bedeutung**: Læraðr ist der Baum, an dem die vier Hirsche (Dáinn, Dvalinn, Duneyrr, Duraþrór) knabbern. Die vier Hirsche knabbern an den Ästen des Weltenbaums.

**Programmiersprache**: Rust

**Aufgaben:**
- **Dáinn**: Data Indexing (Indizierung, Suche)
- **Dvalinn**: Data Validation (Validierung, Schema-Checks)
- **Duneyrr**: Data Aggregation (Aggregation, Statistiken)
- **Duraþrór**: Data Retention (Retention, Archiving, Cleanup)

**Data Management Features:**
- **Ordnung der Daten**: Verwaltung und Organisation von Daten innerhalb Yggdrasil
- **Data Integrity**: Sicherstellung der Datenintegrität
- **Data Lifecycle**: Verwaltung des Datenlebenszyklus
- **Data Cleanup**: Automatische Bereinigung von alten Daten

**Kommunikation:**
- Yggdrasil (Elixir) ↔ Læraðr (Rust): gRPC
- Asynchron: Yggdrasil sendet Data-Management-Requests, Læraðr antwortet mit Ergebnissen

### 8. Global Lock Management
- **Distributed Locking**: Verwaltet Locks für globale Resources
- **Lock-Registry**: Zentrale Registry für alle aktiven Locks
- **Lock-Expiration**: Verwaltet Lock-Expiration (Timeout)
- **Deadlock-Detection**: Erkennt Deadlocks und löst sie auf
- **Priority-Management**: Verwaltet Prioritäten für Lock-Requests
- **Integration**: Global Lock Management wird von Mimir (Database Service) verwaltet

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
```sql
CREATE TABLE providers (
    provider_id UUID PRIMARY KEY,
    user_id UUID NOT NULL REFERENCES users(user_id),
    device_id UUID NOT NULL REFERENCES devices(device_id),
    pricing_per_1000_tokens INTEGER NOT NULL,  -- in cents
    available_models JSONB NOT NULL,
    hardware_specs JSONB,
    capacity_limits JSONB,
    availability_settings JSONB,
    quality_settings JSONB,
    status VARCHAR(20) NOT NULL DEFAULT 'inactive',
    registered_at TIMESTAMP NOT NULL DEFAULT NOW(),
    last_seen TIMESTAMP NOT NULL DEFAULT NOW()
);

-- Indizes
CREATE INDEX idx_providers_user_id ON providers(user_id);
CREATE INDEX idx_providers_device_id ON providers(device_id);
CREATE INDEX idx_providers_status ON providers(status);
CREATE INDEX idx_providers_pricing ON providers(pricing_per_1000_tokens);
CREATE INDEX idx_providers_last_seen ON providers(last_seen);
CREATE INDEX idx_providers_available_models ON providers USING GIN(available_models);
CREATE INDEX idx_providers_active ON providers(status, last_seen) WHERE status = 'active';
```

### Transactions Table (Phase 7)
```sql
CREATE TABLE transactions (
    transaction_id UUID PRIMARY KEY,
    request_id UUID NOT NULL,
    provider_device_id UUID NOT NULL REFERENCES devices(device_id),
    requester_device_id UUID NOT NULL REFERENCES devices(device_id),
    tokens_used INTEGER NOT NULL,
    cost_per_token INTEGER NOT NULL,  -- in cents per 1000 tokens
    total_cost INTEGER NOT NULL,  -- in cents
    company_fee INTEGER NOT NULL,  -- in cents
    provider_earnings INTEGER NOT NULL,  -- in cents
    status VARCHAR(20) NOT NULL DEFAULT 'pending',
    completed_at TIMESTAMP,
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);

-- Indizes
CREATE INDEX idx_transactions_request_id ON transactions(request_id);
CREATE INDEX idx_transactions_provider_device ON transactions(provider_device_id);
CREATE INDEX idx_transactions_requester_device ON transactions(requester_device_id);
CREATE INDEX idx_transactions_status ON transactions(status);
CREATE INDEX idx_transactions_completed_at ON transactions(completed_at) WHERE completed_at IS NOT NULL;
CREATE INDEX idx_transactions_created_at ON transactions(created_at);
```

### Quality Metrics Table
```sql
CREATE TABLE quality_metrics (
    metric_id UUID PRIMARY KEY,
    provider_device_id UUID NOT NULL REFERENCES devices(device_id),
    request_id UUID NOT NULL,
    transaction_id UUID REFERENCES transactions(transaction_id),
    quality_score DECIMAL(5,2) NOT NULL,  -- 0.00 - 100.00
    latency_ms INTEGER NOT NULL,
    accuracy_score DECIMAL(5,2),
    completeness_score DECIMAL(5,2),
    consistency_score DECIMAL(5,2),
    user_feedback_score DECIMAL(5,2),  -- optional
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);

-- Indizes
CREATE INDEX idx_quality_metrics_provider_device ON quality_metrics(provider_device_id);
CREATE INDEX idx_quality_metrics_request_id ON quality_metrics(request_id);
CREATE INDEX idx_quality_metrics_transaction_id ON quality_metrics(transaction_id);
CREATE INDEX idx_quality_metrics_created_at ON quality_metrics(created_at);
CREATE INDEX idx_quality_metrics_provider_created ON quality_metrics(provider_device_id, created_at);
```

### Provider Quality Aggregates Table
```sql
CREATE TABLE provider_quality_aggregates (
    aggregate_id UUID PRIMARY KEY,
    provider_device_id UUID NOT NULL UNIQUE REFERENCES devices(device_id),
    weighted_average_quality DECIMAL(5,2) NOT NULL,  -- 0.00 - 100.00
    total_requests INTEGER NOT NULL DEFAULT 0,
    successful_requests INTEGER NOT NULL DEFAULT 0,
    failed_requests INTEGER NOT NULL DEFAULT 0,
    average_latency_ms INTEGER NOT NULL DEFAULT 0,
    last_updated TIMESTAMP NOT NULL DEFAULT NOW()
);

-- Indizes
CREATE INDEX idx_provider_quality_aggregates_provider_device ON provider_quality_aggregates(provider_device_id);
CREATE INDEX idx_provider_quality_aggregates_quality ON provider_quality_aggregates(weighted_average_quality);
CREATE INDEX idx_provider_quality_aggregates_last_updated ON provider_quality_aggregates(last_updated);
```

**Quality-Metriken-Speicherung:**
- **Granulare Metriken**: Jede Request hat eigene Quality-Metrik-Einträge in `quality_metrics` Tabelle
- **Aggregierte Metriken**: Periodisch aggregierte Metriken in `provider_quality_aggregates` Tabelle
- **Time-Decay-Aggregation**: Gewichteter Durchschnitt mit Time-Decay (neuere Requests haben höheres Gewicht)
- **Automatische Aggregation**: Aggregation erfolgt automatisch (z.B. stündlich) oder on-demand

**Quality-Metriken-Aggregation (Detailliert):**

**Gewichteter Durchschnitt (Time-Decay):**
```
weighted_average = Σ(quality_metric_i * weight_i) / Σ(weight_i)
weight_i = e^(-decay_rate * age_i)
```

- `decay_rate`: 0.1 (10% Decay pro Stunde)
- `age_i`: Alter des Requests in Stunden
- Neuere Requests haben exponentiell höheres Gewicht

**Aggregations-Intervall:**
- **Sofort-Update**: Nach jedem Request wird `provider_quality_aggregates` aktualisiert (für aktuellen Score)
- **Periodische Aggregation**: Stündlich wird vollständige Aggregation durchgeführt (für langfristige Trends)
- **Rolling Window**: Letzte 24 Stunden werden berücksichtigt

**Quality-Metrik-Berechnung (pro Request):**
```
quality_metric = (latency_score * 0.20) + 
                  (accuracy_score * 0.40) + 
                  (completeness_score * 0.25) + 
                  (consistency_score * 0.15)
```

- **Latency-Score**: Basierend auf Response-Zeit (niedrigere Latency = höherer Score)
- **Accuracy-Score**: Automatische Bewertung basierend auf Response-Qualität (0-100)
- **Completeness-Score**: Vollständigkeit der Response (Token-Count, Coverage) (0-100)
- **Consistency-Score**: Konsistenz mit vorherigen Responses (0-100)

**User-Feedback-Integration (optional):**
- **Thumbs Up/Down**: Einfaches Feedback (optional)
- **Rating (1-5)**: Detailliertes Rating (optional)
- **Text-Feedback**: Freitext-Feedback (optional)
- **Feedback-Gewichtung**: User-Feedback wird mit 20% Gewichtung in Quality-Metrik einbezogen (falls vorhanden)

## API Endpoints

### API-Authentifizierung

**JWT-basierte Authentifizierung:**
- **Token-Format**: JWT (JSON Web Token)
- **Token-Header**: `Authorization: Bearer <token>`
- **Token-Generierung**: Token wird nach erfolgreichem Login generiert (via Heimdall)
- **Token-Validierung**: Jeder Request wird auf gültiges Token geprüft
- **Token-Expiration**: Token haben Ablaufzeit (z.B. 24 Stunden)
- **Refresh-Token**: Refresh-Token für Token-Erneuerung (ohne erneutes Login)

**Authentifizierungs-Workflow:**
1. **Login**: User sendet Credentials (`POST /api/v1/users/login`)
2. **Token-Generierung**: System generiert JWT-Token (via Heimdall)
3. **Token-Response**: Token wird an Client zurückgegeben
4. **Token-Usage**: Client sendet Token in `Authorization`-Header bei jedem Request
5. **Token-Validierung**: Server validiert Token bei jedem Request
6. **Token-Refresh**: Bei abgelaufenem Token kann Refresh-Token verwendet werden

**OAuth-Integration (Optional):**
- **OAuth-Provider**: Unterstützung für OAuth-Provider (Google, GitHub, etc.)
- **OAuth-Flow**: Standard OAuth 2.0 Flow
- **OAuth-Token**: OAuth-Token wird in JWT-Token konvertiert

### Request/Response-Formate

**Request-Format:**
- **Content-Type**: `application/json`
- **Accept**: `application/json`
- **Encoding**: UTF-8
- **Request-Body**: JSON-Format für POST/PUT-Requests

**Response-Format:**
- **Content-Type**: `application/json`
- **Encoding**: UTF-8
- **Response-Body**: JSON-Format

**Standard-Response-Struktur:**
```json
{
  "success": true,
  "data": { ... },
  "error": null,
  "timestamp": "2024-01-01T00:00:00Z"
}
```

**Error-Response-Struktur:**
```json
{
  "success": false,
  "data": null,
  "error": {
    "code": "ERROR_CODE",
    "message": "Error message",
    "details": { ... }
  },
  "timestamp": "2024-01-01T00:00:00Z"
}
```

**HTTP-Status-Codes:**
- `200 OK`: Erfolgreiche Request
- `201 Created`: Ressource erfolgreich erstellt
- `400 Bad Request`: Ungültige Request (Validierungsfehler)
- `401 Unauthorized`: Authentifizierung fehlgeschlagen
- `403 Forbidden`: Keine Berechtigung
- `404 Not Found`: Ressource nicht gefunden
- `429 Too Many Requests`: Rate-Limit überschritten
- `500 Internal Server Error`: Server-Fehler

### Rate-Limits

**Rate-Limit-Strategie:**
- **Token-basiert**: Rate-Limits werden pro Token/User angewendet
- **IP-basiert**: Zusätzliche IP-basierte Rate-Limits (für DDoS-Schutz)
- **Endpoint-spezifisch**: Verschiedene Rate-Limits für verschiedene Endpoints

**Rate-Limit-Werte (Beispiele):**
- **Standard-Endpoints**: 100 Requests/Minute pro User
- **Login-Endpoint**: 5 Requests/Minute pro IP
- **Registration-Endpoint**: 3 Requests/Minute pro IP
- **Payment-Endpoints**: 20 Requests/Minute pro User
- **Marketplace-Endpoints**: 50 Requests/Minute pro User

**Rate-Limit-Headers:**
- `X-RateLimit-Limit`: Maximale Anzahl von Requests
- `X-RateLimit-Remaining`: Verbleibende Requests
- `X-RateLimit-Reset`: Zeitpunkt, wann Rate-Limit zurückgesetzt wird

**Rate-Limit-Response (429):**
```json
{
  "success": false,
  "error": {
    "code": "RATE_LIMIT_EXCEEDED",
    "message": "Rate limit exceeded",
    "retry_after": 60
  }
}
```

### User Management

**Endpoints:**
- `POST /api/v1/users/register` - Register new user
  - **Request**: `{ "email": "...", "password": "...", "name": "..." }`
  - **Response**: `{ "user": { ... }, "token": "..." }`
- `POST /api/v1/users/login` - User login
  - **Request**: `{ "email": "...", "password": "..." }`
  - **Response**: `{ "user": { ... }, "token": "...", "refresh_token": "..." }`
- `GET /api/v1/users/me` - Get current user
  - **Auth**: Required (JWT Token)
  - **Response**: `{ "user": { ... } }`
- `PUT /api/v1/users/me` - Update user profile
  - **Auth**: Required (JWT Token)
  - **Request**: `{ "name": "...", "email": "..." }`
  - **Response**: `{ "user": { ... } }`
- `DELETE /api/v1/users/me` - Delete user account
  - **Auth**: Required (JWT Token)
  - **Response**: `{ "success": true }`

### Device Management

**Endpoints:**
- `GET /api/v1/devices` - List user devices
  - **Auth**: Required (JWT Token)
  - **Response**: `{ "devices": [ ... ] }`
- `POST /api/v1/devices` - Register device
  - **Auth**: Required (JWT Token)
  - **Request**: `{ "device_id": "...", "name": "...", "type": "..." }`
  - **Response**: `{ "device": { ... } }`
- `GET /api/v1/devices/:id` - Get device details
  - **Auth**: Required (JWT Token)
  - **Response**: `{ "device": { ... } }`
- `PUT /api/v1/devices/:id` - Update device
  - **Auth**: Required (JWT Token)
  - **Request**: `{ "name": "...", "type": "..." }`
  - **Response**: `{ "device": { ... } }`
- `DELETE /api/v1/devices/:id` - Unregister device
  - **Auth**: Required (JWT Token)
  - **Response**: `{ "success": true }`

### Subscription Management

**Endpoints:**
- `GET /api/v1/subscriptions` - Get user subscription
  - **Auth**: Required (JWT Token)
  - **Response**: `{ "subscription": { ... } }`
- `POST /api/v1/subscriptions` - Create subscription
  - **Auth**: Required (JWT Token)
  - **Request**: `{ "tier": "...", "payment_method_id": "..." }`
  - **Response**: `{ "subscription": { ... } }`
- `PUT /api/v1/subscriptions` - Update subscription
  - **Auth**: Required (JWT Token)
  - **Request**: `{ "tier": "...", "payment_method_id": "..." }`
  - **Response**: `{ "subscription": { ... } }`
- `DELETE /api/v1/subscriptions` - Cancel subscription
  - **Auth**: Required (JWT Token)
  - **Response**: `{ "success": true }`

### Payment Management

**Endpoints:**
- `POST /api/v1/payments/methods` - Add payment method
  - **Auth**: Required (JWT Token)
  - **Request**: `{ "type": "...", "details": { ... } }`
  - **Response**: `{ "payment_method": { ... } }`
- `GET /api/v1/payments/methods` - List payment methods
  - **Auth**: Required (JWT Token)
  - **Response**: `{ "payment_methods": [ ... ] }`
- `DELETE /api/v1/payments/methods/:id` - Remove payment method
  - **Auth**: Required (JWT Token)
  - **Response**: `{ "success": true }`
- `POST /api/v1/payments/process` - Process payment
  - **Auth**: Required (JWT Token)
  - **Request**: `{ "amount": 100, "currency": "EUR", "payment_method_id": "..." }`
  - **Response**: `{ "transaction": { ... } }`

### Marketplace (Phase 7)

**Endpoints:**
- `POST /api/v1/marketplace/providers/register` - Register device as provider
  - **Auth**: Required (JWT Token)
  - **Request**: `{ "device_id": "...", "capabilities": [ ... ], "pricing": { ... } }`
  - **Response**: `{ "provider": { ... } }`
- `GET /api/v1/marketplace/providers` - List available providers
  - **Auth**: Required (JWT Token)
  - **Query-Params**: `?capability=...&min_quality=...`
  - **Response**: `{ "providers": [ ... ] }`
- `GET /api/v1/marketplace/providers/:id` - Get provider details
  - **Auth**: Required (JWT Token)
  - **Response**: `{ "provider": { ... } }`
- `PUT /api/v1/marketplace/providers/:id` - Update provider configuration
  - **Auth**: Required (JWT Token)
  - **Request**: `{ "capabilities": [ ... ], "pricing": { ... } }`
  - **Response**: `{ "provider": { ... } }`
- `DELETE /api/v1/marketplace/providers/:id` - Unregister as provider
  - **Auth**: Required (JWT Token)
  - **Response**: `{ "success": true }`
- `POST /api/v1/marketplace/requests` - Create compute request
  - **Auth**: Required (JWT Token)
  - **Request**: `{ "provider_id": "...", "request": { ... } }`
  - **Response**: `{ "request": { ... } }`
- `GET /api/v1/marketplace/requests/:id` - Get request status
  - **Auth**: Required (JWT Token)
  - **Response**: `{ "request": { ... } }`
- `GET /api/v1/marketplace/transactions` - List transactions
  - **Auth**: Required (JWT Token)
  - **Query-Params**: `?type=provider|requester&limit=...&offset=...`
  - **Response**: `{ "transactions": [ ... ] }`
- `GET /api/v1/marketplace/transactions/:id` - Get transaction details
  - **Auth**: Required (JWT Token)
  - **Response**: `{ "transaction": { ... } }`
- `GET /api/v1/marketplace/analytics/provider` - Get provider analytics
  - **Auth**: Required (JWT Token)
  - **Query-Params**: `?start_date=...&end_date=...`
  - **Response**: `{ "analytics": { ... } }`
- `GET /api/v1/marketplace/analytics/requester` - Get requester analytics
  - **Auth**: Required (JWT Token)
  - **Query-Params**: `?start_date=...&end_date=...`
  - **Response**: `{ "analytics": { ... } }`

## Infrastructure

### Deployment-Strategien

**Zero-Downtime-Deployments:**

**Blue-Green-Deployment:**
- **Zwei Umgebungen**: Blue (aktuell) und Green (neu)
- **Parallele Bereitstellung**: Neue Version wird in Green-Umgebung deployed
- **Health-Check**: Green-Umgebung wird auf Health geprüft
- **Traffic-Switch**: Traffic wird von Blue zu Green umgeschaltet (via Load-Balancer)
- **Rollback**: Bei Problemen kann Traffic sofort zurück zu Blue geschaltet werden

**Rolling-Update-Deployment:**
- **Schrittweise Updates**: Instanzen werden schrittweise aktualisiert
- **Batch-Update**: Instanzen werden in Batches aktualisiert (z.B. 10% pro Batch)
- **Health-Check**: Jeder Batch wird auf Health geprüft vor nächstem Batch
- **Rollback**: Bei Problemen kann Update gestoppt werden (verbleibende Instanzen bleiben auf alter Version)

**Canary-Deployment:**
- **Kleine Gruppe**: Neue Version wird zunächst auf kleiner Gruppe deployed (z.B. 5% der Instanzen)
- **Monitoring**: Canary-Instanzen werden intensiv überwacht
- **Graduelle Ausweitung**: Bei Erfolg wird Deployment auf weitere Instanzen ausgeweitet
- **Rollback**: Bei Problemen wird Canary-Deployment sofort gestoppt

**Deployment-Pipeline:**

**CI/CD-Pipeline:**
1. **Code-Commit**: Code wird in Repository committed
2. **Automated Tests**: Automatische Tests werden ausgeführt (Unit, Integration, E2E)
3. **Build**: Application wird gebaut (Docker-Image)
4. **Image-Registry**: Docker-Image wird in Registry gepusht
5. **Staging-Deployment**: Deployment in Staging-Umgebung
6. **Staging-Tests**: Tests in Staging-Umgebung
7. **Production-Deployment**: Deployment in Production (mit gewählter Strategie)
8. **Health-Check**: Health-Check nach Deployment
9. **Monitoring**: Kontinuierliche Überwachung nach Deployment

**Rollback-Mechanismus:**

**Automatischer Rollback:**
- **Health-Check-Failure**: Bei Health-Check-Failure wird automatisch zurückgerollt
- **Error-Rate-Threshold**: Bei Error-Rate über Threshold wird automatisch zurückgerollt
- **Latency-Threshold**: Bei Latency über Threshold wird automatisch zurückgerollt
- **Rollback-Zeit**: Rollback erfolgt innerhalb von Sekunden

**Manueller Rollback:**
- **One-Click-Rollback**: Manueller Rollback mit einem Klick (via Deployment-Tool)
- **Version-Selection**: Vorherige Version kann ausgewählt werden
- **Rollback-Confirmation**: Rollback erfordert Bestätigung (Sicherheit)

**Deployment-Tools:**
- **Kubernetes**: Container-Orchestrierung (für Rolling-Updates, Blue-Green, Canary)
- **Docker**: Container-Images
- **CI/CD**: GitHub Actions, GitLab CI, Jenkins (für Pipeline)
- **Helm**: Kubernetes Package Manager (für Deployment-Management)
- **Cloud Providers**: AWS, GCP, Azure
- **Multi-Region**: Deployment in mehreren Regionen

### Scalability
- **Horizontal Scaling**: Skalierung über mehrere Instanzen
- **Load Balancing**: Lastverteilung
- **Database Sharding**: Datenbank-Sharding
- **Caching**: Redis/Memcached für Caching

### Monitoring & Logging

**Monitoring-Tools:**
- **Prometheus**: Metriken-Sammlung und -Speicherung
- **Grafana**: Visualisierung und Dashboards
- **Jaeger**: Distributed Tracing (optional, für Request-Tracing)
- **Alertmanager**: Alert-Management (für Prometheus-Alerts)

**Performance-Monitoring:**
- **Application-Metriken**: Response-Zeiten, Durchsatz, Error-Rate
- **Database-Metriken**: Query-Zeit, Connection-Pool-Usage, Transaction-Rate
- **Network-Metriken**: Latency, Throughput, Connection-Count
- **Resource-Metriken**: CPU, Memory, Disk, Network-Usage

**Health-Checks:**
- **Liveness-Probe**: Prüft, ob Service läuft (für Kubernetes)
- **Readiness-Probe**: Prüft, ob Service bereit für Traffic ist (für Kubernetes)
- **Custom Health-Endpoints**: Custom Health-Endpoints für detaillierte Health-Checks
- **Health-Check-Interval**: Regelmäßige Health-Checks (z.B. alle 10 Sekunden)

**Alerting:**

**Alert-Konfiguration:**
- **Alert-Rules**: Alert-Rules werden in Prometheus konfiguriert
- **Alert-Thresholds**: Schwellenwerte für Alerts (z.B. Error-Rate > 5%, Latency > 200ms)
- **Alert-Channels**: Alerts werden an verschiedene Channels gesendet (Email, Slack, PagerDuty)
- **Alert-Severity**: Alerts haben verschiedene Severity-Levels (Critical, Warning, Info)

**Alert-Types:**
- **Error-Rate-Alerts**: Alerts bei hoher Error-Rate
- **Latency-Alerts**: Alerts bei hoher Latency
- **Resource-Alerts**: Alerts bei hoher Resource-Usage (CPU, Memory)
- **Health-Check-Alerts**: Alerts bei Health-Check-Failure
- **Deployment-Alerts**: Alerts bei Deployment-Problemen

**Centralized Logging:**

**Logging-Stack:**
- **ELK Stack**: Elasticsearch, Logstash, Kibana (für Centralized Logging)
- **Fluentd/Fluent Bit**: Log-Forwarder (für Log-Sammlung)
- **Loki**: Alternative zu ELK (lightweight, für Prometheus-Integration)

**Log-Sammlung:**
- **Structured Logging**: Alle Services verwenden Structured Logging (JSON-Format)
- **Log-Aggregation**: Logs werden zentral aggregiert (via Fluentd/Fluent Bit)
- **Log-Storage**: Logs werden in Elasticsearch oder Loki gespeichert
- **Log-Retention**: Logs werden für konfigurierte Zeit aufbewahrt (z.B. 30 Tage)

**Log-Levels:**
- **DEBUG**: Detaillierte Debug-Informationen (nur in Development)
- **INFO**: Allgemeine Informations-Logs
- **WARN**: Warnungen (potenzielle Probleme)
- **ERROR**: Fehler (müssen behoben werden)
- **FATAL**: Kritische Fehler (Service kann nicht weiterlaufen)

**Log-Features:**
- **Context-Tracking**: Context wird mitgeloggt (Request-ID, User-ID, Session-ID)
- **Log-Rotation**: Automatische Log-Rotation (verhindert Disk-Füllung)
- **Log-Compression**: Alte Logs werden komprimiert (Platzersparnis)
- **Log-Search**: Logs können in Kibana/Loki durchsucht werden
- **Log-Alerts**: Alerts basierend auf Log-Patterns (z.B. viele ERROR-Logs)

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
  - Vollständiges Logging aller Datenzugriffe
  - Immutable Logs: Logs können nicht verändert werden
  - Compliance-Logging erfüllt GDPR-Anforderungen
  - Langfristige Aufbewahrung von Logs
  - Strukturiertes Logging mit Log Levels (DEBUG, INFO, WARN, ERROR)
  - Context Tracking: Context wird mitgeloggt
  - Log Rotation: Automatische Log-Rotation
  - Audit-Logs für Security-Audits

## Abhängigkeiten

### Keine Core Library

- **WICHTIG**: Es gibt keine Edda Core Library
- **Separate Projekte**: Wenn gemeinsame Komponenten benötigt werden (DTOs, Protocols, Utils), sollte ein separates Projekt erstellt werden
- **Selektive Nutzung**: Dies hält Apps klein, da genau gewählt werden kann, was benötigt wird
- **Keine Abhängigkeit**: Yggdrasil sollte nicht auf Dateien/Protocols/Utils aus dem `edda` Verzeichnis verweisen (KEIN PROJEKT - nur Metadaten-Sammlung)

### Technische Abhängigkeiten

- Database (PostgreSQL)
- Payment Providers (Stripe, PayPal, etc.)
- Cloud Infrastructure
- Monitoring Tools

## Service-Integration

Yggdrasil stellt alle Services von Asgard bereit, zusätzlich zu den globalen Features:

### Verfügbare Services

**Hinweis**: Yggdrasil hat keinen eigenen Odin. User-Devices (Midgard/Alfheim/Asgard) haben eigene Odin-Instanzen, die direkt mit Yggdrasil kommunizieren.

**Yggdrasil-Services:**
- **Nidhöggr**: Connection Endpoint für Ratatoskr-Protocol
- **Nornen (Urd, Verdandi)**: Decision Service
- **Mimir**: Privacy Database Service
- **Heidrun**: Token & Pricing Service
- **Eikthyrnir**: Quality Assessment Service
- **Læraðr**: Data Management Service
- **Bifrost**: Communication Service (für Device-zu-Device-Relay)
- **Frigg**: Healthcare Plugin (automatisch vorhanden)
- **Valkyries**: Coding Agent (automatisch vorhanden)

### Multi-Tenant-Architektur
- **Strikte Netzwerk-Isolation**: Jeder User und jedes Netzwerk ist vollständig isoliert
- **Keine Sichtbarkeit**: User können andere Netze nicht sehen oder auf sie zugreifen
- **Automatische Model-Auswahl**: Yggdrasil koordiniert automatische Model-Auswahl über alle User
- **Provider-Registrierung**: User können sich als Compute-Provider registrieren

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

### Yggdrasil-spezifische Settings

**Settings-Inhalt (wird während Implementierung definiert)**
- Elixir-Konfiguration
- Rust-Microservice-Konfigurationen
- Marketplace-Einstellungen
- Bifrost-Relay-Einstellungen
- Database-Konfiguration

## Integration

- **Alle Devices**: Midgard, Alfheim, Asgard verbinden sich mit Yggdrasil über Vedrfolnir (Client)
  - **Ratatoskr-Protocol**: Für persistente Business-Verbindungen (Marketplace, Payments, etc.)
  - **gRPC**: Für Request/Response-Patterns (Device-Registry, User-Management, etc.)
  - **Bifrost**: Für Device-zu-Device-Relay und Event-Notifications
- **Device Registry**: Zentrale Registry für alle Devices weltweit (gRPC oder Ratatoskr)
- **User Management**: Zentrale User-Verwaltung (gRPC oder Ratatoskr)
- **Marketplace**: Zentrale Marketplace-Infrastruktur (Ratatoskr für persistente Verbindungen, gRPC für einzelne Requests)
- **Frigg**: Healthcare Plugin ist automatisch bei Yggdrasil vorhanden
- **Valkyries**: Coding Agent ist automatisch bei Yggdrasil vorhanden
- **Bifrost**: Communication Service für Device-zu-Device-Relay

## Performance

### Performance-Optimierungen
- **Horizontal Scaling**: Skalierung über mehrere Instanzen für hohen Durchsatz
- **Load Balancing**: Intelligente Lastverteilung für optimale Performance
- **Database Sharding**: Datenbank-Sharding für bessere Performance bei großen Datenmengen
- **Caching**: Redis/Memcached für schnellen Zugriff auf häufig verwendete Daten
- **CDN Integration**: CDN für statische Assets und API-Responses
- **Database Optimization**: Optimierte Datenbankabfragen mit Indizes und Query-Optimierung
- **Async Processing**: Asynchrone Verarbeitung für bessere Response-Zeiten

**Horizontal Scaling (Detailliert):**

**Elixir/Phoenix Scaling:**
- **Stateless Services**: Elixir-Services sind stateless (können horizontal skaliert werden)
- **Phoenix Channels**: Phoenix Channels unterstützen horizontale Skalierung (via PubSub)
- **OTP-Distribution**: OTP unterstützt verteilte Systeme (für Service-Koordination)
- **GenServer-Clustering**: GenServers können über mehrere Nodes verteilt werden

**Scaling-Strategie:**
1. **Auto-Scaling**: Automatische Skalierung basierend auf Load (CPU, Memory, Request-Rate)
2. **Manual Scaling**: Manuelle Skalierung für erwartete Load-Spitzen
3. **Scaling-Metriken**: 
   - CPU-Usage > 70%
   - Memory-Usage > 80%
   - Request-Rate > 80% der Kapazität
4. **Scaling-Actions**: 
   - Scale-Up: Neue Instanzen hinzufügen
   - Scale-Down: Instanzen entfernen (wenn Load niedrig)

**Load Balancing:**

**Load-Balancer-Strategie:**
- **Round-Robin**: Basis-Lastverteilung (Round-Robin)
- **Least-Connections**: Lastverteilung basierend auf aktiven Verbindungen
- **Health-Check**: Load-Balancer prüft Health der Instanzen (unhealthy Instanzen werden ausgeschlossen)
- **Sticky-Sessions**: Optional Sticky-Sessions für WebSocket-Verbindungen (Bifrost, Ratatoskr)

**Load-Balancer-Features:**
- **Health-Checks**: Regelmäßige Health-Checks (z.B. alle 10 Sekunden)
- **Failover**: Automatisches Failover bei Instanz-Ausfall
- **SSL-Termination**: SSL-Termination am Load-Balancer (TLS 1.3)
- **Rate-Limiting**: Rate-Limiting am Load-Balancer (zusätzliche Sicherheit)

**Database Sharding:**

**Sharding-Strategie:**
- **User-ID-basiert**: Sharding basierend auf User-ID (Hash-basiert)
- **Shard-Keys**: User-ID wird als Shard-Key verwendet
- **Shard-Anzahl**: Konfigurierbare Anzahl von Shards (z.B. 16, 32, 64)
- **Shard-Mapping**: Shard-Mapping wird in separater Tabelle oder Config gespeichert

**Sharding-Implementierung:**
- **Horizontal Partitioning**: Daten werden horizontal partitioniert (nach User-ID)
- **Shard-Routing**: Queries werden an entsprechenden Shard geroutet
- **Cross-Shard-Queries**: Cross-Shard-Queries werden vermieden (wenn möglich)
- **Shard-Rebalancing**: Automatisches Rebalancing bei Shard-Ungleichgewicht

**Sharding-Beispiel:**
```
shard_id = hash(user_id) % num_shards
```

- `hash(user_id)`: Konsistenter Hash der User-ID
- `num_shards`: Anzahl der Shards
- Jeder User wird konsistent dem gleichen Shard zugeordnet

**Performance-Metriken:**

**Zielwerte:**
- **API-Latenz**: < 100ms für Standard-Requests (95. Perzentil)
- **Durchsatz**: 1000+ Requests/Sekunde pro Instanz
- **Database-Queries**: < 50ms für Standard-Queries (95. Perzentil)
- **WebSocket-Verbindungen**: Millionen von gleichzeitigen Verbindungen (Elixir/Phoenix)
- **Skalierung**: Linear mit zusätzlichen Instanzen

**Metriken-Messung:**
- **Application-Metriken**: Prometheus-Metriken für Application-Performance
- **Database-Metriken**: Database-Performance-Metriken (Query-Zeit, Connection-Pool-Usage)
- **Network-Metriken**: Network-Performance-Metriken (Latency, Throughput)
- **Real-time-Monitoring**: Real-time-Monitoring-Dashboard (Grafana)

**Monitoring-Tools (Empfehlungen):**
- **Prometheus**: Metriken-Sammlung
- **Grafana**: Visualisierung und Dashboards
- **ELK Stack**: Centralized Logging (Elasticsearch, Logstash, Kibana)
- **Jaeger**: Distributed Tracing (optional)

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

**VPC-Isolation-Implementierung:**

**Multi-Tenant-VPC-Architektur:**
- **Separate VPCs**: Jeder User/Network erhält eigene VPC (Virtual Private Cloud)
- **VPC-ID**: Jeder Tenant hat eindeutige VPC-ID (basierend auf User-ID/Network-ID)
- **Subnet-Isolation**: Separate Subnets für jeden Tenant innerhalb der VPC
- **Route-Tables**: Separate Route-Tables für jeden Tenant (verhindert Cross-Tenant-Routing)
- **Security-Groups**: Separate Security-Groups für jeden Tenant (verhindert Cross-Tenant-Zugriff)

**VPC-Isolation-Mechanismen:**
- **Network-ACLs**: Network Access Control Lists verhindern Cross-VPC-Traffic
- **VPC-Peering**: Kein VPC-Peering zwischen verschiedenen Tenants
- **Internet-Gateway**: Jeder Tenant hat eigenen Internet-Gateway (isoliert)
- **NAT-Gateway**: Separate NAT-Gateways für jeden Tenant (optional)

**Kubernetes Network Policies:**

**Network-Policy-Implementierung:**
- **Namespace-Isolation**: Jeder Tenant erhält eigenen Kubernetes-Namespace
- **Network-Policy-Rules**: Strikte Network-Policy-Rules verhindern Cross-Namespace-Kommunikation
- **Pod-Isolation**: Pods können nur mit Pods im gleichen Namespace kommunizieren
- **Service-Isolation**: Services sind nur innerhalb des Namespaces erreichbar

**Network-Policy-Beispiel:**
```yaml
apiVersion: networking.k8s.io/v1
kind: NetworkPolicy
metadata:
  name: tenant-isolation
  namespace: tenant-{user_id}
spec:
  podSelector: {}
  policyTypes:
  - Ingress
  - Egress
  ingress:
  - from:
    - namespaceSelector:
        matchLabels:
          tenant: {user_id}
  egress:
  - to:
    - namespaceSelector:
        matchLabels:
          tenant: {user_id}
```

**Cross-Network-Zugriff-Verhinderung:**

**Application-Level-Isolation:**
- **Tenant-Context**: Jede Request enthält Tenant-Context (User-ID, Network-ID)
- **Request-Validierung**: System validiert, dass Request nur auf eigene Ressourcen zugreift
- **Database-Isolation**: Database-Queries werden mit Tenant-Filter versehen (WHERE user_id = ?)
- **API-Isolation**: API-Endpoints prüfen Tenant-Context vor Verarbeitung

**Network-Level-Isolation:**
- **Firewall-Rules**: Firewall-Rules verhindern Cross-Tenant-Network-Traffic
- **Routing-Rules**: Routing-Rules verhindern Cross-Tenant-Routing
- **Load-Balancer-Isolation**: Separate Load-Balancer für jeden Tenant (optional)

**Monitoring & Enforcement:**
- **Network-Monitoring**: Kontinuierliche Überwachung auf unautorisierte Cross-Network-Zugriffe
- **Alert-System**: Alerts bei erkannten Cross-Network-Zugriffsversuchen
- **Automatische Blockierung**: Automatische Blockierung von Cross-Network-Zugriffsversuchen
- **Audit-Logging**: Vollständiges Logging aller Network-Zugriffe für Compliance

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

**Pre-Authorization (Detailliert):**

**Pre-Authorization-Workflow:**
1. **Kosten-Schätzung**: System schätzt Kosten basierend auf:
   - Geschätzte Token-Anzahl (Input + Output)
   - Provider-Preis pro 1000 Tokens
   - Formel: `estimated_cost = (estimated_tokens / 1000) * price_per_token` (aufgerundet)
2. **Pre-Authorization-Request**: Pre-Authorization wird bei Payment-Provider angefordert
   - Betrag: Geschätzte Kosten + Buffer (z.B. 10% Buffer für Sicherheit)
   - Payment-Method: User's Standard-Zahlungsmethode
3. **Pre-Authorization-Response**: Payment-Provider bestätigt oder lehnt ab
   - **Erfolg**: Pre-Authorization erfolgreich, Request wird verarbeitet
   - **Fehler**: Pre-Authorization fehlgeschlagen, Request wird abgelehnt, User wird benachrichtigt
4. **Request-Verarbeitung**: Request wird verarbeitet (wenn Pre-Authorization erfolgreich)
5. **Final-Authorization**: Nach Request-Abschluss wird finaler Betrag autorisiert
   - **Capture**: Pre-Authorization wird auf finalen Betrag angepasst
   - **Refund**: Überschüssiger Betrag wird zurückerstattet (falls Buffer zu groß war)

**Pre-Authorization-Features:**
- **Buffer**: 10% Buffer für Sicherheit (verhindert, dass tatsächliche Kosten Pre-Authorization überschreiten)
- **Expiration**: Pre-Authorization läuft nach 24 Stunden ab (falls Request nicht abgeschlossen)
- **Automatic Release**: Abgelaufene Pre-Authorizations werden automatisch freigegeben
- **Multiple Requests**: Mehrere Pre-Authorizations können gleichzeitig aktiv sein (für parallele Requests)

**Payout-Processing (Detailliert):**

**Payout-Workflow:**
1. **Earnings-Akkumulation**: Provider-Earnings werden akkumuliert
   - Jede abgeschlossene Transaction erhöht Provider-Earnings
   - Earnings werden in Datenbank getrackt
2. **Payout-Trigger**:
   - **Automatisch**: Wenn Minimum-Payout erreicht ist (z.B. €10)
   - **Manuell**: User kann manuelle Auszahlung anfordern
   - **Periodisch**: Optional periodische Auszahlung (täglich/wöchentlich/monatlich)
3. **Payout-Request**: Payout-Request wird an Payment-Provider gesendet
   - Betrag: Provider-Earnings (nach Netting, falls aktiviert)
   - Payment-Method: Provider's Auszahlungsmethode
4. **Payout-Processing**: Payment-Provider verarbeitet Auszahlung
   - **Erfolg**: Auszahlung erfolgreich, Earnings werden auf 0 gesetzt
   - **Fehler**: Auszahlung fehlgeschlagen, Earnings bleiben bestehen, User wird benachrichtigt
5. **Payout-Confirmation**: Provider wird über erfolgreiche Auszahlung benachrichtigt

**Payout-Methoden:**
- **Bank Account (SEPA)**: SEPA-Überweisung (1-3 Werktage)
- **PayPal**: PayPal-Auszahlung (sofort)
- **Stripe Connect**: Stripe-Auszahlung (1-2 Werktage)
- **Andere Methoden**: Je nach Region verfügbar

### Beide Rollen gleichzeitig

**User als Consumer UND Provider**
- **Separate Zahlungsmethoden möglich**:
  - Eine für Requests (Consumer)
  - Eine für Earnings (Provider)
  - Oder: Gleiche Zahlungsmethode für beide

**Netting (Optional) - Detailliert:**

**Netting-Mechanismus:**
- **Netting-Aktivierung**: User kann Netting aktivieren (optional)
- **Netting-Berechnung**: 
  - `net_amount = provider_earnings - requester_costs`
  - Wenn `net_amount > 0`: User erhält Auszahlung
  - Wenn `net_amount < 0`: User wird belastet
  - Wenn `net_amount = 0`: Keine Transaktion nötig
- **Netting-Period**: Netting erfolgt pro Billing-Period (monatlich)

**Netting-Workflow:**
1. **Earnings-Tracking**: Provider-Earnings werden getrackt
2. **Costs-Tracking**: Requester-Costs werden getrackt
3. **Netting-Berechnung**: Am Ende der Billing-Period wird Netting berechnet
4. **Payout/Charge**:
   - **Positive Balance**: User erhält Auszahlung (Netting-Betrag)
   - **Negative Balance**: User wird belastet (Netting-Betrag)
   - **Zero Balance**: Keine Transaktion nötig
5. **Reset**: Earnings und Costs werden auf 0 gesetzt

**Netting-Vorteile:**
- **Weniger Transaktionen**: Weniger Payment-Transaktionen (nur Net-Betrag)
- **Kostenersparnis**: Weniger Payment-Provider-Gebühren
- **Einfachere Abrechnung**: Einfacheres Accounting für User

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
- World Type (Midgard, Asgard, Alfheim, Jotunheim)
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
   - Odin (auf Device A) verarbeitet Command
   - Odin entscheidet, dass Action auf Device B ausgeführt werden soll

2. **Connection Establishment**
   - Device A verbindet sich mit Device B über Bifrost (WebSocket)
   - Heimdall auf Device B prüft Permissions
   - gRPC-Verbindung wird über Bifrost etabliert

3. **Action Routing**
   - Device A sendet `ThorAction` via gRPC an Device B
   - Type-safe Kommunikation mit Protobuf
   - Bessere Performance als WebSocket für einzelne Requests

4. **Action Execution**
   - Thor auf Device B führt Action aus
   - `ThorResult` wird zurück an Device A via gRPC gesendet
   - Streaming möglich für lange Action-Executions

5. **Response**
   - Device A empfängt Result
   - User erhält Response
   - gRPC-Verbindung kann wiederverwendet werden für weitere Actions

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

**Programmiersprache:**
- **Elixir (Erlang VM/BEAM)**: Yggdrasil wird in Elixir implementiert
- **Warum Elixir**: 
  - **Millionen Verbindungen**: Erlang VM (BEAM) ist speziell für massive Concurrency designed
  - **Bifrost-Verbindungen**: Phoenix Channels (WebSockets) für Bifrost-Verbindungen zu Devices
  - **Fault Tolerance**: Eingebaute Fault Tolerance durch Erlang VM
  - **Hot Code Reloading**: Live-Updates ohne Downtime möglich
  - **Skalierbarkeit**: Kann Millionen von Devices gleichzeitig über Bifrost verbinden
- **Phoenix Framework**: 
  - **Web-API**: Phoenix wird für Web-API verwendet (RESTful Endpoints)
  - **WebSockets**: Phoenix Channels für WebSocket-Verbindungen (Bifrost, Ratatoskr)
  - **Phoenix-Channel-Architektur**: Phoenix-Channel-Architektur für skalierbare WebSocket-Verbindungen
  - **Phoenix-Updates**: Phoenix-Updates werden automatisch gehandhabt (Versionierung, Migration)
- **Ecto-Integration**: 
  - **Database-Access**: Ecto wird für Database-Access verwendet (PostgreSQL)
  - **Ecto-Schema-Definitionen**: Ecto-Schema-Definitionen für alle Datenmodelle
  - **Ecto-Migrations**: Ecto-Migrations für Schema-Updates (automatisch oder manuell)
- **OTP-Integration**: 
  - **Verteilte Systeme**: OTP wird für verteilte Systeme verwendet (Erlang Distribution)
  - **Supervision-Trees**: Supervision-Trees für Fault Tolerance (automatische Restarts)
  - **OTP-Fehler**: Bei OTP-Fehlern wird automatisch Restart durchgeführt (Supervision-Tree)

**Technische Anforderungen:**
- **Eigenständiger Server**: Yggdrasil ist ein eigenständiger Server ohne eigenen Odin
- **User-Devices kommunizieren direkt**: User-Devices (Midgard/Alfheim/Asgard) haben eigene Odin-Instanzen, die direkt mit Yggdrasil kommunizieren
- **Kommunikations-Protokolle**: Bifrost (WebSocket), Ratatoskr (WebSocket), gRPC
- **Multi-Tenant-Architektur**: Strikte Isolation zwischen Usern und Netzwerken
- **Netzwerk-Isolation oberste Priorität**: User dürfen nicht in andere Netze eindringen können
- **Bifrost-Relay**: Yggdrasil baut Bifrost-WebSocket-Verbindungen zu Devices auf
- **Ratatoskr-Protocol**: Yggdrasil empfängt Business-Requests über Ratatoskr-Protocol (via Nidhöggr) für persistente Verbindungen
- **gRPC-Support**: Yggdrasil unterstützt gRPC für Request/Response-Patterns und effiziente API-Calls
- **Connection Management**: Millionen von gleichzeitigen Bifrost-WebSocket-Verbindungen
- **Message-Routing**: Yggdrasil routet Messages zwischen Devices über Bifrost
- **Event-Notifications**: Alle Events werden über Bifrost-Messages gesendet
- **Connection-Initiation**: Devices verbinden sich über Bifrost, nicht über Webhooks
- **Rust-Microservices**: Yggdrasil koordiniert Rust-Microservices für CPU-intensive Tasks
- **gRPC-Kommunikation**: Yggdrasil kommuniziert mit Rust-Microservices über gRPC
- **Protokoll-Auswahl**: gRPC für Request/Response, Ratatoskr für persistente Business-Verbindungen, Bifrost für Device-Relay
- Muss hochverfügbar sein (99.9%+ Uptime)
- Sollte skalierbar sein (horizontal scaling über mehrere Nodes)
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


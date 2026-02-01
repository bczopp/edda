# Frigg - Healthcare Plugin

## Übersicht

**Tests ausführen:** Von `frigg/`: `docker compose -f docker-compose.test.yml run --rm frigg-test` oder `./scripts/run-tests.sh` (bzw. `.\scripts\run-tests.ps1` unter Windows). **CI:** Bei Push/PR auf `frigg/**` läuft die Pipeline [.github/workflows/frigg.yml](../.github/workflows/frigg.yml) (Test im Container, Lint).

Frigg ist ein Plugin für Gesundheitsfragen, das von Odin orchestriert wird. **Frigg ist ausschließlich für persönliche und Gesundheitsfragen (mentale und körperliche) zuständig.** Wenn Gesundheitsfragen erkannt werden, delegiert Odin die Aufgabe an Frigg (wenn verfügbar). Das Healthcare Plugin bietet zertifizierte Kurse, die von Krankenkassen genehmigt und bezahlt werden können. **Frigg** (Göttin der Fürsorge und Mutterschaft) führt das Healthcare Plugin an und koordiniert die Healthcare-Services. **Fulla** (Göttin der Fürsorge und Unterstützung) kann als Service für Daten und Behandlungspläne fungieren, falls benötigt.

**Wichtig**: Frigg ist **nicht** für allgemeine Antworten zuständig. Verschiedene Persönlichkeiten können vergeben werden. Ein Therapeut sollte nicht jeden Tag die Nachrichten vorlesen oder Emails beantworten.

**Plugin-Architektur**: 
- **Plugin-Integration**: Frigg wird als Plugin in Odin integriert (modular, optional)
- **Plugin-Loading**: Plugin-Loading-Mechanismen (dynamisches Laden zur Laufzeit)
- **Plugin-Updates**: Bei Plugin-Updates wird Plugin neu geladen (Hot-Reload) oder Service wird neu gestartet
- Frigg ist ein optionales Plugin, das modular zur Odin-Installation hinzugefügt werden kann. Odin entscheidet selbst, ob eine Aufgabe an Frigg delegiert werden muss (bei Gesundheitsfragen) oder ob Odin selbst antworten kann (bei einfachen Fragen). Die Kommunikation mit Odin erfolgt über Thor (queue-basiert, wenn Thor verfügbar ist) oder direkt.

**Plugin-Interface**: Frigg implementiert das `OdinPlugin`-Interface mit:
- `get_title()`: Gibt "Frigg - Healthcare Plugin" zurück
- `get_description()`: Beschreibt die Healthcare-Funktionalität
- `get_functions()`: Gibt alle verfügbaren Healthcare-Funktionen zurück (Function Call Protocol)

**Einherjar Protocol**: 
- **Implementierung**: Frigg implementiert das Einherjar-Protocol für Funktions-Offenlegung
- **Funktions-Offenlegung**: Frigg legt alle verfügbaren Healthcare-Funktionen offen
- **Zuständigkeits-Domains**: Zuständigkeits-Domains werden definiert (Healthcare, Mental Health, Physical Health, etc.)
- Frigg implementiert das Einherjar Protocol, um ihre verfügbaren Funktionen und Zuständigkeits-Domains offenzulegen. Odin nutzt diese Informationen, um automatisch zu erkennen, wann Frigg zuständig ist.

**Responsibility Service**: 
- **Implementierung**: Frigg implementiert das Responsibility-Service für Zuständigkeits-Management
- **Zuständigkeits-Management**: Zuständigkeit wird übernommen, zurückgegeben oder zurückgewiesen
- **Zuständigkeits-Konflikte**: Bei Zuständigkeits-Konflikten wird Zuständigkeit an Odin zurückgegeben oder User wird um Klärung gebeten
- Frigg implementiert das Responsibility Service, um Zuständigkeit zu übernehmen, zurückzugeben oder zurückzuweisen. Wenn eine Unterhaltung nicht mehr primär gesundheitsbezogen ist, gibt Frigg die Zuständigkeit an Odin zurück.

## Komponenten

### Frigg - Healthcare Plugin Lead
- **Verantwortlichkeiten**: 
  - Koordination aller Healthcare-Services
  - Course Management
  - Certification Management
  - Insurance Integration
  - Progress Tracking
- **Delegation**: Kann Aufgaben an Fulla delegieren (z.B. Datenbereitstellung, Behandlungspläne)

### Fulla - Healthcare Data Service (Optional)
- **Verantwortlichkeiten** (falls benötigt):
  - Datenbereitstellung für Healthcare-Kurse
  - Behandlungspläne bereitstellen
  - Unterstützung bei Course-Content
  - Datenverwaltung für Healthcare-Module
- **Delegation**: Wird von Frigg koordiniert

## Features

### 1. Course Management System (Frigg)
- Course Creation & Management
- Module Management
- Content Management
- Quiz/Assessment System
- **Delegation**: Frigg kann Fulla für Daten/Behandlungspläne nutzen

### 2. Certification Engine (Frigg)
- Certification Generation
- Certification Validation
- Certification Tracking
- Compliance Verification

### 3. Insurance Integration (Frigg)
- Insurance Provider Integration
- Claim Processing
- Payment Processing
- Approval Workflow

### 4. Progress Tracking (Frigg)
- User Progress Monitoring
- Completion Tracking
- Certification Tracking
- Analytics

### 5. Data & Treatment Plans (Fulla - Optional)
- **Datenbereitstellung**: Healthcare-Daten für Kurse
- **Behandlungspläne**: Bereitstellung von Behandlungsplänen
- **Content Support**: Unterstützung bei Course-Content mit Daten
- **Wird von Frigg koordiniert**: Fulla arbeitet unter Anleitung von Frigg

## Service-Architektur

### Frigg Service
- Haupt-Service für Healthcare Plugin
- Koordiniert alle Healthcare-Funktionen
- Kann Fulla-Service nutzen, falls benötigt
- **Eigenes RAG**: Frigg braucht ein eigenes RAG (können die Wölfe helfen), es muss aber eine separate Datenbank sein (getrennt von Freki)
- **Eigenes RAG**: Frigg braucht ein eigenes RAG (können die Wölfe helfen), es muss aber eine separate Datenbank sein (getrennt von Freki)

### Fulla Service (Optional)
- Wird von Frigg koordiniert
- Stellt Daten und Behandlungspläne bereit
- Unterstützt Frigg bei Content-Management

## Datenbank-Architektur

### Eigenständige Datenbank für Datenschutz

Frigg verwendet eine **eigenständige, isolierte Datenbank** für alle Gesundheitsdaten, um:
- **Datenschutz zu gewährleisten**: Gesundheitsdaten sind strikt von anderen Systemdaten getrennt
- **Schnellen und einfachen Zugriff**: Gesundheitsdaten eines Users sind schnell und einfach abrufbar
- **Compliance**: Erfüllung von Datenschutz-Anforderungen (GDPR, etc.)
- **Isolation**: Keine Vermischung mit anderen Daten

**Wichtig**: Frigg braucht ein eigenes RAG (können die Wölfe helfen), es muss aber eine separate Datenbank sein (getrennt von Freki). Die RAG-Datenbank für Frigg ist komplett getrennt von der Freki-Datenbank.

### Datenbank-Features

#### Datenschutz
- **Isolierte Datenbank**: Komplett getrennt von Yggdrasil und anderen Systemen
- **Verschlüsselung**: Alle Gesundheitsdaten werden verschlüsselt gespeichert
- **Access Control**: Strikte Zugriffskontrolle basierend auf User-Identität
- **Audit Logging**: Vollständiges Audit-Logging aller Datenzugriffe
- **Data Minimization**: Nur notwendige Daten werden gespeichert

#### Performance
- **Optimierte Indizes**: Schnelle Abfragen für User-spezifische Daten
- **Caching**: Intelligentes Caching für häufig abgerufene Daten
- **Query Optimization**: Optimierte Datenbankabfragen für schnellen Zugriff

### Datenbank-Schema

#### Users Table
```sql
CREATE TABLE users (
    user_id UUID PRIMARY KEY,
    encrypted_health_data BYTEA NOT NULL,  -- Verschlüsselt mit AES-256-GCM
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    last_access TIMESTAMP NOT NULL DEFAULT NOW()
);

-- Indizes
CREATE INDEX idx_users_last_access ON users(last_access);
CREATE INDEX idx_users_updated_at ON users(updated_at);
```

#### Course Progress Table
```sql
CREATE TABLE course_progress (
    progress_id UUID PRIMARY KEY,
    user_id UUID NOT NULL REFERENCES users(user_id) ON DELETE CASCADE,
    course_id VARCHAR(255) NOT NULL,
    module_id VARCHAR(255) NOT NULL,
    progress_data BYTEA NOT NULL,  -- JSON, verschlüsselt
    completed_at TIMESTAMP,
    started_at TIMESTAMP NOT NULL DEFAULT NOW()
);

-- Indizes
CREATE INDEX idx_course_progress_user_id ON course_progress(user_id);
CREATE INDEX idx_course_progress_course_id ON course_progress(course_id);
CREATE INDEX idx_course_progress_user_course ON course_progress(user_id, course_id);
CREATE INDEX idx_course_progress_completed_at ON course_progress(completed_at) WHERE completed_at IS NOT NULL;
```

#### Treatment Plans Table
```sql
CREATE TABLE treatment_plans (
    plan_id UUID PRIMARY KEY,
    user_id UUID NOT NULL REFERENCES users(user_id) ON DELETE CASCADE,
    plan_name VARCHAR(255) NOT NULL,
    plan_data BYTEA NOT NULL,  -- JSON, verschlüsselt
    insurance_code VARCHAR(50),
    activated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    expires_at TIMESTAMP
);

-- Indizes
CREATE INDEX idx_treatment_plans_user_id ON treatment_plans(user_id);
CREATE INDEX idx_treatment_plans_insurance_code ON treatment_plans(insurance_code);
CREATE INDEX idx_treatment_plans_expires_at ON treatment_plans(expires_at) WHERE expires_at IS NOT NULL;
CREATE INDEX idx_treatment_plans_active ON treatment_plans(user_id, activated_at) WHERE expires_at IS NULL OR expires_at > NOW();
```

#### Health Records Table
```sql
CREATE TABLE health_records (
    record_id UUID PRIMARY KEY,
    user_id UUID NOT NULL REFERENCES users(user_id) ON DELETE CASCADE,
    record_type VARCHAR(50) NOT NULL,
    record_data BYTEA NOT NULL,  -- JSON, verschlüsselt
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

-- Indizes
CREATE INDEX idx_health_records_user_id ON health_records(user_id);
CREATE INDEX idx_health_records_record_type ON health_records(record_type);
CREATE INDEX idx_health_records_user_type ON health_records(user_id, record_type);
CREATE INDEX idx_health_records_created_at ON health_records(created_at);
CREATE INDEX idx_health_records_updated_at ON health_records(updated_at);
```

**Verschlüsselung-Implementierung:**
- **Column-Level Encryption**: Alle sensiblen Daten werden auf Spaltenebene verschlüsselt (AES-256-GCM)
- **Key Management**: Verschlüsselungs-Keys werden in OS-spezifischem Secure Storage gespeichert (Keychain, Credential Manager)
- **Key-Rotation**: Keys werden regelmäßig rotiert (automatisch alle 90 Tage)
- **Transparent Encryption**: Verschlüsselung/Entschlüsselung erfolgt transparent in der Anwendungsschicht

**Schneller User-Datenzugriff:**
- **User-spezifische Indizes**: Alle Tabellen haben Indizes auf `user_id` für schnelle User-spezifische Abfragen
- **Composite Indizes**: Indizes für häufige Query-Patterns (z.B. `user_id + course_id`)
- **Partitionierung**: Optional nach `user_id` für sehr große Datenmengen
- **Connection Pooling**: Effiziente Datenbankverbindungen mit Connection Pooling
- **Query Optimization**: Spezielle Abfragen für häufige Zugriffe (z.B. alle Daten eines Users)

### Datenbank-Technologie

- **Empfohlene Datenbank**: PostgreSQL (mit Verschlüsselung) oder spezialisierte Healthcare-Datenbank
- **Backup & Recovery**: Regelmäßige Backups mit Verschlüsselung
- **Replication**: Optional für Hochverfügbarkeit
- **Migration**: Sichere Datenmigration zwischen Umgebungen

### Datenzugriff

#### Schneller User-Datenzugriff
- **User-spezifische Indizes**: Optimierte Indizes für user_id
- **Partitionierung**: Optional nach User oder Zeitraum
- **Connection Pooling**: Effiziente Datenbankverbindungen
- **Query Optimization**: Spezielle Abfragen für häufige Zugriffe

#### API für Datenzugriff
- `GET /api/health/user/:userId` - Alle Gesundheitsdaten eines Users
- `GET /api/health/user/:userId/courses` - Kurs-Fortschritt eines Users
- `GET /api/health/user/:userId/plans` - Behandlungspläne eines Users
- `GET /api/health/user/:userId/records` - Gesundheitsaufzeichnungen eines Users

## Integration mit Odin und Thor

### Direkte Kommunikation mit Odin

**Wichtig**: Odin kommuniziert direkt mit Frigg via gRPC. Die Plugins kommunizieren nicht untereinander.

**Odin → Frigg Kommunikation:**
- **Direkte gRPC-Kommunikation**: Odin ruft Frigg direkt via gRPC auf (Einherjar Protocol, Responsibility Service)
- **Task-Delegation**: Odin delegiert Healthcare-Aufgaben direkt an Frigg
- **Zuständigkeits-Management**: Frigg übernimmt Zuständigkeit via Responsibility Service
- **Chat-Leitung**: Odin kann die Chat-Leitung an Frigg abgeben (automatisch oder explizit durch User)

**Frigg → Odin Kommunikation:**
- **Ergebnis-Rückgabe**: Frigg gibt Ergebnisse direkt an Odin zurück
- **Strukturierte Ergebnisse**: Falls Actions benötigt werden, gibt Frigg strukturierte Ergebnisse zurück

### Chat-Leitung an Frigg

**Zwei Szenarien für direkte Chat-Leitung:**

1. **Automatische Weiterleitung durch Odin**
   - Odin erkennt Healthcare-Aufgabe und verweist automatisch auf Frigg
   - Durch ein Flag in den Settings bleibt der Chat bei Frigg
   - User-Eingaben werden direkt an Frigg geleitet, statt über Odin, der immer wieder entscheiden muss

2. **Explizite Chat-Erstellung durch User**
   - User erstellt explizit einen Chat mit Frigg
   - Der Chat bleibt immer bei Frigg und kann auch nicht an Odin übergeben werden
   - Alle Eingaben in diesem Chat gehen direkt an Frigg

**Vorteile:**
- **Effizienz**: Keine wiederholte Entscheidungsfindung durch Odin bei jedem Input
- **Konsistenz**: Chat bleibt bei Frigg für konsistente Gesundheits-Unterhaltung
- **User-Kontrolle**: User kann explizit einen Frigg-Chat starten

### Action-Execution über Thor

**Odin → Thor Kommunikation (falls Actions nötig):**
- **Odin analysiert Ergebnis**: Odin analysiert Frigg-Ergebnisse und erkennt, ob Actions benötigt werden
- **Weiterleitung an Thor**: Falls Actions benötigt werden, leitet Odin die strukturierten Ergebnisse an Thor zur Action-Execution weiter
- **Action-Ausführung**: Thor führt Actions aus (via Mjölnir)
- **Ergebnis-Rückgabe**: Thor gibt `ThorResult` an Odin zurück

### Workflow: Odin → Frigg → Odin → Thor (falls Actions nötig) → Odin

1. **Odin erkennt Healthcare-Aufgabe**
   - Odin erkennt via Einherjar Protocol, dass es sich um eine Healthcare-Aufgabe handelt
   - Odin delegiert direkt an Frigg via gRPC (Responsibility Service)

2. **Frigg verarbeitet Task**
   - Frigg analysiert Task und verarbeitet Healthcare-Request
   - Frigg kann Fulla für Daten/Behandlungspläne nutzen

3. **Ergebnis-Rückgabe an Odin**
   - Frigg gibt Ergebnis direkt an Odin zurück
   - Falls Actions benötigt werden, gibt Frigg strukturierte Ergebnisse zurück

4. **Action-Execution über Thor (falls nötig)**
   - Odin analysiert Ergebnis und erkennt, ob Actions benötigt werden
   - Falls ja: Odin leitet strukturierte Ergebnisse an Thor zur Action-Execution weiter
   - Thor führt Actions aus und gibt `ThorResult` an Odin zurück

5. **Odin gibt Response an User**
   - Odin verarbeitet Ergebnis und gibt Response an User zurück

## Insurance Integration

### Freischaltcode-System
- **User benötigt Freischaltcode**: User muss einen Freischaltcode von der Versicherung erhalten
- **Yggdrasil als Proxy**: Integration mit Insurance Providers erfolgt über Yggdrasil (Yggdrasil als Proxy)
- **Freischaltung**: User gibt Freischaltcode ein, Yggdrasil validiert mit Insurance Provider
- **Nur freigeschaltete Pläne**: User darf nur für die Pläne freigeschaltet werden, die die Krankenkasse finanziert
- **Pläne werden freigeschaltet**: Nach Validierung werden entsprechende Behandlungspläne freigeschaltet

### Workflow: Freischaltung

1. **User erhält Freischaltcode**
   - User erhält Freischaltcode von seiner Krankenkasse
   - Code ist spezifisch für bestimmte Behandlungspläne

2. **User gibt Code ein**
   - User gibt Freischaltcode in Frigg ein
   - Frigg sendet Code an Yggdrasil zur Validierung

3. **Yggdrasil validiert**
   - Yggdrasil kontaktiert Insurance Provider API
   - Insurance Provider validiert Code
   - Insurance Provider teilt mit, welche Pläne finanziert werden

4. **Freischaltung**
   - Yggdrasil schaltet entsprechende Pläne frei
   - Frigg erhält Liste der freigeschalteten Pläne
   - User kann nur freigeschaltete Pläne starten

## Certification Management

### Zentrale Speicherung auf Yggdrasil
- **Yggdrasil speichert**: Certifications werden auf Yggdrasil gespeichert (zentrale Speicherung)
- **Validierung**: Certifications werden von Yggdrasil validiert
- **Zugriff**: Frigg kann Certifications von Yggdrasil abrufen
- **Synchronisation**: Certifications werden zwischen Devices synchronisiert (über Yggdrasil)

## Zuständigkeits-Management

### Zuständigkeits-Bereich
- **Persönliche Fragen**: Alles was User preisgibt, persönliche Themen
- **Gesundheitsfragen**: Mentale und körperliche Gesundheit
- **Behandlungspläne**: Gesundheitspläne (Bonus von Krankenversicherung)
- **Nicht für allgemeine Antworten**: Frigg liest keine Nachrichten vor oder beantwortet Emails

### Zuständigkeits-Weiterleitung durch Odin
- **Odin erkennt Gesundheitsfragen**: Odin nutzt Einherjar Protocol, um zu erkennen, wann Frigg zuständig ist
- **Delegation**: Odin delegiert Gesundheitsfragen an Frigg
- **Frigg bleibt zuständig**: Frigg übernimmt die Unterhaltung und bleibt zuständig
- **Zuständigkeits-Rückgabe**: Frigg analysiert Requests und erkennt, wenn Unterhaltung nicht mehr primär gesundheitsbezogen ist
- **Zurück zu Odin**: Frigg sendet `ResponsibilityReturn` an Odin, Zuständigkeit geht zurück zu Odin
- **Odin entscheidet wieder**: Odin übernimmt wieder Entscheidungsfindung

### Rückweisungs-Mechanismus
- **Frigg kann zurückweisen**: Wenn Request nicht in Friggs Bereich ist, kann Frigg zurückweisen
- **ResponsibilityRejection**: Frigg sendet `ResponsibilityRejection` mit Hinweis auf besseren Gott
- **Odin weicht aus**: Odin wählt nächstwahrscheinlichste Wahl aus Einherjar Protocol

### Persönlichkeiten
- **Verschiedene Persönlichkeiten**: Verschiedene Persönlichkeiten können vergeben werden
- **Spezifische Persönlichkeit**: Frigg hat spezifische Persönlichkeit für Gesundheitsfragen
- **Nicht für allgemeine Aufgaben**: Therapeut sollte nicht jeden Tag die Nachrichten vorlesen oder Emails beantworten
- **Fokus auf Gesundheit**: Frigg fokussiert sich ausschließlich auf persönliche und Gesundheitsfragen

## Erkennung von Healthcare-Aufgaben

### Automatische Erkennung durch Odin (via Einherjar Protocol)
- **Einherjar Protocol**: Odin nutzt Einherjar Protocol, um Friggs Funktionen und Zuständigkeits-Domains zu erkennen
- **Keyword-basiert**: Odin erkennt Keywords aus Friggs `responsibility_keywords`
- **Domain-basiert**: Odin erkennt Zuständigkeit basierend auf `responsibility_domains`
- **Context-basiert**: Odin analysiert Context der Anforderung

### Explizite Anforderung durch User
- **User kann anfordern**: User kann explizit Frigg anfordern oder eine Behandlung starten wollen
- **Einfachere Erkennung**: Macht Erkennung einfacher und zuverlässiger
- **Beispiele**: "Starte Gesundheitskurs", "Zeige verfügbare Behandlungen", "Frigg: Starte Therapie"

## Deployment

### Als Extension zu Asgard
- **Optional**: Frigg kann als Extension zu Asgard hinzugefügt werden
- **Installation**: User kann Frigg separat installieren
- **Integration**: Nach Installation integriert sich Frigg mit Asgard
- **Thor-Integration**: Thor erkennt, ob Frigg verfügbar ist

### Automatisch bei Yggdrasil
- **Standard**: Frigg ist automatisch bei Yggdrasil vorhanden
- **Keine Installation nötig**: User muss nichts installieren
- **Immer verfügbar**: Healthcare-Aufgaben können immer über Yggdrasil verarbeitet werden
- **Thor-Integration**: Thor erkennt automatisch, dass Frigg verfügbar ist

## gRPC Communication

**gRPC Service Communication:**
- **Odin ↔ Frigg**: gRPC für Plugin-Kommunikation
- **Type-Safe**: Protobuf garantiert korrekte Service-Interfaces
- **Streaming**: Built-in Streaming für große Responses

**gRPC Connection-Management:**
- **Connection-Pooling**: Wiederverwendung von Verbindungen für bessere Performance
- **Connection Reuse**: Connections werden effizient wiederverwendet
- **Automatische Reconnection**: Kombination aus sofortigem Versuch + Exponential Backoff
  - Sofortiger Reconnect-Versuch bei Verbindungsabbruch
  - Nach erstem Fehler beginnt Exponential Backoff
  - Maximale Wartezeit (z.B. 60 Sekunden)
  - Kontinuierliche Versuche zur Wiederherstellung
- **Connection Monitoring**: Verbindungsstatus wird überwacht

**gRPC Error-Handling:**
- **gRPC Status-Codes**: gRPC-Fehler werden über Status-Codes behandelt
- **Retry-Mechanismen**: Automatischer Retry mit Exponential Backoff (siehe gemeinsame Klärungspunkte)
- **Timeout-Konfiguration**: Adaptive Timeouts mit Minimum/Maximum
- **Fallback**: Bei Fehler Fallback zu alternativen Routen

## Abhängigkeiten

### Keine Core Library

- **WICHTIG**: Es gibt keine Edda Core Library
- **Separate Projekte**: Wenn gemeinsame Komponenten benötigt werden (DTOs, Protocols, Utils), sollte ein separates Projekt erstellt werden
- **Selektive Nutzung**: Dies hält Apps klein, da genau gewählt werden kann, was benötigt wird
- **Keine Abhängigkeit**: Frigg sollte nicht auf Dateien/Protocols/Utils aus dem `edda` Verzeichnis verweisen (KEIN PROJEKT - nur Metadaten-Sammlung)

### Service-Abhängigkeiten

- **Odin**: Für Plugin-Orchestrierung (Frigg ist ein Plugin für Odin)
- **Thor**: Für Action-Execution, wenn Frigg-Ergebnisse Actions benötigen (wenn Thor verfügbar)
- **Yggdrasil**: Für Insurance Provider Integration und Certification Storage

### Technische Abhängigkeiten

- **Eigenständige Datenbank**: PostgreSQL oder spezialisierte Healthcare-Datenbank für Gesundheitsdaten
- Insurance Provider APIs (über Yggdrasil)
- Certification Authorities
- Compliance Frameworks
- Fulla Service (optional, falls Daten/Behandlungspläne benötigt werden)

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

### Frigg-spezifische Settings

**Settings-Inhalt (wird während Implementierung definiert)**
- Healthcare-Konfigurationen
- Course-Management-Einstellungen
- Insurance-Integration-Einstellungen
- Database-Konfiguration
- **RAG-Konfiguration**: Eigenes RAG mit separater Datenbank (getrennt von Freki)
- **Chat-Leitung-Flags**: Flags für direkte Chat-Leitung an Frigg

## Integration

- **Odin**: 
  - Erkennt Gesundheitsfragen via Einherjar Protocol
  - Delegiert Aufgaben an Frigg via Responsibility Service
  - Empfängt Zuständigkeits-Rückgaben von Frigg
  - Empfängt Rückweisungen von Frigg
- **Thor**: Führt Actions aus, wenn Frigg-Ergebnisse Actions benötigen (wenn Thor verfügbar)
- **Frigg**: 
  - Verarbeitet Healthcare-Tasks
  - Implementiert Einherjar Protocol für Funktions-Offenlegung
  - Implementiert Responsibility Service für Zuständigkeits-Management
  - Gibt Zuständigkeit zurück, wenn Unterhaltung nicht mehr gesundheitsbezogen ist
- **Fulla**: Optional, für Daten/Behandlungspläne
- **Yggdrasil**: Für Insurance Provider Integration und Certification Storage

## Performance

### Performance-Optimierungen
- **Optimierte Datenbankabfragen**: Schnelle Abfragen für User-spezifische Daten
- **Caching**: Intelligentes Caching für häufig abgerufene Gesundheitsdaten
- **Connection Pooling**: Effizientes Connection-Pooling für Datenbankverbindungen
- **Index Optimization**: Optimierte Indizes für schnellen Datenzugriff
- **Query Optimization**: Spezielle Abfragen für häufige Zugriffe
- **Partitionierung**: Optional nach User oder Zeitraum für bessere Performance

### Performance-Metriken
- Schneller User-Datenzugriff (< 50ms für Standard-Queries)
- Effiziente Datenbankabfragen (optimierte Indizes)
- Hoher Durchsatz für parallele Requests

## Datenschutz

### Datenschutz-Features
- **Isolierte Datenbank**: Komplett getrennt von Yggdrasil und anderen Systemen
- **Verschlüsselung**: Alle Gesundheitsdaten werden verschlüsselt gespeichert
- **Access Control**: Strikte Zugriffskontrolle basierend auf User-Identität
- **Audit Logging**: Vollständiges Audit-Logging aller Datenzugriffe
- **Data Minimization**: Nur notwendige Daten werden gespeichert
- **User Control**: User hat volle Kontrolle über seine Gesundheitsdaten

### Compliance
- **GDPR-konform**: Vollständige Einhaltung der GDPR-Anforderungen
- **Healthcare Compliance**: Einhaltung von Healthcare-spezifischen Datenschutz-Anforderungen
- **Data Minimization**: Nur notwendige Daten werden gespeichert
- **Right to Deletion**: User kann alle Gesundheitsdaten löschen
- **Transparency**: User wird über Datenverarbeitung informiert
- **Data Portability**: User kann seine Daten exportieren

## Sicherheit

### Security-Features
- **Verschlüsselung**: Alle Gesundheitsdaten werden verschlüsselt gespeichert (at rest und in transit)
- **Access Control**: Strikte Zugriffskontrolle für Gesundheitsdaten
- **Authentication**: Sichere Authentifizierung über Heimdall
- **Authorization**: Granulares Permission-System für Gesundheitsdaten
- **Audit Logging**: Vollständiges Logging aller Datenzugriffe für Security-Audits
- **Threat Detection**: Erkennung von verdächtigen Zugriffen
- **Secure Backup**: Verschlüsselte Backups für Disaster Recovery

### Security-Best-Practices
- **Keine Hardcoded Secrets**: Keine Hardcoded Passwords oder Keys
- **Secure Defaults**: Sichere Standard-Konfigurationen
- **Regular Updates**: Automatische Security-Updates
- **Vulnerability Scanning**: Kontinuierliches Scanning für bekannte Vulnerabilities
- **Penetration Testing**: Regelmäßige Penetration Tests
- **Incident Response**: Automatische Response auf Security-Incidents
- **Compliance Audits**: Regelmäßige Compliance-Audits

## Implementierungs-Notizen

- Frigg muss als separates Projekt implementiert werden
- **Eigenständige Datenbank**: Muss eine isolierte Datenbank für Gesundheitsdaten haben
- **Datenschutz**: Alle Gesundheitsdaten müssen verschlüsselt gespeichert werden
- **Performance**: Datenbank muss für schnellen User-Datenzugriff optimiert sein
- Thor muss Healthcare-Aufgaben erkennen können (automatisch + explizit)
- Queue-System muss zwischen Thor und Frigg funktionieren
- Frigg kann als Extension zu Asgard hinzugefügt werden
- Frigg ist automatisch bei Yggdrasil vorhanden
- Thor muss prüfen, ob Frigg verfügbar ist
- Insurance-Integration erfolgt über Yggdrasil (nicht direkt)
- Freischaltcode-System muss implementiert werden
- Certifications werden auf Yggdrasil gespeichert (nicht in Frigg-Datenbank)
- Fulla ist optional und wird von Frigg koordiniert
- **Datenbank-Backup**: Regelmäßige, verschlüsselte Backups müssen implementiert werden
- **Access Control**: Strikte Zugriffskontrolle für Gesundheitsdaten
- **Audit Logging**: Vollständiges Logging aller Datenzugriffe
- **Muss Einherjar Protocol implementieren**: Für Funktions-Offenlegung und Zuständigkeits-Domains
- **Muss Responsibility Service implementieren**: Für Zuständigkeits-Management (TakeResponsibility, ReturnResponsibility, RejectResponsibility)
- **Muss Zuständigkeits-Rückgabe haben**: Wenn Unterhaltung nicht mehr gesundheitsbezogen ist
- **Muss Rückweisungs-Mechanismus haben**: Kann Requests zurückweisen, wenn nicht in Friggs Bereich
- **Muss Persönlichkeiten unterstützen**: Verschiedene Persönlichkeiten können vergeben werden
- **Muss ausschließlich für Behandlungen sein**: Nicht für allgemeine Antworten
- **Performance**: Muss optimiert sein für schnellen Zugriff auf Gesundheitsdaten
- **Datenschutz**: Muss höchste Datenschutz-Standards erfüllen (GDPR, Healthcare Compliance)
- **Sicherheit**: Muss Enterprise-Grade Security haben mit kontinuierlichem Monitoring


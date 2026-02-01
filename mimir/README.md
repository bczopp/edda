# Mimir (Mímisbrunnr) - Privacy Database Service

## Übersicht

**Tests ausführen:** Von `mimir/`: `docker compose -f docker-compose.test.yml run --rm mimir-test` oder `./scripts/run-tests.sh` / `.\scripts\run-tests.ps1`. Von Repo-Root: `mimir/scripts/run-tests.sh` bzw. `.\mimir\scripts\run-tests.ps1`. **CI:** Bei Push/PR auf `mimir/**` läuft die Pipeline [.github/workflows/mimir.yml](../.github/workflows/mimir.yml) (Test im Container, Lint).

Mimir ist der Privacy Database Service bei Yggdrasil. Er verwaltet eine eigene, isolierte Datenbank für personenbezogene Daten mit extra Sicherheitsschicht.

**Mythologische Bedeutung**: Mimir ist der Wächter des Brunnens Mímisbrunnr (Brunnen der Weisheit). Der Brunnen selbst ist die Datenbank, die Mimir verwaltet.

**Programmiersprache**: Rust

**Dokumentation**: [API](docs/API.md) · [GDPR-Compliance-Guide](docs/GDPR-Compliance-Guide.md) · [Security-Best-Practices](docs/Security-Best-Practices.md) · [Deployment-Guide](docs/Deployment-Guide.md)

## Verantwortlichkeiten

### 1. Privacy Database Management
- **Isolierte Datenbank**: 
  - **Technische Implementierung**: Datenbank-Isolation wird technisch implementiert (separate Datenbank-Instanzen, separate Schemas)
  - **Separate Instanzen**: Separate Datenbank-Instanzen für Privacy-Daten (PostgreSQL, etc.)
  - **Datenbank-Migrations**: Datenbank-Migrations werden gehandhabt (Schema-Migration, Data-Migration)
- **Extra Sicherheitsschicht**: 
  - **Security-Layer-Architektur**: Multi-Layer-Security-Architektur (Verschlüsselung, Access Control, Audit-Logging)
  - **Security-Layer-Updates**: Bei Security-Layer-Updates wird automatisch Migration durchgeführt
  - Verschlüsselung, Access Control, Audit-Logging
- **GDPR-Compliance**: 
  - **Compliance-Implementierung**: GDPR-Compliance wird implementiert (Data Subject Rights, Data Protection)
  - **Compliance-Validierung**: Automatische Compliance-Validierung (Policy-Checks, Audit-Checks)
  - **Compliance-Fehler**: Bei Compliance-Fehlern wird Fehler geloggt, Security-Alert wird ausgelöst
  - Vollständige Einhaltung der GDPR-Anforderungen
- **Data Isolation**: Strikte Isolation von anderen Datenbanken

### 2. Database Operations
- **Query-Optimierung**: Optimierte Datenbankabfragen
- **Connection-Pooling**: Effizientes Connection-Pooling
- **Transaction-Management**: Verwaltung von Datenbank-Transactions
- **Database-Sharding**: Unterstützung für Database-Sharding

### 3. Security & Compliance
- **Verschlüsselung**: Verschlüsselung aller personenbezogenen Daten
- **Access Control**: Strikte Zugriffskontrolle basierend auf User-Identität
- **Audit-Logging**: Vollständiges Audit-Logging aller Datenzugriffe
- **Data Minimization**: Nur notwendige Daten werden gespeichert

### 4. Data Management
- **Data Retention**: Verwaltung von Data-Retention-Policies
- **Data Deletion**: Unterstützung für "Right to Deletion" (GDPR)
- **Data Export**: Unterstützung für "Right to Data Portability" (GDPR)
- **Data Backup**: Sichere Backups mit Verschlüsselung

## Service-Interfaces

### Inputs
- `DatabaseQuery` (von Nornen/anderen Services) - Datenbankabfragen
  - User-Data-Queries
  - Privacy-Data-Queries
  - Configuration-Queries

- `DatabaseWrite` (von Nornen/anderen Services) - Datenbank-Schreiboperationen
  - User-Data-Writes
  - Privacy-Data-Writes
  - Configuration-Writes

### Outputs
- `DatabaseResult` (an Nornen/andere Services) - Abfrage-Ergebnisse
- `DatabaseConfirmation` (an Nornen/andere Services) - Schreib-Bestätigungen

## Workflow

### Database Query

1. **Service sendet Query**
   - Nornen oder anderer Service sendet Database-Query an Mimir
   - Query enthält: Query-Type, Parameters, User-Context

2. **Mimir validiert Query**
   - Access Control wird geprüft
   - User-Context wird validiert
   - Query wird auf Sicherheit geprüft

3. **Query Execution**
   - Query wird ausgeführt
   - Daten werden aus isolierter Datenbank abgerufen
   - Verschlüsselte Daten werden entschlüsselt

4. **Response**
   - Mimir sendet Database-Result zurück
   - Audit-Log wird erstellt

### Database Write

1. **Service sendet Write-Request**
   - Nornen oder anderer Service sendet Database-Write an Mimir
   - Write enthält: Data, User-Context, Write-Type

2. **Mimir validiert Write**
   - Access Control wird geprüft
   - User-Context wird validiert
   - Data wird validiert

3. **Data Encryption**
   - Daten werden verschlüsselt
   - Verschlüsselte Daten werden in Datenbank geschrieben

4. **Confirmation**
   - Mimir sendet Database-Confirmation zurück
   - Audit-Log wird erstellt

## Der Brunnen (Mímisbrunnr)

**Mythologische Bedeutung**: Der Brunnen Mímisbrunnr ist die Quelle der Weisheit. In diesem Kontext ist der Brunnen die Datenbank selbst.

**Datenbank-Struktur:**
- **Isolierte Datenbank**: Komplett getrennt von anderen Datenbanken
- **Verschlüsselte Speicherung**: Alle personenbezogenen Daten sind verschlüsselt
- **Access Control**: Strikte Zugriffskontrolle
- **Audit-Logging**: Vollständiges Audit-Logging

## Security Features

### Verschlüsselung
- **At-Rest Encryption**: Verschlüsselung aller Daten in der Datenbank
- **In-Transit Encryption**: Verschlüsselung aller Datenübertragungen
- **Key Management**: Sichere Verwaltung von Verschlüsselungsschlüsseln
- **Encryption-Algorithms**: Moderne Verschlüsselungsalgorithmen (AES-256, etc.)

### Access Control
- **Role-Based Access Control (RBAC)**: Zugriffskontrolle basierend auf Rollen
- **User-Context-Validation**: Validierung des User-Contexts bei jedem Zugriff
- **Permission-Checking**: Prüfung von Permissions für jeden Datenzugriff
- **Multi-Factor Authentication**: Unterstützung für Multi-Factor Authentication

### Audit-Logging
- **Vollständiges Logging**: Alle Datenzugriffe werden geloggt
- **Immutable Logs**: Logs können nicht verändert werden
- **Compliance-Logging**: Logging erfüllt Compliance-Anforderungen (GDPR)
- **Log-Retention**: Langfristige Aufbewahrung von Logs
- **Strukturiertes Logging**: Structured Logging mit Log Levels (DEBUG, INFO, WARN, ERROR)
- **Context Tracking**: Context wird mitgeloggt für besseres Tracking
- **Log Rotation**: Automatische Log-Rotation
- **Audit-Logs für Security-Audits**: Logs werden für Security-Audits aufbewahrt

## GDPR-Compliance

### Data Subject Rights
- **Right to Access**: User können ihre Daten abrufen
- **Right to Rectification**: User können ihre Daten korrigieren
- **Right to Erasure**: User können ihre Daten löschen ("Right to Deletion")
- **Right to Data Portability**: User können ihre Daten exportieren
- **Right to Object**: User können der Datenverarbeitung widersprechen

### Data Protection
- **Data Minimization**: Nur notwendige Daten werden gespeichert
- **Purpose Limitation**: Daten werden nur für spezifische Zwecke verwendet
- **Storage Limitation**: Daten werden nur so lange gespeichert wie nötig
- **Integrity and Confidentiality**: Datenintegrität und Vertraulichkeit werden gewährleistet

## Abhängigkeiten

### Keine Core Library

- **WICHTIG**: Es gibt keine Edda Core Library
- **Separate Projekte**: Wenn gemeinsame Komponenten benötigt werden (DTOs, Protocols, Utils), sollte ein separates Projekt erstellt werden
- **Selektive Nutzung**: Dies hält Apps klein, da genau gewählt werden kann, was benötigt wird
- **Keine Abhängigkeit**: Mimir sollte nicht auf Dateien/Protocols/Utils aus dem `edda` Verzeichnis verweisen (KEIN PROJEKT - nur Metadaten-Sammlung)

### Technische Abhängigkeiten

- **Database**: PostgreSQL oder spezialisierte Privacy-Datenbank
- **Encryption Libraries**: Für Verschlüsselung
- **Security Libraries**: Für Access Control und Audit-Logging

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

### Mimir-spezifische Settings

**Settings-Inhalt (wird während Implementierung definiert)**
- Database-Konfiguration
- Security-Einstellungen
- Data-Retention-Einstellungen
- **Logging** (optional): `logging.log_format` (`"text"` | `"json"`, Standard: text), `logging.log_directory` (optional, Pfad für Datei-Logs mit täglicher Rotation). Log-Level über Umgebungsvariable `RUST_LOG` (z. B. `RUST_LOG=info` oder `RUST_LOG=mimir=debug`).

## Integration

- **Nornen**: Nutzt Mimir für Datenbank-Zugriff
- **Yggdrasil (Elixir)**: Hauptprozess, koordiniert alle Services
- **Andere Services**: Können Mimir für Privacy-Data-Zugriff nutzen

## Performance

### Performance-Optimierungen
- **Query-Optimierung**: Optimierte Datenbankabfragen mit Indizes
- **Connection-Pooling**: Effizientes Connection-Pooling
- **Caching**: Intelligentes Caching für häufig abgerufene Daten
- **Database-Sharding**: Unterstützung für Database-Sharding bei großen Datenmengen

### Performance-Metriken
- Schnelle Queries (< 50ms für Standard-Queries)
- Effiziente Writes (< 100ms für Standard-Writes)
- Hoher Durchsatz (1000+ Queries/Sekunde pro Instanz)

### Performance-Monitoring

**Performance-Monitoring:**
- Performance-Metriken: Response-Zeiten, Durchsatz, Resource-Usage
- Performance-Tracking für alle Database-Operations
- Kontinuierliche Überwachung und Performance-Optimierung
- Alerts bei Performance-Problemen

## Implementierungs-Notizen

- **Programmiersprache**: Rust
- **Isolierte Datenbank**: Komplett getrennt von anderen Datenbanken
- **Verschlüsselung**: At-Rest und In-Transit Verschlüsselung
- **Access Control**: Strikte Zugriffskontrolle
- **Audit-Logging**: Vollständiges Audit-Logging
- **GDPR-Compliance**: Vollständige Einhaltung der GDPR-Anforderungen
- **Performance**: Optimiert für schnelle Datenbankoperationen


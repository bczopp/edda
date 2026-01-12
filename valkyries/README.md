# Valkyries - Coding Agent Plugin

## Übersicht

Valkyries ist das Coding-Agent-Plugin, das als separates Projekt implementiert wird. Brünhild führt die Valkyries an und koordiniert Sub-Agents für verschiedene Aufgaben. Valkyries kann als Extension zu Asgard hinzugefügt werden oder ist automatisch bei Yggdrasil vorhanden.

### Valkyries-Struktur

**Gesamt: 12 spezialisierte Valkyries + Brünnhilde (Lead Agent) = 13 Agents**

Jede Valkyrie hat einen engen, spezialisierten Fokus, um das Context-Fenster klein zu halten. Die Namen und Aufgaben entsprechen ihren mythologischen Rollen:

1. **Brünnhilde (Brynhildr)** - "Kampfpanzerin/Schutzkämpferin" → Lead Agent (Orchestrierung, Task-Decomposition, Quality Assurance, Schutz des Projekts)
2. **Gunnr** - "Krieg/Schlacht" → Test Agent (Kampf gegen Bugs, Qualitätssicherung)
3. **Hildr** - "Kampf" → Security Agent (Kampf gegen Angriffe, Sicherheitsprotokolle)
4. **Skögul** - "die Rüttelnde" → Performance Agent (System optimieren, Performance erschüttern)
5. **Hrist** - "die Erschütternde" → Refactoring Agent (Code erschüttern/verbessern, Code-Qualität)
6. **Mist** - "Nebel" → DevOps Agent (Cloud-Infrastructure, verschleierte Deployment-Umgebungen)
7. **Skeggjöld** - "Axt-Zeit" → Configuration Agent (Schnelle Konfiguration, Build-Zeit-Optimierung)
8. **Göll** - "Tumult/Kampf" → Frontend Agent (Dynamische UI, interaktive Komponenten)
9. **Geirskögul** - "Speerträgerin" → Database Agent (Durchdringende Datenbankarbeit, präzise Queries)
10. **Þrúðr (Thrúd)** - "Stärke" → Backend Agent (Robuste Backend-Logik, starke Services)
11. **Hlökk** - "Lärm/Kampfgetöse" → Documentation Agent (Kommunikation, laute Dokumentation)
12. **Róta** - "Sturm/Aufruhr" → API Design Agent (Echtzeit-Systeme, Event-Architekturen, stürmische APIs)
13. **Sigrún** - "Geheimnis des Sieges" → Performance/Optimization Agent (Algorithmus-Optimierung, Erfolgsgeheimnisse)

**Prinzip**: Jede Valkyrie arbeitet nur mit ihrem spezialisierten Bereich, um Context-Fenster klein und fokussiert zu halten. Die mythologischen Namen spiegeln ihre Aufgaben wider.

### Schnellübersicht: Alle Valkyries und ihre Aufgaben

| Valkyrie | Mythologische Bedeutung | Software-Aufgabe | Spezialisierung |
|----------|------------------------|------------------|-----------------|
| **Brünnhilde** | "Kampfpanzerin" | Lead Agent | Orchestrierung, Task-Decomposition, Quality Assurance |
| **Gunnr** | "Krieg/Schlacht" | Test Agent | Unit/Integration/E2E-Tests, Qualitätssicherung |
| **Hildr** | "Kampf" | Security Agent | Security-Reviews, Vulnerability-Scanning, Verteidigung |
| **Skögul** | "die Rüttelnde" | Performance Agent | Performance-Analyse, Optimierung, Profiling |
| **Hrist** | "die Erschütternde" | Refactoring Agent | Code-Qualität, Clean Code, Transformation |
| **Mist** | "Nebel" | DevOps Agent | Docker, CI/CD, Cloud-Infrastructure |
| **Skeggjöld** | "Axt-Zeit" | Configuration Agent | Config-Files, Environment-Setup, Build-Config |
| **Göll** | "Tumult/Kampf" | Frontend Agent | UI-Komponenten, Styling, State-Management |
| **Geirskögul** | "Speerträgerin" | Database Agent | Schema-Design, Migrations, Query-Optimierung |
| **Þrúðr** | "Stärke" | Backend Agent | APIs, Business-Logic, Services |
| **Hlökk** | "Lärm/Kampfgetöse" | Documentation Agent | Code-Docs, API-Docs, README |
| **Róta** | "Sturm/Aufruhr" | API Design Agent | API-Design, Contracts, Echtzeit-Systeme |
| **Sigrún** | "Geheimnis des Sieges" | Optimization Agent | Algorithmus-Optimierung, Erfolgsgeheimnisse |

## Projektstruktur

```
valkyries/
├── src/
│   ├── cli/            # CLI Interface
│   │   ├── main.ts
│   │   ├── commands/
│   │   └── prompts/
│   ├── agents/         # Agent Implementations
│   │   ├── brünnhilde/ # Brünnhilde - Lead Agent
│   │   ├── gunnr/      # Gunnr - Test Agent
│   │   ├── hildr/      # Hildr - Security Agent
│   │   ├── skögul/     # Skögul - Performance Agent
│   │   ├── hrist/      # Hrist - Refactoring Agent
│   │   ├── mist/       # Mist - DevOps Agent
│   │   ├── skeggjöld/  # Skeggjöld - Configuration Agent
│   │   ├── göll/       # Göll - Frontend Agent
│   │   ├── geirskögul/ # Geirskögul - Database Agent
│   │   ├── thrúd/      # Þrúðr - Backend Agent
│   │   ├── hlökk/      # Hlökk - Documentation Agent
│   │   ├── róta/       # Róta - API Design Agent
│   │   └── sigrún/     # Sigrún - Optimization Agent
│   ├── services/       # Services
│   │   ├── git/
│   │   ├── llm/
│   │   ├── file/
│   │   └── execution/
│   └── utils/
├── config/
└── tests/
```

## Komponenten

### Brünnhilde (Brynhildr) - Lead Coding Agent

**Mythologische Bedeutung**: "Kampfpanzerin" oder "Schutzkämpferin" - Die mächtigste und bekannteste Walküre, die für Schutz und strategische Führung steht.

#### Spezialisierte Verantwortlichkeiten

**1. Task Analysis & Decomposition**
- **Task-Analyse**: Analysiert eingehende Tasks auf Komplexität, Abhängigkeiten und Anforderungen
- **Dependency-Mapping**: Identifiziert Abhängigkeiten zwischen Sub-Tasks
- **Task-Zerlegung**: Zerlegt komplexe Tasks in atomare, ausführbare Sub-Tasks
- **Prioritätsbestimmung**: Bestimmt Prioritäten für Sub-Tasks basierend auf Abhängigkeiten
- **Resource-Schätzung**: Schätzt benötigte Ressourcen für jeden Sub-Task

**2. Sub-Agent Orchestration**
- **Agent-Auswahl**: Entscheidet, welche Valkyries für welche Sub-Tasks benötigt werden
- **Multi-Instance-Planung**: Plant, wie viele Instanzen jeder Valkyrie benötigt werden
- **Parallelisierungs-Strategie**: Bestimmt, welche Tasks parallel ausgeführt werden können
- **Sequenzielle Koordination**: Koordiniert sequenzielle Tasks mit Abhängigkeiten
- **Load-Balancing**: Verteilt Tasks gleichmäßig auf verfügbare Instanzen

**3. Workflow Management**
- **Workflow-Erstellung**: Erstellt detaillierte Workflows für Task-Ausführung
- **Progress-Tracking**: Verfolgt Fortschritt aller laufenden Sub-Tasks
- **Deadline-Management**: Verwaltet Deadlines und Timeouts für Tasks
- **Retry-Strategien**: Plant Retry-Strategien für fehlgeschlagene Tasks
- **Rollback-Planung**: Plant Rollback-Strategien für kritische Fehler

**4. Quality Assurance & Verification**
- **Code-Review-Koordination**: Koordiniert Code-Reviews durch spezialisierte Agents
- **Test-Verification**: Verifiziert, dass alle Tests erfolgreich sind
- **Integration-Checking**: Prüft Integration zwischen Frontend, Backend und anderen Komponenten
- **Completeness-Verification**: Verifiziert, dass alle Anforderungen erfüllt wurden
- **Quality-Metriken**: Sammelt und analysiert Quality-Metriken von allen Valkyries

**5. Statement Collection & Analysis**
- **Statement-Sammlung**: Sammelt Statements von allen Valkyries nach Abschluss
- **Statement-Analyse**: Analysiert Statements auf Vollständigkeit und Konsistenz
- **Gap-Erkennung**: Erkennt Lücken zwischen erwarteten und tatsächlichen Ergebnissen
- **Iterations-Planung**: Plant zusätzliche Iterationen für fehlende Teile

**6. Communication & Coordination**
- **Thor-Kommunikation**: Empfängt Tasks aus Queue von Thor, sendet Ergebnisse zurück
- **Valkyrie-Kommunikation**: Delegiert Tasks an Sub-Valkyries über interne Queue
- **Status-Updates**: Sendet regelmäßige Status-Updates an Thor
- **Error-Reporting**: Meldet Fehler und Probleme an Thor

**7. Resource Management**
- **Resource-Allokation**: Allokiert Ressourcen für Sub-Agents
- **Resource-Überwachung**: Überwacht Resource-Usage aller Instanzen
- **Resource-Optimierung**: Optimiert Resource-Allokation für beste Performance
- **Cleanup-Koordination**: Koordiniert Cleanup von Ressourcen nach Task-Abschluss

#### Workflow (Detailliert)
1. **Task empfangen**: Task wird aus Queue von Thor abgeholt
2. **Task-Analyse**: Task wird analysiert (Komplexität, Abhängigkeiten, Anforderungen)
3. **Task-Decomposition**: Task wird in Sub-Tasks zerlegt mit Dependency-Mapping
4. **Workflow-Erstellung**: Detaillierter Workflow wird erstellt
5. **Agent-Auswahl & Instanzierung**: Entscheidet, welche Valkyries benötigt werden, startet Instanzen
6. **Task-Delegation**: Delegiert Sub-Tasks an entsprechende Valkyries
7. **Progress-Monitoring**: Überwacht Fortschritt aller laufenden Tasks
8. **Statement-Collection**: Sammelt Statements von allen Valkyries nach Abschluss
9. **Quality-Verification**: Prüft Qualität und Vollständigkeit aller Ergebnisse
10. **Iteration (falls nötig)**: Plant und delegiert zusätzliche Tasks für fehlende Teile
11. **Task-Completion**: Task wird abgeschlossen, Results werden zurückgegeben

### Sub-Valkyries

**Wichtig**: Jede Valkyrie hat ihre eigene, klar definierte Aufgabe, um Context-Fenster klein und fokussiert zu halten. Nach Abschluss gibt jede Valkyrie ein kurzes Statement ab.

#### Göll - Frontend Agent

**Mythologische Bedeutung**: "Tumult" oder "Kampf" - Steht für dynamische, kämpferische Energie und Bewegung.

**Spezialisierte Verantwortlichkeiten:**
- **UI-Komponenten-Generierung**: Erstellt React/Vue/Angular-Komponenten
- **Styling & Layout**: Generiert CSS, SCSS, Tailwind-Klassen, Layout-Strukturen
- **State-Management**: Implementiert State-Management (Redux, Zustand, Context API)
- **Routing**: Erstellt Routing-Strukturen und Navigation
- **Form-Handling**: Implementiert Formulare mit Validierung
- **API-Integration**: Integriert Frontend mit Backend-APIs
- **Responsive Design**: Erstellt responsive Designs für verschiedene Bildschirmgrößen
- **Accessibility**: Implementiert Accessibility-Features (ARIA, Keyboard-Navigation)
- **Performance-Optimierung**: Optimiert Bundle-Size, Lazy-Loading, Code-Splitting

**Spezialisierte Tools & Technologien:**
- **Frameworks**: React, Vue.js, Angular, Svelte
- **Styling**: CSS, SCSS, Tailwind CSS, Styled Components, CSS Modules
- **State**: Redux, Zustand, MobX, Context API, Pinia
- **Routing**: React Router, Vue Router, Angular Router
- **Build-Tools**: Vite, Webpack, Parcel, esbuild
- **Testing**: Jest, Vitest, React Testing Library, Cypress (für E2E)
- **Type-Safety**: TypeScript, PropTypes, Flow

**Context-Isolation:**
- Arbeitet nur mit Frontend-relevantem Code
- Kein Zugriff auf Backend-Code oder Datenbank-Schemas
- Minimale Abhängigkeiten zu anderen Valkyries (nur API-Contracts)

**Multi-Instance-Szenarien:**
- Eine Instanz pro Komponente (bei großen Tasks)
- Eine Instanz pro Feature-Modul
- Parallele Bearbeitung von unabhängigen UI-Bereichen

**Statement-Format:**
- "Frontend: [Anzahl] Komponenten erstellt, [Anzahl] Stylesheets generiert, Routing implementiert"

#### Þrúðr (Thrúd) - Backend Agent

**Mythologische Bedeutung**: "Stärke" - Tochter Thors, steht für Robustheit, Kraft und Ausdauer.

**Spezialisierte Verantwortlichkeiten:**
- **API-Entwicklung**: Erstellt REST/GraphQL/gRPC APIs
- **Datenbank-Integration**: Implementiert Datenbank-Zugriffe, ORM-Mappings
- **Business-Logic**: Implementiert Geschäftslogik und Service-Layer
- **Authentication & Authorization**: Implementiert Auth-Systeme (JWT, OAuth, etc.)
- **Middleware-Entwicklung**: Erstellt Middleware für Request-Processing
- **Error-Handling**: Implementiert umfassendes Error-Handling
- **Data-Validation**: Implementiert Input-Validierung und Sanitization
- **Caching-Strategien**: Implementiert Caching-Layer (Redis, Memcached)
- **Background-Jobs**: Erstellt Background-Job-Systeme (Queues, Workers)

**Spezialisierte Tools & Technologien:**
- **Frameworks**: Express, FastAPI, Spring Boot, NestJS, Django, Flask
- **Datenbanken**: PostgreSQL, MySQL, MongoDB, Redis, SQLite
- **ORM/ODM**: Prisma, TypeORM, Sequelize, Mongoose, SQLAlchemy
- **API-Formate**: REST, GraphQL, gRPC, WebSocket
- **Authentication**: JWT, OAuth2, Passport.js, Auth0
- **Testing**: Jest, pytest, Mocha, JUnit
- **Validation**: Zod, Joi, class-validator, Pydantic

**Context-Isolation:**
- Arbeitet nur mit Backend-relevantem Code
- Kein Zugriff auf Frontend-Code oder UI-Komponenten
- Kennt API-Contracts für Frontend-Integration

**Multi-Instance-Szenarien:**
- Eine Instanz pro Service/Microservice
- Eine Instanz pro API-Endpoint-Gruppe
- Parallele Bearbeitung von unabhängigen Backend-Modulen

**Statement-Format:**
- "Backend: [Anzahl] API-Endpoints erstellt, [Anzahl] Services implementiert, Datenbank-Schema aktualisiert"

#### Gunnr - Test Agent

**Mythologische Bedeutung**: "Krieg" oder "Schlacht" - Steht für den Kampf gegen Fehler, Qualitätssicherung und den Sieg über Bugs.

**Spezialisierte Verantwortlichkeiten:**
- **Unit-Test-Generierung**: Erstellt Unit-Tests für einzelne Funktionen/Komponenten
- **Integration-Test-Generierung**: Erstellt Integration-Tests für Service-Interaktionen
- **E2E-Test-Generierung**: Erstellt End-to-End-Tests für User-Flows
- **Test-Data-Generierung**: Erstellt Mock-Daten und Test-Fixtures
- **Test-Coverage-Analyse**: Analysiert Code-Coverage und identifiziert Lücken
- **Test-Refactoring**: Refactoriert bestehende Tests für bessere Wartbarkeit
- **Performance-Tests**: Erstellt Performance- und Load-Tests
- **Security-Tests**: Erstellt Security-Tests (Penetration-Tests, Vulnerability-Scans)

**Spezialisierte Tools & Technologien:**
- **Unit-Testing**: Jest, Vitest, pytest, JUnit, Mocha, RSpec
- **E2E-Testing**: Cypress, Playwright, Selenium, Puppeteer
- **Integration-Testing**: Supertest, Testcontainers, WireMock
- **Mocking**: Sinon, Mockito, unittest.mock, MSW (Mock Service Worker)
- **Coverage**: Istanbul, Coverage.py, JaCoCo
- **Performance**: k6, Artillery, JMeter, Locust
- **Security**: OWASP ZAP, Burp Suite (Integration)

**Context-Isolation:**
- Arbeitet nur mit Test-relevantem Code
- Kennt Code-Struktur für Test-Generierung
- Minimaler Zugriff auf Produktions-Code (nur für Test-Erstellung)

**Multi-Instance-Szenarien:**
- Eine Instanz pro Test-Suite (Unit, Integration, E2E)
- Eine Instanz pro Modul/Feature
- Parallele Bearbeitung von unabhängigen Test-Bereichen

**Statement-Format:**
- "Tests: [Anzahl] Unit-Tests, [Anzahl] Integration-Tests, [Anzahl] E2E-Tests erstellt, Coverage: [Prozent]%"

#### Hlökk - Documentation Agent

**Mythologische Bedeutung**: "Lärm" oder "Kampfgetöse" - Steht für Kommunikation, laute Verkündigung und klare Dokumentation.

**Spezialisierte Verantwortlichkeiten:**
- **Code-Dokumentation**: Generiert JSDoc/TSDoc/Python-Docstrings
- **API-Dokumentation**: Erstellt OpenAPI/Swagger/GraphQL-Schema-Dokumentation
- **README-Generierung**: Erstellt und aktualisiert README-Dateien
- **Architektur-Dokumentation**: Dokumentiert System-Architektur und Design-Entscheidungen
- **User-Guides**: Erstellt Benutzerhandbücher und Tutorials
- **Changelog-Verwaltung**: Verwaltet CHANGELOG.md und Release-Notes
- **Code-Comments**: Fügt sinnvolle Code-Kommentare hinzu
- **Diagramm-Generierung**: Erstellt Architektur-Diagramme (Mermaid, PlantUML)

**Spezialisierte Tools & Technologien:**
- **Markup**: Markdown, reStructuredText, AsciiDoc
- **API-Docs**: OpenAPI/Swagger, GraphQL Schema, Postman Collections
- **Code-Docs**: JSDoc, TSDoc, Sphinx, Doxygen
- **Diagramme**: Mermaid, PlantUML, Graphviz
- **Generatoren**: Docusaurus, MkDocs, Sphinx, GitBook
- **Formatting**: Prettier, Markdownlint

**Context-Isolation:**
- Arbeitet nur mit Dokumentations-relevantem Code
- Analysiert Code-Struktur für Dokumentations-Generierung
- Minimaler Zugriff auf Code-Implementierung (nur für Dokumentation)

**Multi-Instance-Szenarien:**
- Eine Instanz pro Dokumentations-Bereich (API, User-Guide, Architecture)
- Eine Instanz pro Modul/Feature
- Parallele Bearbeitung von unabhängigen Dokumentations-Bereichen

**Statement-Format:**
- "Dokumentation: [Anzahl] README-Dateien aktualisiert, API-Dokumentation generiert, [Anzahl] Code-Kommentare hinzugefügt"

#### Geirskögul - Database Agent

**Mythologische Bedeutung**: "Speerträgerin" - Steht für Präzision, Durchdringung und gezielte Aktionen.

**Spezialisierte Verantwortlichkeiten:**
- **Schema-Design**: Erstellt und optimiert Datenbank-Schemas
- **Migration-Generierung**: Erstellt Datenbank-Migrations (up/down)
- **Query-Optimierung**: Optimiert SQL-Queries für Performance
- **Index-Management**: Erstellt und verwaltet Datenbank-Indizes
- **Data-Modeling**: Erstellt Entity-Relationship-Diagramme
- **Seed-Data-Generierung**: Erstellt Seed-Daten für Development/Testing
- **Backup-Strategien**: Plant Backup- und Recovery-Strategien
- **Connection-Pooling**: Konfiguriert Connection-Pooling

**Spezialisierte Tools & Technologien:**
- **Datenbanken**: PostgreSQL, MySQL, MongoDB, SQLite, Redis
- **ORM/ODM**: Prisma, TypeORM, Sequelize, Mongoose, SQLAlchemy, Drizzle
- **Migration-Tools**: Prisma Migrate, TypeORM Migrations, Alembic, Flyway
- **Query-Tools**: SQL, NoSQL-Queries, Query-Builder
- **Schema-Tools**: pgAdmin, MySQL Workbench, MongoDB Compass

**Context-Isolation:**
- Arbeitet nur mit Datenbank-relevantem Code
- Kein Zugriff auf Frontend-Code oder Business-Logic
- Kennt API-Contracts für Datenbank-Integration

**Multi-Instance-Szenarien:**
- Eine Instanz pro Datenbank-Schema
- Eine Instanz pro Migration-Set
- Parallele Bearbeitung von unabhängigen Datenbank-Bereichen

**Statement-Format:**
- "Database: Schema erstellt, [Anzahl] Migrations generiert, [Anzahl] Indizes optimiert"

#### Róta - API Design Agent

**Mythologische Bedeutung**: "Sturm" oder "Aufruhr" - Steht für Echtzeit-Events, dynamische Systeme und stürmische Aktivität.

**Spezialisierte Verantwortlichkeiten:**
- **API-Design**: Entwirft REST/GraphQL/gRPC API-Strukturen
- **Contract-Definition**: Erstellt API-Contracts (OpenAPI, GraphQL Schema)
- **Endpoint-Design**: Plant API-Endpoints mit Request/Response-Schemas
- **Versioning-Strategien**: Implementiert API-Versioning
- **Rate-Limiting-Design**: Plant Rate-Limiting-Strategien
- **Error-Response-Design**: Standardisiert Error-Response-Formate
- **API-Dokumentation-Struktur**: Erstellt Struktur für API-Dokumentation
- **Mock-Server-Generierung**: Generiert Mock-Server für Frontend-Entwicklung

**Spezialisierte Tools & Technologien:**
- **API-Formate**: REST, GraphQL, gRPC, WebSocket
- **Contract-Tools**: OpenAPI/Swagger, GraphQL Schema, Protocol Buffers
- **Design-Tools**: Postman Collections, Insomnia, API Blueprint
- **Mocking**: MSW, json-server, Mockoon, WireMock
- **Validation**: JSON Schema, Zod, Joi

**Context-Isolation:**
- Arbeitet nur mit API-Design-relevantem Code
- Definiert Contracts, die von Backend und Frontend verwendet werden
- Kein Zugriff auf Implementierungs-Details

**Multi-Instance-Szenarien:**
- Eine Instanz pro API-Version
- Eine Instanz pro Service/Microservice
- Parallele Bearbeitung von unabhängigen API-Bereichen

**Statement-Format:**
- "API Design: [Anzahl] Endpoints entworfen, OpenAPI-Schema erstellt, Mock-Server generiert"

#### Hildr - Security Agent

**Mythologische Bedeutung**: "Kampf" - Steht für den Kampf gegen Angriffe, Schutz und Verteidigung.

**Spezialisierte Verantwortlichkeiten:**
- **Security-Reviews**: Führt automatische Security-Reviews durch
- **Vulnerability-Scanning**: Scannt Code auf bekannte Vulnerabilities
- **Authentication-Review**: Prüft Authentication-Implementierungen
- **Authorization-Review**: Prüft Authorization-Logic
- **Input-Validation-Review**: Prüft Input-Validierung und Sanitization
- **Secret-Management**: Prüft auf Hardcoded Secrets, empfiehlt Secret-Management
- **Dependency-Scanning**: Scannt Dependencies auf Security-Issues
- **Security-Best-Practices**: Implementiert Security-Best-Practices

**Spezialisierte Tools & Technologien:**
- **Scanning**: OWASP ZAP, Snyk, npm audit, pip-audit, Safety
- **SAST**: SonarQube, CodeQL, Semgrep, Bandit
- **Dependency-Scanning**: Dependabot, Renovate, Snyk
- **Secret-Detection**: GitGuardian, TruffleHog, detect-secrets
- **Security-Frameworks**: OWASP Top 10, CWE, CVE

**Context-Isolation:**
- Arbeitet nur mit Security-relevantem Code
- Analysiert Code auf Security-Issues
- Minimaler Zugriff auf Code-Implementierung (nur für Security-Analyse)

**Multi-Instance-Szenarien:**
- Eine Instanz pro Security-Bereich (Auth, Input-Validation, etc.)
- Eine Instanz pro Modul/Service
- Parallele Bearbeitung von unabhängigen Security-Bereichen

**Statement-Format:**
- "Security: [Anzahl] Vulnerabilities gefunden, [Anzahl] Security-Issues behoben, Dependency-Scan durchgeführt"

#### Skögul - Performance Agent

**Mythologische Bedeutung**: "die Rüttelnde" - Steht für Bewegung, Optimierung und das Erschüttern von Ineffizienzen.

**Spezialisierte Verantwortlichkeiten:**
- **Performance-Analyse**: Analysiert Code auf Performance-Bottlenecks
- **Profiling**: Führt Code-Profiling durch
- **Optimierung**: Optimiert langsame Code-Stellen
- **Caching-Strategien**: Implementiert Caching-Strategien
- **Database-Query-Optimierung**: Optimiert langsame Datenbank-Queries
- **Bundle-Size-Optimierung**: Optimiert Frontend-Bundle-Sizes
- **Lazy-Loading**: Implementiert Lazy-Loading-Strategien
- **Performance-Metriken**: Sammelt und analysiert Performance-Metriken

**Spezialisierte Tools & Technologien:**
- **Profiling**: Chrome DevTools, Node.js Profiler, Python cProfile
- **Performance-Testing**: Lighthouse, WebPageTest, k6, Artillery
- **Bundle-Analysis**: webpack-bundle-analyzer, source-map-explorer
- **Caching**: Redis, Memcached, CDN-Konfiguration
- **Monitoring**: Performance Monitoring Tools, APM

**Context-Isolation:**
- Arbeitet nur mit Performance-relevantem Code
- Analysiert Code auf Performance-Issues
- Minimaler Zugriff auf Code-Implementierung (nur für Performance-Analyse)

**Multi-Instance-Szenarien:**
- Eine Instanz pro Performance-Bereich (Frontend, Backend, Database)
- Eine Instanz pro Modul/Service
- Parallele Bearbeitung von unabhängigen Performance-Bereichen

**Statement-Format:**
- "Performance: [Anzahl] Bottlenecks identifiziert, [Anzahl] Optimierungen durchgeführt, Bundle-Size um [Prozent]% reduziert"

#### Mist - DevOps Agent

**Mythologische Bedeutung**: "Nebel" - Steht für verschleierte Umgebungen, Cloud-Infrastructure und mysteriöse Deployment-Prozesse.

**Spezialisierte Verantwortlichkeiten:**
- **Docker-Containerisierung**: Erstellt Dockerfiles und docker-compose.yml
- **CI/CD-Pipeline**: Erstellt CI/CD-Pipelines (GitHub Actions, GitLab CI, etc.)
- **Infrastructure-as-Code**: Erstellt IaC-Skripte (Terraform, CloudFormation)
- **Deployment-Skripte**: Erstellt Deployment-Skripte und -Konfigurationen
- **Environment-Konfiguration**: Konfiguriert Development/Staging/Production-Environments
- **Monitoring-Setup**: Konfiguriert Monitoring und Logging
- **Kubernetes-Manifests**: Erstellt Kubernetes-Manifests (falls benötigt)
- **Build-Konfiguration**: Konfiguriert Build-Prozesse

**Spezialisierte Tools & Technologien:**
- **Containerization**: Docker, Podman, containerd
- **Orchestration**: Kubernetes, Docker Swarm, Nomad
- **CI/CD**: GitHub Actions, GitLab CI, Jenkins, CircleCI, Azure DevOps
- **IaC**: Terraform, CloudFormation, Pulumi, Ansible
- **Cloud**: AWS, Azure, GCP, Cloudflare
- **Monitoring**: Prometheus, Grafana, Datadog, New Relic

**Context-Isolation:**
- Arbeitet nur mit DevOps-relevantem Code
- Kein Zugriff auf Application-Code (nur Deployment-Konfiguration)
- Kennt Deployment-Anforderungen

**Multi-Instance-Szenarien:**
- Eine Instanz pro Environment (Dev, Staging, Prod)
- Eine Instanz pro Service/Microservice
- Parallele Bearbeitung von unabhängigen DevOps-Bereichen

**Statement-Format:**
- "DevOps: Dockerfile erstellt, CI/CD-Pipeline konfiguriert, [Anzahl] Environments eingerichtet"

#### Hrist - Refactoring Agent

**Mythologische Bedeutung**: "die Erschütternde" - Steht für das Erschüttern und Verbessern von Code, Transformation und Erneuerung.

**Spezialisierte Verantwortlichkeiten:**
- **Code-Qualität-Analyse**: Analysiert Code auf Code-Smells
- **Refactoring-Planung**: Plant Refactoring-Strategien
- **Code-Struktur-Optimierung**: Optimiert Code-Struktur und -Organisation
- **Clean-Code-Prinzipien**: Wendet Clean-Code-Prinzipien an
- **Design-Pattern-Implementierung**: Implementiert Design-Patterns
- **Legacy-Code-Modernisierung**: Modernisiert Legacy-Code
- **Code-Duplikation-Eliminierung**: Eliminiert Code-Duplikation
- **Naming-Conventions**: Verbessert Naming-Conventions

**Spezialisierte Tools & Technologien:**
- **Code-Analysis**: SonarQube, ESLint, Pylint, RuboCop
- **Refactoring-Tools**: IDE-Refactoring-Tools, jscodeshift, codemods
- **Metrics**: Code-Complexity-Metriken, Maintainability-Index
- **Pattern-Libraries**: Design-Pattern-Implementierungen

**Context-Isolation:**
- Arbeitet nur mit Code-Qualität-relevantem Code
- Analysiert Code-Struktur für Refactoring
- Minimaler Zugriff auf Code-Implementierung (nur für Refactoring)

**Multi-Instance-Szenarien:**
- Eine Instanz pro Refactoring-Bereich (Module, Services, etc.)
- Eine Instanz pro Code-Qualität-Aspekt
- Parallele Bearbeitung von unabhängigen Refactoring-Bereichen

**Statement-Format:**
- "Refactoring: [Anzahl] Code-Smells behoben, [Anzahl] Design-Patterns implementiert, Code-Duplikation um [Prozent]% reduziert"

#### Skeggjöld - Configuration Agent

**Mythologische Bedeutung**: "Axt-Zeit" - Steht für schnelle, präzise Aktionen, Zeit-Optimierung und effiziente Konfiguration.

**Spezialisierte Verantwortlichkeiten:**
- **Config-File-Generierung**: Erstellt Konfigurationsdateien (.env, config.json, etc.)
- **Environment-Variable-Management**: Verwaltet Environment-Variablen
- **Build-Konfiguration**: Konfiguriert Build-Tools (webpack.config.js, tsconfig.json, etc.)
- **Linter-Konfiguration**: Konfiguriert Linter (ESLint, Prettier, etc.)
- **Test-Konfiguration**: Konfiguriert Test-Frameworks (Jest, pytest, etc.)
- **Editor-Konfiguration**: Erstellt Editor-Konfigurationen (.editorconfig, .vscode/)
- **Package-Management**: Verwaltet package.json, requirements.txt, etc.
- **Secret-Management-Konfiguration**: Konfiguriert Secret-Management

**Spezialisierte Tools & Technologien:**
- **Config-Formate**: JSON, YAML, TOML, .env, XML
- **Build-Tools**: webpack, Vite, Rollup, esbuild
- **Linters**: ESLint, Prettier, Pylint, RuboCop
- **Package-Manager**: npm, yarn, pnpm, pip, poetry, cargo
- **Secret-Management**: Vault, AWS Secrets Manager, Azure Key Vault

**Context-Isolation:**
- Arbeitet nur mit Konfigurations-Dateien
- Kein Zugriff auf Application-Code
- Kennt Konfigurations-Anforderungen

**Multi-Instance-Szenarien:**
- Eine Instanz pro Konfigurations-Bereich (Build, Lint, Test, etc.)
- Eine Instanz pro Environment
- Parallele Bearbeitung von unabhängigen Konfigurations-Bereichen

**Statement-Format:**
- "Configuration: [Anzahl] Config-Dateien erstellt, Environment-Variablen konfiguriert, Build-Tools eingerichtet"

## Valkyries-Übersicht mit mythologischen Namen

**Gesamt: 12 spezialisierte Valkyries + Brünnhilde (Lead Agent)**

### Vollständige Liste der Valkyries und ihre mythologischen Aufgaben:

1. **Brünnhilde (Brynhildr)** - "Kampfpanzerin/Schutzkämpferin"
   - **Mythologie**: Die mächtigste Walküre, Beschützerin, strategische Führerin
   - **Software-Aufgabe**: Lead Agent - Orchestrierung, Task-Decomposition, Quality Assurance, Schutz des gesamten Projekts

2. **Gunnr** - "Krieg/Schlacht"
   - **Mythologie**: Kämpferin im Krieg, Siegerin über Feinde
   - **Software-Aufgabe**: Test Agent - Kampf gegen Bugs, Qualitätssicherung, Sieg über Fehler

3. **Hildr** - "Kampf"
   - **Mythologie**: Kämpferin, Verteidigerin
   - **Software-Aufgabe**: Security Agent - Kampf gegen Angriffe, Sicherheitsprotokolle, Verteidigung des Systems

4. **Skögul** - "die Rüttelnde"
   - **Mythologie**: Erschüttert und bewegt Dinge, bringt Bewegung
   - **Software-Aufgabe**: Performance Agent - System optimieren, Performance erschüttern, Bewegung in langsame Systeme bringen

5. **Hrist** - "die Erschütternde"
   - **Mythologie**: Erschüttert und transformiert
   - **Software-Aufgabe**: Refactoring Agent - Code erschüttern/verbessern, Code-Qualität, Transformation

6. **Mist** - "Nebel"
   - **Mythologie**: Verschleiert, verhüllt, mysteriös
   - **Software-Aufgabe**: DevOps Agent - Cloud-Infrastructure, verschleierte Deployment-Umgebungen, mysteriöse Infrastruktur

7. **Skeggjöld** - "Axt-Zeit"
   - **Mythologie**: Schnelle, präzise Aktionen, Zeit-Optimierung
   - **Software-Aufgabe**: Configuration Agent - Schnelle Konfiguration, Build-Zeit-Optimierung, präzise Setup

8. **Göll** - "Tumult/Kampf"
   - **Mythologie**: Dynamische Energie, Bewegung, Kampf
   - **Software-Aufgabe**: Frontend Agent - Dynamische UI, interaktive Komponenten, kämpferische Benutzeroberflächen

9. **Geirskögul** - "Speerträgerin"
   - **Mythologie**: Präzision, Durchdringung, gezielte Aktionen
   - **Software-Aufgabe**: Database Agent - Durchdringende Datenbankarbeit, präzise Queries, gezielte Datenbank-Operationen

10. **Þrúðr (Thrúd)** - "Stärke"
    - **Mythologie**: Tochter Thors, Robustheit, Kraft, Ausdauer
    - **Software-Aufgabe**: Backend Agent - Robuste Backend-Logik, starke Services, ausdauernde APIs

11. **Hlökk** - "Lärm/Kampfgetöse"
    - **Mythologie**: Kommunikation, laute Verkündigung
    - **Software-Aufgabe**: Documentation Agent - Kommunikation, laute Dokumentation, klare Verkündigung

12. **Róta** - "Sturm/Aufruhr"
    - **Mythologie**: Stürmische Aktivität, Echtzeit-Events
    - **Software-Aufgabe**: API Design Agent - Echtzeit-Systeme, Event-Architekturen, stürmische APIs

13. **Sigrún** - "Geheimnis des Sieges"
    - **Mythologie**: Sieg, Erfolg, Geheimnisse des Erfolgs
    - **Software-Aufgabe**: Performance/Optimization Agent - Algorithmus-Optimierung, Erfolgsgeheimnisse, Sieg über Ineffizienz

**Jede Valkyrie hat einen engen, spezialisierten Fokus, um das Context-Fenster klein zu halten. Die mythologischen Namen und Bedeutungen spiegeln ihre Software-Entwicklungsaufgaben wider.**

## Features

### 1. Claude Code Features (Übernommen)
- **Alle Features von Claude Code**: Alle bewährten Features von Claude Code werden übernommen
- **Ralph-ähnliche Funktionalität**: Ähnlich wie das neue Anthropic Plugin "Ralph" für Claude Code
- **Persistent Execution**: Arbeitet nicht aufhören bis Task vollständig erledigt ist
- **Iterative Improvement**: Verbessert Code iterativ bis zur Vollständigkeit
- **Context-Aware**: Nutzt Codebase-Kontext intelligent

### 2. Multi-Instance Orchestration
- **Mehrere Instanzen einer Valkyrie**: Brünhild kann mehrere Instanzen einer Valkyrie starten
- **Beispiel**: Mehrere Frontend-Agents für verschiedene Komponenten parallel
- **Resource Management**: Verwaltung von Ressourcen für mehrere Instanzen
- **Coordination**: Koordination zwischen Instanzen derselben Valkyrie

### 3. Statement System
- **Task Completion Statements**: Jede Valkyrie gibt Statement ab nach Abschluss
- **Statement Format**: Kurz, prägnant (z.B. "Frontend-Komponenten X, Y, Z erstellt")
- **Statement Collection**: Brünhild sammelt alle Statements
- **Vollständigkeitsprüfung**: Brünhild prüft basierend auf Statements, ob Task vollständig ist

### 4. Git Integration
- **Repository Detection**: Automatische Erkennung von Git-Repositories
- **Branch Management**: Branch-Verwaltung
- **Change Tracking**: Tracking von Änderungen
- **Commit Management**: Commit-Verwaltung

### 5. Dynamic Parallelization
- **Parallel Execution**: Parallele Ausführung von Sub-Agents (inkl. mehrerer Instanzen)
- **Git-based Coordination**: Koordination basierend auf Git
- **Resource-based Scaling**: Skalierung basierend auf System-Ressourcen
- **Conflict Resolution**: Auflösung von Konflikten

### 6. Context Management
- **Isolated Contexts**: Isolierte Contexts für Sub-Agents (jede Valkyrie hat eigene Aufgabe)
- **Kleine Context-Fenster**: Jede Valkyrie arbeitet nur mit relevantem Code, um Context-Fenster klein zu halten
- **Minimal Context Sharing**: Minimales Context-Sharing zwischen Valkyries
- **Context Cleanup**: Context-Bereinigung nach Task

### 7. Quality Assurance & Completion Verification
- **Code Review**: Automatische Code-Review
- **Testing**: Automatisches Testing
- **Validation**: Validierung von Änderungen
- **Error Detection**: Erkennung von Fehlern
- **Vollständigkeitsprüfung**: Brünhild stellt sicher, dass Task korrekt und vollständig ausgeführt wurde
- **Iteration**: Falls unvollständig, werden fehlende Teile identifiziert und delegiert

## Spezialisierte Workflows & Interaktionen

### Workflow-Beispiele

#### Beispiel 1: Vollständige Feature-Implementierung
**Task**: "Füge User-Authentication mit Login-Formular hinzu"

**Brünnhilde's Decomposition:**
1. API Design: API-Design für Login/Register (Róta - API Design Agent)
2. Database: User-Schema und Migrations (Geirskögul - Database Agent)
3. Backend: API-Endpoints für Login/Register (Þrúðr - Backend Agent)
4. Frontend: Login-Komponente mit Formular (Göll - Frontend Agent)
5. Security: Security-Review für Authentication (Hildr - Security Agent)
6. Tests: Unit-Tests für Backend + Frontend (Gunnr - Test Agent)
7. Configuration: Environment-Variablen für Secrets (Skeggjöld - Configuration Agent)
8. Dokumentation: API-Docs + README-Update (Hlökk - Documentation Agent)

**Parallele Ausführung:**
- API Design Agent erstellt API-Contract zuerst
- Database Agent und Backend Agent arbeiten parallel (mit API-Contract)
- Frontend Agent wartet auf API-Contract, dann parallele Implementierung
- Security Agent wartet auf Backend/Frontend, dann Security-Review
- Test Agent wartet auf Backend/Frontend, dann parallele Test-Generierung
- Configuration Agent arbeitet parallel (unabhängig)
- Documentation Agent wartet auf alle, dann Dokumentations-Generierung

**Brünnhilde's Verification:**
- Prüft, dass API-Contract existiert
- Prüft, dass Database-Schema existiert
- Prüft, dass API-Endpoints existieren
- Prüft, dass Frontend-Komponente existiert und API integriert
- Prüft, dass Security-Review durchgeführt wurde
- Prüft, dass Tests existieren und erfolgreich sind
- Prüft, dass Konfiguration vollständig ist
- Prüft, dass Dokumentation vollständig ist

#### Beispiel 2: Refactoring mit Tests
**Task**: "Refactoriere User-Service und füge Tests hinzu"

**Brünnhilde's Decomposition:**
1. Refactoring: Code-Qualität-Analyse und Refactoring-Planung (Hrist - Refactoring Agent)
2. Backend: Refactoring des User-Service (Þrúðr - Backend Agent)
3. Performance: Performance-Analyse und Optimierung (Skögul - Performance Agent)
4. Tests: Unit-Tests für refactorierten Service (Gunnr - Test Agent)
5. Security: Security-Review für refactorierten Code (Hildr - Security Agent)
6. Dokumentation: Update der Service-Dokumentation (Hlökk - Documentation Agent)

**Sequenzielle Ausführung:**
- Refactoring Agent analysiert Code und plant Refactoring
- Backend Agent refactoriert basierend auf Plan
- Performance Agent analysiert refactorierten Code
- Test Agent generiert Tests basierend auf refactoriertem Code
- Security Agent führt Security-Review durch
- Documentation Agent aktualisiert Dokumentation

#### Beispiel 3: Multi-Component Frontend-Task
**Task**: "Erstelle Dashboard mit 5 verschiedenen Widgets"

**Brünnhilde's Decomposition:**
- 5x Göll (Frontend Agent) Instanzen (eine pro Widget)
- 1x Göll (Frontend Agent) für Dashboard-Container
- 1x Skögul (Performance Agent) für Bundle-Optimierung
- 1x Gunnr (Test Agent) für Widget-Tests
- 1x Hlökk (Documentation Agent) für Dashboard-Dokumentation

**Parallele Ausführung:**
- Alle 5 Widget-Instanzen arbeiten parallel
- Dashboard-Container wartet auf Widgets
- Performance Agent analysiert Bundle-Size nach Widget-Erstellung
- Test Agent wartet auf alle Widgets, dann Test-Generierung

### Inter-Valkyrie-Kommunikation

**API-Contract-Flow:**
1. Róta (API Design Agent) erstellt API-Contract zuerst
2. Þrúðr (Backend Agent) implementiert basierend auf Contract
3. Göll (Frontend Agent) integriert basierend auf Contract
4. Brünnhilde koordiniert diese Abstimmung über interne Queue

**Database-Integration:**
- Geirskögul (Database Agent) erstellt Schema und Migrations
- Þrúðr (Backend Agent) verwendet Schema für ORM-Mappings
- Róta (API Design Agent) berücksichtigt Datenbank-Struktur für API-Design

**Security-Integration:**
- Hildr (Security Agent) prüft Code von Backend und Frontend Agent
- Hildr scannt Dependencies (Skeggjöld - Configuration Agent)
- Hildr empfiehlt Security-Verbesserungen

**Performance-Integration:**
- Skögul (Performance Agent) analysiert Code von Frontend und Backend Agent
- Skögul optimiert Database-Queries (Geirskögul - Database Agent)
- Skögul empfiehlt Caching-Strategien

**Test-Integration:**
- Gunnr (Test Agent) benötigt Zugriff auf generierten Code
- Brünnhilde stellt Code-Kontext für Gunnr bereit
- Gunnr generiert Tests basierend auf Code-Struktur

**DevOps-Integration:**
- Mist (DevOps Agent) konfiguriert Deployment für Backend/Frontend
- Mist verwendet Konfiguration von Skeggjöld (Configuration Agent)
- Mist erstellt CI/CD-Pipelines für Tests

**Refactoring-Integration:**
- Hrist (Refactoring Agent) analysiert Code von allen Code-Generierenden Agents
- Hrist empfiehlt Verbesserungen
- Þrúðr/Göll implementieren Refactoring-Vorschläge

**Dokumentations-Integration:**
- Hlökk (Documentation Agent) analysiert generierten Code
- Hlökk verwendet API-Contracts von Róta (API Design Agent)
- Brünnhilde stellt Code-Kontext für Hlökk bereit

### Statement-System Details

**Statement-Struktur:**
```typescript
interface ValkyrieStatement {
  valkyrie: 'frontend' | 'backend' | 'test' | 'docs';
  instanceId: string;
  taskId: string;
  completed: boolean;
  summary: string;  // Kurze Zusammenfassung
  details: {
    filesCreated: string[];
    filesModified: string[];
    metrics?: Record<string, number>;  // z.B. { components: 3, tests: 10 }
  };
  errors?: string[];
  warnings?: string[];
}
```

**Brünnhilde's Statement-Analyse:**
- Prüft Vollständigkeit: Alle erwarteten Statements vorhanden?
- Prüft Konsistenz: Stimmen Statements mit erwarteten Ergebnissen überein?
- Identifiziert Lücken: Fehlen erwartete Dateien oder Features?
- Plant Iterationen: Falls Lücken gefunden, plant zusätzliche Tasks

**ValkyrieResult-Struktur (an Thor):**
```typescript
interface ValkyrieResult {
  taskId: string;
  completed: boolean;
  summary: string;  // Zusammenfassung für Odin
  changes: {
    filesCreated: FileChange[];
    filesModified: FileChange[];
    filesDeleted: string[];
  };
  metadata: {
    statements: ValkyrieStatement[];
    qualityMetrics: QualityMetrics;
    warnings: string[];
    errors: string[];
  };
  // Thor analysiert dies und entscheidet über Actions
}

interface FileChange {
  path: string;
  content: string;  // Vollständiger Datei-Inhalt
  operation: 'create' | 'modify' | 'delete';
}
```

**Thor's Analyse von ValkyrieResult:**
- **Datei-Änderungen erkennen**: Thor erkennt alle `filesCreated`, `filesModified`, `filesDeleted`
- **FILE_OPERATION Actions erstellen**: Thor erstellt `FILE_OPERATION` Actions für jede Datei-Änderung
- **System-Commands erkennen**: Thor erkennt, ob System-Commands nötig sind (z.B. npm install, git commit)
- **Text-Response erstellen**: Thor erstellt Text-Response für Odin basierend auf `summary` und `metadata`
- **Actions ausführen**: Thor führt alle Actions aus (via Mjölnir)
- **ThorResult erstellen**: Thor erstellt `ThorResult` mit Text-Response und Action-Results

## Integration mit Thor

### Thor als Tool-Calling-Agent

**Wichtig**: Thor ist der Tool-Calling-Agent des Systems. Er entscheidet, welche Actions ausgeführt werden müssen.

**Brünnhilde → Thor Kommunikation:**
- **Strukturierte Ergebnisse**: Brünnhilde gibt strukturierte Ergebnisse an Thor weiter
- **Ergebnis-Format**: Brünnhilde sendet strukturierte `ValkyrieResult` mit:
  - Code-Änderungen (Dateien, die geändert/erstellt/gelöscht wurden)
  - Dokumentation
  - Tests
  - Metadaten (Statements, Quality-Metriken)
- **Keine direkten Actions**: Brünnhilde führt KEINE direkten File-Operations oder System-Commands aus
- **Thor entscheidet**: Thor analysiert die Ergebnisse und entscheidet, welche Actions nötig sind

**Thor's Entscheidungslogik:**
- **Datei-Änderungen**: Thor erkennt, welche Dateien geändert/erstellt werden müssen → `FILE_OPERATION` Actions
- **System-Commands**: Thor erkennt, ob System-Commands nötig sind → `SYSTEM_COMMAND` Actions
- **Antwort für Odin**: Thor erkennt, ob es nur eine Antwort für Odin ist → `ThorResult` mit Text-Response
- **Kombination**: Oft Kombination aus File-Operations und Antwort

### Thor als Vermittler zwischen Odin und Brünnhilde
- **Odin ruft Valkyries NICHT direkt auf**: Odin arbeitet mit Thor und übergibt ihm Aufgaben
- **Thor erkennt Coding-Aufgaben**: Thor erkennt automatisch, ob es sich um eine Coding-Aufgabe handelt
- **User kann explizit angeben**: User kann auch explizit angeben, dass es eine Coding-Aufgabe ist (macht Erkennung einfacher)
- **Queue-basierte Kommunikation**: Thor legt Task in Queue, Brünnhilde holt Task ab und verarbeitet ihn
- **Strukturierte Ergebnis-Rückgabe**: Brünnhilde gibt strukturierte Ergebnisse an Thor, Thor entscheidet über Actions

### Workflow: Odin → Thor → Brünnhilde → Valkyries → Thor → Odin

1. **Odin erkennt Anforderung**: Odin erkennt, dass etwas verlangt wird, erstellt `ThorAction` und sendet es an Thor
2. **Thor erkennt Coding-Aufgabe**: Thor prüft, ob es sich um eine Coding-Aufgabe handelt, legt Task in Queue für Brünnhilde
3. **Brünnhilde verarbeitet Task**: Brünnhilde holt Task aus Queue, analysiert Task und delegiert an Sub-Valkyries
4. **Valkyries arbeiten**: Sub-Valkyries arbeiten an ihren Aufgaben (über interne Queue), Brünnhilde koordiniert und überwacht
5. **Brünnhilde sammelt Ergebnisse**: Brünnhilde sammelt alle Statements und Ergebnisse von Valkyries
6. **Brünnhilde prüft und strukturiert**: Brünnhilde prüft Vollständigkeit und erstellt strukturiertes `ValkyrieResult`
7. **Brünnhilde → Thor**: Brünnhilde sendet strukturiertes Ergebnis an Thor (über Queue)
8. **Thor analysiert Ergebnisse**: Thor analysiert `ValkyrieResult` und entscheidet:
   - Welche Dateien müssen geändert/erstellt werden? → `FILE_OPERATION` Actions
   - Welche System-Commands sind nötig? → `SYSTEM_COMMAND` Actions
   - Was ist die Antwort für Odin? → Text-Response
9. **Thor führt Actions aus**: Thor führt die erkannten Actions aus (via Mjölnir)
10. **Thor → Odin**: Thor sendet `ThorResult` an Odin zurück (mit Text-Response und Action-Results)

## CLI Interface

### Commands
- `valkyries init` - Initialize Valkyries in project
- `valkyries task <description>` - Execute task
- `valkyries status` - Check task status
- `valkyries history` - View task history
- `valkyries config` - Configure Valkyries

### Example Usage
```bash
# Initialize in project
valkyries init

# Execute task
valkyries task "Add user authentication with JWT"

# Check status
valkyries status

# View history
valkyries history
```

## Deployment

### Als Extension zu Asgard
- **Optional**: Valkyries kann als Extension zu Asgard hinzugefügt werden
- **Installation**: User kann Valkyries separat installieren
- **Integration**: Nach Installation integriert sich Valkyries mit Asgard
- **Thor-Integration**: Thor erkennt, ob Valkyries verfügbar ist

### Automatisch bei Yggdrasil
- **Standard**: Valkyries ist automatisch bei Yggdrasil vorhanden
- **Keine Installation nötig**: User muss nichts installieren
- **Immer verfügbar**: Coding-Aufgaben können immer über Yggdrasil verarbeitet werden
- **Thor-Integration**: Thor erkennt automatisch, dass Valkyries verfügbar ist

## LLM-Konfiguration

### Standard-Konfiguration
- **Per Default**: Alle Valkyries nutzen dasselbe LLM (konfigurierbar über Geri)
- **Einheitliche Model-Auswahl**: Brünhild und alle Sub-Valkyries verwenden standardmäßig das gleiche Model
- **Konsistente Ergebnisse**: Einheitliche Model-Auswahl sorgt für konsistente Code-Qualität

### Individuelle Konfiguration
- **Konfigurierbar**: Jede Valkyrie kann ein eigenes LLM konfiguriert bekommen
- **Use-Case-spezifisch**: Verschiedene Valkyries können verschiedene Models nutzen (z.B. spezialisierte Coding-Models)
- **Konfiguration**: Über `valkyries config` oder Konfigurationsdatei
- **Gilt auch außerhalb von Ragnarok**: Diese Konfigurationsmöglichkeit gilt für alle Valkyries-Installationen

### Beispiel-Konfiguration
```json
{
  "defaultLLM": "llama-3.1-8b",
  "valkyries": {
    "brünnhilde": "llama-3.1-8b",
    "gunnr": "llama-3.1-8b",
    "hildr": "llama-3.1-8b",
    "skögul": "llama-3.1-8b",
    "hrist": "deepseek-coder-7b",
    "mist": "llama-3.1-8b",
    "skeggjöld": "llama-3.1-8b",
    "göll": "llama-3.1-8b",
    "geirskögul": "deepseek-coder-7b",
    "thrúd": "deepseek-coder-7b",
    "hlökk": "llama-3.1-8b",
    "róta": "llama-3.1-8b",
    "sigrún": "llama-3.1-8b"
  }
}
```

## Abhängigkeiten

- **Edda Core Library**: DTOs, Protocols, Utils
- **Thor**: Für Integration mit Odin (Queue-basierte Kommunikation)
- **Geri**: LLM Service für Code-Generierung (konfigurierbar pro Valkyrie)
- Git Library
- File System APIs
- Execution Environment

## Integration

- **Odin**: Erstellt `ThorAction` und erhält `ThorResult`
- **Thor**: Erkennt Coding-Aufgaben und vermittelt zwischen Odin und Brünhild
- **Brünhild**: Verarbeitet Coding-Tasks und orchestriert Valkyries
- **Valkyries**: Führen Sub-Tasks aus
- **Queue-System**: Für Kommunikation zwischen Thor und Brünhild, sowie Brünhild und Valkyries

## Performance

### Performance-Optimierungen
- **Parallele Execution**: Parallele Ausführung von Sub-Agents für schnellere Completion
- **Context-Isolation**: Isolierte Contexts reduzieren Memory-Usage und verbessern Performance
- **Effiziente Git-Operations**: Optimierte Git-Operations für schnelles Change-Tracking
- **Resource Management**: Intelligentes Resource-Management für optimale Performance
- **Caching**: Caching von häufig verwendeten Code-Patterns und LLM-Responses
- **Streaming**: Streaming von LLM-Responses für bessere UX

### Performance-Metriken
- Schnelle Task-Decomposition (< 5s für komplexe Tasks)
- Effiziente parallele Execution (mehrere Valkyries gleichzeitig)
- Optimierte Context-Management (minimaler Memory-Overhead)

## Datenschutz

### Datenschutz-Features
- **Lokale Verarbeitung**: Code wird lokal verarbeitet, keine unnötige Cloud-Übertragung
- **Minimale Datensammlung**: Nur notwendige Daten werden gespeichert
- **Keine Tracking-Daten**: Keine User-Tracking oder Analytics-Daten
- **User Control**: User hat volle Kontrolle über seine Daten
- **Code-Privacy**: Code bleibt lokal, wird nicht an Dritte weitergegeben

### Compliance
- **GDPR-konform**: Unterstützung für GDPR-Anforderungen
- **Data Minimization**: Nur notwendige Daten werden gespeichert
- **Right to Deletion**: User kann alle Daten löschen
- **Transparency**: User wird über Datenverarbeitung informiert

## Sicherheit

### Security-Features
- **Sandboxing**: Sandboxing für Code-Execution zum Schutz vor schädlichem Code
- **Input Validation**: Umfassende Validierung aller Inputs
- **Code Review**: Automatische Code-Review für Security-Issues
- **Secure Git-Operations**: Sichere Git-Operations ohne Credential-Exposure
- **Permission Checking**: Prüfung von Permissions für File-Operations
- **Audit Logging**: Logging aller Code-Änderungen für Security-Audits

### Security-Best-Practices
- **Keine Hardcoded Secrets**: Keine Hardcoded Passwords oder API-Keys im generierten Code
- **Secure Defaults**: Sichere Standard-Konfigurationen
- **Vulnerability Scanning**: Automatisches Scanning für bekannte Vulnerabilities im Code
- **Dependency Checking**: Prüfung von Dependencies auf Security-Issues
- **Code Signing**: Optional Code-Signing für generierte Artefakte

## Implementierungs-Notizen

- Sollte als CLI-Tool implementiert werden
- **Muss alle Claude Code Features übernehmen**: Besonders bewährte Features von Claude Code
- **Ralph als Referenz**: Das neue Anthropic Plugin "Ralph" für Claude Code dient als Beispiel
- **Brünhild als Orchestrator**: Brünhild delegiert und orchestriert alle anderen Valkyries
- **Multi-Instance Support**: Muss mehrere Instanzen einer Valkyrie unterstützen können
- **Statement System**: Jede Valkyrie muss Statement-System implementieren
- **Kleine Context-Fenster**: Jede Valkyrie arbeitet nur mit relevantem Code (eigene Aufgabe)
- Muss Git-Integration haben
- Sollte parallele Execution unterstützen
- Muss Context-Isolation haben
- Sollte Quality-Assurance haben
- **Vollständigkeitsprüfung**: Brünhild muss sicherstellen, dass Task korrekt und vollständig ist
- **Iterative Completion**: Falls unvollständig, werden fehlende Teile identifiziert und delegiert
- Muss Error-Handling haben
- Sollte User-Feedback geben
- **Persistent Execution**: Muss ähnlich wie Claude Code/Ralph funktionieren (nicht aufhören bis fertig)
- **LLM-Konfiguration**: Per Default nutzen alle Valkyries dasselbe LLM, aber jede Valkyrie kann individuell konfiguriert werden (gilt für alle Installationen)
- **Performance**: Muss optimiert sein für schnelle Code-Generierung und parallele Execution
- **Datenschutz**: Muss Privacy-by-Design implementieren und Code-Privacy gewährleisten
- **Sicherheit**: Muss robuste Security-Mechanismen haben, insbesondere für Code-Execution


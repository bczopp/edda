# IMPLEMENTATION_PLAN - Heimdall (Security Service)

## Übersicht

Dieser Plan beschreibt die kleinstmöglichen Schritte zur Implementierung von Heimdall - dem Security Service. Heimdall ist verantwortlich für Authentication, Authorization, Bifrost Connection Validation, Security Monitoring, Mesh-Membership/Device-Attestation und Guest Network Functionality.

**Mythologische Bedeutung**: Heimdall ist der Wächter der Götter.

**Programmiersprache**: Rust

**Service-Typ**: Core Security Service (Teil aller Platformen)

## Entschiedene Konfiguration

### Protobuf-Rust-Tool
✅ **ENTSCHEIDUNG**: prost + tonic
**Begründung**: Moderne Rust-Lösung, async-native, beste gRPC-Integration

### Database für Token/Device-Registry
✅ **ENTSCHEIDUNG**: PostgreSQL
**Begründung**: Robust, persistent, ACID-compliant, beste Verlässlichkeit für Security-kritische Daten

### Crypto-Library
✅ **ENTSCHEIDUNG**: ring
**Begründung**: Moderne Rust-Crypto, battle-tested, beste Performance, verwendet von Firefox/Cloudflare

### Token-Expiration-Defaults
✅ **ENTSCHEIDUNG**: Heimdall-Token: 24h, Session-Token: 1h, Refresh-Token: 30d
**Begründung**: Gute Balance zwischen Security und User-Experience, industry-standard

### OAuth-Provider-Integration
✅ **ENTSCHEIDUNG**: Google + GitHub + Microsoft (konfigurierbar)
**Begründung**: Maximale User-Abdeckung, alle major Provider, beste Flexibilität

---

## Phase 1: Projekt-Setup & Grundstruktur

### 1.1 Projekt-Initialisierung

**Abhängigkeiten**: Keine
**Erforderliche USER-Eingaben**: Protobuf-Rust-Tool, Database-Wahl, Crypto-Library

#### 1.1.1 Cargo-Projekt erstellen
- [x] `Cargo.toml` erstellen
- [x] Basis-Dependencies definieren
  - Async Runtime (tokio)
  - gRPC (tonic, prost) - oder rust-protobuf
  - Database (sqlx mit PostgreSQL/SQLite) - oder in-memory
  - Crypto (ring, rust-crypto, oder sodiumoxide)
  - Serialization (serde, serde_json)
  - Logging (tracing, tracing-subscriber)
  - Error-Handling (anyhow, thiserror)
- [x] `.gitignore` erstellen

#### 1.1.2 Verzeichnisstruktur erstellen
- [x] `heimdall/src/main.rs` erstellen
- [x] `heimdall/src/lib.rs` erstellen
- [x] `heimdall/src/auth/` für Authentication erstellen
- [x] `heimdall/src/authz/` für Authorization erstellen
- [x] `heimdall/src/token/` für Token-Management erstellen
- [x] `heimdall/src/keys/` für Key-Management erstellen
- [x] `heimdall/src/bifrost/` für Bifrost-Validation erstellen
- [x] `heimdall/src/mesh/` für Mesh-Membership/Device-Attestation erstellt (Bifrost nutzt Device-Mesh)
- [x] `heimdall/src/guest/` für Guest-Network erstellen
- [x] `heimdall/src/security/` für Security-Monitoring erstellen
- [x] `heimdall/src/grpc/` für gRPC-Service erstellen
- [x] `heimdall/src/utils/` für Utilities erstellen
- [x] `heimdall/config/` für Konfigurationsdateien erstellen
- [x] `heimdall/tests/` für Tests erstellen

#### 1.1.3 Build-System einrichten
- [x] Build-Scripts in `Cargo.toml` definieren
- [x] Code-Generierungs-Pipeline einrichten (Protobuf → Rust)
- [x] Cargo-Features definieren (z.B. `postgres`, `sqlite`, `in-memory`)

### 1.2 Test-Infrastruktur

**Abhängigkeiten**: 1.1 (Projekt-Initialisierung)

#### 1.2.1 Container-Setup für Tests
- [x] `Dockerfile` für Test-Umgebung erstellen (Dockerfile.test)
- [x] Docker Compose für Test-Services konfigurieren (docker-compose.test.yml)
  - Database-Container (PostgreSQL/SQLite/In-Memory)
  - Mock-Bifrost-Service
  - Mock-Odin-Service
  - Mock-OAuth-Provider
- [x] Test-Container-Startup-Scripts erstellen (run-tests.sh; CMD nutzt ./run-tests.sh)
- [x] **WICHTIG**: Alle Tests müssen in Containern laufen - keine lokalen Dependencies, Tools oder Services auf der Entwicklungsmaschine installieren (Tests nutzen DATABASE_URL aus docker-compose; TestDatabase erfordert DATABASE_URL)

#### 1.2.2 Test-Framework konfigurieren
- [x] Test-Dependencies hinzufügen (tokio-test, mockall, etc.)
- [x] Test-Utilities und Helpers erstellen (tests/utils/test_helpers.rs: wait_for_service, get_service_url)
- [ ] Mock-Setup für Services (bei Bedarf für Integrationstests)
- [x] Test-Data-Generators für Security-Data erstellen (tests/utils/security_helpers.rs: random_owner_id, random_device_id, random_public_key_base64)
- [x] Security-Test-Utilities erstellen (tests/utils/security_helpers.rs)

#### 1.2.3 CI/CD-Pipeline
- [x] GitHub Actions / GitLab CI Workflow erstellen (.github/workflows/heimdall.yml)
- [x] Automatische Test-Ausführung bei Commits konfigurieren (push/PR auf heimdall/** → Test in Container, Timeout 15 min)
- [ ] Code-Coverage-Reporting einrichten (cargo-tarpaulin, optional)
- [ ] Security-Scanning integrieren (cargo audit, optional)
- [x] Linting und Formatting (cargo clippy, cargo fmt --check im Lint-Job)

### 1.3 Settings-System

**Abhängigkeiten**: 1.1 (Projekt-Initialisierung)
**Erforderliche USER-Eingaben**: Token-Expiration-Defaults, OAuth-Provider

#### 1.3.1 Settings-Schema definieren
- [x] Settings-Struktur entwerfen (JSON-Format) (src/utils/config.rs, config/heimdall.json.example)
  - security_policy, token_configuration, permission_system, session_management, oauth (optional), grpc_port, database_url

#### 1.3.2 Settings-Validierung
- [x] Tests für Settings-Validierung schreiben (tests/unit/config_test.rs: validate_settings, zero token/port, empty database_url)
- [x] Rust-Structs für Settings definieren (HeimdallSettings, TokenConfiguration, etc.)
- [x] Settings-Validator implementieren (validate_settings; genutzt von load() und Hot-Reload)
- [x] Tests ausführen und bestehen

#### 1.3.3 Settings-Loader & Hot-Reload
- [x] Tests für Settings-Loader schreiben (test_load_default_settings, test_load_custom_settings, test_validate_settings)
- [x] Settings-Loader implementieren (SettingsManager::load, get, reload)
- [x] Hot-Reload-Mechanismus implementieren (start_hot_reload mit notify; Reload validiert via validate_settings)
- [x] Tests ausführen und bestehen

---

## Phase 2: Protobuf & gRPC Setup

### 2.1 Protobuf Definitions

**Abhängigkeiten**: 1.1 (Projekt-Initialisierung)

#### 2.1.1 Shared Protobuf-Projekt verwenden
❓ **HINWEIS**: Protobuf-Definitions sollten in einem separaten Projekt sein
- [ ] Heimdall als Dependency zu Protobuf-Projekt hinzufügen
- [ ] Protobuf-Definitions importieren

#### 2.1.2 Authentication Protocol
- [x] `AuthenticationService.proto` definieren (proto/authentication.proto; ChallengeRequest/Response, ProofRequest, AuthenticationTokenResponse)
- [x] Code-Generierung konfigurieren (build.rs)

#### 2.1.3 Authorization Protocol
- [x] `AuthorizationService.proto` definieren (proto/authorization.proto)
- [x] Code-Generierung konfigurieren

#### 2.1.4 Token Management Protocol
- [x] `TokenService.proto` definieren (proto/token.proto)
- [x] Code-Generierung konfigurieren

#### 2.1.5 Bifrost Validation Protocol
- [x] `BifrostValidationService.proto` definieren (proto/bifrost_validation.proto)
- [x] Code-Generierung konfigurieren

#### 2.1.6 Mesh-Membership Protocol
- [x] `MeshMembershipService.proto` definiert (proto/mesh_membership.proto)
  - `MeshMembershipRequest` Message
  - `MeshMembershipResponse` Message
  - `MeshAuthTokenRequest` Message
  - `MeshAuthTokenResponse` Message
- [x] Code-Generierung konfiguriert (build.rs kompiliert mesh_membership.proto)

### 2.2 gRPC Server Implementation

**Abhängigkeiten**: 2.1 (Protobuf Definitions)

#### 2.2.1 gRPC Server Setup
- [x] Tests für gRPC-Server-Setup schreiben (tests/unit/grpc_server_test.rs: health_service_can_be_created_and_set_serving)
- [x] gRPC-Server-Setup implementieren (start_grpc_server in src/grpc/server.rs)
  - tonic-Server konfigurieren (Server::builder, add_service)
  - Health-Check-Service (tonic-health: health_reporter, set_service_status, add_service(health_service))
- [x] Tests ausführen und bestehen

#### 2.2.2 Authentication Service
- [x] Tests für Authentication-Service schreiben (tests/unit/authentication_service_test.rs: generate_token_returns_unimplemented)
- [x] `AuthenticationServiceImpl` implementiert (src/grpc/server.rs)
  - `RequestChallenge()` RPC
  - `ProveIdentity()` RPC (nutzt permissions aus prove_identity)
  - `GenerateToken()` RPC (Unimplemented)
- [x] Tests ausführen und bestehen

#### 2.2.3 Authorization Service
- [x] Tests für Authorization-Service schreiben (tests/unit/authorization_service_test.rs: invalid user_id, device not found)
- [x] `AuthorizationServiceImpl` implementiert (src/grpc/server.rs)
  - `CheckPermission()` RPC
  - `CheckRole()` RPC
- [x] Tests ausführen und bestehen

#### 2.2.4 Token Service
- [x] Tests für Token-Service schreiben (tests/unit/token_service_test.rs: validate_token mit invalid token)
- [x] `TokenServiceImpl` implementiert (src/grpc/server.rs)
  - `ValidateToken()` RPC (inkl. .await für async validate_token)
  - `RenewToken()` RPC
  - `RevokeToken()` RPC
- [x] Tests ausführen und bestehen

#### 2.2.5 Bifrost Validation Service
- [x] Tests für Bifrost-Validation-Service schreiben (tests/unit/bifrost_validation_service_test.rs: validate_connection device not found, validate_message invalid token)
- [x] `BifrostValidationServiceImpl` implementiert (src/grpc/server.rs)
  - `ValidateConnection()` RPC
  - `ValidateMessage()` RPC (inkl. .await für async validate_token)
- [x] Tests ausführen und bestehen

#### 2.2.6 Mesh-Membership Service
- [x] `MeshMembershipServiceImpl` implementiert (src/grpc/server.rs)
  - `RegisterDevice()` RPC (register_device mit mesh_public_key)
  - `GenerateMeshAuthToken()` RPC (generate_mesh_auth_token mit mesh_token)
- [x] Tests für Mesh-Membership-Service schreiben (tests/unit/mesh_membership_service_test.rs)
- [x] Tests ausführen und bestehen

---

## Phase 3: Key Management & Crypto

### 3.1 Key-Generation

**Abhängigkeiten**: 1.1 (Projekt-Initialisierung)
**Erforderliche USER-Eingaben**: Crypto-Library

#### 3.1.1 Key-Generator
- [x] Tests für Key-Generator schreiben (tests/unit/keys_test.rs: test_generate_ed25519_keypair, test_generate_random_bytes)
- [x] `KeyGenerator` implementiert (src/keys/generator.rs); Ed25519 + CSPRNG; RSA-Fallback optional
- [x] Tests ausführen und bestehen

### 3.2 Key-Storage

**Abhängigkeiten**: 3.1 (Key-Generation)

#### 3.2.1 Secure-Key-Storage
- [x] Tests für Secure-Key-Storage schreiben (keys_test: test_store_and_load_keypair, test_encrypted_key_storage, test_load_public_key)
- [x] `SecureKeyStorage` implementiert (src/keys/storage.rs); verschlüsselt/Key-Loading
- [x] Tests ausführen und bestehen

### 3.3 Key-Rotation

**Abhängigkeiten**: 3.1 (Key-Generation), 3.2 (Key-Storage)

#### 3.3.1 Key-Rotation-Manager
- [x] Tests für Key-Rotation schreiben (tests/unit/keys_rotation_test.rs)
- [x] `KeyRotationManager` implementiert (src/keys/rotation.rs)
  - Automatische Key-Rotation (rotation_interval, should_rotate)
  - Event-basierte Rotation (rotate_key aufrufbar)
  - Rollover-Mechanismus (Grace-Period: deprecated Key, cleanup_deprecated)
- [x] Tests ausführen und bestehen

### 3.4 Digital Signatures

**Abhängigkeiten**: 3.1 (Key-Generation)

#### 3.4.1 Signature-Manager
- [x] Tests für Signature-Manager schreiben (keys_test: test_sign_and_verify)
- [x] `SignatureManager` implementiert (src/keys/signature.rs)
  - Message-Signatur mit Private Key (Ed25519)
  - Signature-Verification mit Public Key
- [x] Tests ausführen und bestehen

---

## Phase 4: Database Setup (Token/Device-Registry)

### 4.1 Database Configuration

**Abhängigkeiten**: 1.1 (Projekt-Initialisierung)
**Erforderliche USER-Eingaben**: Database-Wahl

#### 4.1.1 Database-Client Setup
- [x] Database-Connection-Manager (src/utils/database.rs: DatabaseManager mit PgPool, Connection-Timeout); Tests indirekt via TestDatabase/Integration
- [x] Tests ausführen und bestehen

#### 4.1.2 Database-Migration System
- [x] Migration-Framework (sqlx migrate! in database.rs / tests)
- [x] Initial-Migrations (migrations/001–004: devices, tokens, sessions, permissions, device_permissions, mesh_devices, etc.)
- [x] Migration-Runner (sqlx::migrate! beim Start/Load)
- [x] Tests ausführen und bestehen (Tests nutzen TestDatabase mit Migrationen)

### 4.2 Database Models

**Abhängigkeiten**: 4.1 (Database Configuration)

#### 4.2.1 Device Model
- [x] Tests (tests/unit/device_repository_test.rs)
- [x] Device/CRUD (src/utils/device_repository.rs, models)
- [x] Tests ausführen und bestehen

#### 4.2.2 Token Model
- [x] Token Struct/CRUD (src/utils/token_repository.rs, models)
- [x] Tests ausführen und bestehen (indirekt via Token-Service/Integration)

#### 4.2.3 Session Model
- [x] Tests (tests/unit/session_tracker_test.rs, session_hijacking_test.rs)
- [x] Session/CRUD (src/utils/session_repository.rs, session_manager.rs)
- [x] Tests ausführen und bestehen

#### 4.2.4 Permission Model
- [x] PermissionManager/CRUD (src/authz/permission.rs; device_permissions, role_permissions)
- [x] Tests ausführen und bestehen (indirekt via Authorization-Service)

#### 4.2.5 Mesh-Device Model (Mesh-Membership)
- [x] Tests (tests/unit/mesh_registry_test.rs)
- [x] MeshDevice/CRUD (src/mesh/registry.rs, utils/models.rs)
- [x] Tests ausführen und bestehen

---

## Phase 5: Authentication Engine

### 5.1 Challenge-Response Protocol

**Abhängigkeiten**: 3.4 (Digital Signatures), 4.2 (Database Models)

#### 5.1.1 Challenge-Generator
- [x] ChallengeGenerator implementiert (src/auth/challenge.rs: generate_challenge, validate_proof)
  - CSPRNG-Challenge, Challenge-Expiration; genutzt in AuthenticationServiceImpl
- [x] Tests indirekt (authentication_service_test, Auth-Manager)

#### 5.1.2 Proof-Validator
- [x] Proof-Validierung in ChallengeGenerator (validate_proof: Signatur, Expiration, Device)
- [x] Tests ausführen und bestehen

### 5.2 Device-Identity-Verification

**Abhängigkeiten**: 3.2 (Key-Storage), 3.4 (Digital Signatures)

#### 5.2.1 Device-Identity-Verifier
- [x] Tests für Device-Identity-Verifier schreiben (tests/unit/device_identity_verifier_test.rs)
- [x] `DeviceIdentityVerifier` implementiert (src/auth/device_verifier.rs)
  - Device-Public-Key validieren (validate_public_key, validate_public_key_base64; Ed25519 32 bytes)
  - Device-Certificate validieren (optional, nicht implementiert)
  - Device-Identity verifizieren (verify_identity via SignatureManager)
- [x] Tests ausführen und bestehen

### 5.3 Email/Code-Verification (optional)

**Abhängigkeiten**: 2.2.2 (Authentication Service)

#### 5.3.1 Email-Verification-Manager
- [ ] Tests für Email-Verification schreiben
- [ ] `EmailVerificationManager` implementieren (TDD)
  - 6-stelligen Code generieren (CSPRNG)
  - Code per Email senden
  - Code validieren (Code, Expiration, Rate-Limiting)
  - Brute-Force-Schutz
- [ ] Tests ausführen und bestehen

### 5.4 OAuth-Integration (optional)

**Abhängigkeiten**: 2.2.2 (Authentication Service)
**Erforderliche USER-Eingaben**: OAuth-Provider

#### 5.4.1 OAuth-Client
- [ ] Tests für OAuth-Client schreiben
- [ ] `OAuthClient` implementieren (TDD)
  - Authorization Code Flow
  - Token-Exchange
  - User-Info-Abfrage
- [ ] Tests ausführen und bestehen

#### 5.4.2 OAuth-Provider-Integration
- [ ] Tests für OAuth-Provider-Integration schreiben
- [ ] OAuth-Provider implementieren (TDD)
  - Google-OAuth-Provider
  - GitHub-OAuth-Provider
  - (Optional: Microsoft-OAuth-Provider)
- [ ] Tests ausführen und bestehen

---

## Phase 6: Token Management

### 6.1 Token-Generation

**Abhängigkeiten**: 3.4 (Digital Signatures), 4.2.2 (Token Model)
**Erforderliche USER-Eingaben**: Token-Expiration-Defaults

#### 6.1.1 Token-Generator
- [x] TokenGenerator implementiert (src/token/generator.rs: heimdall/session/refresh token)
- [x] Tests indirekt (token_service_test, mesh_membership_service_test, authentication_service_test)

### 6.2 Token-Validation

**Abhängigkeiten**: 6.1 (Token-Generation), 3.4 (Digital Signatures)

#### 6.2.1 Token-Validator
- [x] TokenValidator implementiert (src/token/validator.rs: validate_token, with_cache)
- [x] Tests (token_service_test.rs, mesh_token_validator_test.rs)

### 6.3 Token-Renewal

**Abhängigkeiten**: 6.1 (Token-Generation), 6.2 (Token-Validation)

#### 6.3.1 Token-Renewal-Manager
- [x] Tests für Token-Renewal schreiben (tests/unit/token_renewal_test.rs)
- [x] `TokenRenewalManager` implementiert (src/token/renewal.rs)
  - Proaktive Erneuerung (should_renew nach proactive_renewal_seconds vor Ablauf)
  - Automatische Erneuerung via Refresh-Token (renew_heimdall_with_refresh_token)
  - Transparente Erneuerung (ohne User-Intervention)
- [x] Tests ausführen und bestehen

### 6.4 Token-Revocation

**Abhängigkeiten**: 6.2 (Token-Validation), 4.2.2 (Token Model)

#### 6.4.1 Token-Revocation-Manager
- [x] Tests für Token-Revocation schreiben (tests/unit/token_revocation_test.rs)
- [x] `TokenRevocationManager` implementiert (src/token/revocation.rs)
  - Sofortige Revocation (revoke in DB + Cache-Invalidierung)
  - Revocation-Liste in DB (tokens.is_revoked), is_revoked prüft DB
  - Timeout als Fallback (Expiration weiterhin in Validator)
- [x] Tests ausführen und bestehen

### 6.5 Token-Rotation

**Abhängigkeiten**: 6.1 (Token-Generation), 6.3 (Token-Renewal)

#### 6.5.1 Token-Rotation-Manager
- [x] Tests für Token-Rotation schreiben (tests/unit/token_rotation_test.rs)
- [x] `TokenRotationManager` implementiert (src/token/rotation.rs)
  - Regelmäßige Rotation (should_rotate nach rotation_interval_seconds seit issued_at)
  - Event-basierte Rotation (rotate_heimdall_token aufrufbar)
  - Automatische Rotation (Caller ruft rotate auf wenn should_rotate)
- [x] Tests ausführen und bestehen

### 6.6 Token-Leak-Detection

**Abhängigkeiten**: 6.2 (Token-Validation), 4.2.3 (Session Model)

#### 6.6.1 Token-Leak-Detector
- [x] Tests für Token-Leak-Detection schreiben (tests/unit/token_leak_detector_test.rs)
- [x] `TokenLeakDetector` implementiert (src/token/leak_detector.rs)
  - Anomalie-Erkennung (check_anomaly bei Nutzung von > max_devices_per_token)
  - Device-Tracking (record_usage, get_usage_by_device; Fenster window_seconds)
  - Alerts bei verdächtigen Aktivitäten (LeakAlert mit token_id, device_ids, message)
- [x] Tests ausführen und bestehen

---

## Phase 7: Authorization Engine

### 7.1 Permission System

**Abhängigkeiten**: 4.2.4 (Permission Model)

#### 7.1.1 Permission-Manager
- [x] PermissionManager implementiert (src/authz/permission.rs: check_permission, check_role, get_roles)
- [x] Tests (authorization_service_test.rs)

### 7.2 Role-Based Access Control (RBAC)

**Abhängigkeiten**: 7.1 (Permission System)

#### 7.2.1 Role-Manager
- [x] Tests für Role-Manager schreiben (tests/unit/role_manager_test.rs)
- [x] `RoleManager` implementiert (src/authz/role_manager.rs)
  - Basis-Rollen (ensure_base_roles: admin, user, guest mit Hierarchie)
  - Custom-Rollen (create_role mit optionalem parent, description)
  - Rollen-Hierarchie (parent_role_id in roles)
  - Rollen-Vererbung (get_inherited_role_ids für Permission-Check)
  - assign_role_to_device, remove_role_from_device, get_roles_for_device
- [x] Tests ausführen und bestehen

### 7.3 Conditional-Permissions

**Abhängigkeiten**: 7.1 (Permission System)

#### 7.3.1 Conditional-Permission-Evaluator
- [x] Tests für Conditional-Permissions schreiben (tests/unit/conditional_permission_test.rs)
- [x] `ConditionalPermissionEvaluator` implementiert (src/authz/conditional.rs)
  - Zeit-basierte Bedingungen (time_window: start/end hour UTC)
  - Context-basierte Bedingungen (required_context: key-value)
  - IP-basierte Bedingungen (allowed_ips)
  - EvaluationContext (current_hour_utc, context, client_ip)
- [x] Tests ausführen und bestehen

---

## Phase 8: Session Management

### 8.1 Session-Tracking

**Abhängigkeiten**: 4.2.3 (Session Model)

#### 8.1.1 Session-Tracker
- [x] SessionManager/SessionRepository (session_manager.rs, session_repository.rs)
- [x] Tests (session_tracker_test.rs)

### 8.2 Session-Hijacking-Protection

**Abhängigkeiten**: 8.1 (Session-Tracking)

#### 8.2.1 Hijacking-Detector
- [x] HijackingDetector (src/utils/hijacking_detector.rs)
- [x] Tests (session_hijacking_test.rs)

---

## Phase 9: Bifrost Connection Validation

### 9.1 Connection-Validation

**Abhängigkeiten**: 2.2.5 (Bifrost Validation Service), 7.1 (Permission System)

#### 9.1.1 Connection-Validator
- [x] Tests für Connection-Validator schreiben (bifrost_validation_service_test: validate_connection, validate_message)
- [x] `ConnectionValidator` implementiert (src/bifrost/validator.rs)
  - User-Verification (gleicher User vs. verschiedene User)
  - Device-Identity-Validation
  - Permission-Check
  - Security-Policy-Check
  - User-Isolation-Rules (9.2.1)
- [x] Tests ausführen und bestehen

### 9.2 User-Isolation

**Abhängigkeiten**: 9.1 (Connection-Validation)

#### 9.2.1 User-Isolation-Manager
- [x] User-Isolation in ConnectionValidator (gleicher User erlaubt; verschiedene User: Guest/RELAY; DIRECT blockiert)
- [x] Tests ausführen und bestehen

### 9.3 Connection-Monitoring

**Abhängigkeiten**: 9.1 (Connection-Validation)

#### 9.3.1 Connection-Monitor
- [x] Tests für Connection-Monitoring schreiben (tests/unit/connection_monitor_test.rs)
- [x] `ConnectionMonitor` implementiert (src/bifrost/monitor.rs)
  - Connection-Status tracken (ConnectionStatus: Active, Idle, Suspicious, Blocked)
  - Heartbeat-Validierung (record_heartbeat, is_heartbeat_valid)
  - Message-Monitoring (record_message)
  - Status-Updates (set_status, get_status); Bifrost-Integration durch Caller
- [x] Tests ausführen und bestehen

### 9.4 Connection-Blocking

**Abhängigkeiten**: 9.3 (Connection-Monitoring)

#### 9.4.1 Connection-Blocker
- [x] Tests für Connection-Blocking schreiben (tests/unit/connection_blocker_test.rs)
- [x] `ConnectionBlocker` implementiert (src/bifrost/blocker.rs)
  - Sofortige Blockierung (block_connection, Monitor-Status Blocked)
  - Token-Revocation (optional token_id_to_revoke)
  - Blocking-Dauer (duration_secs: temporär, None: permanent)
  - Unblocking (unblock_connection manuell, apply_auto_unblock / is_blocked auto-unblock bei Ablauf)
- [x] Tests ausführen und bestehen

### 9.5 Message-Validation

**Abhängigkeiten**: 3.4 (Digital Signatures)

#### 9.5.1 Message-Validator
- [x] Tests für Message-Validation schreiben (tests/unit/message_validator_test.rs)
- [x] `MessageValidator` implementiert (src/bifrost/message_validator.rs)
  - Message-Signatur-Prüfung (verify_message via SignatureManager)
  - Ungültige Messages verwerfen (Ok(false) / Err)
  - Security-Alert bei ungültigen Messages (durch Caller)
- [x] Tests ausführen und bestehen

### 9.6 Validation-Caching

**Abhängigkeiten**: 9.1 (Connection-Validation)

#### 9.6.1 Validation-Cache-Manager
- [x] Tests für Validation-Caching schreiben (tests/unit/validation_cache_test.rs)
- [x] `ValidationCacheManager` implementiert (src/bifrost/validation_cache.rs)
  - Connection-Validierungen cachen (TTL konfigurierbar, z. B. 5 Min)
  - Cache-Invalidierung bei Permission-Änderungen (invalidate_device)
  - Cache-Miss-Handling (get returns None)
- [x] Tests ausführen und bestehen

---

## Phase 10: Mesh-Membership / Device-Attestation

Heimdall validiert Mesh-Membership: „Darf dieses Device in diesem User-Mesh mitmachen?“ – Bifrost nutzt ein Device-Mesh (Meshtastic-inspiriert).

### 10.1 Mesh-Device-Registry

**Abhängigkeiten**: 4.2.5 (Mesh-Device Model)

#### 10.1.1 Mesh-Device-Registry-Manager
- [x] `MeshDeviceRegistry` implementiert (src/mesh/registry.rs)
  - Device registrieren (register_device)
  - Device-Status prüfen (get_by_device_id)
  - Device-Registry verwalten (update_last_seen)
- [x] Tests für Mesh-Device-Registry schreiben (tests/unit/mesh_registry_test.rs: register_device, get_by_device_id, update_last_seen, DeviceNotFound)
- [x] Tests ausführen und bestehen (Container: docker compose -f docker-compose.test.yml run --rm heimdall-test)

### 10.2 Mesh-Membership-Workflow

**Abhängigkeiten**: 10.1 (Mesh-Device-Registry), 2.2.6 (Mesh-Membership Service)

#### 10.2.1 Mesh-Membership-Manager
- [x] Mesh-Membership-Workflow implementiert (MeshMembershipServiceImpl in grpc/server.rs)
  - Mesh-Membership-Request verarbeiten (RegisterDevice RPC)
  - Device-Status prüfen (registriert/nicht registriert via MeshDeviceRegistry)
  - Mesh-Auth-Token generieren (GenerateMeshAuthToken RPC)
- [x] Email-Benachrichtigung an Owner (für neue Devices) – Stub implementiert (src/mesh/notification.rs: notify_owner_new_device loggt nur)
- [x] Tests für Mesh-Membership schreiben (tests/unit/mesh_membership_service_test.rs: RegisterDevice, GenerateMeshAuthToken, Fehlerfälle)
- [x] Tests ausführen und bestehen (Container: docker compose -f docker-compose.test.yml run --rm heimdall-test)

### 10.3 Mesh-Auth-Token-Management

**Abhängigkeiten**: 6.1 (Token-Generation), 10.1 (Mesh-Device-Registry)

#### 10.3.1 Mesh-Token-Generator
- [x] Mesh-Auth-Token-Generierung implementiert (MeshMembershipServiceImpl::generate_mesh_auth_token nutzt TokenGenerator)
  - Mesh-Auth-Token generieren (mesh_token via generate_session_token)
  - Role zuweisen (Admin, User, Guest aus MeshDevice.role)
  - Token signieren (SignatureManager.sign)
- [x] Tests für Mesh-Token-Generator schreiben (tests/unit/mesh_token_generator_test.rs: Token-Format, Role, Ablauf)
- [x] Tests ausführen und bestehen

#### 10.3.2 Mesh-Token-Validator
- [x] Tests für Mesh-Token-Validator schreiben (tests/unit/mesh_token_validator_test.rs)
- [x] `MeshTokenValidator` implementiert (src/mesh/validator.rs)
  - Mesh-Token validieren (via TokenValidator)
  - Device-Status prüfen (MeshDeviceRegistry.get_by_device_id, is_active)
  - Role zurückgeben (MeshTokenValidationResult.role)
- [x] Tests ausführen und bestehen

### 10.4 Owner-Authorization

**Abhängigkeiten**: 10.2 (Mesh-Membership-Workflow)

#### 10.4.1 Owner-Authorization-Manager
- [x] Tests für Owner-Authorization schreiben (tests/unit/owner_authorization_test.rs)
- [x] `OwnerAuthorizationManager` implementiert (src/mesh/owner_authorization.rs)
  - Device-Details für Owner (get_device_details_for_owner; E-Mail-Versand noch nicht)
  - Rechte-Auswahl (approve_device mit Role admin/user/guest)
  - Device-Autorisierung (approve_device, reject_device; MeshDeviceRegistry.update_role_and_approve, reject_device)
- [x] Tests ausführen und bestehen

---

## Phase 11: Guest Network Functionality

### 11.1 Guest-Network-Creation

**Abhängigkeiten**: 9.1 (Connection-Validation)

#### 11.1.1 Guest-Network-Manager
- [x] GuestNetworkManager (src/guest/network.rs); Tests (guest_network_test.rs)

### 11.2 Guest-Network-Isolation

**Abhängigkeiten**: 11.1 (Guest-Network-Creation)

#### 11.2.1 Guest-Network-Isolator
- [x] GuestNetworkIsolator (src/guest/network.rs); Tests (guest_network_test.rs)

### 11.3 Data-Transfer-Permission

**Abhängigkeiten**: 11.2 (Guest-Network-Isolation)

#### 11.3.1 Data-Transfer-Permission-Manager
- [x] DataTransferPermissionManager (src/guest/network.rs); Tests (guest_network_test.rs)

### 11.4 Explicit-Main-Network-Access

**Abhängigkeiten**: 11.1 (Guest-Network-Creation)

#### 11.4.1 Explicit-Access-Manager
- [x] ExplicitAccessManager (src/guest/network.rs); Tests (guest_network_test.rs)

---

## Phase 12: Security Monitoring & Threat Detection

### 12.1 Audit-Logging

**Abhängigkeiten**: 2.2 (gRPC Server Implementation)

#### 12.1.1 Audit-Logger
- [x] AuditLogger (src/security/audit.rs); Tests (security_monitoring_test.rs)

### 12.2 Threat-Detection

**Abhängigkeiten**: 9.3 (Connection-Monitoring), 8.1 (Session-Tracking)

#### 12.2.1 Threat-Detector
- [x] ThreatDetector (src/security/audit.rs); Tests (security_monitoring_test.rs)

### 12.3 Incident-Response

**Abhängigkeiten**: 12.2 (Threat-Detection)

#### 12.3.1 Incident-Response-Manager
- [x] IncidentResponseManager (src/security/audit.rs); Tests (security_monitoring_test.rs)

### 12.4 Security-Analytics

**Abhängigkeiten**: 12.1 (Audit-Logging)

#### 12.4.1 Security-Analytics-Engine
- [x] SecurityAnalyticsEngine (src/security/audit.rs); Tests (security_monitoring_test.rs)

---

## Phase 13: Caching System

### 13.1 Token-Validation-Cache

**Abhängigkeiten**: 6.2 (Token-Validation)

#### 13.1.1 Token-Cache-Manager
- [x] Tests für Token-Cache schreiben (tests/unit/token_cache_test.rs)
- [x] `TokenValidationCache` (utils/cache.rs) als Token-Cache-Manager; get/set/invalidate, TTL
  - Token-Validierungen cachen (TTL konfigurierbar, z. B. 5 Min)
  - Cache-Invalidierung bei Revocation (invalidate in TokenRevocationManager)
  - TTL-basierte Expiration
- [x] Tests ausführen und bestehen

### 13.2 Permission-Check-Cache

**Abhängigkeiten**: 7.1 (Permission System)

#### 13.2.1 Permission-Cache-Manager
- [x] Tests für Permission-Cache schreiben (tests/unit/permission_cache_test.rs)
- [x] `PermissionCheckCache` (utils/cache.rs) als Permission-Cache-Manager; get/set/invalidate_device
  - Permission-Checks cachen
  - Cache-Invalidierung bei Permission-Änderungen (invalidate_device)
  - TTL-basierte Expiration
- [x] Tests ausführen und bestehen

---

## Phase 14: Encryption & TLS

### 14.1 TLS-Configuration

**Abhängigkeiten**: 3.1 (Key-Generation)

#### 14.1.1 TLS-Config-Manager ✅
- [x] Tests für TLS-Config schreiben
- [x] `TLSConfigManager` implementieren (TDD)
  - [x] TLS 1.3 konfigurieren
  - [x] Cipher Suites (TLS_AES_256_GCM_SHA384, TLS_CHACHA20_POLY1305_SHA256)
  - [x] Certificate Validation
- [x] Tests ausführen und bestehen

### 14.2 End-to-End Encryption (optional)

**Abhängigkeiten**: 3.1 (Key-Generation)

#### 14.2.1 E2E-Encryption-Manager ✅
- [x] Tests für E2E-Encryption schreiben
- [x] `E2EEncryptionManager` implementieren (TDD)
  - [x] Session-Keys generieren (Perfect Forward Secrecy)
  - [x] Key-Austausch (ECDH)
  - [x] Message-Encryption (AES-256-GCM)
- [x] Tests ausführen und bestehen

---

## Phase 15: Performance Optimization

### 15.1 Parallel-Processing

**Abhängigkeiten**: 2.2 (gRPC Server Implementation)

#### 15.1.1 Parallel-Processor
- [x] Tests für Parallel-Processing schreiben (tests/unit/parallel_processing_test.rs)
- [x] Parallele Verarbeitung von Security-Checks implementieren (src/utils/performance.rs: ParallelProcessor)
- [x] Tests ausführen und bestehen

### 15.2 Performance-Benchmarks

**Abhängigkeiten**: 6.2 (Token-Validation), 7.1 (Permission System)

#### 15.2.1 Performance-Benchmarking
- [x] Performance-Benchmarks schreiben (src/utils/performance.rs: PerformanceBenchmark; tests/performance/token_validation_bench_test.rs)
  - Token-Validation (< 10ms)
  - Permission-Check (< 5ms)
- [x] Benchmarks ausführen und Ziele erreichen (run_all_benchmarks mit Mock-Validatoren)

---

## Phase 16: Monitoring & Logging

### 16.1 Structured Logging

**Abhängigkeiten**: 1.1 (Projekt-Initialisierung)

#### 16.1.1 Logging Setup
- [x] Structured-Logging konfigurieren (tracing; main.rs: tracing_subscriber::fmt, EnvFilter)
- [x] Security-specific Log-Levels (README: RUST_LOG, heimdall=info/warn/error für Produktion)
- [x] Log-Rotation konfigurierbar (README: stderr/stdout, Rotation extern z. B. Systemd/Docker)

### 16.2 Performance-Monitoring

**Abhängigkeiten**: 15.2 (Performance-Benchmarks) – optional; Stub unabhängig.

#### 16.2.1 Metrics-Collector
- [x] Tests für Metrics-Collector schreiben (tests/unit/metrics_test.rs)
- [x] `MetricsCollector`-Stub implementiert (src/utils/metrics.rs: token_validations-Zähler)
  - Stub: record_token_validation, get_token_validations; Response-Zeiten/Durchsatz/Resource-Usage optional später
- [x] Tests ausführen und bestehen

---

## Phase 17: Documentation

### 17.1 API Documentation

**Abhängigkeiten**: 2.2 (gRPC Server Implementation)

#### 17.1.1 gRPC Service Documentation
- [x] gRPC-Service-Documentation erstellen (docs/api-grpc.md)
- [x] Authentication-Service-API dokumentieren
- [x] Authorization-Service-API dokumentieren
- [x] Token-Service-API dokumentieren
- [x] Bifrost-Validation-Service-API dokumentieren
- [x] Mesh-Membership-Service-API dokumentieren

### 17.2 Security-Documentation

**Abhängigkeiten**: Alle Security-Phasen

#### 17.2.1 Security-Guide
- [x] Security-Best-Practices dokumentieren (docs/security-guide.md)
- [x] Threat-Models dokumentieren (docs/security-guide.md)
- [x] Security-Workflows dokumentieren (docs/security-guide.md)

---

## Phase 18: Testing & Quality Assurance

### 18.1 Integration Testing

**Abhängigkeiten**: Alle vorherigen Phasen

#### 18.1.1 End-to-End Tests
- [x] E2E-Test-Skelett erstellt (tests/integration/security_workflows_test.rs: Mesh-Register Happy-Path)
  - Erweiterbar: Challenge-Response → Token; Connection-Validation; Mesh-Membership → Mesh-Auth-Token
- [x] E2E-Skelett ausführbar (mit TestDatabase/Container)

### 18.2 Security Testing

**Abhängigkeiten**: Alle Security-Phasen

#### 18.2.1 Security Test Suite
- [x] Security-Test-Skelett erstellt (tests/security/auth_bypass_test.rs: invalid token rejected, empty token rejected)
  - Erweiterbar: Auth-/Authorization-Bypass, Token-/Connection-Security
- [x] Security-Skelett-Tests ausführbar

### 18.3 Performance Testing

**Abhängigkeiten**: 15.1 (Parallel-Processing)

#### 18.3.1 Performance Test Suite
- [x] Performance-Test-Skelett erstellt (tests/performance/token_validation_bench_test.rs; Ziel < 10ms in Kommentar)
  - Token-Validation-Performance-Skelett; Schwellwert optional erweiterbar
- [x] Performance-Skelett ausführbar

---

## Zusammenfassung

**Gesamtanzahl Phasen**: 18
**Gesamtanzahl Schritte**: ~450+

**Kritische Abhängigkeiten**:
1. Protobuf-Rust-Tool (prost + tonic empfohlen)
2. Database für Token/Device-Registry (PostgreSQL, SQLite, In-Memory+File)
3. Crypto-Library (ring, rust-crypto, sodiumoxide)
4. Token-Expiration-Defaults (24h/1h/30d empfohlen)
5. OAuth-Provider-Integration (Google, GitHub optional)

**Offene Fragen für USER**:
1. Protobuf-Rust-Tool (prost + tonic, rust-protobuf)
2. Database (PostgreSQL, SQLite, In-Memory+File)
3. Crypto-Library (ring, rust-crypto, sodiumoxide)
4. Token-Expiration-Defaults (24h/1h/30d, 12h/30min/14d, Konfigurierbar)
5. OAuth-Provider (Google+GitHub, Google+GitHub+Microsoft, Konfigurierbar)

**Hinweise**:
- Alle Schritte folgen TDD (Tests zuerst, dann Implementierung)
- **ALLE Tests müssen in Containern laufen** - keine lokalen Dependencies, Tools oder Services auf der Entwicklungsmaschine installieren
- Container-Setup muss vollständig isolierte Test-Umgebung bereitstellen
- Security ist kritisch: Authentication, Authorization, Token-Management
- Performance: < 10ms Token-Validation, < 5ms Permission-Check
- Fail-Safe: Bei Fehler Deny statt Allow
- Automatisierung: Minimale User-Störung

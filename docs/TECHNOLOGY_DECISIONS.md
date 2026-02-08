# Technology Decisions - Edda Project

**Datum**: 2026-01-18
**Kriterien**: Beste Performance, Verlässlichkeit und Robustheit

---

## 🎯 Übergreifende Entscheidungen (alle Projekte)

### Protobuf & gRPC
- **Protobuf-Rust-Tool**: `prost` + `tonic`
- **Protobuf-TypeScript-Tool**: `ts-proto`
- **Protobuf-Elixir-Tool**: `protobuf-elixir` + `grpc`
- **Ausnahme IoT**: `prost-lite` + `tonic` (Jotunheim, Loki)

**Begründung**: Moderne async-native Lösung, idiomatisches Rust, beste Performance, production-ready

### Security & Encryption
- **Crypto-Library**: `ring`
- **TLS-Library**: `rustls`

**Begründung**: Pure Rust, moderne Kryptographie, battle-tested (Firefox, Cloudflare), keine OpenSSL-Dependencies

### Networking
- **WebSocket-Library**: `tokio-tungstenite`
- **mDNS-Library**: `mdns`
- **NAT-Traversal**: `webrtc-rs`

**Begründung**: Beste tokio-Integration, async-native, robuste Performance, production-ready

### Frontend
- **Package Manager & Runtime**: `bun` (NICHT npm/yarn/pnpm!)
- **Regel**: Wo immer npm verwendet werden könnte, wird stattdessen **bun** genutzt (z. B. `bun install`, `bun run`, Docker: `oven/bun`-Image statt Node + npm).
- **Frontend-Framework**: React (alle Platforms)
- **Build-Tool**: Vite (mit bun)

**Begründung**: bun = 10-100x schneller, React = größte Community, beste Tooling

### Audio & Video
- **Audio-Library**: `cpal`
- **Video-Processing**: `ffmpeg-next`

**Begründung**: Cross-platform, low-latency, umfassende Format-Support

---

## 📱 Platform-spezifische Entscheidungen

### Alfheim (Mobile Platform)
- **Framework**: React Native
- **Package Manager**: bun
- **Audio-Integration**: Text + Voice in Phase 1
- **Voice-Assistants**: Siri + Google Assistant in Phase 1

### Asgard (Homeserver Platform)
- **Database**: PostgreSQL
- **Web-Dashboard**: Minimal in Phase 1
- **Web-Dashboard-Framework**: React
- **Package Manager**: bun
- **API-Framework**: Axum
- **Voice-Input**: Ja

### Jotunheim (IoT Platform)
- **Primary-Target**: ESP32
- **Network-Stack**: esp-idf
- **gRPC-Client**: tonic-lightweight
- **Lua-Engine**: mlua
- **Protobuf**: prost-lite (minimaler Footprint)

### Midgard (Desktop Platform)
- **UI-Framework**: Tauri
- **Frontend-Framework**: React
- **Package Manager**: bun
- **Audio-Library**: cpal

### Ragnarok (Terminal Platform)
- **CLI-Framework**: clap
- **TUI-Framework**: ratatui (optional)

---

## 🛠️ Service-spezifische Entscheidungen

### Databases (alle Services)
- **Standard-Database**: PostgreSQL
- **Betrifft**: Yggdrasil, Nornen, Mimir, Heidrun, Njörðr, Asgard, Frigg, Heimdall, Skuld, Geri, Eikthyrnir

**Begründung**: Robust, ACID-compliant, beste Performance für Queries, Sharding-Support, production-ready

### Bifrost (Communication Service)
- **Message-Format**: Protobuf

**Begründung**: Kompakt, typsicher, schnellste Serialisierung, konsistent mit gRPC

### Eikthyrnir (Quality Assessment)
- **Database**: PostgreSQL
- **Cache**: Redis
- **Aggregation**: Hybrid (sofort + batch)

**Begründung**: Persistente Quality-Daten, schnelle Lookups, beste Balance Real-time vs. Effizienz

### Freki (RAG Service)
- **Vector-Database**: Qdrant
- **Embedding-Model**: all-MiniLM-L6-v2
- **Document-Storage**: Vector-DB
- **Chunking**: tiktoken
- **Cache**: Redis

**Begründung**: Qdrant = Rust-native, beste Performance | all-MiniLM = bewährtes Sentence-Transformer-Model

### Frigg (Healthcare Plugin)
- **Database**: PostgreSQL
- **Encryption**: ring
- **RAG-Integration**: Freki-Code wiederverwenden
- **Fulla-Service**: Als separater Service
- **Certification-Storage**: gRPC + Ratatoskr (hybrid)

**Begründung**: Enterprise-Grade Security, robuste Compliance, keine Code-Duplikation

### Geri (LLM Service)
- **Default-Local-LLM**: Llama 3 8B
- **Local-LLM-Provider**: llama.cpp (direkt) + BitNet.cpp (1-bit Modelle)
- **1-bit Models**: BitNet.cpp (extreme Effizienz, 90% weniger RAM)
- **Vision-Model**: GPT-4V
- **Model-Registry**: PostgreSQL

**Begründung**: Llama 3 8B = beste Balance Qualität/Größe | llama.cpp = minimaler Resource-Impact | BitNet.cpp = extreme Effizienz für 1-bit Modelle (5-10x schneller, 90% weniger RAM)

### Heimdall (Security Service)
- **Database**: PostgreSQL
- **Crypto**: ring
- **Token-Expiration**: 24h (Access), 7d (Refresh), 30d (Device)
- **OAuth-Provider**: Google + GitHub + Microsoft

**Begründung**: Robuste Persistenz, maximale OAuth-Abdeckung, industry-standard Expiration

### Heidrun (Token & Pricing)
- **Database**: PostgreSQL
- **Commission-Rate**: 15% (konfigurierbar)
- **Pricing-Storage**: Database

**Begründung**: Payment-kritische Daten erfordern maximale Robustheit, dynamische Pricing-Verwaltung

### Huginn-Muninn (STT/TTS)
- **Local-STT**: Whisper.cpp
- **Local-TTS**: Coqui TTS
- **Audio**: cpal
- **Video**: ffmpeg-next

**Begründung**: Whisper = beste STT-Qualität | Coqui = beste TTS-Qualität | cpal = beste Cross-platform-Performance

### Læraðr (Data Management)
- **Indexing-Engine**: Tantivy
- **Schema-Validation**: jsonschema
- **Archiving**: S3-compatible

**Begründung**: Tantivy = Rust full-text search | S3 = skalierbar, industry-standard

### Loki (Script Execution)
- **Script-Engine**: mlua
- **Config-Format**: TOML
- **Script-Storage**: Filesystem + inline (hybrid)

**Begründung**: mlua = robuste Lua-Bindings | TOML = Rust-friendly | hybrid = maximale Flexibilität

### Nornen (Decision Service)
- **Database**: PostgreSQL

**Begründung**: Robust, production-ready

### Odin (Main Orchestrator)
- **Plugin-System**: Compile-Time

**Begründung**: Bessere Performance, bessere Sicherheit, robustere Integration

### Ratatoskr (Business Protocol)
- **Message-Format**: Protobuf
- **Serialization**: serde

**Begründung**: Konsistenz mit gRPC, typsicher, beste Performance

### Skuld (LLM Selection)
- **Database**: PostgreSQL

**Begründung**: Robust, konsistent mit anderen Services

### Thor (Action Executor)
- **Sandboxing**: bubblewrap

**Begründung**: Robustes Linux-Sandboxing, production-ready, sicher

### Valkyries (Coding Agent)
- **Code-Analyse**: tree-sitter

**Begründung**: Sprach-agnostisch, robust, beste Performance für Code-Parsing

### Bifrost Device-Mesh (ersetzt Valhalla VPN)
- **Mesh-Protocol**: Meshtastic-inspiriert (MeshPacket, Managed Flood, Hop-Limit, Discovery)
- **Transport**: IP (TCP/WebSocket bzw. UDP), optional LoRa (z.B. Jotunheim)
- **Encryption**: Verschlüsselung/Integrität konsequent (ChaCha20-Poly1305 oder TLS), Mesh-Membership über Heimdall
- **Implementation**: Rust, integriert in Bifrost (ein Dienst für Device-Kommunikation)

**Begründung**: Valhalla (VPN) wurde verworfen. Stattdessen erweitert Bifrost um ein Device-Mesh; ein Dienst, ein Name. Meshtastic-Konzepte für Multi-Hop, Managed Flood; mehr Security (Heimdall-Identität, kein offenes Mesh).

### Vedrfolnir (Connection Builder Client)
- **WebSocket**: tokio-tungstenite
- **TLS**: rustls

**Begründung**: Konsistent mit Bifrost/Nidhöggr, beste Performance

### Yggdrasil (Cloud Server)
- **Framework**: Phoenix (Elixir)
- **Database**: PostgreSQL
- **gRPC-Client**: grpc (Elixir)

**Begründung**: Phoenix = beste Concurrency für Millionen Verbindungen | PostgreSQL = robuste Enterprise-Database

### Forseti (ML/DL/RL Service)
- **Primary-Language**: Rust (Hybrid mit Python-FFI)
- **Python-FFI**: pyo3
- **ML-Frameworks**: PyTorch, TensorFlow, JAX
- **Rust-ML**: burn, candle, linfa
- **RL-Libraries**: stable-baselines3, ray[rllib]
- **Model-Export**: GGUF (llama.cpp/bitnet.cpp), ONNX, SafeTensors
- **Database**: PostgreSQL (Model Registry)

**Begründung**: Hybrid-Architektur kombiniert Rust-Performance mit Python-ML-Ecosystem | Alle Major-Frameworks für maximale Flexibilität | GGUF-Export für Integration mit Geri

---

## 🚀 Quick Reference: Technology Stack

### Rust Services (alle Core Services)
```toml
[dependencies]
tokio = { version = "1", features = ["full"] }
tonic = "0.11"
prost = "0.12"
sqlx = { version = "0.7", features = ["postgres", "runtime-tokio-rustls"] }
ring = "0.17"
rustls = "0.21"
tokio-tungstenite = "0.21"
serde = { version = "1", features = ["derive"] }
tracing = "0.1"
anyhow = "1"
```

### IoT Services (Jotunheim, Loki)
```toml
[dependencies]
prost-lite = "0.1"
tonic = { version = "0.11", default-features = false }
mlua = { version = "0.9", features = ["lua54", "vendored"] }
esp-idf-sys = "0.33"
```

### TypeScript/Frontend (alle Frontends)
```bash
# Installation mit bun (NICHT npm!)
bun create vite my-app --template react-ts
bun install
bun add @grpc/grpc-js ts-proto
bun run dev
```

### Elixir Services (Yggdrasil)
```elixir
defp deps do
  [
    {:phoenix, "~> 1.7"},
    {:postgrex, "~> 0.17"},
    {:protobuf, "~> 0.11"},
    {:grpc, "~> 0.7"}
  ]
end
```

---

## 📊 Zusammenfassung nach Kategorie

### Databases
- **PostgreSQL**: Yggdrasil, Nornen, Mimir, Heidrun, Njörðr, Asgard, Frigg, Heimdall, Skuld, Geri, Eikthyrnir, Forseti
- **Redis**: Cache für Freki, Eikthyrnir
- **Qdrant**: Vector-DB für Freki

### Frontend/UI
- **Tauri**: Midgard
- **React Native**: Alfheim
- **React**: Asgard (Web-Dashboard), Midgard (Frontend)
- **ratatui**: Ragnarok (TUI)
- **Package Manager**: bun (überall!)

### Crypto & Security
- **ring**: Alle Services (Encryption)
- **rustls**: Alle Services (TLS)
- **bubblewrap**: Thor (Sandboxing)
- **ChaCha20-Poly1305 + TLS**: Bifrost Device-Mesh (Mesh-Verschlüsselung, Heimdall-Validierung)

### Audio & Video
- **cpal**: Huginn-Muninn, Alfheim, Midgard
- **Whisper.cpp**: STT (Huginn)
- **Coqui TTS**: TTS (Muninn)
- **ffmpeg-next**: Video-Processing (Huginn)

### AI/ML
- **llama.cpp**: Local-LLM-Provider (Geri - direkt, minimaler Overhead)
- **BitNet.cpp**: 1-bit Modelle (Geri - extreme Effizienz, 90% weniger RAM, 5-10x schneller)
- **Llama 3 8B**: Default-Model (Geri - Standard-Quantisierung)
- **BitNet 3B**: Alternative für schwache Hardware (Geri - 1-bit)
- **GPT-4V**: Vision-Model (Geri)
- **Qdrant**: Vector-Database (Freki)
- **all-MiniLM-L6-v2**: Embedding-Model (Freki)
- **PyTorch, TensorFlow, JAX**: ML/DL Frameworks (Forseti)
- **burn, candle, linfa**: Rust-native ML (Forseti)
- **stable-baselines3, ray[rllib]**: RL Libraries (Forseti)
- **pyo3**: Python-FFI (Forseti)

### Script Execution
- **mlua**: Lua-Engine (Loki, Jotunheim)
- **TOML**: Config-Format (Loki)

### Build & Development
- **bun**: Frontend-Package-Manager (überall!)
- **cargo**: Rust-Build-Tool
- **mix**: Elixir-Build-Tool

---

## ✅ Status: Alle Entscheidungen getroffen

Alle 29 Projekte haben nun klare Technology-Entscheidungen. Die IMPLEMENTATION_PLAN.md Dateien wurden aktualisiert von "Offene Fragen" zu "Entschiedene Konfiguration".

**Nächster Schritt**: Implementation kann beginnen! 🚀

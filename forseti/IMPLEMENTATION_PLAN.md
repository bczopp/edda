# IMPLEMENTATION_PLAN - Forseti (ML/DL/RL Service)

## Übersicht

Dieser Plan beschreibt die kleinstmöglichen Schritte zur Implementierung von Forseti - dem ML/DL/RL (Machine Learning, Deep Learning, Reinforcement Learning) Service. Forseti bietet Custom Model Training, Fine-Tuning, RL-Agent-Training, Inference und Model-Export-Capabilities mit Hybrid-Architektur (Rust + Python-FFI).

**Mythologische Bedeutung**: Forseti ist der nordische Gott der Gerechtigkeit und Entscheidungen - passend für einen Service, der durch ML/RL optimale Entscheidungen trifft.

**Programmiersprache**: Rust (Hybrid mit Python-FFI via pyo3)

**Service-Typ**: Core Service (Gott)

## Entschiedene Konfiguration

### Primary-Language
✅ **ENTSCHEIDUNG**: Rust (Hybrid mit Python-FFI)
**Begründung**: Rust-Performance mit Python-ML-Ecosystem kombiniert, maximale Flexibilität, beste Integration mit Edda-Ecosystem

### Python-FFI
✅ **ENTSCHEIDUNG**: pyo3
**Begründung**: Moderne Python-Rust-Bindings, async-Support via pyo3-asyncio, zero-copy für große Tensors, production-ready

### ML-Frameworks
✅ **ENTSCHEIDUNG**: PyTorch, TensorFlow, JAX (alle drei)
**Begründung**: Maximale Flexibilität für User, PyTorch dominant, TensorFlow production-ready, JAX high-performance

### Rust-ML-Libraries
✅ **ENTSCHEIDUNG**: burn, candle, linfa
**Begründung**: Rust-native ML ohne Python-Overhead, burn = modern, candle = PyTorch-like, linfa = scikit-learn-like

### RL-Libraries
✅ **ENTSCHEIDUNG**: stable-baselines3, ray[rllib]
**Begründung**: stable-baselines3 = bewährt, ray[rllib] = scalable, beide production-ready

### Model-Export-Formate
✅ **ENTSCHEIDUNG**: GGUF (llama.cpp/bitnet.cpp), ONNX, SafeTensors
**Begründung**: GGUF für Integration mit Geri, ONNX für Cross-framework, SafeTensors für Sicherheit

### Database
✅ **ENTSCHEIDUNG**: PostgreSQL (Model Registry)
**Begründung**: Robust, ACID-compliant, strukturiert, konsistent mit anderen Services

---

## Phase 1: Projekt-Setup & Grundstruktur

### 1.1 Projekt-Initialisierung

**Abhängigkeiten**: Keine
**Erforderliche USER-Eingaben**: Keine (alle Konfigurationsentscheidungen bereits getroffen)

#### 1.1.1 Cargo-Projekt erstellen
- [ ] `Cargo.toml` erstellen
- [ ] Basis-Dependencies definieren
  - Async Runtime (tokio)
  - gRPC (tonic, prost)
  - Python-FFI (pyo3, pyo3-asyncio)
  - Rust-ML (burn, candle, linfa)
  - Database (sqlx mit postgres-Feature)
  - Serialization (serde, serde_json)
  - Logging (tracing, tracing-subscriber)
  - Error-Handling (anyhow, thiserror)
  - Crypto (ring)
- [ ] `.gitignore` erstellen

#### 1.1.2 Verzeichnisstruktur erstellen
- [ ] `forseti/src/main.rs` erstellen
- [ ] `forseti/src/lib.rs` erstellen
- [ ] `forseti/src/grpc/` für gRPC-Server erstellen
- [ ] `forseti/src/training/` für Training-Logic erstellen
- [ ] `forseti/src/rl/` für RL-Logic erstellen
- [ ] `forseti/src/inference/` für Inference-Engine erstellen
- [ ] `forseti/src/models/` für Model-Management erstellen
- [ ] `forseti/src/python/` für Python-FFI erstellen
- [ ] `forseti/src/utils/` für Utilities erstellen
- [ ] `forseti/config/` für Konfigurationsdateien erstellen
- [ ] `forseti/tests/` für Tests erstellen
- [ ] `forseti/python/` für Python-Code erstellen

#### 1.1.3 Python-Projekt erstellen
- [ ] `forseti/python/requirements.txt` erstellen
- [ ] Python Dependencies definieren
  - PyTorch
  - TensorFlow
  - JAX
  - stable-baselines3
  - ray[rllib]
  - numpy, pandas
- [ ] Python Verzeichnisstruktur erstellen
  - `forseti/python/training/` für Training-Wrapper
  - `forseti/python/rl/` für RL-Wrapper

### 1.2 Test-Infrastruktur

**Abhängigkeiten**: 1.1 (Projekt-Initialisierung)

#### 1.2.1 Container-Setup für Tests
- [ ] `Dockerfile` für Test-Umgebung erstellen (mit CUDA-Support)
- [ ] Docker Compose für Test-Services konfigurieren
  - PostgreSQL für Model-Registry
  - Mock-Odin-Service
  - Mock-Geri-Service
  - Mock-Thor-Service
  - GPU-enabled Container für PyTorch/TensorFlow
- [ ] Test-Container-Startup-Scripts erstellen
- [ ] **WICHTIG**: Alle Tests müssen in Containern laufen - keine lokalen Dependencies, Tools oder Services auf der Entwicklungsmaschine installieren

#### 1.2.2 Test-Framework konfigurieren
- [ ] Rust Test-Dependencies hinzufügen (tokio-test, mockall, etc.)
- [ ] Python Test-Dependencies hinzufügen (pytest, unittest)
- [ ] Test-Utilities und Helpers erstellen
- [ ] Mock-Setup für Training-Frameworks
- [ ] Test-Data-Generators für Training/RL erstellen
- [ ] Synthetic-Dataset-Generators erstellen

#### 1.2.3 CI/CD-Pipeline
- [ ] GitHub Actions / GitLab CI Workflow erstellen
- [ ] Automatische Test-Ausführung bei Commits konfigurieren
- [ ] Code-Coverage-Reporting einrichten (cargo-tarpaulin)
- [ ] Linting und Formatting (cargo clippy, cargo fmt, black, pylint)
- [ ] GPU-CI-Runner konfigurieren (für ML-Tests)

### 1.3 Settings-System

**Abhängigkeiten**: 1.1 (Projekt-Initialisierung)

#### 1.3.1 Settings-Schema definieren
- [ ] Settings-Struktur entwerfen (JSON/TOML)
  - training_settings
  - rl_settings
  - inference_settings
  - export_settings
  - python_settings
  - database_settings

#### 1.3.2 Settings-Validierung
- [ ] Tests für Settings-Validierung schreiben
- [ ] Rust-Structs für Settings definieren
- [ ] Settings-Validator implementieren (TDD)
- [ ] Tests ausführen und bestehen

#### 1.3.3 Settings-Loader & Hot-Reload
- [ ] Tests für Settings-Loader schreiben
- [ ] Settings-Loader implementieren (TDD)
  - File-Watcher für Hot-Reload
  - Runtime-Settings-Reload
- [ ] Tests ausführen und bestehen

---

## Phase 2: Protobuf & gRPC Server

### 2.1 Protobuf Definitions

**Abhängigkeiten**: 1.1 (Projekt-Initialisierung)

#### 2.1.1 Shared Protobuf-Projekt verwenden
❓ **HINWEIS**: Protobuf-Definitions sollten in einem separaten Projekt sein
- [ ] Forseti zu Protobuf-Projekt hinzufügen
- [ ] Protobuf-Definitions für Forseti erstellen

#### 2.1.2 ForsetiRequest Messages
- [ ] Tests für ForsetiRequest-Serialization schreiben
- [ ] `ForsetiRequest` Message definieren
- [ ] `TrainingTask` Message definieren
- [ ] `RLTask` Message definieren
- [ ] `InferenceTask` Message definieren
- [ ] Tests ausführen und bestehen

#### 2.1.3 ForsetiResponse Messages
- [ ] Tests für ForsetiResponse-Serialization schreiben
- [ ] `ForsetiResponse` Message definieren
- [ ] `TrainingResult` Message definieren
- [ ] `RLResult` Message definieren
- [ ] `InferenceResult` Message definieren
- [ ] Tests ausführen und bestehen

### 2.2 gRPC Server

**Abhängigkeiten**: 2.1 (Protobuf Definitions)

#### 2.2.1 gRPC-Server-Setup
- [ ] Tests für gRPC-Server schreiben
- [ ] gRPC-Server implementieren (tonic)
- [ ] Server-Lifecycle-Management implementieren
- [ ] Tests ausführen und bestehen

#### 2.2.2 gRPC-Handlers
- [ ] Tests für Training-Handler schreiben
- [ ] Training-Handler implementieren (TDD)
- [ ] Tests für RL-Handler schreiben
- [ ] RL-Handler implementieren (TDD)
- [ ] Tests für Inference-Handler schreiben
- [ ] Inference-Handler implementieren (TDD)
- [ ] Tests ausführen und bestehen

---

## Phase 3: Python-FFI Integration (pyo3)

### 3.1 Python-Runtime Setup

**Abhängigkeiten**: 1.1 (Projekt-Initialisierung), 1.2 (Test-Infrastructure)

#### 3.1.1 pyo3-Konfiguration
- [ ] Tests für Python-Runtime-Initialization schreiben
- [ ] pyo3-Runtime-Initialization implementieren (TDD)
- [ ] Python-Interpreter-Lifecycle implementieren
- [ ] Tests ausführen und bestehen

#### 3.1.2 Python-Module-Loading
- [ ] Tests für Python-Module-Loading schreiben
- [ ] Python-Module-Loader implementieren (TDD)
  - Load PyTorch module
  - Load TensorFlow module
  - Load JAX module
  - Load RL-Libraries module
- [ ] Tests ausführen und bestehen

### 3.2 Python-FFI-Bridge

**Abhängigkeiten**: 3.1 (Python-Runtime-Setup)

#### 3.2.1 FFI-Type-Conversion
- [ ] Tests für Type-Conversion schreiben
- [ ] Rust → Python Type-Conversion implementieren (TDD)
- [ ] Python → Rust Type-Conversion implementieren (TDD)
- [ ] Tests ausführen und bestehen

#### 3.2.2 Tensor-Transfer (Zero-Copy)
- [ ] Tests für Tensor-Transfer schreiben
- [ ] Zero-Copy-Tensor-Transfer implementieren (TDD)
  - ndarray → numpy (zero-copy)
  - numpy → ndarray (zero-copy)
- [ ] Tests ausführen und bestehen

#### 3.2.3 Async-Bridge (pyo3-asyncio)
- [ ] Tests für Async-Python-Calls schreiben
- [ ] Async-Python-Call-Bridge implementieren (TDD)
- [ ] Tests ausführen und bestehen

---

## Phase 4: PyTorch Training Integration

### 4.1 PyTorch-Wrapper (Python)

**Abhängigkeiten**: 3.2 (Python-FFI-Bridge)

#### 4.1.1 PyTorch-Training-Wrapper
- [ ] Tests für PyTorch-Training-Wrapper schreiben (Python)
- [ ] `PyTorchTrainer` Klasse implementieren (TDD)
  - Forward Pass
  - Backward Pass
  - Optimizer Step
  - Loss Calculation
- [ ] Tests ausführen und bestehen

#### 4.1.2 PyTorch-Model-Management
- [ ] Tests für Model-Loading schreiben
- [ ] PyTorch-Model-Loader implementieren (TDD)
- [ ] Tests für Model-Saving schreiben
- [ ] PyTorch-Model-Saver implementieren (TDD)
- [ ] Tests ausführen und bestehen

### 4.2 PyTorch-FFI-Bridge (Rust)

**Abhängigkeiten**: 4.1 (PyTorch-Wrapper)

#### 4.2.1 PyTorch-Trainer-Client (Rust)
- [ ] Tests für PyTorch-Trainer-Client schreiben
- [ ] `PyTorchTrainerClient` implementieren (TDD)
  - Call Python `PyTorchTrainer` via FFI
  - Handle Training-Requests
  - Handle Training-Responses
- [ ] Tests ausführen und bestehen

#### 4.2.2 PyTorch-Training-Orchestration
- [ ] Tests für Training-Orchestration schreiben
- [ ] Training-Orchestrator implementieren (TDD)
  - Data Loading
  - Training Loop
  - Validation Loop
  - Checkpointing
- [ ] Tests ausführen und bestehen

---

## Phase 5: TensorFlow Training Integration

### 5.1 TensorFlow-Wrapper (Python)

**Abhängigkeiten**: 3.2 (Python-FFI-Bridge)

#### 5.1.1 TensorFlow-Training-Wrapper
- [ ] Tests für TensorFlow-Training-Wrapper schreiben (Python)
- [ ] `TensorFlowTrainer` Klasse implementieren (TDD)
  - Model Compilation
  - Model Fitting
  - Model Evaluation
- [ ] Tests ausführen und bestehen

#### 5.1.2 TensorFlow-Model-Management
- [ ] Tests für Model-Loading schreiben
- [ ] TensorFlow-Model-Loader implementieren (TDD)
- [ ] Tests für Model-Saving schreiben
- [ ] TensorFlow-Model-Saver implementieren (TDD)
- [ ] Tests ausführen und bestehen

### 5.2 TensorFlow-FFI-Bridge (Rust)

**Abhängigkeiten**: 5.1 (TensorFlow-Wrapper)

#### 5.2.1 TensorFlow-Trainer-Client (Rust)
- [ ] Tests für TensorFlow-Trainer-Client schreiben
- [ ] `TensorFlowTrainerClient` implementieren (TDD)
- [ ] Tests ausführen und bestehen

#### 5.2.2 TensorFlow-Training-Orchestration
- [ ] Tests für Training-Orchestration schreiben
- [ ] Training-Orchestrator implementieren (TDD)
- [ ] Tests ausführen und bestehen

---

## Phase 6: JAX Training Integration

### 6.1 JAX-Wrapper (Python)

**Abhängigkeiten**: 3.2 (Python-FFI-Bridge)

#### 6.1.1 JAX-Training-Wrapper
- [ ] Tests für JAX-Training-Wrapper schreiben (Python)
- [ ] `JAXTrainer` Klasse implementieren (TDD)
  - JIT-Compilation
  - Gradient Calculation (jax.grad)
  - Model Training
- [ ] Tests ausführen und bestehen

#### 6.1.2 JAX-Model-Management
- [ ] Tests für Model-Loading schreiben
- [ ] JAX-Model-Loader implementieren (TDD)
- [ ] Tests für Model-Saving schreiben
- [ ] JAX-Model-Saver implementieren (TDD)
- [ ] Tests ausführen und bestehen

### 6.2 JAX-FFI-Bridge (Rust)

**Abhängigkeiten**: 6.1 (JAX-Wrapper)

#### 6.2.1 JAX-Trainer-Client (Rust)
- [ ] Tests für JAX-Trainer-Client schreiben
- [ ] `JAXTrainerClient` implementieren (TDD)
- [ ] Tests ausführen und bestehen

#### 6.2.2 JAX-Training-Orchestration
- [ ] Tests für Training-Orchestration schreiben
- [ ] Training-Orchestrator implementieren (TDD)
- [ ] Tests ausführen und bestehen

---

## Phase 7: Rust-Native ML (burn, candle, linfa)

### 7.1 burn Integration

**Abhängigkeiten**: 2.2 (gRPC Server)

#### 7.1.1 burn-Training-Pipeline
- [ ] Tests für burn-Training schreiben
- [ ] burn-Trainer implementieren (TDD)
  - Model Definition
  - Training Loop
  - Validation
- [ ] Tests ausführen und bestehen

#### 7.1.2 burn-Model-Export
- [ ] Tests für burn-Model-Export schreiben
- [ ] burn-Exporter implementieren (TDD)
- [ ] Tests ausführen und bestehen

### 7.2 candle Integration

**Abhängigkeiten**: 2.2 (gRPC Server)

#### 7.2.1 candle-Inference-Engine
- [ ] Tests für candle-Inference schreiben
- [ ] candle-Inference-Engine implementieren (TDD)
- [ ] Tests ausführen und bestehen

#### 7.2.2 candle-Model-Loading
- [ ] Tests für Model-Loading schreiben
- [ ] candle-Model-Loader implementieren (TDD)
- [ ] Tests ausführen und bestehen

### 7.3 linfa Integration

**Abhängigkeiten**: 2.2 (gRPC Server)

#### 7.3.1 linfa-ML-Algorithms
- [ ] Tests für linfa-Algorithms schreiben
- [ ] linfa-Wrapper implementieren (TDD)
  - Classification (SVM, Random Forest)
  - Regression (Linear, Ridge)
  - Clustering (K-Means)
- [ ] Tests ausführen und bestehen

---

## Phase 8: RL Engine Core

### 8.1 Environment-Interface

**Abhängigkeiten**: 3.2 (Python-FFI-Bridge)

#### 8.1.1 Environment-Abstraction
- [ ] Tests für Environment-Interface schreiben
- [ ] `Environment` Trait definieren
- [ ] Environment-Lifecycle implementieren (TDD)
  - Reset
  - Step
  - Render
- [ ] Tests ausführen und bestehen

#### 8.1.2 Custom-Environment-Support
- [ ] Tests für Custom-Environment schreiben
- [ ] Custom-Environment-Loader implementieren (TDD)
- [ ] Tests ausführen und bestehen

### 8.2 Agent-Management

**Abhängigkeiten**: 8.1 (Environment-Interface)

#### 8.2.1 Agent-Registry
- [ ] Tests für Agent-Registry schreiben
- [ ] Agent-Registry implementieren (TDD)
  - Agent Creation
  - Agent Storage
  - Agent Loading
- [ ] Tests ausführen und bestehen

#### 8.2.2 Policy-Management
- [ ] Tests für Policy-Management schreiben
- [ ] Policy-Manager implementieren (TDD)
  - Policy Storage
  - Policy Loading
  - Policy Versioning
- [ ] Tests ausführen und bestehen

---

## Phase 9: RL Algorithms (PPO, SAC, DQN, A3C)

### 9.1 PPO (Proximal Policy Optimization)

**Abhängigkeiten**: 8.2 (Agent-Management), 3.2 (Python-FFI-Bridge)

#### 9.1.1 PPO-Wrapper (Python - stable-baselines3)
- [ ] Tests für PPO-Wrapper schreiben (Python)
- [ ] PPO-Trainer implementieren (TDD)
- [ ] Tests ausführen und bestehen

#### 9.1.2 PPO-FFI-Bridge (Rust)
- [ ] Tests für PPO-Client schreiben
- [ ] PPO-Client implementieren (TDD)
- [ ] Tests ausführen und bestehen

### 9.2 SAC (Soft Actor-Critic)

**Abhängigkeiten**: 8.2 (Agent-Management), 3.2 (Python-FFI-Bridge)

#### 9.2.1 SAC-Wrapper (Python - stable-baselines3)
- [ ] Tests für SAC-Wrapper schreiben (Python)
- [ ] SAC-Trainer implementieren (TDD)
- [ ] Tests ausführen und bestehen

#### 9.2.2 SAC-FFI-Bridge (Rust)
- [ ] Tests für SAC-Client schreiben
- [ ] SAC-Client implementieren (TDD)
- [ ] Tests ausführen und bestehen

### 9.3 DQN (Deep Q-Network)

**Abhängigkeiten**: 8.2 (Agent-Management), 3.2 (Python-FFI-Bridge)

#### 9.3.1 DQN-Wrapper (Python - stable-baselines3)
- [ ] Tests für DQN-Wrapper schreiben (Python)
- [ ] DQN-Trainer implementieren (TDD)
- [ ] Tests ausführen und bestehen

#### 9.3.2 DQN-FFI-Bridge (Rust)
- [ ] Tests für DQN-Client schreiben
- [ ] DQN-Client implementieren (TDD)
- [ ] Tests ausführen und bestehen

### 9.4 A3C (Asynchronous Advantage Actor-Critic)

**Abhängigkeiten**: 8.2 (Agent-Management), 3.2 (Python-FFI-Bridge)

#### 9.4.1 A3C-Wrapper (Python - ray[rllib])
- [ ] Tests für A3C-Wrapper schreiben (Python)
- [ ] A3C-Trainer implementieren (TDD)
- [ ] Tests ausführen und bestehen

#### 9.4.2 A3C-FFI-Bridge (Rust)
- [ ] Tests für A3C-Client schreiben
- [ ] A3C-Client implementieren (TDD)
- [ ] Tests ausführen und bestehen

---

## Phase 10: Inference Engine

### 10.1 Rust-Native Inference

**Abhängigkeiten**: 7.2 (candle Integration)

#### 10.1.1 Inference-Engine (candle)
- [ ] Tests für Inference-Engine schreiben
- [ ] Inference-Engine implementieren (TDD)
  - Batch Inference
  - Streaming Inference
  - Real-time Inference
- [ ] Tests ausführen und bestehen

#### 10.1.2 Model-Loader
- [ ] Tests für Model-Loader schreiben
- [ ] Model-Loader implementieren (TDD)
  - Load ONNX Models
  - Load GGUF Models
  - Load SafeTensors Models
- [ ] Tests ausführen und bestehen

### 10.2 Performance-Optimization

**Abhängigkeiten**: 10.1 (Rust-Native Inference)

#### 10.2.1 Batch-Optimization
- [ ] Tests für Batch-Inference schreiben
- [ ] Batch-Optimizer implementieren (TDD)
- [ ] Tests ausführen und bestehen

#### 10.2.2 Caching
- [ ] Tests für Inference-Caching schreiben
- [ ] Cache-Layer implementieren (TDD)
- [ ] Tests ausführen und bestehen

---

## Phase 11: Model Management & Registry

### 11.1 Model Registry (PostgreSQL)

**Abhängigkeiten**: 1.1 (Projekt-Initialisierung)

#### 11.1.1 Database Schema
- [ ] Tests für Database-Schema schreiben
- [ ] Database-Migrations erstellen
  - models Table
  - training_runs Table
  - rl_agents Table
- [ ] Tests ausführen und bestehen

#### 11.1.2 Model-CRUD-Operations
- [ ] Tests für Model-CRUD schreiben
- [ ] Model-Repository implementieren (TDD)
  - Create Model
  - Read Model
  - Update Model
  - Delete Model
- [ ] Tests ausführen und bestehen

### 11.2 Model-Storage

**Abhängigkeiten**: 11.1 (Model Registry)

#### 11.2.1 Model-File-Storage
- [ ] Tests für Model-Storage schreiben
- [ ] Model-Storage implementieren (TDD)
  - Store Model Files
  - Load Model Files
  - Delete Model Files
- [ ] Tests ausführen und bestehen

#### 11.2.2 Model-Versioning
- [ ] Tests für Model-Versioning schreiben
- [ ] Model-Version-Manager implementieren (TDD)
- [ ] Tests ausführen und bestehen

---

## Phase 12: Model Export (GGUF, ONNX, SafeTensors)

### 12.1 GGUF-Export (llama.cpp/bitnet.cpp)

**Abhängigkeiten**: 11.2 (Model-Storage)

#### 12.1.1 GGUF-Converter
- [ ] Tests für GGUF-Converter schreiben
- [ ] GGUF-Converter implementieren (TDD)
  - PyTorch → GGUF
  - TensorFlow → GGUF
  - JAX → GGUF
- [ ] Tests ausführen und bestehen

#### 12.1.2 Integration mit Geri
- [ ] Tests für Geri-Integration schreiben
- [ ] Geri-Notification implementieren (TDD)
  - Notify Geri of new model
  - Geri loads new model
- [ ] Tests ausführen und bestehen

### 12.2 ONNX-Export

**Abhängigkeiten**: 11.2 (Model-Storage)

#### 12.2.1 ONNX-Converter
- [ ] Tests für ONNX-Converter schreiben
- [ ] ONNX-Converter implementieren (TDD)
  - PyTorch → ONNX
  - TensorFlow → ONNX
  - JAX → ONNX
- [ ] Tests ausführen und bestehen

### 12.3 SafeTensors-Export

**Abhängigkeiten**: 11.2 (Model-Storage)

#### 12.3.1 SafeTensors-Converter
- [ ] Tests für SafeTensors-Converter schreiben
- [ ] SafeTensors-Converter implementieren (TDD)
- [ ] Tests ausführen und bestehen

---

## Phase 13: Einherjar Protocol Implementation

### 13.1 Capability-Exposure

**Abhängigkeiten**: 2.2 (gRPC Server)

#### 13.1.1 Capability-Definition
- [ ] Tests für Capability-Definition schreiben
- [ ] Capabilities definieren
  - Training (pytorch, tensorflow, jax, rust-native)
  - RL (ppo, sac, dqn, a3c)
  - Inference (onnx, gguf, rust-native)
  - Export (llama-cpp, bitnet-cpp, onnx, safetensors)
- [ ] Tests ausführen und bestehen

#### 13.1.2 Einherjar-Protocol-Implementation
- [ ] Tests für Einherjar-Protocol schreiben
- [ ] `EinherjarProtocol` Trait implementieren (TDD)
  - get_capabilities()
  - get_responsibility_domains()
- [ ] Tests ausführen und bestehen

### 13.2 Responsibility-Domains

**Abhängigkeiten**: 13.1 (Capability-Exposure)

#### 13.2.1 Domain-Definition
- [ ] Tests für Domain-Definition schreiben
- [ ] Responsibility-Domains definieren
  - machine-learning
  - deep-learning
  - reinforcement-learning
  - model-training
  - model-finetuning
  - prediction
  - anomaly-detection
  - time-series-forecasting
- [ ] Tests ausführen und bestehen

---

## Phase 14: Service Integration (Odin, Geri, Thor, Freki)

### 14.1 Odin Integration

**Abhängigkeiten**: 2.2 (gRPC Server), 13.1 (Einherjar Protocol)

#### 14.1.1 Odin-Client
- [ ] Tests für Odin-Client schreiben
- [ ] Odin-Client implementieren (TDD)
  - Send Training-Progress to Odin
  - Receive Training-Requests from Odin
- [ ] Tests ausführen und bestehen

#### 14.1.2 Integration-Tests mit Odin
- [ ] E2E-Tests für Odin-Integration schreiben
- [ ] E2E-Tests ausführen und bestehen

### 14.2 Geri Integration

**Abhängigkeiten**: 12.1 (GGUF-Export)

#### 14.2.1 Geri-Client
- [ ] Tests für Geri-Client schreiben
- [ ] Geri-Client implementieren (TDD)
  - Export Model to GGUF
  - Notify Geri of new model
- [ ] Tests ausführen und bestehen

#### 14.2.2 Integration-Tests mit Geri
- [ ] E2E-Tests für Geri-Integration schreiben
- [ ] E2E-Tests ausführen und bestehen

### 14.3 Thor Integration

**Abhängigkeiten**: 9.1 (RL Algorithms)

#### 14.3.1 Thor-Client
- [ ] Tests für Thor-Client schreiben
- [ ] Thor-Client implementieren (TDD)
  - RL-Agent uses Thor for actions
  - Thor returns action results
- [ ] Tests ausführen und bestehen

#### 14.3.2 Integration-Tests mit Thor
- [ ] E2E-Tests für Thor-Integration schreiben
- [ ] E2E-Tests ausführen und bestehen

### 14.4 Freki Integration

**Abhängigkeiten**: 4.2 (PyTorch Training)

#### 14.4.1 Freki-Client
- [ ] Tests für Freki-Client schreiben
- [ ] Freki-Client implementieren (TDD)
  - Get Training-Data from Freki
  - Use RAG for Context-aware Training
- [ ] Tests ausführen und bestehen

#### 14.4.2 Integration-Tests mit Freki
- [ ] E2E-Tests für Freki-Integration schreiben
- [ ] E2E-Tests ausführen und bestehen

---

## Phase 15: Performance Optimization

### 15.1 Multi-GPU Support

**Abhängigkeiten**: 4.2 (PyTorch Training), 5.2 (TensorFlow Training)

#### 15.1.1 Data-Parallel Training
- [ ] Tests für Data-Parallel Training schreiben
- [ ] Data-Parallel-Trainer implementieren (TDD)
- [ ] Tests ausführen und bestehen

#### 15.1.2 Model-Parallel Training
- [ ] Tests für Model-Parallel Training schreiben
- [ ] Model-Parallel-Trainer implementieren (TDD)
- [ ] Tests ausführen und bestehen

### 15.2 Distributed Training

**Abhängigkeiten**: 15.1 (Multi-GPU Support)

#### 15.2.1 Multi-Node Training
- [ ] Tests für Multi-Node Training schreiben
- [ ] Multi-Node-Trainer implementieren (TDD)
- [ ] Tests ausführen und bestehen

### 15.3 FFI-Optimization

**Abhängigkeiten**: 3.2 (Python-FFI-Bridge)

#### 15.3.1 Batch-Processing
- [ ] Tests für Batch-Processing schreiben
- [ ] Batch-Processor implementieren (TDD)
- [ ] Tests ausführen und bestehen

#### 15.3.2 Shared-Memory-Optimization
- [ ] Tests für Shared-Memory schreiben
- [ ] Shared-Memory-Manager implementieren (TDD)
- [ ] Tests ausführen und bestehen

---

## Phase 16: Documentation & Testing

### 16.1 End-to-End Tests

**Abhängigkeiten**: Alle vorherigen Phasen

#### 16.1.1 Training-Workflows
- [ ] E2E-Tests für PyTorch-Training schreiben
- [ ] E2E-Tests für TensorFlow-Training schreiben
- [ ] E2E-Tests für JAX-Training schreiben
- [ ] E2E-Tests für Rust-Native-Training schreiben
- [ ] E2E-Tests ausführen und bestehen

#### 16.1.2 RL-Workflows
- [ ] E2E-Tests für PPO-Training schreiben
- [ ] E2E-Tests für SAC-Training schreiben
- [ ] E2E-Tests für DQN-Training schreiben
- [ ] E2E-Tests für A3C-Training schreiben
- [ ] E2E-Tests ausführen und bestehen

#### 16.1.3 Complete-Workflows
- [ ] E2E-Tests für Training → Export → Geri schreiben
- [ ] E2E-Tests für RL → Thor → Inference schreiben
- [ ] E2E-Tests ausführen und bestehen

### 16.2 Performance Tests

**Abhängigkeiten**: 15 (Performance Optimization)

#### 16.2.1 Benchmark-Suite
- [ ] Benchmarks für Training-Throughput erstellen
- [ ] Benchmarks für RL-Inference-Latency erstellen
- [ ] Benchmarks für Model-Export-Time erstellen
- [ ] Benchmarks für FFI-Overhead erstellen
- [ ] Benchmarks ausführen und Ziele erreichen

### 16.3 Documentation

**Abhängigkeiten**: Alle vorherigen Phasen

#### 16.3.1 API-Documentation
- [ ] Rust API-Docs generieren (cargo doc)
- [ ] Python API-Docs generieren (Sphinx)
- [ ] gRPC API-Docs erstellen

#### 16.3.2 User-Documentation
- [ ] Training-Guides erstellen
- [ ] RL-Guides erstellen
- [ ] Deployment-Guide erstellen
- [ ] Troubleshooting-Guide erstellen

#### 16.3.3 Developer-Documentation
- [ ] Architecture-Documentation erstellen
- [ ] FFI-Integration-Guide erstellen
- [ ] Testing-Guide erstellen
- [ ] Contributing-Guide erstellen

---

## Zusammenfassung

**Gesamtanzahl Phasen**: 16
**Gesamtanzahl Schritte**: ~500+

**Kritische Abhängigkeiten**:
1. Python-FFI Integration (Phase 3) - Basis für alle Python-Framework-Integrationen
2. Protobuf & gRPC Server (Phase 2) - Basis für Service-Kommunikation
3. Model Registry (Phase 11) - Basis für Model-Management
4. Einherjar Protocol (Phase 13) - Basis für Service-Integration

**Performance-Ziele**:
- Training-Throughput: > 1000 samples/sec (GPU)
- RL-Inference: < 10ms latency
- Model-Export: < 5min für 7B-Model
- FFI-Overhead: < 5% vs. native Python

**Hinweise**:
- Alle Schritte folgen TDD
- Alle Tests in GPU-enabled Containern
- Hybrid-Architektur: Rust + Python-FFI
- Unterstützung für alle Major-Frameworks (PyTorch, TensorFlow, JAX)
- Rust-native ML für maximale Performance
- Model-Export für Integration mit Geri (GGUF)
- RL-Integration mit Thor für Action-Execution
- GDPR-compliant (Encryption, Audit-Logging, Right to Deletion)

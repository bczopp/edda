# Geri Provider-Integration-Guide

Dieses Dokument beschreibt die Integration von LLM- und Vision-Providern (API-Keys, lokale Models, Konfiguration).

---

## API-Key-Setup

### Sichere Speicherung

- **SecureKeyStorage** (`src/keys/storage.rs`): Speichert und lädt API-Keys pro Provider über das Trait **SecureKeyBackend**.
- **Backends**: OS-Secure-Storage (Keychain, Credential Manager) für Production; **InMemoryKeyBackend** für Tests.
- **Keine Plain-Text-Konfiguration**: API-Keys gehören nicht in `geri.json`; sie werden über `store_key(provider_id, api_key)` bzw. `load_key(provider_id)` verwaltet.

### Key-Rotation

- **KeyRotationManager** (`src/keys/rotation.rs`): `rotate(storage, provider_id, new_api_key)` speichert den neuen Key (Backend kann alte Einträge ersetzen).
- Alte Keys werden durch den Backend-Tausch entfernt; Rotation erfolgt außerhalb von Geri (z. B. durch Yggdrasil oder Admin-Tool).

### Provider-IDs (Beispiele)

- `openai`, `anthropic`, `google` – für Cloud-Provider (wenn Phase 5 implementiert ist).
- Keys nur setzen, wenn der entsprechende Provider genutzt werden soll.

---

## Local-LLM-Setup

### Konfiguration

- **config/geri.json** (bzw. `config/geri.json.example`):
  - **default_local_llm**: Modellname für Text-Generierung (z. B. `llama3-8b`). Wird vom lokalen LLM-Provider (Phase 4) verwendet.
  - **vision_model**: Modellname für Bild-/Video-Analyse (z. B. `gpt-4v`). Kann Cloud oder lokal sein.

### Geplante lokale Provider (Phase 4)

- **llama.cpp**: Standard-Local-LLM (GGUF); Default z. B. Llama 3 8B.
- **BitNet.cpp**: 1-bit-Modelle für geringeren Ressourcenbedarf.
- Model-Pfade und Binaries werden in späteren Phasen (z. B. Phase 17 Default Local LLM Installation) konfiguriert.

### Ablauf

1. Konfiguration laden (`GeriSettings`: `default_local_llm`, `vision_model`).
2. Lokaler Provider nutzt `default_local_llm`; Vision-Processor nutzt `vision_model`.
3. Keine API-Keys für rein lokale Models nötig.

---

## Cloud-Provider (geplant)

- **OpenAI, Anthropic, Google**: Geplant in Phase 5; dann API-Keys über SecureKeyStorage mit jeweiligem `provider_id` hinterlegen.
- Bis dahin: Nur lokale und Mock-Provider nutzbar.

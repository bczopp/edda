# Geri gRPC Service Documentation

Package: `geri`  
Proto: `proto/geri.proto`

## Service: GeriService

Geri exposes one gRPC service with two RPCs: LLM prompt processing (Wolf) and vision analysis.

---

## Wolf Service (LLM) – ProcessPrompt

Text generation from a prompt, optionally with RAG context.

### RPC: ProcessPrompt

| Direction | Message |
|-----------|---------|
| Request  | `ProcessPromptRequest` |
| Response | `ProcessPromptResponse` |

### ProcessPromptRequest

| Field       | Type   | Description |
|------------|--------|-------------|
| `prompt`   | string | User prompt (required). |
| `context`  | string | Optional RAG context. |
| `model_name` | string | Optional: specific model. |
| `max_tokens` | uint32 | Max tokens to generate; 0 = use default. |

### ProcessPromptResponse

| Field        | Type   | Description |
|-------------|--------|-------------|
| `text`      | string | Generated text. |
| `tokens_used` | uint32 | Tokens used for the response. |
| `model_used`  | string | Model that was used. |

### Errors

- `INTERNAL`: LLM processing failed (e.g. provider error, model not available).

---

## Vision Service – ProcessVision

Image analysis (description and optional JSON analysis).

### RPC: ProcessVision

| Direction | Message |
|-----------|---------|
| Request  | `ProcessVisionRequest` |
| Response | `ProcessVisionResponse` |

### ProcessVisionRequest

| Field       | Type  | Description |
|------------|-------|-------------|
| `image_data` | bytes | Raw image bytes. |
| `prompt`     | string | Optional question about the image. |
| `model_name` | string | Optional: specific vision model. |

### ProcessVisionResponse

| Field          | Type   | Description |
|----------------|--------|-------------|
| `description`  | string | Text description of the image. |
| `analysis_data` | bytes | JSON-encoded analysis (e.g. structured metadata). |
| `model_used`   | string | Vision model that was used. |

### Errors

- `INTERNAL`: Vision processing failed or analysis serialization failed.

---

## Usage

- Default port: see `config/geri.json` (`grpc_port`).
- Clients: use the generated code from `proto/geri.proto` (e.g. tonic) and connect to the configured address.

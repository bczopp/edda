-- Model registry for Skuld (LLM Selection Service)
CREATE TABLE IF NOT EXISTS model_registry (
    model_name VARCHAR(255) PRIMARY KEY,
    model_type VARCHAR(100),
    provider VARCHAR(100),
    is_active BOOLEAN NOT NULL DEFAULT true,
    registered_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_model_registry_is_active ON model_registry(is_active);
CREATE INDEX IF NOT EXISTS idx_model_registry_provider ON model_registry(provider);

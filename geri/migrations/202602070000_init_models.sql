-- Create models table
CREATE TABLE IF NOT EXISTS models (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    provider TEXT NOT NULL,
    model_type TEXT NOT NULL,
    parameter_count BIGINT,
    max_context_tokens INTEGER NOT NULL,
    is_local BOOLEAN NOT NULL DEFAULT FALSE,
    cost_per_token_input DOUBLE PRECISION DEFAULT 0.0,
    cost_per_token_output DOUBLE PRECISION DEFAULT 0.0,
    metadata JSONB DEFAULT '{}',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Index for filtering
CREATE INDEX IF NOT EXISTS idx_models_provider ON models(provider);
CREATE INDEX IF NOT EXISTS idx_models_type ON models(model_type);

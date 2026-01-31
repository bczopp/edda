-- Initial database schema for Nornen
-- Urd: Provider Registry

CREATE TABLE IF NOT EXISTS providers (
    provider_id VARCHAR(255) PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    endpoint VARCHAR(500) NOT NULL,
    status VARCHAR(50) NOT NULL DEFAULT 'active',
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    metadata JSONB
);

CREATE TABLE IF NOT EXISTS provider_capabilities (
    id SERIAL PRIMARY KEY,
    provider_id VARCHAR(255) NOT NULL REFERENCES providers(provider_id) ON DELETE CASCADE,
    capability VARCHAR(255) NOT NULL,
    UNIQUE(provider_id, capability)
);

CREATE INDEX idx_providers_status ON providers(status);
CREATE INDEX idx_providers_created_at ON providers(created_at);
CREATE INDEX idx_provider_capabilities_provider_id ON provider_capabilities(provider_id);
CREATE INDEX idx_provider_capabilities_capability ON provider_capabilities(capability);

-- Audit log table
CREATE TABLE IF NOT EXISTS audit_logs (
    id SERIAL PRIMARY KEY,
    event_type VARCHAR(100) NOT NULL,
    provider_id VARCHAR(255),
    event_timestamp TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    details JSONB
);

CREATE INDEX idx_audit_logs_provider_id ON audit_logs(provider_id);
CREATE INDEX idx_audit_logs_event_type ON audit_logs(event_type);
CREATE INDEX idx_audit_logs_timestamp ON audit_logs(event_timestamp);

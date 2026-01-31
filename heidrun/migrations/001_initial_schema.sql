-- Initial database schema for Heidrun
-- Token counting, pricing, settlement, and pre-authorization

CREATE TABLE IF NOT EXISTS token_counts (
    id SERIAL PRIMARY KEY,
    request_id VARCHAR(255) UNIQUE NOT NULL,
    text_hash VARCHAR(64) NOT NULL,
    token_count BIGINT NOT NULL,
    model VARCHAR(100),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS pricing_records (
    id SERIAL PRIMARY KEY,
    request_id VARCHAR(255) UNIQUE NOT NULL,
    token_count BIGINT NOT NULL,
    model VARCHAR(100) NOT NULL,
    provider_id VARCHAR(255) NOT NULL,
    base_price DECIMAL(20, 8) NOT NULL,
    commission DECIMAL(20, 8) NOT NULL,
    net_price DECIMAL(20, 8) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS settlements (
    settlement_id VARCHAR(255) PRIMARY KEY,
    provider_id VARCHAR(255) NOT NULL,
    amount DECIMAL(20, 2) NOT NULL,
    status VARCHAR(50) NOT NULL DEFAULT 'pending',
    period_start TIMESTAMP WITH TIME ZONE NOT NULL,
    period_end TIMESTAMP WITH TIME ZONE NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    executed_at TIMESTAMP WITH TIME ZONE
);

CREATE TABLE IF NOT EXISTS pre_authorizations (
    authorization_id VARCHAR(255) PRIMARY KEY,
    user_id VARCHAR(255) NOT NULL,
    amount DECIMAL(20, 2) NOT NULL,
    currency VARCHAR(10) NOT NULL DEFAULT 'USD',
    status VARCHAR(50) NOT NULL DEFAULT 'pending',
    expires_at TIMESTAMP WITH TIME ZONE NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_token_counts_text_hash ON token_counts(text_hash);
CREATE INDEX idx_token_counts_model ON token_counts(model);
CREATE INDEX idx_pricing_records_provider_id ON pricing_records(provider_id);
CREATE INDEX idx_pricing_records_model ON pricing_records(model);
CREATE INDEX idx_settlements_provider_id ON settlements(provider_id);
CREATE INDEX idx_settlements_status ON settlements(status);
CREATE INDEX idx_pre_authorizations_user_id ON pre_authorizations(user_id);
CREATE INDEX idx_pre_authorizations_status ON pre_authorizations(status);
CREATE INDEX idx_pre_authorizations_expires_at ON pre_authorizations(expires_at);

-- Audit log table
CREATE TABLE IF NOT EXISTS audit_logs (
    id SERIAL PRIMARY KEY,
    event_type VARCHAR(100) NOT NULL,
    provider_id VARCHAR(255),
    user_id VARCHAR(255),
    settlement_id VARCHAR(255),
    authorization_id VARCHAR(255),
    event_timestamp TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    details JSONB
);

CREATE INDEX idx_audit_logs_provider_id ON audit_logs(provider_id);
CREATE INDEX idx_audit_logs_user_id ON audit_logs(user_id);
CREATE INDEX idx_audit_logs_event_type ON audit_logs(event_type);
CREATE INDEX idx_audit_logs_timestamp ON audit_logs(event_timestamp);

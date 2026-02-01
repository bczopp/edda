-- Initial database schema for Mimir

CREATE TABLE IF NOT EXISTS encrypted_data (
    id VARCHAR(255) PRIMARY KEY,
    user_id VARCHAR(255) NOT NULL,
    encrypted_data BYTEA NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_encrypted_data_user_id ON encrypted_data(user_id);
CREATE INDEX idx_encrypted_data_created_at ON encrypted_data(created_at);
-- Composite index for common query pattern: user_id + created_at (for user data queries with ordering)
CREATE INDEX idx_encrypted_data_user_created ON encrypted_data(user_id, created_at);

-- Audit log table for GDPR compliance
CREATE TABLE IF NOT EXISTS audit_logs (
    id SERIAL PRIMARY KEY,
    event_type VARCHAR(100) NOT NULL,
    user_id VARCHAR(255),
    data_id VARCHAR(255),
    event_timestamp TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    details JSONB
);

CREATE INDEX idx_audit_logs_user_id ON audit_logs(user_id);
CREATE INDEX idx_audit_logs_event_type ON audit_logs(event_type);
CREATE INDEX idx_audit_logs_timestamp ON audit_logs(event_timestamp);

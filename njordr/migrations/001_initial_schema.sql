-- Initial database schema for Njörðr
-- Provider Earnings, Settlements, and Trade Management

CREATE TABLE IF NOT EXISTS provider_earnings (
    id SERIAL PRIMARY KEY,
    provider_id VARCHAR(255) NOT NULL,
    period_start TIMESTAMP WITH TIME ZONE NOT NULL,
    period_end TIMESTAMP WITH TIME ZONE NOT NULL,
    total_amount DECIMAL(20, 2) NOT NULL,
    commission DECIMAL(20, 2) NOT NULL,
    net_earnings DECIMAL(20, 2) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS settlements (
    settlement_id VARCHAR(255) PRIMARY KEY,
    provider_id VARCHAR(255) NOT NULL,
    amount DECIMAL(20, 2) NOT NULL,
    status VARCHAR(50) NOT NULL DEFAULT 'pending',
    payment_method VARCHAR(50),
    transaction_id VARCHAR(255),
    period_start TIMESTAMP WITH TIME ZONE NOT NULL,
    period_end TIMESTAMP WITH TIME ZONE NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    executed_at TIMESTAMP WITH TIME ZONE
);

CREATE TABLE IF NOT EXISTS trades (
    trade_id VARCHAR(255) PRIMARY KEY,
    provider_id VARCHAR(255) NOT NULL,
    amount DECIMAL(20, 2) NOT NULL,
    status VARCHAR(50) NOT NULL DEFAULT 'completed',
    timestamp TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    metadata JSONB
);

CREATE INDEX idx_provider_earnings_provider_id ON provider_earnings(provider_id);
CREATE INDEX idx_provider_earnings_period ON provider_earnings(period_start, period_end);
CREATE INDEX idx_settlements_provider_id ON settlements(provider_id);
CREATE INDEX idx_settlements_status ON settlements(status);
CREATE INDEX idx_settlements_period ON settlements(period_start, period_end);
CREATE INDEX idx_trades_provider_id ON trades(provider_id);
CREATE INDEX idx_trades_timestamp ON trades(timestamp);
CREATE INDEX idx_trades_status ON trades(status);-- Audit log table
CREATE TABLE IF NOT EXISTS audit_logs (
    id SERIAL PRIMARY KEY,
    event_type VARCHAR(100) NOT NULL,
    provider_id VARCHAR(255),
    settlement_id VARCHAR(255),
    trade_id VARCHAR(255),
    event_timestamp TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    details JSONB
);CREATE INDEX idx_audit_logs_provider_id ON audit_logs(provider_id);
CREATE INDEX idx_audit_logs_event_type ON audit_logs(event_type);
CREATE INDEX idx_audit_logs_timestamp ON audit_logs(event_timestamp);
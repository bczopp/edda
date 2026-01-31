-- Initial database schema for Eikthyrnir
-- Quality Assessment, Aggregation, and Metrics

CREATE TABLE IF NOT EXISTS quality_assessments (
    id SERIAL PRIMARY KEY,
    assessment_id VARCHAR(255) UNIQUE NOT NULL,
    provider_id VARCHAR(255) NOT NULL,
    service_id VARCHAR(255) NOT NULL,
    quality_score DECIMAL(5, 4) NOT NULL,
    response_time DECIMAL(10, 3),
    accuracy DECIMAL(5, 4),
    availability DECIMAL(5, 4),
    custom_metrics JSONB,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS quality_aggregations (
    id SERIAL PRIMARY KEY,
    aggregation_id VARCHAR(255) UNIQUE NOT NULL,
    provider_id VARCHAR(255) NOT NULL,
    period_start TIMESTAMP WITH TIME ZONE NOT NULL,
    period_end TIMESTAMP WITH TIME ZONE NOT NULL,
    aggregated_score DECIMAL(5, 4) NOT NULL,
    aggregated_metrics JSONB,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS quality_metrics (
    id SERIAL PRIMARY KEY,
    metric_id VARCHAR(255) UNIQUE NOT NULL,
    provider_id VARCHAR(255) NOT NULL,
    metric_name VARCHAR(255) NOT NULL,
    value DECIMAL(20, 8) NOT NULL,
    timestamp TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_quality_assessments_provider_id ON quality_assessments(provider_id);
CREATE INDEX idx_quality_assessments_service_id ON quality_assessments(service_id);
CREATE INDEX idx_quality_assessments_created_at ON quality_assessments(created_at);
CREATE INDEX idx_quality_aggregations_provider_id ON quality_aggregations(provider_id);
CREATE INDEX idx_quality_aggregations_period ON quality_aggregations(period_start, period_end);
CREATE INDEX idx_quality_metrics_provider_id ON quality_metrics(provider_id);
CREATE INDEX idx_quality_metrics_metric_name ON quality_metrics(metric_name);
CREATE INDEX idx_quality_metrics_timestamp ON quality_metrics(timestamp);

-- Audit log table
CREATE TABLE IF NOT EXISTS audit_logs (
    id SERIAL PRIMARY KEY,
    event_type VARCHAR(100) NOT NULL,
    provider_id VARCHAR(255),
    service_id VARCHAR(255),
    assessment_id VARCHAR(255),
    aggregation_id VARCHAR(255),
    event_timestamp TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    details JSONB
);

CREATE INDEX idx_audit_logs_provider_id ON audit_logs(provider_id);
CREATE INDEX idx_audit_logs_event_type ON audit_logs(event_type);
CREATE INDEX idx_audit_logs_timestamp ON audit_logs(event_timestamp);

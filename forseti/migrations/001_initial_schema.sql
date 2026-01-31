-- Initial database schema for Forseti
-- Model Registry for ML/DL/RL models

CREATE TABLE IF NOT EXISTS models (
    id VARCHAR(255) PRIMARY KEY,
    model_type VARCHAR(100) NOT NULL,
    framework VARCHAR(50) NOT NULL,
    status VARCHAR(50) NOT NULL DEFAULT 'registered',
    model_path VARCHAR(500),
    metadata JSONB,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS training_jobs (
    id VARCHAR(255) PRIMARY KEY,
    model_id VARCHAR(255) REFERENCES models(id),
    job_type VARCHAR(50) NOT NULL,
    status VARCHAR(50) NOT NULL DEFAULT 'pending',
    training_data_path VARCHAR(500),
    hyperparameters JSONB,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    completed_at TIMESTAMP WITH TIME ZONE
);

CREATE TABLE IF NOT EXISTS inference_requests (
    id VARCHAR(255) PRIMARY KEY,
    model_id VARCHAR(255) REFERENCES models(id),
    input_data BYTEA,
    output_data BYTEA,
    status VARCHAR(50) NOT NULL DEFAULT 'pending',
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    completed_at TIMESTAMP WITH TIME ZONE
);

CREATE INDEX idx_models_model_type ON models(model_type);
CREATE INDEX idx_models_framework ON models(framework);
CREATE INDEX idx_models_status ON models(status);
CREATE INDEX idx_training_jobs_model_id ON training_jobs(model_id);
CREATE INDEX idx_training_jobs_status ON training_jobs(status);
CREATE INDEX idx_inference_requests_model_id ON inference_requests(model_id);
CREATE INDEX idx_inference_requests_status ON inference_requests(status);-- Audit log table
CREATE TABLE IF NOT EXISTS audit_logs (
    id SERIAL PRIMARY KEY,
    event_type VARCHAR(100) NOT NULL,
    model_id VARCHAR(255),
    training_job_id VARCHAR(255),
    inference_request_id VARCHAR(255),
    event_timestamp TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    details JSONB
);CREATE INDEX idx_audit_logs_model_id ON audit_logs(model_id);
CREATE INDEX idx_audit_logs_event_type ON audit_logs(event_type);
CREATE INDEX idx_audit_logs_timestamp ON audit_logs(event_timestamp);
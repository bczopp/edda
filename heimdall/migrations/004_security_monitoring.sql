-- Security Monitoring Migration

-- Blocked devices table
CREATE TABLE IF NOT EXISTS blocked_devices (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    device_id UUID NOT NULL REFERENCES devices(id) ON DELETE CASCADE,
    user_id UUID NOT NULL,
    reason VARCHAR(255) NOT NULL,
    blocked_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    expires_at TIMESTAMP WITH TIME ZONE,
    is_permanent BOOLEAN DEFAULT false,
    UNIQUE(device_id)
);

CREATE INDEX idx_blocked_devices_device_id ON blocked_devices(device_id);
CREATE INDEX idx_blocked_devices_user_id ON blocked_devices(user_id);
CREATE INDEX idx_blocked_devices_expires_at ON blocked_devices(expires_at);

-- Security metrics table (for analytics)
CREATE TABLE IF NOT EXISTS security_metrics (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    device_id UUID REFERENCES devices(id) ON DELETE CASCADE,
    user_id UUID,
    metric_type VARCHAR(100) NOT NULL,
    metric_value NUMERIC NOT NULL,
    period_start TIMESTAMP WITH TIME ZONE NOT NULL,
    period_end TIMESTAMP WITH TIME ZONE NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_security_metrics_device_id ON security_metrics(device_id);
CREATE INDEX idx_security_metrics_user_id ON security_metrics(user_id);
CREATE INDEX idx_security_metrics_type ON security_metrics(metric_type);
CREATE INDEX idx_security_metrics_period ON security_metrics(period_start, period_end);

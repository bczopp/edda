-- Initial database schema for Heimdall

-- Devices table
CREATE TABLE IF NOT EXISTS devices (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    device_id VARCHAR(255) UNIQUE NOT NULL,
    user_id UUID NOT NULL,
    public_key TEXT NOT NULL,
    device_name VARCHAR(255),
    device_type VARCHAR(50),
    is_active BOOLEAN DEFAULT true,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_devices_device_id ON devices(device_id);
CREATE INDEX idx_devices_user_id ON devices(user_id);

-- Tokens table
CREATE TABLE IF NOT EXISTS tokens (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    token_id VARCHAR(255) UNIQUE NOT NULL,
    device_id UUID NOT NULL REFERENCES devices(id) ON DELETE CASCADE,
    user_id UUID NOT NULL,
    token_type VARCHAR(50) NOT NULL, -- 'heimdall', 'session', 'refresh'
    token_data TEXT NOT NULL,
    expires_at TIMESTAMP WITH TIME ZONE NOT NULL,
    is_revoked BOOLEAN DEFAULT false,
    revoked_at TIMESTAMP WITH TIME ZONE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_tokens_token_id ON tokens(token_id);
CREATE INDEX idx_tokens_device_id ON tokens(device_id);
CREATE INDEX idx_tokens_user_id ON tokens(user_id);
CREATE INDEX idx_tokens_expires_at ON tokens(expires_at);
CREATE INDEX idx_tokens_is_revoked ON tokens(is_revoked);

-- Sessions table
CREATE TABLE IF NOT EXISTS sessions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    session_id VARCHAR(255) UNIQUE NOT NULL,
    device_id UUID NOT NULL REFERENCES devices(id) ON DELETE CASCADE,
    user_id UUID NOT NULL,
    token_id UUID REFERENCES tokens(id) ON DELETE CASCADE,
    last_activity TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    expires_at TIMESTAMP WITH TIME ZONE NOT NULL,
    is_active BOOLEAN DEFAULT true,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_sessions_session_id ON sessions(session_id);
CREATE INDEX idx_sessions_device_id ON sessions(device_id);
CREATE INDEX idx_sessions_user_id ON sessions(user_id);
CREATE INDEX idx_sessions_expires_at ON sessions(expires_at);

-- Permissions table
CREATE TABLE IF NOT EXISTS permissions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    permission_name VARCHAR(255) UNIQUE NOT NULL,
    description TEXT,
    resource_type VARCHAR(100),
    action VARCHAR(100),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_permissions_permission_name ON permissions(permission_name);
CREATE INDEX idx_permissions_resource_type ON permissions(resource_type);

-- Device permissions (many-to-many)
CREATE TABLE IF NOT EXISTS device_permissions (
    device_id UUID NOT NULL REFERENCES devices(id) ON DELETE CASCADE,
    permission_id UUID NOT NULL REFERENCES permissions(id) ON DELETE CASCADE,
    granted_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (device_id, permission_id)
);

CREATE INDEX idx_device_permissions_device_id ON device_permissions(device_id);
CREATE INDEX idx_device_permissions_permission_id ON device_permissions(permission_id);

-- Roles table
CREATE TABLE IF NOT EXISTS roles (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    role_name VARCHAR(255) UNIQUE NOT NULL,
    description TEXT,
    parent_role_id UUID REFERENCES roles(id),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_roles_role_name ON roles(role_name);

-- Device roles (many-to-many)
CREATE TABLE IF NOT EXISTS device_roles (
    device_id UUID NOT NULL REFERENCES devices(id) ON DELETE CASCADE,
    role_id UUID NOT NULL REFERENCES roles(id) ON DELETE CASCADE,
    granted_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (device_id, role_id)
);

CREATE INDEX idx_device_roles_device_id ON device_roles(device_id);
CREATE INDEX idx_device_roles_role_id ON device_roles(role_id);

-- Role permissions (many-to-many)
CREATE TABLE IF NOT EXISTS role_permissions (
    role_id UUID NOT NULL REFERENCES roles(id) ON DELETE CASCADE,
    permission_id UUID NOT NULL REFERENCES permissions(id) ON DELETE CASCADE,
    granted_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (role_id, permission_id)
);

CREATE INDEX idx_role_permissions_role_id ON role_permissions(role_id);
CREATE INDEX idx_role_permissions_permission_id ON role_permissions(permission_id);

-- Mesh devices table (Bifrost nutzt Device-Mesh für Mesh-Membership)
CREATE TABLE IF NOT EXISTS mesh_devices (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    device_id UUID NOT NULL REFERENCES devices(id) ON DELETE CASCADE,
    mesh_public_key TEXT NOT NULL, -- Public key für Mesh-Membership
    role VARCHAR(50) NOT NULL DEFAULT 'user', -- 'admin', 'user', 'guest'
    is_active BOOLEAN DEFAULT true,
    registered_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    last_seen TIMESTAMP WITH TIME ZONE,
    owner_user_id UUID NOT NULL
);

CREATE INDEX idx_mesh_devices_device_id ON mesh_devices(device_id);
CREATE INDEX idx_mesh_devices_owner_user_id ON mesh_devices(owner_user_id);

-- Audit log table
CREATE TABLE IF NOT EXISTS audit_logs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    event_type VARCHAR(100) NOT NULL,
    device_id UUID REFERENCES devices(id),
    user_id UUID,
    resource_type VARCHAR(100),
    action VARCHAR(100),
    status VARCHAR(50), -- 'success', 'failure', 'denied'
    ip_address INET,
    user_agent TEXT,
    details JSONB,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_audit_logs_event_type ON audit_logs(event_type);
CREATE INDEX idx_audit_logs_device_id ON audit_logs(device_id);
CREATE INDEX idx_audit_logs_user_id ON audit_logs(user_id);
CREATE INDEX idx_audit_logs_created_at ON audit_logs(created_at);

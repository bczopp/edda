-- Guest Network Permissions Migration

-- Data transfer permissions table
CREATE TABLE IF NOT EXISTS data_transfer_permissions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    source_device_id UUID NOT NULL REFERENCES devices(id) ON DELETE CASCADE,
    target_device_id UUID NOT NULL REFERENCES devices(id) ON DELETE CASCADE,
    session_id UUID,
    granted_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    expires_at TIMESTAMP WITH TIME ZONE NOT NULL,
    UNIQUE(source_device_id, target_device_id, session_id)
);

CREATE INDEX idx_data_transfer_permissions_source ON data_transfer_permissions(source_device_id);
CREATE INDEX idx_data_transfer_permissions_target ON data_transfer_permissions(target_device_id);
CREATE INDEX idx_data_transfer_permissions_expires ON data_transfer_permissions(expires_at);

-- Explicit access requests (for guest to main network access)
CREATE TABLE IF NOT EXISTS explicit_access_requests (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    guest_device_id UUID NOT NULL REFERENCES devices(id) ON DELETE CASCADE,
    main_device_id UUID NOT NULL REFERENCES devices(id) ON DELETE CASCADE,
    confirmation_count INTEGER DEFAULT 0,
    requested_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    last_confirmed_at TIMESTAMP WITH TIME ZONE,
    UNIQUE(guest_device_id, main_device_id)
);

CREATE INDEX idx_explicit_access_requests_guest ON explicit_access_requests(guest_device_id);
CREATE INDEX idx_explicit_access_requests_main ON explicit_access_requests(main_device_id);

-- Explicit access grants (granted after 3 confirmations)
CREATE TABLE IF NOT EXISTS explicit_access_grants (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    guest_device_id UUID NOT NULL REFERENCES devices(id) ON DELETE CASCADE,
    main_device_id UUID NOT NULL REFERENCES devices(id) ON DELETE CASCADE,
    granted_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    expires_at TIMESTAMP WITH TIME ZONE NOT NULL,
    UNIQUE(guest_device_id, main_device_id)
);

CREATE INDEX idx_explicit_access_grants_guest ON explicit_access_grants(guest_device_id);
CREATE INDEX idx_explicit_access_grants_main ON explicit_access_grants(main_device_id);
CREATE INDEX idx_explicit_access_grants_expires ON explicit_access_grants(expires_at);

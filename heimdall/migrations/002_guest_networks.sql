-- Guest Networks Migration

-- Guest networks table
CREATE TABLE IF NOT EXISTS guest_networks (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    network_id VARCHAR(255) UNIQUE NOT NULL,
    owner_user_id UUID NOT NULL,
    name VARCHAR(255),
    is_active BOOLEAN DEFAULT true,
    expires_at TIMESTAMP WITH TIME ZONE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_guest_networks_network_id ON guest_networks(network_id);
CREATE INDEX idx_guest_networks_owner_user_id ON guest_networks(owner_user_id);

-- Guest network devices (many-to-many)
CREATE TABLE IF NOT EXISTS guest_network_devices (
    network_id UUID NOT NULL REFERENCES guest_networks(id) ON DELETE CASCADE,
    device_id UUID NOT NULL REFERENCES devices(id) ON DELETE CASCADE,
    joined_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (network_id, device_id)
);

CREATE INDEX idx_guest_network_devices_network_id ON guest_network_devices(network_id);
CREATE INDEX idx_guest_network_devices_device_id ON guest_network_devices(device_id);

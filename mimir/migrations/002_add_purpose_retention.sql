-- Add purpose and retention tracking for GDPR Data-Protection compliance

-- Add purpose column to encrypted_data table
ALTER TABLE encrypted_data ADD COLUMN IF NOT EXISTS purpose VARCHAR(255);
ALTER TABLE encrypted_data ADD COLUMN IF NOT EXISTS expires_at TIMESTAMP WITH TIME ZONE;

-- Create index for expiration queries (for automatic cleanup)
CREATE INDEX IF NOT EXISTS idx_encrypted_data_expires_at ON encrypted_data(expires_at) WHERE expires_at IS NOT NULL;

-- Create index for purpose queries
CREATE INDEX IF NOT EXISTS idx_encrypted_data_purpose ON encrypted_data(purpose) WHERE purpose IS NOT NULL;

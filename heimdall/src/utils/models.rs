use chrono::{DateTime, Utc};
use sqlx::FromRow;
use uuid::Uuid;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Device {
    pub id: Uuid,
    pub device_id: String,
    pub user_id: Uuid,
    pub public_key: String,
    pub device_name: Option<String>,
    pub device_type: Option<String>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Token {
    pub id: Uuid,
    pub token_id: String,
    pub device_id: Uuid,
    pub user_id: Uuid,
    pub token_type: String, // 'heimdall', 'session', 'refresh'
    pub token_data: String,
    pub expires_at: DateTime<Utc>,
    pub is_revoked: bool,
    pub revoked_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Session {
    pub id: Uuid,
    pub session_id: String,
    pub device_id: Uuid,
    pub user_id: Uuid,
    pub token_id: Option<Uuid>,
    pub last_activity: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Permission {
    pub id: Uuid,
    pub permission_name: String,
    pub description: Option<String>,
    pub resource_type: Option<String>,
    pub action: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Role {
    pub id: Uuid,
    pub role_name: String,
    pub description: Option<String>,
    pub parent_role_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct MeshDevice {
    pub id: Uuid,
    pub device_id: Uuid,
    pub mesh_public_key: String, // Public key für Mesh-Membership
    pub role: String, // 'admin', 'user', 'guest'
    pub is_active: bool,
    pub registered_at: DateTime<Utc>,
    pub last_seen: Option<DateTime<Utc>>,
    pub owner_user_id: Uuid,
}


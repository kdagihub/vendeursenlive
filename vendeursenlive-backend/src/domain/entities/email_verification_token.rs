use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EmailVerificationToken {
    pub id: Uuid,
    pub user_auth_identity_id: Uuid,
    pub token_hash: String,
    pub used_at: Option<DateTime<Utc>>,
    pub expires_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

impl EmailVerificationToken {
    pub fn new(user_auth_identity_id: Uuid, token_hash: String, expires_at: DateTime<Utc>) -> Self {
        Self {
            id: Uuid::now_v7(),
            user_auth_identity_id,
            token_hash,
            used_at: None,
            expires_at,
            created_at: Utc::now(),
        }
    }
}

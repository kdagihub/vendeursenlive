use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthSession {
    pub id: Uuid,
    pub user_id: Uuid,
    pub refresh_token_hash: String,
    pub user_agent: Option<String>,
    pub ip_address: Option<String>,
    pub revoked_at: Option<DateTime<Utc>>,
    pub expires_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub last_used_at: DateTime<Utc>,
}

impl AuthSession {
    pub fn new(
        user_id: Uuid,
        refresh_token_hash: String,
        expires_at: DateTime<Utc>,
        user_agent: Option<String>,
        ip_address: Option<String>,
    ) -> Self {
        let now = Utc::now();

        Self {
            id: Uuid::now_v7(),
            user_id,
            refresh_token_hash,
            user_agent,
            ip_address,
            revoked_at: None,
            expires_at,
            created_at: now,
            last_used_at: now,
        }
    }

    pub fn revoke(&mut self) {
        self.revoked_at = Some(Utc::now());
    }
}

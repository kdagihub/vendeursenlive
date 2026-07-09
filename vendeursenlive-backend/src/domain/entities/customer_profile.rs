use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CustomerProfile {
    pub id: Uuid,
    pub user_id: Uuid,
    pub default_location: Option<String>,
    pub preferred_payment_method: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl CustomerProfile {
    pub fn new(user_id: Uuid, default_location: Option<String>) -> Self {
        let now = Utc::now();

        Self {
            id: Uuid::now_v7(),
            user_id,
            default_location,
            preferred_payment_method: None,
            created_at: now,
            updated_at: now,
        }
    }
}

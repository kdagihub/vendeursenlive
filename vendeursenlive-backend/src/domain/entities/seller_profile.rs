use chrono::{DateTime, Duration, Utc};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SellerProfile {
    pub id: Uuid,
    pub user_id: Uuid,
    pub shop_name: String,
    pub payment_link: Option<String>,
    pub trial_ends_at: DateTime<Utc>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
}

impl SellerProfile {
    pub fn start_trial(user_id: Uuid, shop_name: String, payment_link: Option<String>) -> Self {
        let now = Utc::now();

        Self {
            id: Uuid::now_v7(),
            user_id,
            shop_name,
            payment_link,
            trial_ends_at: now + Duration::days(14),
            is_active: true,
            created_at: now,
        }
    }
}

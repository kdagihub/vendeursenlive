use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::domain::value_objects::Price;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EphemeralProduct {
    pub id: Uuid,
    pub live_session_id: Uuid,
    pub image_url: String,
    pub price: Price,
    pub description: String,
    pub is_retained: bool,
    pub created_at: DateTime<Utc>,
}

impl EphemeralProduct {
    pub fn capture(
        live_session_id: Uuid,
        image_url: String,
        price: Price,
        description: String,
    ) -> Self {
        Self {
            id: Uuid::now_v7(),
            live_session_id,
            image_url,
            price,
            description,
            is_retained: false,
            created_at: Utc::now(),
        }
    }

    pub fn retain_post_live(&mut self) {
        self.is_retained = true;
    }
}

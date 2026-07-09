use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::domain::value_objects::TikTokLiveUrl;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LiveSessionStatus {
    Ongoing,
    Ended,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LiveSession {
    pub id: Uuid,
    pub seller_profile_id: Uuid,
    pub tiktok_url: TikTokLiveUrl,
    pub status: LiveSessionStatus,
    pub created_at: DateTime<Utc>,
    pub ended_at: Option<DateTime<Utc>>,
}

impl LiveSession {
    pub fn start(seller_profile_id: Uuid, tiktok_url: TikTokLiveUrl) -> Self {
        Self {
            id: Uuid::now_v7(),
            seller_profile_id,
            tiktok_url,
            status: LiveSessionStatus::Ongoing,
            created_at: Utc::now(),
            ended_at: None,
        }
    }

    pub fn end(&mut self) {
        self.status = LiveSessionStatus::Ended;
        self.ended_at = Some(Utc::now());
    }
}

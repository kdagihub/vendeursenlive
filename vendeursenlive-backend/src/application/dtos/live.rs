use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::entities::{LiveSession, LiveSessionStatus};

#[derive(Debug, Clone, Deserialize)]
pub struct StartLiveSessionRequest {
    pub tiktok_live_url: String,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct LiveSessionResponse {
    pub id: Uuid,
    pub tiktok_live_url: String,
    pub tiktok_username: String,
    pub status: &'static str,
    pub created_at: DateTime<Utc>,
    pub ended_at: Option<DateTime<Utc>>,
}

impl From<&LiveSession> for LiveSessionResponse {
    fn from(session: &LiveSession) -> Self {
        Self {
            id: session.id,
            tiktok_live_url: session.tiktok_url.as_str().to_owned(),
            tiktok_username: session.tiktok_url.username().to_owned(),
            status: match session.status {
                LiveSessionStatus::Ongoing => "ongoing",
                LiveSessionStatus::Ended => "ended",
            },
            created_at: session.created_at,
            ended_at: session.ended_at,
        }
    }
}

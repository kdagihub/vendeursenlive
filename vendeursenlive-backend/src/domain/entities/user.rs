use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UserStatus {
    Active,
    Disabled,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct User {
    pub id: Uuid,
    pub full_name: Option<String>,
    pub avatar_url: Option<String>,
    pub status: UserStatus,
    pub is_admin: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl User {
    pub fn new(full_name: Option<String>, avatar_url: Option<String>) -> Self {
        let now = Utc::now();

        Self {
            id: Uuid::now_v7(),
            full_name,
            avatar_url,
            status: UserStatus::Active,
            is_admin: false,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn disable(&mut self) {
        self.status = UserStatus::Disabled;
        self.updated_at = Utc::now();
    }
}

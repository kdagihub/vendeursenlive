use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::domain::value_objects::{EmailAddress, PhoneNumber};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuthProvider {
    Email,
    Phone,
    Google,
    TikTok,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserAuthIdentity {
    pub id: Uuid,
    pub user_id: Uuid,
    pub provider: AuthProvider,
    pub provider_subject: Option<String>,
    pub email: Option<EmailAddress>,
    pub phone_number: Option<PhoneNumber>,
    pub password_hash: Option<String>,
    pub email_verified: bool,
    pub phone_verified: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl UserAuthIdentity {
    pub fn email(user_id: Uuid, email: EmailAddress, password_hash: String) -> Self {
        Self::new(
            user_id,
            AuthProvider::Email,
            None,
            Some(email),
            None,
            Some(password_hash),
            false,
            false,
        )
    }

    pub fn phone(user_id: Uuid, phone_number: PhoneNumber, password_hash: String) -> Self {
        Self::new(
            user_id,
            AuthProvider::Phone,
            None,
            None,
            Some(phone_number),
            Some(password_hash),
            false,
            false,
        )
    }

    pub fn oauth(user_id: Uuid, provider: AuthProvider, provider_subject: String) -> Self {
        Self::new(
            user_id,
            provider,
            Some(provider_subject),
            None,
            None,
            None,
            false,
            false,
        )
    }

    #[allow(clippy::too_many_arguments)]
    fn new(
        user_id: Uuid,
        provider: AuthProvider,
        provider_subject: Option<String>,
        email: Option<EmailAddress>,
        phone_number: Option<PhoneNumber>,
        password_hash: Option<String>,
        email_verified: bool,
        phone_verified: bool,
    ) -> Self {
        let now = Utc::now();

        Self {
            id: Uuid::now_v7(),
            user_id,
            provider,
            provider_subject,
            email,
            phone_number,
            password_hash,
            email_verified,
            phone_verified,
            created_at: now,
            updated_at: now,
        }
    }
}

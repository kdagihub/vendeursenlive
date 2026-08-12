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
    pub fn is_account_verified(&self) -> bool {
        match self.provider {
            AuthProvider::Google | AuthProvider::TikTok => true,
            AuthProvider::Email => self.email_verified,
            AuthProvider::Phone => self.phone_verified,
        }
    }

    pub fn verification_channel(&self) -> Option<&'static str> {
        match self.provider {
            AuthProvider::Email if !self.email_verified => Some("email"),
            AuthProvider::Phone if !self.phone_verified => Some("phone"),
            _ => None,
        }
    }

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

    pub fn verified_phone(user_id: Uuid, phone_number: PhoneNumber) -> Self {
        Self::new(
            user_id,
            AuthProvider::Phone,
            None,
            None,
            Some(phone_number),
            None,
            false,
            true,
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

    pub fn google(user_id: Uuid, provider_subject: String, email: Option<EmailAddress>) -> Self {
        Self::new(
            user_id,
            AuthProvider::Google,
            Some(provider_subject),
            email,
            None,
            None,
            true,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tiktok_identity_is_verified_without_email() {
        let identity =
            UserAuthIdentity::oauth(Uuid::now_v7(), AuthProvider::TikTok, "open-id".to_owned());

        assert!(identity.is_account_verified());
        assert_eq!(identity.verification_channel(), None);
    }

    #[test]
    fn email_identity_requires_verification() {
        let identity = UserAuthIdentity::email(
            Uuid::now_v7(),
            EmailAddress::new("client@example.com").expect("valid email"),
            "password-hash".to_owned(),
        );

        assert!(!identity.is_account_verified());
        assert_eq!(identity.verification_channel(), Some("email"));
    }

    #[test]
    fn verified_email_identity_unlocks_account() {
        let mut identity = UserAuthIdentity::email(
            Uuid::now_v7(),
            EmailAddress::new("seller@example.com").expect("valid email"),
            "password-hash".to_owned(),
        );
        identity.email_verified = true;

        assert!(identity.is_account_verified());
        assert_eq!(identity.verification_channel(), None);
    }

    #[test]
    fn otp_phone_identity_is_verified_without_a_password() {
        let identity = UserAuthIdentity::verified_phone(
            Uuid::now_v7(),
            PhoneNumber::new("+2250700000000").expect("valid phone"),
        );

        assert!(identity.is_account_verified());
        assert!(identity.password_hash.is_none());
        assert_eq!(identity.verification_channel(), None);
    }

    #[test]
    fn google_identity_uses_a_verified_email_without_a_password() {
        let identity = UserAuthIdentity::google(
            Uuid::now_v7(),
            "google-subject".to_owned(),
            Some(EmailAddress::new("client@example.com").expect("valid email")),
        );

        assert!(identity.is_account_verified());
        assert!(identity.email_verified);
        assert!(identity.password_hash.is_none());
    }
}

use async_trait::async_trait;
use uuid::Uuid;

use crate::application::errors::ApplicationError;

pub trait PasswordHasher: Send + Sync {
    fn hash_password(&self, password: &str) -> Result<String, ApplicationError>;
    fn verify_password(
        &self,
        password: &str,
        password_hash: &str,
    ) -> Result<bool, ApplicationError>;
}

pub trait RefreshTokenService: Send + Sync {
    fn generate(&self) -> String;
    fn hash(&self, refresh_token: &str) -> Result<String, ApplicationError>;
    fn verify(
        &self,
        refresh_token: &str,
        refresh_token_hash: &str,
    ) -> Result<bool, ApplicationError>;
}

pub trait AccessTokenIssuer: Send + Sync {
    fn issue_access_token(
        &self,
        user_id: Uuid,
        session_id: Uuid,
        is_admin: bool,
        is_seller: bool,
        account_verified: bool,
        verification_channel: Option<&str>,
    ) -> Result<String, ApplicationError>;
}

#[async_trait]
pub trait AuthEmailSender: Send + Sync {
    async fn send_password_reset(
        &self,
        to_email: &str,
        reset_url: &str,
    ) -> Result<(), ApplicationError>;
    async fn send_email_verification(
        &self,
        to_email: &str,
        verification_url: &str,
    ) -> Result<(), ApplicationError>;
}

#[derive(Debug, Clone)]
pub struct TikTokTokenResponse {
    pub open_id: String,
    pub access_token: String,
    pub scope: String,
}

#[derive(Debug, Clone)]
pub struct TikTokUserProfile {
    pub open_id: String,
    pub union_id: Option<String>,
    pub display_name: Option<String>,
    pub avatar_url: Option<String>,
}

#[async_trait]
pub trait TikTokOAuthClient: Send + Sync {
    async fn exchange_code(&self, code: &str) -> Result<TikTokTokenResponse, ApplicationError>;
    async fn fetch_user_profile(
        &self,
        access_token: &str,
    ) -> Result<TikTokUserProfile, ApplicationError>;
}

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::application::{
    dtos::auth::{AccountType, PhoneOtpPurpose},
    errors::ApplicationError,
};

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

#[derive(Debug, Clone)]
pub struct GoogleTokenResponse {
    pub access_token: String,
    pub scope: String,
}

#[derive(Debug, Clone)]
pub struct GoogleUserProfile {
    pub subject: String,
    pub email: String,
    pub email_verified: bool,
    pub display_name: Option<String>,
    pub avatar_url: Option<String>,
}

#[async_trait]
pub trait GoogleOAuthClient: Send + Sync {
    async fn exchange_code(&self, code: &str) -> Result<GoogleTokenResponse, ApplicationError>;
    async fn fetch_user_profile(
        &self,
        access_token: &str,
    ) -> Result<GoogleUserProfile, ApplicationError>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhoneOtpChallenge {
    pub id: Uuid,
    pub phone_number: String,
    pub provider_token: String,
    pub purpose: PhoneOtpPurpose,
    pub full_name: Option<String>,
    pub account_type: Option<AccountType>,
    pub shop_name: Option<String>,
}

#[async_trait]
pub trait PhoneOtpProvider: Send + Sync {
    async fn request_otp(&self, identity: &str) -> Result<String, ApplicationError>;
    async fn verify_otp(
        &self,
        identity: &str,
        otp: &str,
        provider_token: &str,
    ) -> Result<bool, ApplicationError>;
}

#[async_trait]
pub trait PhoneOtpChallengeStore: Send + Sync {
    async fn reserve_send(
        &self,
        phone_number: &str,
        ttl_seconds: i64,
    ) -> Result<bool, ApplicationError>;
    async fn release_send(&self, phone_number: &str) -> Result<(), ApplicationError>;
    async fn save(
        &self,
        challenge: &PhoneOtpChallenge,
        ttl_seconds: i64,
    ) -> Result<(), ApplicationError>;
    async fn find(&self, challenge_id: Uuid)
        -> Result<Option<PhoneOtpChallenge>, ApplicationError>;
    async fn record_failed_attempt(
        &self,
        challenge_id: Uuid,
        ttl_seconds: i64,
    ) -> Result<u32, ApplicationError>;
    async fn delete(&self, challenge_id: Uuid) -> Result<(), ApplicationError>;
}

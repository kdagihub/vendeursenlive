use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AccountType {
    Customer,
    Seller,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RegisterRequest {
    pub full_name: Option<String>,
    pub email: Option<String>,
    pub phone_number: Option<String>,
    pub password: String,
    pub account_type: AccountType,
    pub default_location: Option<String>,
    pub shop_name: Option<String>,
    pub payment_link: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct LoginRequest {
    pub identifier: String,
    pub password: String,
}

#[derive(Debug, Clone)]
pub struct RefreshSessionRequest {
    pub session_id: Uuid,
    pub refresh_token: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ChangePasswordRequest {
    pub current_password: String,
    pub new_password: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PasswordResetRequest {
    pub email: String,
}

#[derive(Debug, Clone)]
pub struct RequestPasswordResetCommand {
    pub email: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ConfirmPasswordResetRequest {
    pub reset_token: String,
    pub new_password: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ConfirmEmailVerificationRequest {
    pub verification_token: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct AuthResponse {
    pub user_id: Uuid,
    pub session_id: Uuid,
    pub access_token: String,
    pub refresh_token: String,
    pub token_type: &'static str,
    pub expires_in_seconds: i64,
    pub is_seller: bool,
    pub is_admin: bool,
    pub account_verified: bool,
    pub verification_channel: Option<&'static str>,
}

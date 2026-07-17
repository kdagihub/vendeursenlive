pub mod application;
pub mod domain;
pub mod infrastructure;
pub mod presentation;

use infrastructure::{
    auth::jwt::JwtService,
    config::{AuthConfig, EmailConfig, TikTokConfig},
};

#[derive(Clone)]
pub struct AppState {
    pub db: sea_orm::DatabaseConnection,
    pub redis: redis::Client,
    pub jwt: JwtService,
    pub auth_config: AuthConfig,
    pub tiktok_config: TikTokConfig,
    pub email_config: EmailConfig,
}

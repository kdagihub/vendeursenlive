use std::env;

use thiserror::Error;

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub app_env: AppEnv,
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub redis: RedisConfig,
    pub auth: AuthConfig,
    pub cors: CorsConfig,
    pub rate_limit: RateLimitConfig,
    pub security: SecurityConfig,
    pub migrations: MigrationConfig,
    pub tiktok: TikTokConfig,
    pub google: GoogleConfig,
    pub email: EmailConfig,
    pub ikoddi: IkoddiConfig,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppEnv {
    Development,
    Test,
    Production,
}

#[derive(Debug, Clone)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Clone)]
pub struct DatabaseConfig {
    pub url: String,
}

#[derive(Debug, Clone)]
pub struct RedisConfig {
    pub url: String,
}

#[derive(Debug, Clone)]
pub struct AuthConfig {
    pub access_token_secret: String,
    pub refresh_token_secret: String,
    pub access_token_ttl_seconds: i64,
    pub refresh_token_ttl_seconds: i64,
    pub cookie_secure: bool,
    pub cookie_domain: Option<String>,
    pub password_reset_token_ttl_seconds: i64,
    pub email_verification_token_ttl_seconds: i64,
    pub email_verification_resend_cooldown_seconds: i64,
}

#[derive(Debug, Clone)]
pub struct CorsConfig {
    pub allowed_origins: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct RateLimitConfig {
    pub enabled: bool,
    pub requests_per_minute: u32,
}

#[derive(Debug, Clone)]
pub struct SecurityConfig {
    pub csrf_protection_enabled: bool,
}

#[derive(Debug, Clone)]
pub struct MigrationConfig {
    pub run_on_start: bool,
}

#[derive(Debug, Clone)]
pub struct TikTokConfig {
    pub client_key: Option<String>,
    pub client_secret: Option<String>,
    pub redirect_uri: Option<String>,
    pub scopes: Vec<String>,
    pub auth_url: String,
    pub token_url: String,
    pub user_info_url: String,
    pub success_redirect_url: String,
}

#[derive(Debug, Clone)]
pub struct GoogleConfig {
    pub client_id: Option<String>,
    pub client_secret: Option<String>,
    pub redirect_uri: Option<String>,
    pub scopes: Vec<String>,
    pub auth_url: String,
    pub token_url: String,
    pub user_info_url: String,
    pub success_redirect_url: String,
}

#[derive(Debug, Clone)]
pub struct EmailConfig {
    pub delivery_enabled: bool,
    pub smtp_host: Option<String>,
    pub smtp_port: u16,
    pub smtp_username: Option<String>,
    pub smtp_password: Option<String>,
    pub from_email: String,
    pub from_name: String,
    pub password_reset_url: String,
    pub email_verification_url: String,
}

#[derive(Debug, Clone)]
pub struct IkoddiConfig {
    pub enabled: bool,
    pub base_url: String,
    pub api_key: Option<String>,
    pub organization_id: Option<String>,
    pub otp_app_id: Option<String>,
    pub challenge_ttl_seconds: i64,
    pub resend_cooldown_seconds: i64,
    pub max_attempts: u32,
}

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("missing required environment variable {0}")]
    MissingEnv(&'static str),
    #[error("invalid value for environment variable {name}: {source}")]
    InvalidEnv {
        name: &'static str,
        source: Box<dyn std::error::Error + Send + Sync>,
    },
    #[error("invalid application configuration: {0}")]
    InvalidConfiguration(&'static str),
}

impl AppConfig {
    pub fn from_env() -> Result<Self, ConfigError> {
        let config = Self {
            app_env: parse_app_env(env_or_default("APP_ENV", "development")?),
            server: ServerConfig {
                host: env_or_default("SERVER_HOST", "0.0.0.0")?,
                port: parse_env("SERVER_PORT", "8080")?,
            },
            database: DatabaseConfig {
                url: required_env("DATABASE_URL")?,
            },
            redis: RedisConfig {
                url: env_or_default("REDIS_URL", "redis://127.0.0.1:6379")?,
            },
            auth: AuthConfig {
                access_token_secret: required_env("JWT_ACCESS_TOKEN_SECRET")?,
                refresh_token_secret: required_env("JWT_REFRESH_TOKEN_SECRET")?,
                access_token_ttl_seconds: parse_env("JWT_ACCESS_TOKEN_TTL_SECONDS", "900")?,
                refresh_token_ttl_seconds: parse_env("JWT_REFRESH_TOKEN_TTL_SECONDS", "2592000")?,
                cookie_secure: parse_env("AUTH_COOKIE_SECURE", "false")?,
                cookie_domain: optional_env("AUTH_COOKIE_DOMAIN"),
                password_reset_token_ttl_seconds: parse_env(
                    "PASSWORD_RESET_TOKEN_TTL_SECONDS",
                    "1800",
                )?,
                email_verification_token_ttl_seconds: parse_env(
                    "EMAIL_VERIFICATION_TOKEN_TTL_SECONDS",
                    "86400",
                )?,
                email_verification_resend_cooldown_seconds: parse_env(
                    "EMAIL_VERIFICATION_RESEND_COOLDOWN_SECONDS",
                    "60",
                )?,
            },
            cors: CorsConfig {
                allowed_origins: parse_csv_env(
                    "CORS_ALLOWED_ORIGINS",
                    "http://localhost:5173,http://127.0.0.1:5173,https://vendeursenlive.shop",
                ),
            },
            rate_limit: RateLimitConfig {
                enabled: parse_env("RATE_LIMIT_ENABLED", "true")?,
                requests_per_minute: parse_env("RATE_LIMIT_REQUESTS_PER_MINUTE", "120")?,
            },
            security: SecurityConfig {
                csrf_protection_enabled: parse_env("CSRF_PROTECTION_ENABLED", "true")?,
            },
            migrations: MigrationConfig {
                run_on_start: parse_env("RUN_MIGRATIONS_ON_START", "true")?,
            },
            tiktok: TikTokConfig {
                client_key: optional_env("TIKTOK_CLIENT_KEY"),
                client_secret: optional_env("TIKTOK_CLIENT_SECRET"),
                redirect_uri: optional_env("TIKTOK_REDIRECT_URI"),
                scopes: parse_csv_env("TIKTOK_SCOPES", "user.info.basic"),
                auth_url: env_or_default(
                    "TIKTOK_AUTH_URL",
                    "https://www.tiktok.com/v2/auth/authorize/",
                )?,
                token_url: env_or_default(
                    "TIKTOK_TOKEN_URL",
                    "https://open.tiktokapis.com/v2/oauth/token/",
                )?,
                user_info_url: env_or_default(
                    "TIKTOK_USER_INFO_URL",
                    "https://open.tiktokapis.com/v2/user/info/",
                )?,
                success_redirect_url: env_or_default(
                    "TIKTOK_SUCCESS_REDIRECT_URL",
                    "http://localhost:5173/",
                )?,
            },
            google: GoogleConfig {
                client_id: optional_env("GOOGLE_CLIENT_ID"),
                client_secret: optional_env("GOOGLE_CLIENT_SECRET"),
                redirect_uri: optional_env("GOOGLE_REDIRECT_URI"),
                scopes: parse_csv_env("GOOGLE_SCOPES", "openid,email,profile"),
                auth_url: env_or_default(
                    "GOOGLE_AUTH_URL",
                    "https://accounts.google.com/o/oauth2/v2/auth",
                )?,
                token_url: env_or_default(
                    "GOOGLE_TOKEN_URL",
                    "https://oauth2.googleapis.com/token",
                )?,
                user_info_url: env_or_default(
                    "GOOGLE_USER_INFO_URL",
                    "https://openidconnect.googleapis.com/v1/userinfo",
                )?,
                success_redirect_url: env_or_default(
                    "GOOGLE_SUCCESS_REDIRECT_URL",
                    "http://localhost:5173/",
                )?,
            },
            email: EmailConfig {
                delivery_enabled: parse_env("EMAIL_DELIVERY_ENABLED", "false")?,
                smtp_host: optional_env("SMTP_HOST"),
                smtp_port: parse_env("SMTP_PORT", "465")?,
                smtp_username: optional_env("SMTP_USERNAME"),
                smtp_password: optional_env("SMTP_PASSWORD"),
                from_email: env_or_default("SMTP_FROM_EMAIL", "contact@vendeursenlive.shop")?,
                from_name: env_or_default("SMTP_FROM_NAME", "VendeursEnLive")?,
                password_reset_url: env_or_default(
                    "PASSWORD_RESET_URL",
                    "http://localhost:5173/reset-password",
                )?,
                email_verification_url: env_or_default(
                    "EMAIL_VERIFICATION_URL",
                    "http://localhost:5173/verify-email",
                )?,
            },
            ikoddi: IkoddiConfig {
                enabled: parse_env("IKODDI_ENABLED", "false")?,
                base_url: env_or_default("IKODDI_BASE_URL", "https://api.staging.ikoddi.com")?,
                api_key: optional_env("IKODDI_API_KEY"),
                organization_id: optional_env("IKODDI_ORGANIZATION_ID"),
                otp_app_id: optional_env("IKODDI_OTP_APP_ID"),
                challenge_ttl_seconds: parse_env("OTP_CHALLENGE_TTL_SECONDS", "300")?,
                resend_cooldown_seconds: parse_env("OTP_RESEND_COOLDOWN_SECONDS", "60")?,
                max_attempts: parse_env("OTP_MAX_ATTEMPTS", "5")?,
            },
        };

        config.ikoddi.validate()?;
        config.google.validate()?;
        Ok(config)
    }
}

impl GoogleConfig {
    fn validate(&self) -> Result<(), ConfigError> {
        let configured_values = [
            self.client_id.is_some(),
            self.client_secret.is_some(),
            self.redirect_uri.is_some(),
        ];

        if configured_values.iter().any(|configured| *configured)
            && !configured_values.iter().all(|configured| *configured)
        {
            return Err(ConfigError::InvalidConfiguration(
                "GOOGLE_CLIENT_ID, GOOGLE_CLIENT_SECRET and GOOGLE_REDIRECT_URI must be configured together",
            ));
        }

        if self.scopes.iter().any(|scope| scope == "openid") {
            Ok(())
        } else {
            Err(ConfigError::InvalidConfiguration(
                "GOOGLE_SCOPES must include openid",
            ))
        }
    }
}

impl IkoddiConfig {
    fn validate(&self) -> Result<(), ConfigError> {
        if !self.enabled {
            return Ok(());
        }

        if self.api_key.is_none() || self.organization_id.is_none() || self.otp_app_id.is_none() {
            return Err(ConfigError::InvalidConfiguration(
                "IKODDI_ENABLED requires IKODDI_API_KEY, IKODDI_ORGANIZATION_ID and IKODDI_OTP_APP_ID",
            ));
        }

        if self.challenge_ttl_seconds <= 0
            || self.resend_cooldown_seconds <= 0
            || self.max_attempts == 0
        {
            return Err(ConfigError::InvalidConfiguration(
                "IKODDI OTP limits must be greater than zero",
            ));
        }

        Ok(())
    }
}

impl ServerConfig {
    pub fn bind_address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}

fn parse_app_env(value: String) -> AppEnv {
    match value.to_ascii_lowercase().as_str() {
        "production" | "prod" => AppEnv::Production,
        "test" => AppEnv::Test,
        _ => AppEnv::Development,
    }
}

fn required_env(name: &'static str) -> Result<String, ConfigError> {
    env::var(name).map_err(|_| ConfigError::MissingEnv(name))
}

fn env_or_default(name: &'static str, default: &str) -> Result<String, ConfigError> {
    Ok(env::var(name).unwrap_or_else(|_| default.to_owned()))
}

fn optional_env(name: &'static str) -> Option<String> {
    env::var(name).ok().and_then(|value| {
        let value = value.trim().to_owned();
        (!value.is_empty()).then_some(value)
    })
}

fn parse_csv_env(name: &'static str, default: &str) -> Vec<String> {
    env::var(name)
        .unwrap_or_else(|_| default.to_owned())
        .split(',')
        .filter_map(|value| {
            let value = value.trim().to_owned();
            (!value.is_empty()).then_some(value)
        })
        .collect()
}

fn parse_env<T>(name: &'static str, default: &str) -> Result<T, ConfigError>
where
    T: std::str::FromStr,
    T::Err: std::error::Error + Send + Sync + 'static,
{
    env_or_default(name, default)?
        .parse::<T>()
        .map_err(|source| ConfigError::InvalidEnv {
            name,
            source: Box::new(source),
        })
}

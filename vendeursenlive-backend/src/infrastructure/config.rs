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
    pub expose_password_reset_token: bool,
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
}

impl AppConfig {
    pub fn from_env() -> Result<Self, ConfigError> {
        Ok(Self {
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
                expose_password_reset_token: parse_env(
                    "AUTH_EXPOSE_PASSWORD_RESET_TOKEN",
                    "false",
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
            },
        })
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

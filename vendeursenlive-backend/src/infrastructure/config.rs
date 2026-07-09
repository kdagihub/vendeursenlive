use std::env;

use thiserror::Error;

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub app_env: AppEnv,
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub redis: RedisConfig,
    pub auth: AuthConfig,
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

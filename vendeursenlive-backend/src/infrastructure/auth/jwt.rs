use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    application::{errors::ApplicationError, ports::auth::AccessTokenIssuer},
    infrastructure::config::AuthConfig,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessTokenClaims {
    pub sub: Uuid,
    pub sid: Uuid,
    pub is_admin: bool,
    pub is_seller: bool,
    #[serde(default)]
    pub account_verified: bool,
    #[serde(default)]
    pub verification_channel: Option<String>,
    pub exp: usize,
    pub iat: usize,
}

#[derive(Clone)]
pub struct JwtService {
    access_encoding_key: EncodingKey,
    access_decoding_key: DecodingKey,
    access_ttl_seconds: i64,
}

impl JwtService {
    pub fn new(config: &AuthConfig) -> Self {
        Self {
            access_encoding_key: EncodingKey::from_secret(config.access_token_secret.as_bytes()),
            access_decoding_key: DecodingKey::from_secret(config.access_token_secret.as_bytes()),
            access_ttl_seconds: config.access_token_ttl_seconds,
        }
    }

    pub fn issue_access_token(
        &self,
        user_id: Uuid,
        session_id: Uuid,
        is_admin: bool,
        is_seller: bool,
        account_verified: bool,
        verification_channel: Option<&str>,
    ) -> Result<String, jsonwebtoken::errors::Error> {
        let issued_at = Utc::now();
        let expires_at = issued_at + Duration::seconds(self.access_ttl_seconds);
        let claims = AccessTokenClaims {
            sub: user_id,
            sid: session_id,
            is_admin,
            is_seller,
            account_verified,
            verification_channel: verification_channel.map(str::to_owned),
            exp: expires_at.timestamp() as usize,
            iat: issued_at.timestamp() as usize,
        };

        jsonwebtoken::encode(&Header::default(), &claims, &self.access_encoding_key)
    }

    pub fn verify_access_token(
        &self,
        token: &str,
    ) -> Result<AccessTokenClaims, jsonwebtoken::errors::Error> {
        jsonwebtoken::decode::<AccessTokenClaims>(
            token,
            &self.access_decoding_key,
            &Validation::default(),
        )
        .map(|data| data.claims)
    }
}

impl AccessTokenIssuer for JwtService {
    fn issue_access_token(
        &self,
        user_id: Uuid,
        session_id: Uuid,
        is_admin: bool,
        is_seller: bool,
        account_verified: bool,
        verification_channel: Option<&str>,
    ) -> Result<String, ApplicationError> {
        JwtService::issue_access_token(
            self,
            user_id,
            session_id,
            is_admin,
            is_seller,
            account_verified,
            verification_channel,
        )
        .map_err(|error| ApplicationError::Infrastructure(error.to_string()))
    }
}

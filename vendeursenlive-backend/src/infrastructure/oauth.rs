use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::{
    application::{
        errors::ApplicationError,
        ports::auth::{TikTokOAuthClient, TikTokTokenResponse, TikTokUserProfile},
    },
    infrastructure::config::TikTokConfig,
};

#[derive(Debug, Clone)]
pub struct ReqwestTikTokOAuthClient {
    http: Client,
    config: TikTokConfig,
}

impl ReqwestTikTokOAuthClient {
    pub fn new(config: TikTokConfig) -> Self {
        Self {
            http: Client::new(),
            config,
        }
    }
}

#[async_trait]
impl TikTokOAuthClient for ReqwestTikTokOAuthClient {
    async fn exchange_code(&self, code: &str) -> Result<TikTokTokenResponse, ApplicationError> {
        let client_key = self.config.client_key.as_deref().ok_or_else(|| {
            ApplicationError::Infrastructure("TikTok client key is not configured".to_owned())
        })?;
        let client_secret = self.config.client_secret.as_deref().ok_or_else(|| {
            ApplicationError::Infrastructure("TikTok client secret is not configured".to_owned())
        })?;
        let redirect_uri = self.config.redirect_uri.as_deref().ok_or_else(|| {
            ApplicationError::Infrastructure("TikTok redirect URI is not configured".to_owned())
        })?;

        let request = TikTokTokenRequest {
            client_key,
            client_secret,
            code,
            grant_type: "authorization_code",
            redirect_uri,
        };

        let response = self
            .http
            .post(&self.config.token_url)
            .header("Cache-Control", "no-cache")
            .form(&request)
            .send()
            .await
            .map_err(tiktok_request_error)?;

        let status = response.status();
        let payload = response
            .json::<TikTokTokenApiResponse>()
            .await
            .map_err(tiktok_request_error)?;

        if !status.is_success() {
            return Err(tiktok_api_error(
                "TikTok token exchange failed",
                payload.error.as_deref(),
                payload.error_description.as_deref(),
            ));
        }

        Ok(TikTokTokenResponse {
            open_id: non_empty(payload.open_id, "TikTok token response missing open_id")?,
            access_token: non_empty(
                payload.access_token,
                "TikTok token response missing access_token",
            )?,
            scope: payload.scope.unwrap_or_default(),
        })
    }

    async fn fetch_user_profile(
        &self,
        access_token: &str,
    ) -> Result<TikTokUserProfile, ApplicationError> {
        let response = self
            .http
            .get(&self.config.user_info_url)
            .bearer_auth(access_token)
            .query(&[("fields", "open_id,union_id,avatar_url,display_name")])
            .send()
            .await
            .map_err(tiktok_request_error)?;

        let status = response.status();
        let payload = response
            .json::<TikTokUserInfoApiResponse>()
            .await
            .map_err(tiktok_request_error)?;

        if !status.is_success() || payload.error.code != "ok" {
            return Err(tiktok_api_error(
                "TikTok user info request failed",
                Some(payload.error.code.as_str()),
                Some(payload.error.message.as_str()),
            ));
        }
        let data = payload.data.ok_or_else(|| {
            ApplicationError::Infrastructure("TikTok user info response missing data".to_owned())
        })?;

        Ok(TikTokUserProfile {
            open_id: non_empty(
                data.user.open_id,
                "TikTok user info response missing open_id",
            )?,
            union_id: data.user.union_id.and_then(clean_optional),
            display_name: data.user.display_name.and_then(clean_optional),
            avatar_url: data.user.avatar_url.and_then(clean_optional),
        })
    }
}

#[derive(Debug, Serialize)]
struct TikTokTokenRequest<'a> {
    client_key: &'a str,
    client_secret: &'a str,
    code: &'a str,
    grant_type: &'a str,
    redirect_uri: &'a str,
}

#[derive(Debug, Deserialize)]
struct TikTokTokenApiResponse {
    open_id: Option<String>,
    access_token: Option<String>,
    scope: Option<String>,
    error: Option<String>,
    error_description: Option<String>,
}

#[derive(Debug, Deserialize)]
struct TikTokUserInfoApiResponse {
    data: Option<TikTokUserInfoData>,
    error: TikTokErrorObject,
}

#[derive(Debug, Deserialize)]
struct TikTokUserInfoData {
    user: TikTokUserObject,
}

#[derive(Debug, Deserialize)]
struct TikTokUserObject {
    open_id: Option<String>,
    union_id: Option<String>,
    display_name: Option<String>,
    avatar_url: Option<String>,
}

#[derive(Debug, Deserialize)]
struct TikTokErrorObject {
    code: String,
    message: String,
}

fn tiktok_request_error(error: reqwest::Error) -> ApplicationError {
    ApplicationError::Infrastructure(format!("TikTok HTTP request failed: {error}"))
}

fn tiktok_api_error(
    context: &str,
    error: Option<&str>,
    description: Option<&str>,
) -> ApplicationError {
    ApplicationError::Infrastructure(format!(
        "{context}: {}",
        description
            .filter(|value| !value.trim().is_empty())
            .or(error)
            .unwrap_or("unknown TikTok API error")
    ))
}

fn non_empty(value: Option<String>, message: &str) -> Result<String, ApplicationError> {
    value
        .and_then(clean_optional)
        .ok_or_else(|| ApplicationError::Infrastructure(message.to_owned()))
}

fn clean_optional(value: String) -> Option<String> {
    let value = value.trim().to_owned();
    (!value.is_empty()).then_some(value)
}

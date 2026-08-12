use async_trait::async_trait;
use std::time::Duration;

use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::{
    application::{
        errors::ApplicationError,
        ports::auth::{
            GoogleOAuthClient, GoogleTokenResponse, GoogleUserProfile, TikTokOAuthClient,
            TikTokTokenResponse, TikTokUserProfile,
        },
    },
    infrastructure::config::{GoogleConfig, TikTokConfig},
};

const OAUTH_HTTP_TIMEOUT: Duration = Duration::from_secs(10);

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
            .timeout(OAUTH_HTTP_TIMEOUT)
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
            .timeout(OAUTH_HTTP_TIMEOUT)
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

#[derive(Debug, Clone)]
pub struct ReqwestGoogleOAuthClient {
    http: Client,
    config: GoogleConfig,
}

impl ReqwestGoogleOAuthClient {
    pub fn new(config: GoogleConfig) -> Self {
        Self {
            http: Client::new(),
            config,
        }
    }
}

#[async_trait]
impl GoogleOAuthClient for ReqwestGoogleOAuthClient {
    async fn exchange_code(&self, code: &str) -> Result<GoogleTokenResponse, ApplicationError> {
        let client_id = self.config.client_id.as_deref().ok_or_else(|| {
            ApplicationError::ServiceUnavailable(
                "Google authentication is not configured".to_owned(),
            )
        })?;
        let client_secret = self.config.client_secret.as_deref().ok_or_else(|| {
            ApplicationError::ServiceUnavailable(
                "Google authentication is not configured".to_owned(),
            )
        })?;
        let redirect_uri = self.config.redirect_uri.as_deref().ok_or_else(|| {
            ApplicationError::ServiceUnavailable(
                "Google authentication is not configured".to_owned(),
            )
        })?;

        let response = self
            .http
            .post(&self.config.token_url)
            .timeout(OAUTH_HTTP_TIMEOUT)
            .header("Cache-Control", "no-cache")
            .form(&GoogleTokenRequest {
                client_id,
                client_secret,
                code,
                grant_type: "authorization_code",
                redirect_uri,
            })
            .send()
            .await
            .map_err(google_request_error)?;

        let status = response.status();
        let payload = response
            .json::<GoogleTokenApiResponse>()
            .await
            .map_err(google_request_error)?;

        if !status.is_success() {
            return Err(google_api_error(
                "Google token exchange failed",
                payload.error.as_deref(),
                payload.error_description.as_deref(),
            ));
        }

        Ok(GoogleTokenResponse {
            access_token: non_empty(
                payload.access_token,
                "Google token response missing access_token",
            )?,
            scope: payload.scope.unwrap_or_default(),
        })
    }

    async fn fetch_user_profile(
        &self,
        access_token: &str,
    ) -> Result<GoogleUserProfile, ApplicationError> {
        let response = self
            .http
            .get(&self.config.user_info_url)
            .timeout(OAUTH_HTTP_TIMEOUT)
            .bearer_auth(access_token)
            .send()
            .await
            .map_err(google_request_error)?;

        let status = response.status();
        let payload = response
            .json::<GoogleUserInfoApiResponse>()
            .await
            .map_err(google_request_error)?;

        if !status.is_success() {
            return Err(google_api_error(
                "Google user info request failed",
                payload.error.as_deref(),
                payload.error_description.as_deref(),
            ));
        }

        Ok(GoogleUserProfile {
            subject: non_empty(payload.subject, "Google user info response missing sub")?,
            email: non_empty(payload.email, "Google user info response missing email")?,
            email_verified: payload.email_verified.unwrap_or(false),
            display_name: payload.name.and_then(clean_optional),
            avatar_url: payload.picture.and_then(clean_optional),
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

#[derive(Debug, Serialize)]
struct GoogleTokenRequest<'a> {
    client_id: &'a str,
    client_secret: &'a str,
    code: &'a str,
    grant_type: &'a str,
    redirect_uri: &'a str,
}

#[derive(Debug, Deserialize)]
struct GoogleTokenApiResponse {
    access_token: Option<String>,
    scope: Option<String>,
    error: Option<String>,
    error_description: Option<String>,
}

#[derive(Debug, Deserialize)]
struct GoogleUserInfoApiResponse {
    #[serde(rename = "sub")]
    subject: Option<String>,
    email: Option<String>,
    email_verified: Option<bool>,
    name: Option<String>,
    picture: Option<String>,
    error: Option<String>,
    error_description: Option<String>,
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

fn google_request_error(error: reqwest::Error) -> ApplicationError {
    ApplicationError::ServiceUnavailable(format!(
        "Google authentication service could not be reached: {error}"
    ))
}

fn google_api_error(
    context: &str,
    error: Option<&str>,
    description: Option<&str>,
) -> ApplicationError {
    ApplicationError::Infrastructure(format!(
        "{context}: {}",
        description
            .filter(|value| !value.trim().is_empty())
            .or(error)
            .unwrap_or("unknown Google API error")
    ))
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

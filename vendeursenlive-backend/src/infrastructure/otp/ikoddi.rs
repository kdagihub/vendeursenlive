use std::time::Duration;

use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::{
    application::{errors::ApplicationError, ports::auth::PhoneOtpProvider},
    infrastructure::config::IkoddiConfig,
};

#[derive(Debug, Clone)]
pub struct IkoddiPhoneOtpProvider {
    client: Client,
    config: IkoddiConfig,
}

#[derive(Debug, Deserialize)]
struct IkoddiOtpResponse {
    status: i32,
    #[serde(rename = "otpToken")]
    otp_token: Option<String>,
}

#[derive(Debug, Deserialize)]
struct IkoddiVerifyResponse {
    status: i32,
}

#[derive(Debug, Serialize)]
struct IkoddiVerifyRequest<'a> {
    #[serde(rename = "verificationKey")]
    verification_key: &'a str,
    otp: &'a str,
    identity: &'a str,
}

impl IkoddiPhoneOtpProvider {
    pub fn new(config: IkoddiConfig) -> Result<Self, ApplicationError> {
        let client = Client::builder()
            .timeout(Duration::from_secs(10))
            .build()
            .map_err(|error| ApplicationError::Infrastructure(error.to_string()))?;

        Ok(Self { client, config })
    }

    fn credentials(&self) -> Result<(&str, &str, &str), ApplicationError> {
        if !self.config.enabled {
            return Err(ApplicationError::ServiceUnavailable(
                "phone authentication is temporarily unavailable".to_owned(),
            ));
        }

        match (
            self.config.api_key.as_deref(),
            self.config.organization_id.as_deref(),
            self.config.otp_app_id.as_deref(),
        ) {
            (Some(api_key), Some(organization_id), Some(otp_app_id)) => {
                Ok((api_key, organization_id, otp_app_id))
            }
            _ => Err(ApplicationError::ServiceUnavailable(
                "phone authentication is not configured".to_owned(),
            )),
        }
    }

    fn endpoint(&self, organization_id: &str, path: &str) -> String {
        format!(
            "{}/api/v1/groups/{organization_id}/{path}",
            self.config.base_url.trim_end_matches('/')
        )
    }
}

#[async_trait]
impl PhoneOtpProvider for IkoddiPhoneOtpProvider {
    async fn request_otp(&self, identity: &str) -> Result<String, ApplicationError> {
        let (api_key, organization_id, otp_app_id) = self.credentials()?;
        let response = self
            .client
            .post(self.endpoint(organization_id, &format!("otp/{otp_app_id}/sms/{identity}")))
            .header("x-api-key", api_key)
            .send()
            .await
            .map_err(|_| {
                ApplicationError::ServiceUnavailable(
                    "the SMS provider could not be reached".to_owned(),
                )
            })?;

        if !response.status().is_success() {
            return Err(ApplicationError::ServiceUnavailable(format!(
                "the SMS provider returned HTTP {}",
                response.status()
            )));
        }

        let payload = response.json::<IkoddiOtpResponse>().await.map_err(|_| {
            ApplicationError::Infrastructure("invalid SMS provider response".to_owned())
        })?;

        match (
            payload.status,
            payload.otp_token.filter(|token| !token.is_empty()),
        ) {
            (0, Some(token)) => Ok(token),
            _ => Err(ApplicationError::ServiceUnavailable(
                "the SMS provider rejected the OTP request".to_owned(),
            )),
        }
    }

    async fn verify_otp(
        &self,
        identity: &str,
        otp: &str,
        provider_token: &str,
    ) -> Result<bool, ApplicationError> {
        let (api_key, organization_id, otp_app_id) = self.credentials()?;
        let response = self
            .client
            .post(self.endpoint(organization_id, &format!("otp/{otp_app_id}/verify")))
            .header("x-api-key", api_key)
            .json(&IkoddiVerifyRequest {
                verification_key: provider_token,
                otp,
                identity,
            })
            .send()
            .await
            .map_err(|_| {
                ApplicationError::ServiceUnavailable(
                    "the SMS provider could not be reached".to_owned(),
                )
            })?;

        let status = response.status();
        if status == reqwest::StatusCode::UNAUTHORIZED
            || status == reqwest::StatusCode::FORBIDDEN
            || status == reqwest::StatusCode::TOO_MANY_REQUESTS
            || status.is_server_error()
        {
            return Err(ApplicationError::ServiceUnavailable(format!(
                "the SMS provider returned HTTP {status}"
            )));
        }

        if !status.is_success() {
            return Ok(false);
        }

        response
            .json::<IkoddiVerifyResponse>()
            .await
            .map(|payload| payload.status == 0)
            .map_err(|_| {
                ApplicationError::Infrastructure("invalid SMS provider response".to_owned())
            })
    }
}

use std::time::Duration;

use async_trait::async_trait;
use reqwest::{header::LOCATION, redirect::Policy, Client};
use url::Url;

use crate::{
    application::{errors::ApplicationError, ports::live::TikTokLiveLinkResolver},
    domain::value_objects::TikTokLiveUrl,
};

const MAX_REDIRECTS: usize = 5;
const ALLOWED_TIKTOK_HOSTS: &[&str] = &[
    "tiktok.com",
    "www.tiktok.com",
    "m.tiktok.com",
    "vm.tiktok.com",
    "vt.tiktok.com",
];

#[derive(Debug, Clone)]
pub struct ReqwestTikTokLiveLinkResolver {
    client: Client,
}

impl ReqwestTikTokLiveLinkResolver {
    pub fn new() -> Result<Self, ApplicationError> {
        let client = Client::builder()
            .redirect(Policy::none())
            .connect_timeout(Duration::from_secs(4))
            .timeout(Duration::from_secs(8))
            .user_agent("VendeursEnLive/1.0")
            .build()
            .map_err(|error| {
                ApplicationError::Infrastructure(format!(
                    "TikTok LIVE link resolver setup failed: {error}"
                ))
            })?;

        Ok(Self { client })
    }

    fn parse_and_validate_url(value: &str) -> Result<Url, ApplicationError> {
        let url = Url::parse(value.trim())
            .map_err(|_| ApplicationError::Validation("TikTok LIVE link is invalid".to_owned()))?;

        if url.scheme() != "https"
            || url.port().is_some_and(|port| port != 443)
            || url.username() != ""
            || url.password().is_some()
            || !url.host_str().is_some_and(is_allowed_tiktok_host)
        {
            return Err(ApplicationError::Validation(
                "only HTTPS links hosted by TikTok are accepted".to_owned(),
            ));
        }

        Ok(url)
    }

    fn canonical_live_url(url: &Url) -> Result<Option<TikTokLiveUrl>, ApplicationError> {
        let segments = url
            .path_segments()
            .map(|segments| {
                segments
                    .filter(|segment| !segment.is_empty())
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();

        let [profile, live] = segments.as_slice() else {
            return Ok(None);
        };
        if *live != "live" {
            return Ok(None);
        }

        let Some(username) = profile.strip_prefix('@') else {
            return Ok(None);
        };

        TikTokLiveUrl::from_username(username)
            .map(Some)
            .map_err(ApplicationError::from)
    }
}

#[async_trait]
impl TikTokLiveLinkResolver for ReqwestTikTokLiveLinkResolver {
    async fn resolve(&self, shared_url: &str) -> Result<TikTokLiveUrl, ApplicationError> {
        let mut current_url = Self::parse_and_validate_url(shared_url)?;

        for redirect_count in 0..=MAX_REDIRECTS {
            if let Some(canonical_url) = Self::canonical_live_url(&current_url)? {
                return Ok(canonical_url);
            }
            if redirect_count == MAX_REDIRECTS {
                break;
            }

            let response = self
                .client
                .get(current_url.clone())
                .send()
                .await
                .map_err(|error| {
                    ApplicationError::ServiceUnavailable(format!(
                        "TikTok LIVE link could not be resolved: {error}"
                    ))
                })?;

            if !response.status().is_redirection() {
                return Err(ApplicationError::Validation(
                    "the link does not identify a TikTok LIVE".to_owned(),
                ));
            }

            let location = response
                .headers()
                .get(LOCATION)
                .and_then(|value| value.to_str().ok())
                .ok_or_else(|| {
                    ApplicationError::Validation(
                        "TikTok returned an invalid LIVE redirect".to_owned(),
                    )
                })?;
            let redirected_url = current_url.join(location).map_err(|_| {
                ApplicationError::Validation("TikTok returned an invalid LIVE URL".to_owned())
            })?;
            current_url = Self::parse_and_validate_url(redirected_url.as_str())?;
        }

        Err(ApplicationError::Validation(
            "too many redirects while resolving the TikTok LIVE link".to_owned(),
        ))
    }
}

fn is_allowed_tiktok_host(host: &str) -> bool {
    ALLOWED_TIKTOK_HOSTS.contains(&host.to_ascii_lowercase().as_str())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extracts_a_username_from_a_canonical_live_link() {
        let url = ReqwestTikTokLiveLinkResolver::parse_and_validate_url(
            "https://www.tiktok.com/@vendeursenlive/live?is_from_webapp=1",
        )
        .expect("valid TikTok URL");
        let normalized = ReqwestTikTokLiveLinkResolver::canonical_live_url(&url)
            .expect("normalization succeeds")
            .expect("LIVE username exists");

        assert_eq!(normalized.username(), "vendeursenlive");
        assert_eq!(
            normalized.as_str(),
            "https://www.tiktok.com/@vendeursenlive/live"
        );
    }

    #[test]
    fn rejects_lookalike_hosts_and_non_https_links() {
        assert!(ReqwestTikTokLiveLinkResolver::parse_and_validate_url(
            "https://www.tiktok.com.evil.test/@seller/live"
        )
        .is_err());
        assert!(ReqwestTikTokLiveLinkResolver::parse_and_validate_url(
            "http://www.tiktok.com/@seller/live"
        )
        .is_err());
    }

    #[test]
    fn does_not_treat_a_regular_video_as_a_live() {
        let url = ReqwestTikTokLiveLinkResolver::parse_and_validate_url(
            "https://www.tiktok.com/@seller/video/123",
        )
        .expect("valid TikTok URL");

        assert!(ReqwestTikTokLiveLinkResolver::canonical_live_url(&url)
            .expect("normalization succeeds")
            .is_none());
    }
}

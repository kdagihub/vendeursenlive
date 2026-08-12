use std::sync::Arc;

use uuid::Uuid;

use crate::{
    application::{
        dtos::live::{LiveSessionResponse, StartLiveSessionRequest},
        errors::ApplicationError,
        ports::live::TikTokLiveLinkResolver,
    },
    domain::{
        entities::{LiveSession, LiveSessionStatus},
        repositories::{LiveSessionRepository, SellerProfileRepository},
    },
};

pub struct StartLiveSessionUseCase {
    seller_profiles: Arc<dyn SellerProfileRepository>,
    live_sessions: Arc<dyn LiveSessionRepository>,
    live_link_resolver: Arc<dyn TikTokLiveLinkResolver>,
}

impl StartLiveSessionUseCase {
    pub fn new(
        seller_profiles: Arc<dyn SellerProfileRepository>,
        live_sessions: Arc<dyn LiveSessionRepository>,
        live_link_resolver: Arc<dyn TikTokLiveLinkResolver>,
    ) -> Self {
        Self {
            seller_profiles,
            live_sessions,
            live_link_resolver,
        }
    }

    pub async fn execute(
        &self,
        user_id: Uuid,
        request: StartLiveSessionRequest,
    ) -> Result<LiveSessionResponse, ApplicationError> {
        let seller = self
            .seller_profiles
            .find_by_user_id(user_id)
            .await?
            .ok_or_else(|| ApplicationError::Forbidden("seller profile required".to_owned()))?;

        if !seller.is_active {
            return Err(ApplicationError::Forbidden(
                "seller profile is inactive".to_owned(),
            ));
        }
        if match seller.shop_name.as_deref() {
            Some(shop_name) => shop_name.is_empty(),
            None => true,
        } {
            return Err(ApplicationError::Validation(
                "shop name must be completed before starting a LIVE".to_owned(),
            ));
        }
        if self
            .live_sessions
            .find_ongoing_by_seller(seller.id)
            .await?
            .is_some()
        {
            return Err(ApplicationError::Conflict(
                "an ongoing LIVE already exists for this seller".to_owned(),
            ));
        }

        let tiktok_url = self
            .live_link_resolver
            .resolve(&request.tiktok_live_url)
            .await?;
        let session = LiveSession::start(seller.id, tiktok_url);
        self.live_sessions.save(&session).await?;

        Ok(LiveSessionResponse::from(&session))
    }
}

#[cfg(test)]
#[allow(clippy::items_after_test_module)]
mod tests {
    use std::sync::{Arc, Mutex};

    use async_trait::async_trait;

    use super::*;
    use crate::domain::{
        entities::SellerProfile, errors::DomainError, value_objects::TikTokLiveUrl,
    };

    struct FakeSellerProfileRepository {
        profile: Option<SellerProfile>,
    }

    #[async_trait]
    impl SellerProfileRepository for FakeSellerProfileRepository {
        async fn save(&self, _: &SellerProfile) -> Result<(), DomainError> {
            Ok(())
        }

        async fn find_by_id(&self, id: Uuid) -> Result<Option<SellerProfile>, DomainError> {
            Ok(self.profile.clone().filter(|profile| profile.id == id))
        }

        async fn find_by_user_id(
            &self,
            user_id: Uuid,
        ) -> Result<Option<SellerProfile>, DomainError> {
            Ok(self
                .profile
                .clone()
                .filter(|profile| profile.user_id == user_id))
        }

        async fn update_shop_name(&self, _: Uuid, _: String) -> Result<(), DomainError> {
            Ok(())
        }
    }

    #[derive(Default)]
    struct FakeLiveSessionRepository {
        sessions: Mutex<Vec<LiveSession>>,
    }

    #[async_trait]
    impl LiveSessionRepository for FakeLiveSessionRepository {
        async fn save(&self, session: &LiveSession) -> Result<(), DomainError> {
            self.sessions
                .lock()
                .expect("sessions lock")
                .push(session.clone());
            Ok(())
        }

        async fn update(&self, session: &LiveSession) -> Result<(), DomainError> {
            let mut sessions = self.sessions.lock().expect("sessions lock");
            if let Some(stored) = sessions.iter_mut().find(|stored| stored.id == session.id) {
                *stored = session.clone();
            }
            Ok(())
        }

        async fn find_by_id(&self, id: Uuid) -> Result<Option<LiveSession>, DomainError> {
            Ok(self
                .sessions
                .lock()
                .expect("sessions lock")
                .iter()
                .find(|session| session.id == id)
                .cloned())
        }

        async fn find_ongoing_by_seller(
            &self,
            seller_profile_id: Uuid,
        ) -> Result<Option<LiveSession>, DomainError> {
            Ok(self
                .sessions
                .lock()
                .expect("sessions lock")
                .iter()
                .find(|session| {
                    session.seller_profile_id == seller_profile_id
                        && session.status == LiveSessionStatus::Ongoing
                })
                .cloned())
        }
    }

    struct FakeLiveLinkResolver;

    #[async_trait]
    impl TikTokLiveLinkResolver for FakeLiveLinkResolver {
        async fn resolve(&self, _: &str) -> Result<TikTokLiveUrl, ApplicationError> {
            TikTokLiveUrl::from_username("vendeursenlive").map_err(ApplicationError::from)
        }
    }

    #[actix_rt::test]
    async fn starts_one_normalized_live_for_a_complete_seller() {
        let user_id = Uuid::now_v7();
        let seller = SellerProfile::start_trial(user_id, Some("Boutique VEL".to_owned()), None);
        let sessions = Arc::new(FakeLiveSessionRepository::default());
        let use_case = StartLiveSessionUseCase::new(
            Arc::new(FakeSellerProfileRepository {
                profile: Some(seller),
            }),
            sessions.clone(),
            Arc::new(FakeLiveLinkResolver),
        );

        let response = use_case
            .execute(
                user_id,
                StartLiveSessionRequest {
                    tiktok_live_url: "https://vm.tiktok.com/example".to_owned(),
                },
            )
            .await
            .expect("LIVE starts");

        assert_eq!(response.tiktok_username, "vendeursenlive");
        assert_eq!(response.status, "ongoing");
        assert_eq!(sessions.sessions.lock().expect("sessions lock").len(), 1);
    }

    #[actix_rt::test]
    async fn requires_a_shop_name_before_starting_a_live() {
        let user_id = Uuid::now_v7();
        let seller = SellerProfile::start_trial(user_id, None, None);
        let use_case = StartLiveSessionUseCase::new(
            Arc::new(FakeSellerProfileRepository {
                profile: Some(seller),
            }),
            Arc::new(FakeLiveSessionRepository::default()),
            Arc::new(FakeLiveLinkResolver),
        );

        let error = use_case
            .execute(
                user_id,
                StartLiveSessionRequest {
                    tiktok_live_url: "https://www.tiktok.com/@seller/live".to_owned(),
                },
            )
            .await
            .expect_err("incomplete seller is rejected");

        assert!(matches!(error, ApplicationError::Validation(_)));
    }
}

pub struct GetCurrentLiveSessionUseCase {
    seller_profiles: Arc<dyn SellerProfileRepository>,
    live_sessions: Arc<dyn LiveSessionRepository>,
}

impl GetCurrentLiveSessionUseCase {
    pub fn new(
        seller_profiles: Arc<dyn SellerProfileRepository>,
        live_sessions: Arc<dyn LiveSessionRepository>,
    ) -> Self {
        Self {
            seller_profiles,
            live_sessions,
        }
    }

    pub async fn execute(
        &self,
        user_id: Uuid,
    ) -> Result<Option<LiveSessionResponse>, ApplicationError> {
        let seller = self
            .seller_profiles
            .find_by_user_id(user_id)
            .await?
            .ok_or_else(|| ApplicationError::Forbidden("seller profile required".to_owned()))?;
        let session = self.live_sessions.find_ongoing_by_seller(seller.id).await?;

        Ok(session.as_ref().map(LiveSessionResponse::from))
    }
}

pub struct EndLiveSessionUseCase {
    seller_profiles: Arc<dyn SellerProfileRepository>,
    live_sessions: Arc<dyn LiveSessionRepository>,
}

impl EndLiveSessionUseCase {
    pub fn new(
        seller_profiles: Arc<dyn SellerProfileRepository>,
        live_sessions: Arc<dyn LiveSessionRepository>,
    ) -> Self {
        Self {
            seller_profiles,
            live_sessions,
        }
    }

    pub async fn execute(
        &self,
        user_id: Uuid,
        live_session_id: Uuid,
    ) -> Result<LiveSessionResponse, ApplicationError> {
        let seller = self
            .seller_profiles
            .find_by_user_id(user_id)
            .await?
            .ok_or_else(|| ApplicationError::Forbidden("seller profile required".to_owned()))?;
        let mut session = self
            .live_sessions
            .find_by_id(live_session_id)
            .await?
            .ok_or_else(|| ApplicationError::NotFound("LIVE session".to_owned()))?;

        if session.seller_profile_id != seller.id {
            return Err(ApplicationError::Forbidden(
                "this LIVE belongs to another seller".to_owned(),
            ));
        }
        if session.status == LiveSessionStatus::Ended {
            return Err(ApplicationError::Conflict(
                "this LIVE is already ended".to_owned(),
            ));
        }

        session.end();
        self.live_sessions.update(&session).await?;
        Ok(LiveSessionResponse::from(&session))
    }
}

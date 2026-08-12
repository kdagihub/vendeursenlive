use async_trait::async_trait;
use chrono::{DateTime, FixedOffset, Utc};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QueryOrder, Set,
};
use uuid::Uuid;

use crate::{
    domain::{
        entities::{LiveSession, LiveSessionStatus},
        errors::DomainError,
        repositories::LiveSessionRepository,
        value_objects::TikTokLiveUrl,
    },
    infrastructure::{auth::password::repository_error, database::entities::live_sessions},
};

#[derive(Debug, Clone)]
pub struct SeaOrmLiveSessionRepository {
    db: DatabaseConnection,
}

impl SeaOrmLiveSessionRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

#[async_trait]
impl LiveSessionRepository for SeaOrmLiveSessionRepository {
    async fn save(&self, session: &LiveSession) -> Result<(), DomainError> {
        live_sessions::ActiveModel {
            id: Set(session.id),
            seller_profile_id: Set(session.seller_profile_id),
            tiktok_url: Set(session.tiktok_url.as_str().to_owned()),
            status: Set(status_to_db(session.status)),
            created_at: Set(to_db_datetime(session.created_at)),
            ended_at: Set(session.ended_at.map(to_db_datetime)),
        }
        .insert(&self.db)
        .await
        .map(|_| ())
        .map_err(|error| {
            if error
                .to_string()
                .contains("idx_live_sessions_one_ongoing_per_seller")
            {
                DomainError::Conflict("an ongoing LIVE already exists for this seller".to_owned())
            } else {
                repository_error(error)
            }
        })
    }

    async fn update(&self, session: &LiveSession) -> Result<(), DomainError> {
        let Some(model) = live_sessions::Entity::find_by_id(session.id)
            .one(&self.db)
            .await
            .map_err(repository_error)?
        else {
            return Err(DomainError::Repository(
                "LIVE session disappeared before update".to_owned(),
            ));
        };

        let mut active_model: live_sessions::ActiveModel = model.into();
        active_model.status = Set(status_to_db(session.status));
        active_model.ended_at = Set(session.ended_at.map(to_db_datetime));
        active_model
            .update(&self.db)
            .await
            .map(|_| ())
            .map_err(repository_error)
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<LiveSession>, DomainError> {
        live_sessions::Entity::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(repository_error)?
            .map(session_from_model)
            .transpose()
    }

    async fn find_ongoing_by_seller(
        &self,
        seller_profile_id: Uuid,
    ) -> Result<Option<LiveSession>, DomainError> {
        live_sessions::Entity::find()
            .filter(live_sessions::Column::SellerProfileId.eq(seller_profile_id))
            .filter(live_sessions::Column::Status.eq(live_sessions::LiveSessionStatus::Ongoing))
            .order_by_desc(live_sessions::Column::CreatedAt)
            .one(&self.db)
            .await
            .map_err(repository_error)?
            .map(session_from_model)
            .transpose()
    }
}

fn session_from_model(model: live_sessions::Model) -> Result<LiveSession, DomainError> {
    Ok(LiveSession {
        id: model.id,
        seller_profile_id: model.seller_profile_id,
        tiktok_url: TikTokLiveUrl::new(model.tiktok_url).map_err(|error| {
            DomainError::Repository(format!(
                "invalid TikTok LIVE URL stored in database: {error}"
            ))
        })?,
        status: status_from_db(model.status),
        created_at: from_db_datetime(model.created_at),
        ended_at: model.ended_at.map(from_db_datetime),
    })
}

fn status_to_db(status: LiveSessionStatus) -> live_sessions::LiveSessionStatus {
    match status {
        LiveSessionStatus::Ongoing => live_sessions::LiveSessionStatus::Ongoing,
        LiveSessionStatus::Ended => live_sessions::LiveSessionStatus::Ended,
    }
}

fn status_from_db(status: live_sessions::LiveSessionStatus) -> LiveSessionStatus {
    match status {
        live_sessions::LiveSessionStatus::Ongoing => LiveSessionStatus::Ongoing,
        live_sessions::LiveSessionStatus::Ended => LiveSessionStatus::Ended,
    }
}

fn to_db_datetime(value: DateTime<Utc>) -> DateTime<FixedOffset> {
    value.fixed_offset()
}

fn from_db_datetime(value: DateTime<FixedOffset>) -> DateTime<Utc> {
    value.with_timezone(&Utc)
}

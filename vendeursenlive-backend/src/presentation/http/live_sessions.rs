use std::sync::Arc;

use actix_web::{http::StatusCode, web, HttpResponse, ResponseError};
use serde::Serialize;
use uuid::Uuid;

use crate::{
    application::{
        dtos::live::StartLiveSessionRequest,
        errors::ApplicationError,
        use_cases::live::{
            EndLiveSessionUseCase, GetCurrentLiveSessionUseCase, StartLiveSessionUseCase,
        },
    },
    infrastructure::{
        database::repositories::{
            auth::SeaOrmSellerProfileRepository, live::SeaOrmLiveSessionRepository,
        },
        tiktok::live_link::ReqwestTikTokLiveLinkResolver,
    },
    presentation::extractors::authenticated_user::VerifiedSeller,
    AppState,
};

pub fn configure(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("/seller/lives")
            .route("", web::post().to(start_live))
            .route("/current", web::get().to(current_live))
            .route("/{live_session_id}/end", web::post().to(end_live)),
    );
}

async fn start_live(
    state: web::Data<AppState>,
    seller: VerifiedSeller,
    payload: web::Json<StartLiveSessionRequest>,
) -> Result<HttpResponse, LiveHttpError> {
    let resolver = ReqwestTikTokLiveLinkResolver::new()?;
    let use_case = StartLiveSessionUseCase::new(
        Arc::new(SeaOrmSellerProfileRepository::new(state.db.clone())),
        Arc::new(SeaOrmLiveSessionRepository::new(state.db.clone())),
        Arc::new(resolver),
    );
    let response = use_case
        .execute(seller.0.user_id, payload.into_inner())
        .await?;

    Ok(HttpResponse::Created().json(response))
}

async fn current_live(
    state: web::Data<AppState>,
    seller: VerifiedSeller,
) -> Result<HttpResponse, LiveHttpError> {
    let use_case = GetCurrentLiveSessionUseCase::new(
        Arc::new(SeaOrmSellerProfileRepository::new(state.db.clone())),
        Arc::new(SeaOrmLiveSessionRepository::new(state.db.clone())),
    );

    match use_case.execute(seller.0.user_id).await? {
        Some(response) => Ok(HttpResponse::Ok().json(response)),
        None => Ok(HttpResponse::NoContent().finish()),
    }
}

async fn end_live(
    state: web::Data<AppState>,
    seller: VerifiedSeller,
    live_session_id: web::Path<Uuid>,
) -> Result<HttpResponse, LiveHttpError> {
    let use_case = EndLiveSessionUseCase::new(
        Arc::new(SeaOrmSellerProfileRepository::new(state.db.clone())),
        Arc::new(SeaOrmLiveSessionRepository::new(state.db.clone())),
    );
    let response = use_case
        .execute(seller.0.user_id, live_session_id.into_inner())
        .await?;

    Ok(HttpResponse::Ok().json(response))
}

#[derive(Debug)]
struct LiveHttpError(ApplicationError);

#[derive(Debug, Serialize)]
struct ErrorResponse {
    error: String,
}

impl From<ApplicationError> for LiveHttpError {
    fn from(error: ApplicationError) -> Self {
        Self(error)
    }
}

impl std::fmt::Display for LiveHttpError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}", self.0)
    }
}

impl ResponseError for LiveHttpError {
    fn status_code(&self) -> StatusCode {
        match self.0 {
            ApplicationError::Validation(_) => StatusCode::BAD_REQUEST,
            ApplicationError::InvalidCredentials | ApplicationError::Unauthorized => {
                StatusCode::UNAUTHORIZED
            }
            ApplicationError::Forbidden(_) => StatusCode::FORBIDDEN,
            ApplicationError::NotFound(_) => StatusCode::NOT_FOUND,
            ApplicationError::Conflict(_) => StatusCode::CONFLICT,
            ApplicationError::TooManyRequests(_) => StatusCode::TOO_MANY_REQUESTS,
            ApplicationError::ServiceUnavailable(_) => StatusCode::SERVICE_UNAVAILABLE,
            ApplicationError::Infrastructure(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(ErrorResponse {
            error: self.0.to_string(),
        })
    }
}

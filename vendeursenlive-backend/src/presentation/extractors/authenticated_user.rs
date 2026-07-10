use std::future::{ready, Ready};

use actix_web::{dev::Payload, error::ErrorUnauthorized, web, Error, FromRequest, HttpRequest};
use uuid::Uuid;

use crate::{presentation::http::auth::ACCESS_TOKEN_COOKIE, AppState};

#[derive(Debug, Clone)]
pub struct AuthenticatedUser {
    pub user_id: Uuid,
    pub session_id: Uuid,
    pub is_admin: bool,
    pub is_seller: bool,
}

impl FromRequest for AuthenticatedUser {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(request: &HttpRequest, _: &mut Payload) -> Self::Future {
        let Some(state) = request.app_data::<web::Data<AppState>>() else {
            return ready(Err(ErrorUnauthorized("missing application state")));
        };

        let Some(access_token) = access_token_from_request(request) else {
            return ready(Err(ErrorUnauthorized("missing access token")));
        };

        let result = state
            .jwt
            .verify_access_token(&access_token)
            .map(|claims| Self {
                user_id: claims.sub,
                session_id: claims.sid,
                is_admin: claims.is_admin,
                is_seller: claims.is_seller,
            })
            .map_err(|_| ErrorUnauthorized("invalid access token"));

        ready(result)
    }
}

fn access_token_from_request(request: &HttpRequest) -> Option<String> {
    request
        .cookie(ACCESS_TOKEN_COOKIE)
        .map(|cookie| cookie.value().to_owned())
        .or_else(|| bearer_token_from_request(request))
}

fn bearer_token_from_request(request: &HttpRequest) -> Option<String> {
    request
        .headers()
        .get(actix_web::http::header::AUTHORIZATION)?
        .to_str()
        .ok()?
        .strip_prefix("Bearer ")
        .map(str::to_owned)
}

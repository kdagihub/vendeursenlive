use std::future::{ready, Ready};

use actix_web::{
    dev::Payload,
    error::{ErrorForbidden, ErrorUnauthorized},
    web, Error, FromRequest, HttpRequest,
};
use uuid::Uuid;

use crate::{presentation::http::auth::ACCESS_TOKEN_COOKIE, AppState};

#[derive(Debug, Clone)]
pub struct AuthenticatedUser {
    pub user_id: Uuid,
    pub session_id: Uuid,
    pub is_admin: bool,
    pub is_seller: bool,
    pub account_verified: bool,
    pub verification_channel: Option<String>,
}

#[derive(Debug, Clone)]
pub struct VerifiedUser(pub AuthenticatedUser);

#[derive(Debug, Clone)]
pub struct VerifiedSeller(pub AuthenticatedUser);

impl FromRequest for VerifiedSeller {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(request: &HttpRequest, _: &mut Payload) -> Self::Future {
        ready(match authenticated_user_from_request(request) {
            Ok(user) if !user.account_verified => {
                Err(ErrorForbidden("account verification required"))
            }
            Ok(user) if !user.is_seller => Err(ErrorForbidden("seller account required")),
            Ok(user) => Ok(Self(user)),
            Err(error) => Err(error),
        })
    }
}

impl FromRequest for VerifiedUser {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(request: &HttpRequest, _: &mut Payload) -> Self::Future {
        ready(match authenticated_user_from_request(request) {
            Ok(user) if user.account_verified => Ok(Self(user)),
            Ok(_) => Err(ErrorForbidden("account verification required")),
            Err(error) => Err(error),
        })
    }
}

impl FromRequest for AuthenticatedUser {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(request: &HttpRequest, _: &mut Payload) -> Self::Future {
        ready(authenticated_user_from_request(request))
    }
}

fn authenticated_user_from_request(request: &HttpRequest) -> Result<AuthenticatedUser, Error> {
    let state = request
        .app_data::<web::Data<AppState>>()
        .ok_or_else(|| ErrorUnauthorized("missing application state"))?;
    let access_token = access_token_from_request(request)
        .ok_or_else(|| ErrorUnauthorized("missing access token"))?;

    state
        .jwt
        .verify_access_token(&access_token)
        .map(|claims| AuthenticatedUser {
            user_id: claims.sub,
            session_id: claims.sid,
            is_admin: claims.is_admin,
            is_seller: claims.is_seller,
            account_verified: claims.account_verified,
            verification_channel: claims.verification_channel,
        })
        .map_err(|_| ErrorUnauthorized("invalid access token"))
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

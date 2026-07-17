use std::sync::Arc;

use actix_web::{
    cookie::{time::Duration as CookieDuration, Cookie, SameSite},
    http::{header::LOCATION, StatusCode},
    web, HttpRequest, HttpResponse, ResponseError,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    application::{
        dtos::auth::{
            AuthResponse, ChangePasswordRequest, ConfirmEmailVerificationRequest,
            ConfirmPasswordResetRequest, LoginRequest, PasswordResetRequest, RefreshSessionRequest,
            RegisterRequest, RequestPasswordResetCommand,
        },
        errors::ApplicationError,
        use_cases::auth::{
            ChangePasswordUseCase, ConfirmEmailVerificationUseCase, ConfirmPasswordResetUseCase,
            LoginUserUseCase, LogoutUseCase, RefreshSessionUseCase, RegisterUserUseCase,
            RequestEmailVerificationUseCase, RequestPasswordResetUseCase, TikTokLoginUseCase,
        },
    },
    infrastructure::{
        auth::{password::Argon2PasswordHasher, refresh_token::UuidRefreshTokenService},
        database::repositories::auth::{
            SeaOrmAuthSessionRepository, SeaOrmCustomerProfileRepository,
            SeaOrmEmailVerificationTokenRepository, SeaOrmPasswordResetTokenRepository,
            SeaOrmSellerProfileRepository, SeaOrmUserAuthIdentityRepository, SeaOrmUserRepository,
        },
        email::SmtpAuthEmailSender,
        oauth::ReqwestTikTokOAuthClient,
    },
    presentation::extractors::authenticated_user::AuthenticatedUser,
    AppState,
};

pub(crate) const ACCESS_TOKEN_COOKIE: &str = "vel_access_token";
pub(crate) const CSRF_TOKEN_COOKIE: &str = "vel_csrf_token";
const REFRESH_TOKEN_COOKIE: &str = "vel_refresh_token";
const REFRESH_SESSION_COOKIE: &str = "vel_refresh_session";
const TIKTOK_STATE_COOKIE: &str = "vel_tiktok_oauth_state";

pub fn configure(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("/auth")
            .route("/register", web::post().to(register))
            .route("/login", web::post().to(login))
            .route("/csrf", web::get().to(csrf))
            .route("/refresh", web::post().to(refresh))
            .route("/logout", web::post().to(logout))
            .route("/change-password", web::post().to(change_password))
            .route(
                "/password-reset/request",
                web::post().to(request_password_reset),
            )
            .route(
                "/password-reset/confirm",
                web::post().to(confirm_password_reset),
            )
            .route(
                "/email-verification/request",
                web::post().to(request_email_verification),
            )
            .route(
                "/email-verification/confirm",
                web::post().to(confirm_email_verification),
            )
            .route("/tiktok/start", web::get().to(tiktok_start))
            .route("/tiktok/callback", web::get().to(tiktok_callback))
            .route("/me", web::get().to(me)),
    );
}

async fn csrf(state: web::Data<AppState>) -> HttpResponse {
    let csrf_token = new_csrf_token();

    HttpResponse::NoContent()
        .cookie(csrf_cookie(
            csrf_token,
            state.auth_config.cookie_secure,
            state.auth_config.cookie_domain.clone(),
        ))
        .finish()
}

async fn tiktok_start(state: web::Data<AppState>) -> Result<HttpResponse, AuthHttpError> {
    let client_key = state.tiktok_config.client_key.as_deref().ok_or_else(|| {
        ApplicationError::Infrastructure("TikTok client key is not configured".to_owned())
    })?;
    let redirect_uri = state.tiktok_config.redirect_uri.as_deref().ok_or_else(|| {
        ApplicationError::Infrastructure("TikTok redirect URI is not configured".to_owned())
    })?;
    let oauth_state = Uuid::new_v4().to_string();
    let authorization_url = build_tiktok_authorization_url(
        &state.tiktok_config.auth_url,
        client_key,
        redirect_uri,
        &state.tiktok_config.scopes,
        &oauth_state,
    );

    Ok(HttpResponse::Found()
        .insert_header((LOCATION, authorization_url))
        .cookie(oauth_state_cookie(
            oauth_state,
            state.auth_config.cookie_secure,
            state.auth_config.cookie_domain.clone(),
        ))
        .finish())
}

#[derive(Debug, Deserialize)]
struct TikTokCallbackQuery {
    code: Option<String>,
    state: Option<String>,
    error: Option<String>,
    error_description: Option<String>,
}

async fn tiktok_callback(
    state: web::Data<AppState>,
    request: HttpRequest,
    query: web::Query<TikTokCallbackQuery>,
) -> Result<HttpResponse, AuthHttpError> {
    if let Some(error) = &query.error {
        return Err(ApplicationError::Validation(format!(
            "TikTok authorization failed: {}",
            query.error_description.as_deref().unwrap_or(error.as_str())
        ))
        .into());
    }

    let expected_state = request
        .cookie(TIKTOK_STATE_COOKIE)
        .ok_or(ApplicationError::Unauthorized)?
        .value()
        .to_owned();
    let returned_state = query
        .state
        .as_deref()
        .ok_or(ApplicationError::Unauthorized)?;

    if expected_state != returned_state {
        return Err(ApplicationError::Unauthorized.into());
    }

    let authorization_code = query.code.as_deref().ok_or_else(|| {
        ApplicationError::Validation("TikTok authorization code is missing".to_owned())
    })?;
    let use_case = TikTokLoginUseCase::new(
        Arc::new(SeaOrmUserRepository::new(state.db.clone())),
        Arc::new(SeaOrmUserAuthIdentityRepository::new(state.db.clone())),
        Arc::new(SeaOrmAuthSessionRepository::new(state.db.clone())),
        Arc::new(SeaOrmCustomerProfileRepository::new(state.db.clone())),
        Arc::new(SeaOrmSellerProfileRepository::new(state.db.clone())),
        Arc::new(ReqwestTikTokOAuthClient::new(state.tiktok_config.clone())),
        Arc::new(UuidRefreshTokenService),
        Arc::new(state.jwt.clone()),
        state.auth_config.access_token_ttl_seconds,
        state.auth_config.refresh_token_ttl_seconds,
    );
    let response = use_case.execute(authorization_code).await?;

    Ok(auth_cookie_redirect_response(
        HttpResponse::Found(),
        &state,
        response,
        &state.tiktok_config.success_redirect_url,
    )
    .cookie(clear_cookie(
        TIKTOK_STATE_COOKIE,
        state.auth_config.cookie_secure,
        state.auth_config.cookie_domain.clone(),
    ))
    .finish())
}

async fn register(
    state: web::Data<AppState>,
    payload: web::Json<RegisterRequest>,
) -> Result<HttpResponse, AuthHttpError> {
    let use_case = RegisterUserUseCase::new(
        Arc::new(SeaOrmUserRepository::new(state.db.clone())),
        Arc::new(SeaOrmUserAuthIdentityRepository::new(state.db.clone())),
        Arc::new(SeaOrmAuthSessionRepository::new(state.db.clone())),
        Arc::new(SeaOrmCustomerProfileRepository::new(state.db.clone())),
        Arc::new(SeaOrmSellerProfileRepository::new(state.db.clone())),
        Arc::new(Argon2PasswordHasher),
        Arc::new(UuidRefreshTokenService),
        Arc::new(state.jwt.clone()),
        state.auth_config.access_token_ttl_seconds,
        state.auth_config.refresh_token_ttl_seconds,
        Arc::new(SeaOrmEmailVerificationTokenRepository::new(
            state.db.clone(),
        )),
        Arc::new(SmtpAuthEmailSender::new(state.email_config.clone())),
        Arc::new(UuidRefreshTokenService),
        state.auth_config.email_verification_token_ttl_seconds,
        state.email_config.email_verification_url.clone(),
    );

    let response = use_case.execute(payload.into_inner()).await?;

    Ok(auth_cookie_response(
        HttpResponse::Created(),
        &state,
        response,
    ))
}

async fn login(
    state: web::Data<AppState>,
    payload: web::Json<LoginRequest>,
) -> Result<HttpResponse, AuthHttpError> {
    let use_case = LoginUserUseCase::new(
        Arc::new(SeaOrmUserRepository::new(state.db.clone())),
        Arc::new(SeaOrmUserAuthIdentityRepository::new(state.db.clone())),
        Arc::new(SeaOrmAuthSessionRepository::new(state.db.clone())),
        Arc::new(SeaOrmSellerProfileRepository::new(state.db.clone())),
        Arc::new(Argon2PasswordHasher),
        Arc::new(UuidRefreshTokenService),
        Arc::new(state.jwt.clone()),
        state.auth_config.access_token_ttl_seconds,
        state.auth_config.refresh_token_ttl_seconds,
    );

    let response = use_case.execute(payload.into_inner()).await?;

    Ok(auth_cookie_response(HttpResponse::Ok(), &state, response))
}

async fn logout(
    state: web::Data<AppState>,
    user: AuthenticatedUser,
) -> Result<HttpResponse, AuthHttpError> {
    let use_case = LogoutUseCase::new(Arc::new(SeaOrmAuthSessionRepository::new(state.db.clone())));

    use_case.execute(user.session_id).await?;

    Ok(clear_auth_cookie_response(
        HttpResponse::NoContent(),
        &state,
    ))
}

async fn change_password(
    state: web::Data<AppState>,
    user: AuthenticatedUser,
    payload: web::Json<ChangePasswordRequest>,
) -> Result<HttpResponse, AuthHttpError> {
    let use_case = ChangePasswordUseCase::new(
        Arc::new(SeaOrmUserAuthIdentityRepository::new(state.db.clone())),
        Arc::new(Argon2PasswordHasher),
    );

    use_case.execute(user.user_id, payload.into_inner()).await?;

    Ok(HttpResponse::NoContent().finish())
}

async fn request_password_reset(
    state: web::Data<AppState>,
    payload: web::Json<PasswordResetRequest>,
) -> Result<HttpResponse, AuthHttpError> {
    let use_case = RequestPasswordResetUseCase::new(
        Arc::new(SeaOrmUserAuthIdentityRepository::new(state.db.clone())),
        Arc::new(SeaOrmPasswordResetTokenRepository::new(state.db.clone())),
        Arc::new(UuidRefreshTokenService),
        Arc::new(SmtpAuthEmailSender::new(state.email_config.clone())),
        state.auth_config.password_reset_token_ttl_seconds,
        state.email_config.password_reset_url.clone(),
    );

    use_case
        .execute(RequestPasswordResetCommand {
            email: payload.email.clone(),
        })
        .await?;

    Ok(
        HttpResponse::Accepted().json(PasswordResetAcceptedResponse {
            message: "if the account exists, password reset instructions will be sent".to_owned(),
        }),
    )
}

async fn confirm_password_reset(
    state: web::Data<AppState>,
    payload: web::Json<ConfirmPasswordResetRequest>,
) -> Result<HttpResponse, AuthHttpError> {
    let use_case = ConfirmPasswordResetUseCase::new(
        Arc::new(SeaOrmUserAuthIdentityRepository::new(state.db.clone())),
        Arc::new(SeaOrmPasswordResetTokenRepository::new(state.db.clone())),
        Arc::new(SeaOrmAuthSessionRepository::new(state.db.clone())),
        Arc::new(Argon2PasswordHasher),
        Arc::new(UuidRefreshTokenService),
    );

    use_case.execute(payload.into_inner()).await?;

    Ok(clear_auth_cookie_response(
        HttpResponse::NoContent(),
        &state,
    ))
}

async fn request_email_verification(
    state: web::Data<AppState>,
    user: AuthenticatedUser,
) -> Result<HttpResponse, AuthHttpError> {
    let use_case = RequestEmailVerificationUseCase::new(
        Arc::new(SeaOrmUserAuthIdentityRepository::new(state.db.clone())),
        Arc::new(SeaOrmEmailVerificationTokenRepository::new(
            state.db.clone(),
        )),
        Arc::new(UuidRefreshTokenService),
        Arc::new(SmtpAuthEmailSender::new(state.email_config.clone())),
        state.auth_config.email_verification_token_ttl_seconds,
        state.auth_config.email_verification_resend_cooldown_seconds,
        state.email_config.email_verification_url.clone(),
    );

    use_case.execute(user.user_id).await?;

    Ok(HttpResponse::Accepted().json(MessageResponse {
        message: "email verification instructions were sent".to_owned(),
    }))
}

async fn confirm_email_verification(
    state: web::Data<AppState>,
    payload: web::Json<ConfirmEmailVerificationRequest>,
) -> Result<HttpResponse, AuthHttpError> {
    let use_case = ConfirmEmailVerificationUseCase::new(
        Arc::new(SeaOrmUserAuthIdentityRepository::new(state.db.clone())),
        Arc::new(SeaOrmEmailVerificationTokenRepository::new(
            state.db.clone(),
        )),
        Arc::new(UuidRefreshTokenService),
    );

    use_case.execute(payload.into_inner()).await?;
    Ok(HttpResponse::NoContent().finish())
}

async fn me(user: AuthenticatedUser) -> HttpResponse {
    HttpResponse::Ok().json(CurrentUserResponse {
        user_id: user.user_id,
        session_id: user.session_id,
        is_seller: user.is_seller,
        is_admin: user.is_admin,
        account_verified: user.account_verified,
        verification_channel: user.verification_channel,
    })
}

async fn refresh(
    state: web::Data<AppState>,
    request: HttpRequest,
) -> Result<HttpResponse, AuthHttpError> {
    let session_id = request
        .cookie(REFRESH_SESSION_COOKIE)
        .ok_or(ApplicationError::Unauthorized)
        .and_then(|cookie| {
            Uuid::parse_str(cookie.value()).map_err(|_| ApplicationError::Unauthorized)
        })?;
    let refresh_token = request
        .cookie(REFRESH_TOKEN_COOKIE)
        .ok_or(ApplicationError::Unauthorized)?
        .value()
        .to_owned();

    let use_case = RefreshSessionUseCase::new(
        Arc::new(SeaOrmUserRepository::new(state.db.clone())),
        Arc::new(SeaOrmUserAuthIdentityRepository::new(state.db.clone())),
        Arc::new(SeaOrmAuthSessionRepository::new(state.db.clone())),
        Arc::new(SeaOrmSellerProfileRepository::new(state.db.clone())),
        Arc::new(UuidRefreshTokenService),
        Arc::new(state.jwt.clone()),
        state.auth_config.access_token_ttl_seconds,
        state.auth_config.refresh_token_ttl_seconds,
    );

    let response = use_case
        .execute(RefreshSessionRequest {
            session_id,
            refresh_token,
        })
        .await?;

    Ok(auth_cookie_response(HttpResponse::Ok(), &state, response))
}

#[derive(Debug)]
pub struct AuthHttpError(ApplicationError);

#[derive(Debug, Serialize)]
struct ErrorResponse {
    error: String,
}

#[derive(Debug, Serialize)]
struct AuthSessionResponse {
    user_id: Uuid,
    session_id: Uuid,
    token_type: &'static str,
    expires_in_seconds: i64,
    is_seller: bool,
    is_admin: bool,
    account_verified: bool,
    verification_channel: Option<&'static str>,
}

#[derive(Debug, Serialize)]
struct CurrentUserResponse {
    user_id: Uuid,
    session_id: Uuid,
    is_seller: bool,
    is_admin: bool,
    account_verified: bool,
    verification_channel: Option<String>,
}

#[derive(Debug, Serialize)]
struct PasswordResetAcceptedResponse {
    message: String,
}

#[derive(Debug, Serialize)]
struct MessageResponse {
    message: String,
}

fn auth_cookie_response(
    mut builder: actix_web::HttpResponseBuilder,
    state: &web::Data<AppState>,
    response: AuthResponse,
) -> HttpResponse {
    let access_cookie = auth_cookie(
        ACCESS_TOKEN_COOKIE,
        response.access_token,
        state.auth_config.access_token_ttl_seconds,
        state.auth_config.cookie_secure,
        state.auth_config.cookie_domain.clone(),
    );
    let refresh_cookie = auth_cookie(
        REFRESH_TOKEN_COOKIE,
        response.refresh_token,
        state.auth_config.refresh_token_ttl_seconds,
        state.auth_config.cookie_secure,
        state.auth_config.cookie_domain.clone(),
    );
    let refresh_session_cookie = auth_cookie(
        REFRESH_SESSION_COOKIE,
        response.session_id.to_string(),
        state.auth_config.refresh_token_ttl_seconds,
        state.auth_config.cookie_secure,
        state.auth_config.cookie_domain.clone(),
    );
    let csrf_cookie = csrf_cookie(
        new_csrf_token(),
        state.auth_config.cookie_secure,
        state.auth_config.cookie_domain.clone(),
    );

    builder
        .cookie(access_cookie)
        .cookie(refresh_cookie)
        .cookie(refresh_session_cookie)
        .cookie(csrf_cookie)
        .json(AuthSessionResponse {
            user_id: response.user_id,
            session_id: response.session_id,
            token_type: response.token_type,
            expires_in_seconds: response.expires_in_seconds,
            is_seller: response.is_seller,
            is_admin: response.is_admin,
            account_verified: response.account_verified,
            verification_channel: response.verification_channel,
        })
}

fn auth_cookie_redirect_response(
    mut builder: actix_web::HttpResponseBuilder,
    state: &web::Data<AppState>,
    response: AuthResponse,
    redirect_url: &str,
) -> actix_web::HttpResponseBuilder {
    let access_cookie = auth_cookie(
        ACCESS_TOKEN_COOKIE,
        response.access_token,
        state.auth_config.access_token_ttl_seconds,
        state.auth_config.cookie_secure,
        state.auth_config.cookie_domain.clone(),
    );
    let refresh_cookie = auth_cookie(
        REFRESH_TOKEN_COOKIE,
        response.refresh_token,
        state.auth_config.refresh_token_ttl_seconds,
        state.auth_config.cookie_secure,
        state.auth_config.cookie_domain.clone(),
    );
    let refresh_session_cookie = auth_cookie(
        REFRESH_SESSION_COOKIE,
        response.session_id.to_string(),
        state.auth_config.refresh_token_ttl_seconds,
        state.auth_config.cookie_secure,
        state.auth_config.cookie_domain.clone(),
    );
    let csrf_cookie = csrf_cookie(
        new_csrf_token(),
        state.auth_config.cookie_secure,
        state.auth_config.cookie_domain.clone(),
    );

    builder
        .insert_header((LOCATION, redirect_url))
        .cookie(access_cookie)
        .cookie(refresh_cookie)
        .cookie(refresh_session_cookie)
        .cookie(csrf_cookie);

    builder
}

fn csrf_cookie(value: String, secure: bool, domain: Option<String>) -> Cookie<'static> {
    let mut builder = Cookie::build(CSRF_TOKEN_COOKIE, value)
        .path("/")
        .http_only(false)
        .secure(secure)
        .same_site(SameSite::Lax)
        .max_age(CookieDuration::days(30));

    if let Some(domain) = domain {
        builder = builder.domain(domain);
    }

    builder.finish()
}

fn new_csrf_token() -> String {
    Uuid::new_v4().to_string()
}

fn oauth_state_cookie(value: String, secure: bool, domain: Option<String>) -> Cookie<'static> {
    let mut builder = Cookie::build(TIKTOK_STATE_COOKIE, value)
        .path("/")
        .http_only(true)
        .secure(secure)
        .same_site(SameSite::Lax)
        .max_age(CookieDuration::minutes(10));

    if let Some(domain) = domain {
        builder = builder.domain(domain);
    }

    builder.finish()
}

fn build_tiktok_authorization_url(
    auth_url: &str,
    client_key: &str,
    redirect_uri: &str,
    scopes: &[String],
    state: &str,
) -> String {
    let separator = if auth_url.contains('?') { '&' } else { '?' };
    format!(
        "{auth_url}{separator}client_key={}&response_type=code&scope={}&redirect_uri={}&state={}",
        url_encode(client_key),
        url_encode(&scopes.join(",")),
        url_encode(redirect_uri),
        url_encode(state)
    )
}

fn url_encode(value: &str) -> String {
    value
        .bytes()
        .flat_map(|byte| match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                vec![byte as char]
            }
            _ => format!("%{byte:02X}").chars().collect(),
        })
        .collect()
}

fn auth_cookie(
    name: &'static str,
    value: String,
    ttl_seconds: i64,
    secure: bool,
    domain: Option<String>,
) -> Cookie<'static> {
    let mut builder = Cookie::build(name, value)
        .path("/")
        .http_only(true)
        .secure(secure)
        .same_site(SameSite::Lax)
        .max_age(CookieDuration::seconds(ttl_seconds));

    if let Some(domain) = domain {
        builder = builder.domain(domain);
    }

    builder.finish()
}

fn clear_auth_cookie_response(
    mut builder: actix_web::HttpResponseBuilder,
    state: &web::Data<AppState>,
) -> HttpResponse {
    builder
        .cookie(clear_cookie(
            ACCESS_TOKEN_COOKIE,
            state.auth_config.cookie_secure,
            state.auth_config.cookie_domain.clone(),
        ))
        .cookie(clear_cookie(
            REFRESH_TOKEN_COOKIE,
            state.auth_config.cookie_secure,
            state.auth_config.cookie_domain.clone(),
        ))
        .cookie(clear_cookie(
            REFRESH_SESSION_COOKIE,
            state.auth_config.cookie_secure,
            state.auth_config.cookie_domain.clone(),
        ))
        .cookie(clear_cookie(
            CSRF_TOKEN_COOKIE,
            state.auth_config.cookie_secure,
            state.auth_config.cookie_domain.clone(),
        ))
        .finish()
}

fn clear_cookie(name: &'static str, secure: bool, domain: Option<String>) -> Cookie<'static> {
    auth_cookie(name, String::new(), 0, secure, domain)
}

impl From<ApplicationError> for AuthHttpError {
    fn from(error: ApplicationError) -> Self {
        Self(error)
    }
}

impl std::fmt::Display for AuthHttpError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}", self.0)
    }
}

impl ResponseError for AuthHttpError {
    fn status_code(&self) -> StatusCode {
        match self.0 {
            ApplicationError::Validation(_) => StatusCode::BAD_REQUEST,
            ApplicationError::InvalidCredentials => StatusCode::UNAUTHORIZED,
            ApplicationError::Conflict(_) => StatusCode::CONFLICT,
            ApplicationError::Unauthorized => StatusCode::UNAUTHORIZED,
            ApplicationError::Infrastructure(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(ErrorResponse {
            error: self.0.to_string(),
        })
    }
}

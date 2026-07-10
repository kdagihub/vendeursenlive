use std::sync::Arc;

use chrono::{Duration, Utc};

use crate::domain::entities::PasswordResetToken;
use crate::{
    application::{
        dtos::auth::{
            AccountType, AuthResponse, ChangePasswordRequest, ConfirmPasswordResetRequest,
            LoginRequest, PasswordResetRequestResponse, RefreshSessionRequest, RegisterRequest,
            RequestPasswordResetCommand,
        },
        errors::ApplicationError,
        ports::auth::{AccessTokenIssuer, PasswordHasher, RefreshTokenService},
    },
    domain::{
        entities::{
            AuthSession, CustomerProfile, SellerProfile, User, UserAuthIdentity, UserStatus,
        },
        repositories::{
            AuthSessionRepository, CustomerProfileRepository, PasswordResetTokenRepository,
            SellerProfileRepository, UserAuthIdentityRepository, UserRepository,
        },
        value_objects::{EmailAddress, PhoneNumber},
    },
};

pub struct RegisterUserUseCase {
    user_repository: Arc<dyn UserRepository>,
    identity_repository: Arc<dyn UserAuthIdentityRepository>,
    auth_session_repository: Arc<dyn AuthSessionRepository>,
    customer_profile_repository: Arc<dyn CustomerProfileRepository>,
    seller_profile_repository: Arc<dyn SellerProfileRepository>,
    password_hasher: Arc<dyn PasswordHasher>,
    refresh_tokens: Arc<dyn RefreshTokenService>,
    access_tokens: Arc<dyn AccessTokenIssuer>,
    access_token_ttl_seconds: i64,
    refresh_token_ttl_seconds: i64,
}

impl RegisterUserUseCase {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        user_repository: Arc<dyn UserRepository>,
        identity_repository: Arc<dyn UserAuthIdentityRepository>,
        auth_session_repository: Arc<dyn AuthSessionRepository>,
        customer_profile_repository: Arc<dyn CustomerProfileRepository>,
        seller_profile_repository: Arc<dyn SellerProfileRepository>,
        password_hasher: Arc<dyn PasswordHasher>,
        refresh_tokens: Arc<dyn RefreshTokenService>,
        access_tokens: Arc<dyn AccessTokenIssuer>,
        access_token_ttl_seconds: i64,
        refresh_token_ttl_seconds: i64,
    ) -> Self {
        Self {
            user_repository,
            identity_repository,
            auth_session_repository,
            customer_profile_repository,
            seller_profile_repository,
            password_hasher,
            refresh_tokens,
            access_tokens,
            access_token_ttl_seconds,
            refresh_token_ttl_seconds,
        }
    }

    pub async fn execute(
        &self,
        request: RegisterRequest,
    ) -> Result<AuthResponse, ApplicationError> {
        validate_password(&request.password)?;

        let password_hash = self.password_hasher.hash_password(&request.password)?;
        let user = User::new(clean_optional(request.full_name.clone()), None);
        let identity = self
            .build_identity(user.id, &request, password_hash)
            .await?;
        let is_seller = matches!(request.account_type, AccountType::Seller);

        self.user_repository.save(&user).await?;
        self.identity_repository.save(&identity).await?;

        match request.account_type {
            AccountType::Customer => {
                let profile =
                    CustomerProfile::new(user.id, clean_optional(request.default_location));
                self.customer_profile_repository.save(&profile).await?;
            }
            AccountType::Seller => {
                let shop_name = request.shop_name.and_then(clean_optional).ok_or_else(|| {
                    ApplicationError::Validation(
                        "shop_name is required for seller registration".to_owned(),
                    )
                })?;
                let profile = SellerProfile::start_trial(user.id, shop_name, request.payment_link);
                self.seller_profile_repository.save(&profile).await?;
            }
        }

        self.create_auth_response(user.id, user.is_admin, is_seller)
            .await
    }

    async fn build_identity(
        &self,
        user_id: uuid::Uuid,
        request: &RegisterRequest,
        password_hash: String,
    ) -> Result<UserAuthIdentity, ApplicationError> {
        match (&request.email, &request.phone_number) {
            (Some(_), Some(_)) => Err(ApplicationError::Validation(
                "choose either email or phone_number for this registration flow".to_owned(),
            )),
            (Some(email), None) => {
                let email = EmailAddress::new(email)?;

                if self
                    .identity_repository
                    .find_by_email(email.as_str())
                    .await?
                    .is_some()
                {
                    return Err(ApplicationError::Conflict(
                        "email is already registered".to_owned(),
                    ));
                }

                Ok(UserAuthIdentity::email(user_id, email, password_hash))
            }
            (None, Some(phone_number)) => {
                let phone_number = PhoneNumber::new(phone_number)?;

                if self
                    .identity_repository
                    .find_by_phone_number(phone_number.as_str())
                    .await?
                    .is_some()
                {
                    return Err(ApplicationError::Conflict(
                        "phone number is already registered".to_owned(),
                    ));
                }

                Ok(UserAuthIdentity::phone(
                    user_id,
                    phone_number,
                    password_hash,
                ))
            }
            (None, None) => Err(ApplicationError::Validation(
                "email or phone_number is required".to_owned(),
            )),
        }
    }

    async fn create_auth_response(
        &self,
        user_id: uuid::Uuid,
        is_admin: bool,
        is_seller: bool,
    ) -> Result<AuthResponse, ApplicationError> {
        let refresh_token = self.refresh_tokens.generate();
        let refresh_token_hash = self.refresh_tokens.hash(&refresh_token)?;
        let expires_at = Utc::now() + Duration::seconds(self.refresh_token_ttl_seconds);
        let session = AuthSession::new(user_id, refresh_token_hash, expires_at, None, None);
        let access_token = self
            .access_tokens
            .issue_access_token(user_id, session.id, is_admin, is_seller)?;

        self.auth_session_repository.save(&session).await?;

        Ok(AuthResponse {
            user_id,
            session_id: session.id,
            access_token,
            refresh_token,
            token_type: "Bearer",
            expires_in_seconds: self.access_token_ttl_seconds,
            is_seller,
            is_admin,
        })
    }
}

pub struct LoginUserUseCase {
    user_repository: Arc<dyn UserRepository>,
    identity_repository: Arc<dyn UserAuthIdentityRepository>,
    auth_session_repository: Arc<dyn AuthSessionRepository>,
    seller_profile_repository: Arc<dyn SellerProfileRepository>,
    password_hasher: Arc<dyn PasswordHasher>,
    refresh_tokens: Arc<dyn RefreshTokenService>,
    access_tokens: Arc<dyn AccessTokenIssuer>,
    access_token_ttl_seconds: i64,
    refresh_token_ttl_seconds: i64,
}

pub struct LogoutUseCase {
    auth_session_repository: Arc<dyn AuthSessionRepository>,
}

impl LogoutUseCase {
    pub fn new(auth_session_repository: Arc<dyn AuthSessionRepository>) -> Self {
        Self {
            auth_session_repository,
        }
    }

    pub async fn execute(&self, session_id: uuid::Uuid) -> Result<(), ApplicationError> {
        self.auth_session_repository.revoke(session_id).await?;
        Ok(())
    }
}

pub struct ChangePasswordUseCase {
    identity_repository: Arc<dyn UserAuthIdentityRepository>,
    password_hasher: Arc<dyn PasswordHasher>,
}

impl ChangePasswordUseCase {
    pub fn new(
        identity_repository: Arc<dyn UserAuthIdentityRepository>,
        password_hasher: Arc<dyn PasswordHasher>,
    ) -> Self {
        Self {
            identity_repository,
            password_hasher,
        }
    }

    pub async fn execute(
        &self,
        user_id: uuid::Uuid,
        request: ChangePasswordRequest,
    ) -> Result<(), ApplicationError> {
        validate_password(&request.new_password)?;
        let identity = self
            .identity_repository
            .find_local_by_user_id(user_id)
            .await?
            .ok_or_else(|| {
                ApplicationError::Validation(
                    "this account does not have an email or phone password identity".to_owned(),
                )
            })?;
        let current_hash = identity
            .password_hash
            .as_deref()
            .ok_or(ApplicationError::InvalidCredentials)?;

        if !self
            .password_hasher
            .verify_password(&request.current_password, current_hash)?
        {
            return Err(ApplicationError::InvalidCredentials);
        }

        let new_hash = self.password_hasher.hash_password(&request.new_password)?;
        self.identity_repository
            .update_password_hash(identity.id, new_hash)
            .await?;

        Ok(())
    }
}

pub struct RequestPasswordResetUseCase {
    identity_repository: Arc<dyn UserAuthIdentityRepository>,
    reset_token_repository: Arc<dyn PasswordResetTokenRepository>,
    reset_tokens: Arc<dyn RefreshTokenService>,
    token_ttl_seconds: i64,
}

impl RequestPasswordResetUseCase {
    pub fn new(
        identity_repository: Arc<dyn UserAuthIdentityRepository>,
        reset_token_repository: Arc<dyn PasswordResetTokenRepository>,
        reset_tokens: Arc<dyn RefreshTokenService>,
        token_ttl_seconds: i64,
    ) -> Self {
        Self {
            identity_repository,
            reset_token_repository,
            reset_tokens,
            token_ttl_seconds,
        }
    }

    pub async fn execute(
        &self,
        request: RequestPasswordResetCommand,
    ) -> Result<PasswordResetRequestResponse, ApplicationError> {
        let identifier = request.identifier.trim();
        let identity = if identifier.contains('@') {
            self.identity_repository.find_by_email(identifier).await?
        } else {
            self.identity_repository
                .find_by_phone_number(identifier)
                .await?
        };

        let Some(identity) = identity else {
            return Ok(PasswordResetRequestResponse { reset_token: None });
        };

        if identity.password_hash.is_none() {
            return Ok(PasswordResetRequestResponse { reset_token: None });
        }

        let reset_token = self.reset_tokens.generate();
        let reset_token_hash = self.reset_tokens.hash(&reset_token)?;
        let expires_at = Utc::now() + Duration::seconds(self.token_ttl_seconds);
        let token = PasswordResetToken::new(identity.id, reset_token_hash, expires_at);

        self.reset_token_repository.save(&token).await?;

        Ok(PasswordResetRequestResponse {
            reset_token: Some(reset_token),
        })
    }
}

pub struct ConfirmPasswordResetUseCase {
    identity_repository: Arc<dyn UserAuthIdentityRepository>,
    reset_token_repository: Arc<dyn PasswordResetTokenRepository>,
    auth_session_repository: Arc<dyn AuthSessionRepository>,
    password_hasher: Arc<dyn PasswordHasher>,
    reset_tokens: Arc<dyn RefreshTokenService>,
}

impl ConfirmPasswordResetUseCase {
    pub fn new(
        identity_repository: Arc<dyn UserAuthIdentityRepository>,
        reset_token_repository: Arc<dyn PasswordResetTokenRepository>,
        auth_session_repository: Arc<dyn AuthSessionRepository>,
        password_hasher: Arc<dyn PasswordHasher>,
        reset_tokens: Arc<dyn RefreshTokenService>,
    ) -> Self {
        Self {
            identity_repository,
            reset_token_repository,
            auth_session_repository,
            password_hasher,
            reset_tokens,
        }
    }

    pub async fn execute(
        &self,
        request: ConfirmPasswordResetRequest,
    ) -> Result<(), ApplicationError> {
        validate_password(&request.new_password)?;

        let active_tokens = self.reset_token_repository.find_active().await?;
        for token in active_tokens {
            if self
                .reset_tokens
                .verify(&request.reset_token, &token.token_hash)?
            {
                let new_hash = self.password_hasher.hash_password(&request.new_password)?;
                self.identity_repository
                    .update_password_hash(token.user_auth_identity_id, new_hash)
                    .await?;

                let identity = self
                    .identity_repository
                    .find_by_id(token.user_auth_identity_id)
                    .await?
                    .ok_or(ApplicationError::Unauthorized)?;

                self.reset_token_repository.mark_used(token.id).await?;
                self.auth_session_repository
                    .revoke_all_for_user(identity.user_id)
                    .await?;

                return Ok(());
            }
        }

        Err(ApplicationError::Unauthorized)
    }
}

pub struct RefreshSessionUseCase {
    user_repository: Arc<dyn UserRepository>,
    auth_session_repository: Arc<dyn AuthSessionRepository>,
    seller_profile_repository: Arc<dyn SellerProfileRepository>,
    refresh_tokens: Arc<dyn RefreshTokenService>,
    access_tokens: Arc<dyn AccessTokenIssuer>,
    access_token_ttl_seconds: i64,
    refresh_token_ttl_seconds: i64,
}

impl RefreshSessionUseCase {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        user_repository: Arc<dyn UserRepository>,
        auth_session_repository: Arc<dyn AuthSessionRepository>,
        seller_profile_repository: Arc<dyn SellerProfileRepository>,
        refresh_tokens: Arc<dyn RefreshTokenService>,
        access_tokens: Arc<dyn AccessTokenIssuer>,
        access_token_ttl_seconds: i64,
        refresh_token_ttl_seconds: i64,
    ) -> Self {
        Self {
            user_repository,
            auth_session_repository,
            seller_profile_repository,
            refresh_tokens,
            access_tokens,
            access_token_ttl_seconds,
            refresh_token_ttl_seconds,
        }
    }

    pub async fn execute(
        &self,
        request: RefreshSessionRequest,
    ) -> Result<AuthResponse, ApplicationError> {
        let existing_session = self
            .auth_session_repository
            .find_by_id(request.session_id)
            .await?
            .ok_or(ApplicationError::Unauthorized)?;

        if existing_session.revoked_at.is_some() || existing_session.expires_at <= Utc::now() {
            return Err(ApplicationError::Unauthorized);
        }

        if !self
            .refresh_tokens
            .verify(&request.refresh_token, &existing_session.refresh_token_hash)?
        {
            return Err(ApplicationError::Unauthorized);
        }

        let user = self
            .user_repository
            .find_by_id(existing_session.user_id)
            .await?
            .ok_or(ApplicationError::Unauthorized)?;

        if user.status != UserStatus::Active {
            return Err(ApplicationError::Unauthorized);
        }

        let is_seller = self
            .seller_profile_repository
            .find_by_user_id(user.id)
            .await?
            .is_some();

        self.auth_session_repository
            .revoke(existing_session.id)
            .await?;

        let refresh_token = self.refresh_tokens.generate();
        let refresh_token_hash = self.refresh_tokens.hash(&refresh_token)?;
        let expires_at = Utc::now() + Duration::seconds(self.refresh_token_ttl_seconds);
        let session = AuthSession::new(user.id, refresh_token_hash, expires_at, None, None);
        let access_token =
            self.access_tokens
                .issue_access_token(user.id, session.id, user.is_admin, is_seller)?;

        self.auth_session_repository.save(&session).await?;

        Ok(AuthResponse {
            user_id: user.id,
            session_id: session.id,
            access_token,
            refresh_token,
            token_type: "Bearer",
            expires_in_seconds: self.access_token_ttl_seconds,
            is_seller,
            is_admin: user.is_admin,
        })
    }
}

impl LoginUserUseCase {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        user_repository: Arc<dyn UserRepository>,
        identity_repository: Arc<dyn UserAuthIdentityRepository>,
        auth_session_repository: Arc<dyn AuthSessionRepository>,
        seller_profile_repository: Arc<dyn SellerProfileRepository>,
        password_hasher: Arc<dyn PasswordHasher>,
        refresh_tokens: Arc<dyn RefreshTokenService>,
        access_tokens: Arc<dyn AccessTokenIssuer>,
        access_token_ttl_seconds: i64,
        refresh_token_ttl_seconds: i64,
    ) -> Self {
        Self {
            user_repository,
            identity_repository,
            auth_session_repository,
            seller_profile_repository,
            password_hasher,
            refresh_tokens,
            access_tokens,
            access_token_ttl_seconds,
            refresh_token_ttl_seconds,
        }
    }

    pub async fn execute(&self, request: LoginRequest) -> Result<AuthResponse, ApplicationError> {
        let identifier = request.identifier.trim();
        let identity = if identifier.contains('@') {
            self.identity_repository.find_by_email(identifier).await?
        } else {
            self.identity_repository
                .find_by_phone_number(identifier)
                .await?
        }
        .ok_or(ApplicationError::InvalidCredentials)?;

        let password_hash = identity
            .password_hash
            .as_deref()
            .ok_or(ApplicationError::InvalidCredentials)?;

        if !self
            .password_hasher
            .verify_password(&request.password, password_hash)?
        {
            return Err(ApplicationError::InvalidCredentials);
        }

        let user = self
            .user_repository
            .find_by_id(identity.user_id)
            .await?
            .ok_or(ApplicationError::InvalidCredentials)?;

        if user.status != UserStatus::Active {
            return Err(ApplicationError::Unauthorized);
        }

        let is_seller = self
            .seller_profile_repository
            .find_by_user_id(user.id)
            .await?
            .is_some();

        let refresh_token = self.refresh_tokens.generate();
        let refresh_token_hash = self.refresh_tokens.hash(&refresh_token)?;
        let expires_at = Utc::now() + Duration::seconds(self.refresh_token_ttl_seconds);
        let session = AuthSession::new(user.id, refresh_token_hash, expires_at, None, None);
        let access_token =
            self.access_tokens
                .issue_access_token(user.id, session.id, user.is_admin, is_seller)?;

        self.auth_session_repository.save(&session).await?;

        Ok(AuthResponse {
            user_id: user.id,
            session_id: session.id,
            access_token,
            refresh_token,
            token_type: "Bearer",
            expires_in_seconds: self.access_token_ttl_seconds,
            is_seller,
            is_admin: user.is_admin,
        })
    }
}

fn validate_password(password: &str) -> Result<(), ApplicationError> {
    if password.len() < 8 {
        return Err(ApplicationError::Validation(
            "password must contain at least 8 characters".to_owned(),
        ));
    }

    Ok(())
}

fn clean_optional(value: impl Into<Option<String>>) -> Option<String> {
    value.into().and_then(|value| {
        let value = value.trim().to_owned();
        (!value.is_empty()).then_some(value)
    })
}

use std::sync::Arc;

use chrono::{Duration, Utc};
use tracing::error;

use crate::domain::entities::{EmailVerificationToken, PasswordResetToken};
use crate::{
    application::{
        dtos::auth::{
            AccountType, AuthResponse, ChangePasswordRequest, ConfirmEmailVerificationRequest,
            ConfirmPasswordResetRequest, CurrentUserProfileResponse, LoginRequest,
            PhoneOtpChallengeResponse, PhoneOtpPurpose, RefreshSessionRequest, RegisterRequest,
            RequestPasswordResetCommand, RequestPhoneOtpRequest, UpdateProfileRequest,
            VerifyPhoneOtpRequest,
        },
        errors::ApplicationError,
        ports::auth::{
            AccessTokenIssuer, AuthEmailSender, GoogleOAuthClient, GoogleUserProfile,
            PasswordHasher, PhoneOtpChallenge, PhoneOtpChallengeStore, PhoneOtpProvider,
            RefreshTokenService, TikTokOAuthClient, TikTokUserProfile,
        },
    },
    domain::{
        entities::{
            AuthProvider, AuthSession, CustomerProfile, SellerProfile, User, UserAuthIdentity,
            UserStatus,
        },
        repositories::{
            AuthSessionRepository, CustomerProfileRepository, EmailVerificationTokenRepository,
            PasswordResetTokenRepository, SellerProfileRepository, UserAuthIdentityRepository,
            UserRepository,
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
    verification_token_repository: Arc<dyn EmailVerificationTokenRepository>,
    email_sender: Arc<dyn AuthEmailSender>,
    verification_tokens: Arc<dyn RefreshTokenService>,
    verification_token_ttl_seconds: i64,
    email_verification_url: String,
}

pub struct TikTokLoginUseCase {
    user_repository: Arc<dyn UserRepository>,
    identity_repository: Arc<dyn UserAuthIdentityRepository>,
    auth_session_repository: Arc<dyn AuthSessionRepository>,
    customer_profile_repository: Arc<dyn CustomerProfileRepository>,
    seller_profile_repository: Arc<dyn SellerProfileRepository>,
    tiktok_oauth: Arc<dyn TikTokOAuthClient>,
    refresh_tokens: Arc<dyn RefreshTokenService>,
    access_tokens: Arc<dyn AccessTokenIssuer>,
    access_token_ttl_seconds: i64,
    refresh_token_ttl_seconds: i64,
}

pub struct GoogleLoginUseCase {
    user_repository: Arc<dyn UserRepository>,
    identity_repository: Arc<dyn UserAuthIdentityRepository>,
    auth_session_repository: Arc<dyn AuthSessionRepository>,
    customer_profile_repository: Arc<dyn CustomerProfileRepository>,
    seller_profile_repository: Arc<dyn SellerProfileRepository>,
    google_oauth: Arc<dyn GoogleOAuthClient>,
    refresh_tokens: Arc<dyn RefreshTokenService>,
    access_tokens: Arc<dyn AccessTokenIssuer>,
    access_token_ttl_seconds: i64,
    refresh_token_ttl_seconds: i64,
}

impl TikTokLoginUseCase {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        user_repository: Arc<dyn UserRepository>,
        identity_repository: Arc<dyn UserAuthIdentityRepository>,
        auth_session_repository: Arc<dyn AuthSessionRepository>,
        customer_profile_repository: Arc<dyn CustomerProfileRepository>,
        seller_profile_repository: Arc<dyn SellerProfileRepository>,
        tiktok_oauth: Arc<dyn TikTokOAuthClient>,
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
            tiktok_oauth,
            refresh_tokens,
            access_tokens,
            access_token_ttl_seconds,
            refresh_token_ttl_seconds,
        }
    }

    pub async fn execute(
        &self,
        authorization_code: &str,
    ) -> Result<AuthResponse, ApplicationError> {
        let token = self.tiktok_oauth.exchange_code(authorization_code).await?;

        if !token
            .scope
            .split(',')
            .any(|scope| scope.trim() == "user.info.basic")
        {
            return Err(ApplicationError::Unauthorized);
        }

        let mut profile = self
            .tiktok_oauth
            .fetch_user_profile(&token.access_token)
            .await?;

        if profile.open_id != token.open_id {
            return Err(ApplicationError::Unauthorized);
        }

        self.login_or_register(&mut profile).await
    }

    async fn login_or_register(
        &self,
        profile: &mut TikTokUserProfile,
    ) -> Result<AuthResponse, ApplicationError> {
        if let Some(identity) = self
            .identity_repository
            .find_by_provider_subject(AuthProvider::TikTok, &profile.open_id)
            .await?
        {
            let user = self
                .user_repository
                .find_by_id(identity.user_id)
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

            return self
                .create_auth_response(user.id, user.is_admin, is_seller)
                .await;
        }

        let display_name = clean_optional(profile.display_name.take())
            .or_else(|| Some("Utilisateur TikTok".to_owned()));
        let avatar_url = clean_optional(profile.avatar_url.take());
        let user = User::new(display_name, avatar_url);
        let identity =
            UserAuthIdentity::oauth(user.id, AuthProvider::TikTok, profile.open_id.clone());
        let customer_profile = CustomerProfile::new(user.id, None);

        self.user_repository.save(&user).await?;
        self.identity_repository.save(&identity).await?;
        self.customer_profile_repository
            .save(&customer_profile)
            .await?;

        self.create_auth_response(user.id, user.is_admin, false)
            .await
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
            .issue_access_token(user_id, session.id, is_admin, is_seller, true, None)?;

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
            account_verified: true,
            verification_channel: None,
            can_change_password: false,
        })
    }
}

impl GoogleLoginUseCase {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        user_repository: Arc<dyn UserRepository>,
        identity_repository: Arc<dyn UserAuthIdentityRepository>,
        auth_session_repository: Arc<dyn AuthSessionRepository>,
        customer_profile_repository: Arc<dyn CustomerProfileRepository>,
        seller_profile_repository: Arc<dyn SellerProfileRepository>,
        google_oauth: Arc<dyn GoogleOAuthClient>,
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
            google_oauth,
            refresh_tokens,
            access_tokens,
            access_token_ttl_seconds,
            refresh_token_ttl_seconds,
        }
    }

    pub async fn execute(
        &self,
        authorization_code: &str,
        account_type: AccountType,
    ) -> Result<AuthResponse, ApplicationError> {
        let token = self.google_oauth.exchange_code(authorization_code).await?;

        if !token
            .scope
            .split_whitespace()
            .any(|scope| scope == "openid")
        {
            return Err(ApplicationError::Unauthorized);
        }

        let profile = self
            .google_oauth
            .fetch_user_profile(&token.access_token)
            .await?;

        if !profile.email_verified {
            return Err(ApplicationError::Unauthorized);
        }

        self.login_or_register(profile, account_type).await
    }

    async fn login_or_register(
        &self,
        profile: GoogleUserProfile,
        account_type: AccountType,
    ) -> Result<AuthResponse, ApplicationError> {
        if let Some(identity) = self
            .identity_repository
            .find_by_provider_subject(AuthProvider::Google, &profile.subject)
            .await?
        {
            return self.login_existing_user(identity.user_id).await;
        }

        let email = EmailAddress::new(profile.email)?;

        if let Some(existing_identity) = self
            .identity_repository
            .find_by_email(email.as_str())
            .await?
        {
            let user = self
                .user_repository
                .find_by_id(existing_identity.user_id)
                .await?
                .ok_or(ApplicationError::Unauthorized)?;
            if user.status != UserStatus::Active {
                return Err(ApplicationError::Unauthorized);
            }

            let google_identity =
                UserAuthIdentity::google(existing_identity.user_id, profile.subject, None);
            self.identity_repository.save(&google_identity).await?;
            let is_seller = self
                .seller_profile_repository
                .find_by_user_id(user.id)
                .await?
                .is_some();
            return self
                .create_auth_response(user.id, user.is_admin, is_seller)
                .await;
        }

        let display_name = clean_optional(profile.display_name);
        let avatar_url = clean_optional(profile.avatar_url);
        let user = User::new(display_name, avatar_url);
        let identity = UserAuthIdentity::google(user.id, profile.subject, Some(email));

        self.user_repository.save(&user).await?;
        self.identity_repository.save(&identity).await?;

        let is_seller = match account_type {
            AccountType::Customer => {
                self.customer_profile_repository
                    .save(&CustomerProfile::new(user.id, None))
                    .await?;
                false
            }
            AccountType::Seller => {
                self.seller_profile_repository
                    .save(&SellerProfile::start_trial(user.id, None, None))
                    .await?;
                true
            }
        };

        self.create_auth_response(user.id, user.is_admin, is_seller)
            .await
    }

    async fn login_existing_user(
        &self,
        user_id: uuid::Uuid,
    ) -> Result<AuthResponse, ApplicationError> {
        let user = self
            .user_repository
            .find_by_id(user_id)
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

        self.create_auth_response(user.id, user.is_admin, is_seller)
            .await
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
            .issue_access_token(user_id, session.id, is_admin, is_seller, true, None)?;

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
            account_verified: true,
            verification_channel: None,
            can_change_password: false,
        })
    }
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
        verification_token_repository: Arc<dyn EmailVerificationTokenRepository>,
        email_sender: Arc<dyn AuthEmailSender>,
        verification_tokens: Arc<dyn RefreshTokenService>,
        verification_token_ttl_seconds: i64,
        email_verification_url: String,
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
            verification_token_repository,
            email_sender,
            verification_tokens,
            verification_token_ttl_seconds,
            email_verification_url,
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
                let shop_name = request.shop_name.and_then(clean_optional);
                let profile = SellerProfile::start_trial(user.id, shop_name, request.payment_link);
                self.seller_profile_repository.save(&profile).await?;
            }
        }

        if identity.provider == AuthProvider::Email {
            self.send_verification_email(&identity).await;
        }

        self.create_auth_response(user.id, user.is_admin, is_seller, &identity)
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
        identity: &UserAuthIdentity,
    ) -> Result<AuthResponse, ApplicationError> {
        let refresh_token = self.refresh_tokens.generate();
        let refresh_token_hash = self.refresh_tokens.hash(&refresh_token)?;
        let expires_at = Utc::now() + Duration::seconds(self.refresh_token_ttl_seconds);
        let session = AuthSession::new(user_id, refresh_token_hash, expires_at, None, None);
        let access_token = self.access_tokens.issue_access_token(
            user_id,
            session.id,
            is_admin,
            is_seller,
            identity.is_account_verified(),
            identity.verification_channel(),
        )?;

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
            account_verified: identity.is_account_verified(),
            verification_channel: identity.verification_channel(),
            can_change_password: true,
        })
    }

    async fn send_verification_email(&self, identity: &UserAuthIdentity) {
        let Some(email) = identity.email.as_ref() else {
            return;
        };
        let verification_secret = self.verification_tokens.generate();
        let result = async {
            let token_hash = self.verification_tokens.hash(&verification_secret)?;
            let expires_at = Utc::now() + Duration::seconds(self.verification_token_ttl_seconds);
            let token = EmailVerificationToken::new(identity.id, token_hash, expires_at);
            let raw_token = format!("{}.{}", token.id, verification_secret);
            self.verification_token_repository.save(&token).await?;
            self.email_sender
                .send_email_verification(
                    email.as_str(),
                    &build_token_url(&self.email_verification_url, &raw_token),
                )
                .await?;
            self.verification_token_repository
                .invalidate_other_for_identity(identity.id, token.id)
                .await?;
            Ok::<(), ApplicationError>(())
        }
        .await;

        if let Err(error) = result {
            error!(%error, user_id = %identity.user_id, "failed to send registration verification email");
        }
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

pub struct RequestPhoneOtpUseCase {
    identity_repository: Arc<dyn UserAuthIdentityRepository>,
    provider: Arc<dyn PhoneOtpProvider>,
    challenge_store: Arc<dyn PhoneOtpChallengeStore>,
    challenge_ttl_seconds: i64,
    resend_cooldown_seconds: i64,
}

impl RequestPhoneOtpUseCase {
    pub fn new(
        identity_repository: Arc<dyn UserAuthIdentityRepository>,
        provider: Arc<dyn PhoneOtpProvider>,
        challenge_store: Arc<dyn PhoneOtpChallengeStore>,
        challenge_ttl_seconds: i64,
        resend_cooldown_seconds: i64,
    ) -> Self {
        Self {
            identity_repository,
            provider,
            challenge_store,
            challenge_ttl_seconds,
            resend_cooldown_seconds,
        }
    }

    pub async fn execute(
        &self,
        request: RequestPhoneOtpRequest,
    ) -> Result<PhoneOtpChallengeResponse, ApplicationError> {
        let phone_number = PhoneNumber::new(request.phone_number)?;
        let existing_identity = self
            .identity_repository
            .find_by_phone_number(phone_number.as_str())
            .await?;

        match request.purpose {
            PhoneOtpPurpose::Login if existing_identity.is_none() => {
                return Err(ApplicationError::InvalidCredentials);
            }
            PhoneOtpPurpose::Register if existing_identity.is_some() => {
                return Err(ApplicationError::Conflict(
                    "phone number is already registered".to_owned(),
                ));
            }
            _ => {}
        }

        let full_name = clean_optional(request.full_name);
        let shop_name = clean_optional(request.shop_name);
        if request.purpose == PhoneOtpPurpose::Register && request.account_type.is_none() {
            return Err(ApplicationError::Validation(
                "account_type is required for phone registration".to_owned(),
            ));
        }

        if !self
            .challenge_store
            .reserve_send(phone_number.as_str(), self.resend_cooldown_seconds)
            .await?
        {
            return Err(ApplicationError::TooManyRequests(
                "wait before requesting another code".to_owned(),
            ));
        }

        let provider_identity = phone_number.as_str().trim_start_matches('+');
        let provider_token = match self.provider.request_otp(provider_identity).await {
            Ok(token) => token,
            Err(error) => {
                self.challenge_store
                    .release_send(phone_number.as_str())
                    .await?;
                return Err(error);
            }
        };
        let challenge = PhoneOtpChallenge {
            id: uuid::Uuid::new_v4(),
            phone_number: phone_number.as_str().to_owned(),
            provider_token,
            purpose: request.purpose,
            full_name,
            account_type: request.account_type,
            shop_name,
        };

        if let Err(error) = self
            .challenge_store
            .save(&challenge, self.challenge_ttl_seconds)
            .await
        {
            self.challenge_store
                .release_send(phone_number.as_str())
                .await?;
            return Err(error);
        }

        Ok(PhoneOtpChallengeResponse {
            challenge_id: challenge.id,
            expires_in_seconds: self.challenge_ttl_seconds,
            resend_after_seconds: self.resend_cooldown_seconds,
        })
    }
}

pub struct VerifyPhoneOtpUseCase {
    user_repository: Arc<dyn UserRepository>,
    identity_repository: Arc<dyn UserAuthIdentityRepository>,
    auth_session_repository: Arc<dyn AuthSessionRepository>,
    customer_profile_repository: Arc<dyn CustomerProfileRepository>,
    seller_profile_repository: Arc<dyn SellerProfileRepository>,
    provider: Arc<dyn PhoneOtpProvider>,
    challenge_store: Arc<dyn PhoneOtpChallengeStore>,
    refresh_tokens: Arc<dyn RefreshTokenService>,
    access_tokens: Arc<dyn AccessTokenIssuer>,
    access_token_ttl_seconds: i64,
    refresh_token_ttl_seconds: i64,
    challenge_ttl_seconds: i64,
    max_attempts: u32,
}

impl VerifyPhoneOtpUseCase {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        user_repository: Arc<dyn UserRepository>,
        identity_repository: Arc<dyn UserAuthIdentityRepository>,
        auth_session_repository: Arc<dyn AuthSessionRepository>,
        customer_profile_repository: Arc<dyn CustomerProfileRepository>,
        seller_profile_repository: Arc<dyn SellerProfileRepository>,
        provider: Arc<dyn PhoneOtpProvider>,
        challenge_store: Arc<dyn PhoneOtpChallengeStore>,
        refresh_tokens: Arc<dyn RefreshTokenService>,
        access_tokens: Arc<dyn AccessTokenIssuer>,
        access_token_ttl_seconds: i64,
        refresh_token_ttl_seconds: i64,
        challenge_ttl_seconds: i64,
        max_attempts: u32,
    ) -> Self {
        Self {
            user_repository,
            identity_repository,
            auth_session_repository,
            customer_profile_repository,
            seller_profile_repository,
            provider,
            challenge_store,
            refresh_tokens,
            access_tokens,
            access_token_ttl_seconds,
            refresh_token_ttl_seconds,
            challenge_ttl_seconds,
            max_attempts,
        }
    }

    pub async fn execute(
        &self,
        request: VerifyPhoneOtpRequest,
    ) -> Result<AuthResponse, ApplicationError> {
        let otp = request.otp.trim();
        if !(4..=8).contains(&otp.len()) || !otp.chars().all(|character| character.is_ascii_digit())
        {
            return Err(ApplicationError::Validation(
                "OTP must contain 4 to 8 digits".to_owned(),
            ));
        }

        let challenge = self
            .challenge_store
            .find(request.challenge_id)
            .await?
            .ok_or(ApplicationError::Unauthorized)?;
        let provider_identity = challenge.phone_number.trim_start_matches('+');
        if !self
            .provider
            .verify_otp(provider_identity, otp, &challenge.provider_token)
            .await?
        {
            let attempts = self
                .challenge_store
                .record_failed_attempt(challenge.id, self.challenge_ttl_seconds)
                .await?;
            if attempts >= self.max_attempts {
                self.challenge_store.delete(challenge.id).await?;
                return Err(ApplicationError::TooManyRequests(
                    "too many invalid verification attempts".to_owned(),
                ));
            }
            return Err(ApplicationError::InvalidCredentials);
        }

        let response = match challenge.purpose {
            PhoneOtpPurpose::Login => self.login(&challenge).await?,
            PhoneOtpPurpose::Register => self.register(&challenge).await?,
        };
        self.challenge_store.delete(challenge.id).await?;
        Ok(response)
    }

    async fn login(&self, challenge: &PhoneOtpChallenge) -> Result<AuthResponse, ApplicationError> {
        let mut identity = self
            .identity_repository
            .find_by_phone_number(&challenge.phone_number)
            .await?
            .ok_or(ApplicationError::InvalidCredentials)?;
        if !identity.phone_verified {
            self.identity_repository
                .mark_phone_verified(identity.id)
                .await?;
            identity.phone_verified = true;
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
        self.create_auth_response(&user, is_seller).await
    }

    async fn register(
        &self,
        challenge: &PhoneOtpChallenge,
    ) -> Result<AuthResponse, ApplicationError> {
        if self
            .identity_repository
            .find_by_phone_number(&challenge.phone_number)
            .await?
            .is_some()
        {
            return Err(ApplicationError::Conflict(
                "phone number is already registered".to_owned(),
            ));
        }

        let phone_number = PhoneNumber::new(&challenge.phone_number)?;
        let user = User::new(challenge.full_name.clone(), None);
        let identity = UserAuthIdentity::verified_phone(user.id, phone_number);
        let account_type = challenge
            .account_type
            .ok_or_else(|| ApplicationError::Validation("account_type is required".to_owned()))?;

        self.user_repository.save(&user).await?;
        self.identity_repository.save(&identity).await?;
        let is_seller = match account_type {
            AccountType::Customer => {
                self.customer_profile_repository
                    .save(&CustomerProfile::new(user.id, None))
                    .await?;
                false
            }
            AccountType::Seller => {
                self.seller_profile_repository
                    .save(&SellerProfile::start_trial(
                        user.id,
                        challenge.shop_name.clone(),
                        None,
                    ))
                    .await?;
                true
            }
        };

        self.create_auth_response(&user, is_seller).await
    }

    async fn create_auth_response(
        &self,
        user: &User,
        is_seller: bool,
    ) -> Result<AuthResponse, ApplicationError> {
        let refresh_token = self.refresh_tokens.generate();
        let refresh_token_hash = self.refresh_tokens.hash(&refresh_token)?;
        let expires_at = Utc::now() + Duration::seconds(self.refresh_token_ttl_seconds);
        let session = AuthSession::new(user.id, refresh_token_hash, expires_at, None, None);
        let access_token = self.access_tokens.issue_access_token(
            user.id,
            session.id,
            user.is_admin,
            is_seller,
            true,
            None,
        )?;
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
            account_verified: true,
            verification_channel: None,
            can_change_password: false,
        })
    }
}

pub struct UpdateProfileUseCase {
    user_repository: Arc<dyn UserRepository>,
    seller_profile_repository: Arc<dyn SellerProfileRepository>,
}

pub struct GetCurrentUserProfileUseCase {
    user_repository: Arc<dyn UserRepository>,
    identity_repository: Arc<dyn UserAuthIdentityRepository>,
    customer_profile_repository: Arc<dyn CustomerProfileRepository>,
    seller_profile_repository: Arc<dyn SellerProfileRepository>,
}

impl GetCurrentUserProfileUseCase {
    pub fn new(
        user_repository: Arc<dyn UserRepository>,
        identity_repository: Arc<dyn UserAuthIdentityRepository>,
        customer_profile_repository: Arc<dyn CustomerProfileRepository>,
        seller_profile_repository: Arc<dyn SellerProfileRepository>,
    ) -> Self {
        Self {
            user_repository,
            identity_repository,
            customer_profile_repository,
            seller_profile_repository,
        }
    }

    pub async fn execute(
        &self,
        user_id: uuid::Uuid,
        session_id: uuid::Uuid,
    ) -> Result<CurrentUserProfileResponse, ApplicationError> {
        let user = self
            .user_repository
            .find_by_id(user_id)
            .await?
            .ok_or(ApplicationError::Unauthorized)?;
        let identities = self
            .identity_repository
            .find_all_by_user_id(user_id)
            .await?;
        let seller_profile = self
            .seller_profile_repository
            .find_by_user_id(user_id)
            .await?;
        let customer_profile = self
            .customer_profile_repository
            .find_by_user_id(user_id)
            .await?;

        let (account_verified, verification_channel) = verification_status(&identities);
        let email = identities.iter().find_map(|identity| {
            identity
                .email
                .as_ref()
                .map(|email| email.as_str().to_owned())
        });
        let phone_number = identities.iter().find_map(|identity| {
            identity
                .phone_number
                .as_ref()
                .map(|phone| phone.as_str().to_owned())
        });
        let can_change_password = identities
            .iter()
            .any(|identity| identity.password_hash.is_some());
        let mut auth_methods = identities
            .iter()
            .map(|identity| match identity.provider {
                AuthProvider::Email => "email",
                AuthProvider::Phone => "phone",
                AuthProvider::Google => "google",
                AuthProvider::TikTok => "tiktok",
            })
            .collect::<Vec<_>>();
        auth_methods.sort_unstable();
        auth_methods.dedup();

        Ok(CurrentUserProfileResponse {
            user_id: user.id,
            session_id,
            full_name: user.full_name,
            avatar_url: user.avatar_url,
            email,
            phone_number,
            is_seller: seller_profile.is_some(),
            is_admin: user.is_admin,
            account_status: match user.status {
                UserStatus::Active => "active",
                UserStatus::Disabled => "disabled",
            },
            account_verified,
            verification_channel,
            can_change_password,
            auth_methods,
            shop_name: seller_profile.and_then(|profile| profile.shop_name),
            default_location: customer_profile.and_then(|profile| profile.default_location),
            member_since: user.created_at,
        })
    }
}

impl UpdateProfileUseCase {
    pub fn new(
        user_repository: Arc<dyn UserRepository>,
        seller_profile_repository: Arc<dyn SellerProfileRepository>,
    ) -> Self {
        Self {
            user_repository,
            seller_profile_repository,
        }
    }

    pub async fn execute(
        &self,
        user_id: uuid::Uuid,
        is_seller: bool,
        request: UpdateProfileRequest,
    ) -> Result<(), ApplicationError> {
        let full_name = clean_optional(request.full_name);
        let shop_name = clean_optional(request.shop_name);

        if full_name
            .as_ref()
            .is_some_and(|value| value.chars().count() > 255)
        {
            return Err(ApplicationError::Validation(
                "full_name must contain at most 255 characters".to_owned(),
            ));
        }
        if shop_name
            .as_ref()
            .is_some_and(|value| value.chars().count() > 255)
        {
            return Err(ApplicationError::Validation(
                "shop_name must contain at most 255 characters".to_owned(),
            ));
        }

        if full_name.is_none() && shop_name.is_none() {
            return Err(ApplicationError::Validation(
                "full_name or shop_name is required".to_owned(),
            ));
        }
        if shop_name.is_some() && !is_seller {
            return Err(ApplicationError::Validation(
                "shop_name is only available for seller profiles".to_owned(),
            ));
        }

        if let Some(full_name) = full_name {
            self.user_repository
                .update_full_name(user_id, full_name)
                .await?;
        }
        if let Some(shop_name) = shop_name {
            self.seller_profile_repository
                .update_shop_name(user_id, shop_name)
                .await?;
        }

        Ok(())
    }
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
    email_sender: Arc<dyn AuthEmailSender>,
    token_ttl_seconds: i64,
    password_reset_url: String,
}

impl RequestPasswordResetUseCase {
    pub fn new(
        identity_repository: Arc<dyn UserAuthIdentityRepository>,
        reset_token_repository: Arc<dyn PasswordResetTokenRepository>,
        reset_tokens: Arc<dyn RefreshTokenService>,
        email_sender: Arc<dyn AuthEmailSender>,
        token_ttl_seconds: i64,
        password_reset_url: String,
    ) -> Self {
        Self {
            identity_repository,
            reset_token_repository,
            reset_tokens,
            email_sender,
            token_ttl_seconds,
            password_reset_url,
        }
    }

    pub async fn execute(
        &self,
        request: RequestPasswordResetCommand,
    ) -> Result<(), ApplicationError> {
        let email = EmailAddress::new(&request.email)?;
        let identity = self
            .identity_repository
            .find_by_email(email.as_str())
            .await?;

        let Some(identity) = identity else {
            return Ok(());
        };

        if identity.password_hash.is_none() {
            return Ok(());
        }

        let reset_token = self.reset_tokens.generate();
        let reset_token_hash = self.reset_tokens.hash(&reset_token)?;
        let expires_at = Utc::now() + Duration::seconds(self.token_ttl_seconds);
        let token = PasswordResetToken::new(identity.id, reset_token_hash, expires_at);

        self.reset_token_repository.save(&token).await?;
        if let Err(error) = self
            .email_sender
            .send_password_reset(email.as_str(), &self.build_reset_url(&reset_token))
            .await
        {
            error!(%error, "failed to send password reset email");
        }

        Ok(())
    }

    fn build_reset_url(&self, reset_token: &str) -> String {
        let separator = if self.password_reset_url.contains('?') {
            '&'
        } else {
            '?'
        };

        format!(
            "{}{}token={}",
            self.password_reset_url, separator, reset_token
        )
    }
}

pub struct ConfirmPasswordResetUseCase {
    identity_repository: Arc<dyn UserAuthIdentityRepository>,
    reset_token_repository: Arc<dyn PasswordResetTokenRepository>,
    auth_session_repository: Arc<dyn AuthSessionRepository>,
    password_hasher: Arc<dyn PasswordHasher>,
    reset_tokens: Arc<dyn RefreshTokenService>,
}

pub struct RequestEmailVerificationUseCase {
    identity_repository: Arc<dyn UserAuthIdentityRepository>,
    token_repository: Arc<dyn EmailVerificationTokenRepository>,
    tokens: Arc<dyn RefreshTokenService>,
    email_sender: Arc<dyn AuthEmailSender>,
    token_ttl_seconds: i64,
    resend_cooldown_seconds: i64,
    verification_url: String,
}

impl RequestEmailVerificationUseCase {
    pub fn new(
        identity_repository: Arc<dyn UserAuthIdentityRepository>,
        token_repository: Arc<dyn EmailVerificationTokenRepository>,
        tokens: Arc<dyn RefreshTokenService>,
        email_sender: Arc<dyn AuthEmailSender>,
        token_ttl_seconds: i64,
        resend_cooldown_seconds: i64,
        verification_url: String,
    ) -> Self {
        Self {
            identity_repository,
            token_repository,
            tokens,
            email_sender,
            token_ttl_seconds,
            resend_cooldown_seconds,
            verification_url,
        }
    }

    pub async fn execute(&self, user_id: uuid::Uuid) -> Result<(), ApplicationError> {
        let identity = self
            .identity_repository
            .find_local_by_user_id(user_id)
            .await?
            .ok_or_else(|| {
                ApplicationError::Validation("email verification is not required".to_owned())
            })?;
        let email = identity.email.as_ref().ok_or_else(|| {
            ApplicationError::Validation(
                "email verification is not available for this account".to_owned(),
            )
        })?;

        if identity.email_verified {
            return Ok(());
        }

        if self
            .token_repository
            .find_latest_for_identity(identity.id)
            .await?
            .is_some_and(|token| {
                token.created_at > Utc::now() - Duration::seconds(self.resend_cooldown_seconds)
            })
        {
            return Ok(());
        }

        let verification_secret = self.tokens.generate();
        let token_hash = self.tokens.hash(&verification_secret)?;
        let expires_at = Utc::now() + Duration::seconds(self.token_ttl_seconds);
        let token = EmailVerificationToken::new(identity.id, token_hash, expires_at);
        let raw_token = format!("{}.{}", token.id, verification_secret);

        self.token_repository.save(&token).await?;
        self.email_sender
            .send_email_verification(
                email.as_str(),
                &build_token_url(&self.verification_url, &raw_token),
            )
            .await?;
        self.token_repository
            .invalidate_other_for_identity(identity.id, token.id)
            .await?;
        Ok(())
    }
}

pub struct ConfirmEmailVerificationUseCase {
    identity_repository: Arc<dyn UserAuthIdentityRepository>,
    token_repository: Arc<dyn EmailVerificationTokenRepository>,
    tokens: Arc<dyn RefreshTokenService>,
}

impl ConfirmEmailVerificationUseCase {
    pub fn new(
        identity_repository: Arc<dyn UserAuthIdentityRepository>,
        token_repository: Arc<dyn EmailVerificationTokenRepository>,
        tokens: Arc<dyn RefreshTokenService>,
    ) -> Self {
        Self {
            identity_repository,
            token_repository,
            tokens,
        }
    }

    pub async fn execute(
        &self,
        request: ConfirmEmailVerificationRequest,
    ) -> Result<(), ApplicationError> {
        let (token_id, verification_secret) =
            parse_verification_token(&request.verification_token)?;
        let token = self
            .token_repository
            .find_active_by_id(token_id)
            .await?
            .ok_or(ApplicationError::Unauthorized)?;

        if !self.tokens.verify(verification_secret, &token.token_hash)? {
            return Err(ApplicationError::Unauthorized);
        }

        let identity = self
            .identity_repository
            .find_by_id(token.user_auth_identity_id)
            .await?
            .ok_or(ApplicationError::Unauthorized)?;

        if identity.provider != AuthProvider::Email {
            return Err(ApplicationError::Unauthorized);
        }

        self.identity_repository
            .mark_email_verified(identity.id)
            .await?;
        self.token_repository
            .invalidate_for_identity(identity.id)
            .await?;
        Ok(())
    }
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
    identity_repository: Arc<dyn UserAuthIdentityRepository>,
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
        identity_repository: Arc<dyn UserAuthIdentityRepository>,
        auth_session_repository: Arc<dyn AuthSessionRepository>,
        seller_profile_repository: Arc<dyn SellerProfileRepository>,
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
        let identities = self
            .identity_repository
            .find_all_by_user_id(user.id)
            .await?;
        let (account_verified, verification_channel) = verification_status(&identities);
        let can_change_password = identities
            .iter()
            .any(|identity| identity.password_hash.is_some());

        self.auth_session_repository
            .revoke(existing_session.id)
            .await?;

        let refresh_token = self.refresh_tokens.generate();
        let refresh_token_hash = self.refresh_tokens.hash(&refresh_token)?;
        let expires_at = Utc::now() + Duration::seconds(self.refresh_token_ttl_seconds);
        let session = AuthSession::new(user.id, refresh_token_hash, expires_at, None, None);
        let access_token = self.access_tokens.issue_access_token(
            user.id,
            session.id,
            user.is_admin,
            is_seller,
            account_verified,
            verification_channel,
        )?;

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
            account_verified,
            verification_channel,
            can_change_password,
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
            let phone_number =
                PhoneNumber::new(identifier).map_err(|_| ApplicationError::InvalidCredentials)?;
            self.identity_repository
                .find_by_phone_number(phone_number.as_str())
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
        let account_verified = identity.is_account_verified();
        let verification_channel = identity.verification_channel();

        let refresh_token = self.refresh_tokens.generate();
        let refresh_token_hash = self.refresh_tokens.hash(&refresh_token)?;
        let expires_at = Utc::now() + Duration::seconds(self.refresh_token_ttl_seconds);
        let session = AuthSession::new(user.id, refresh_token_hash, expires_at, None, None);
        let access_token = self.access_tokens.issue_access_token(
            user.id,
            session.id,
            user.is_admin,
            is_seller,
            account_verified,
            verification_channel,
        )?;

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
            account_verified,
            verification_channel,
            can_change_password: true,
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

fn build_token_url(base_url: &str, token: &str) -> String {
    let separator = if base_url.contains('?') { '&' } else { '?' };
    format!("{base_url}{separator}token={token}")
}

fn verification_status(identities: &[UserAuthIdentity]) -> (bool, Option<&'static str>) {
    if identities.iter().any(UserAuthIdentity::is_account_verified) {
        return (true, None);
    }

    (
        false,
        identities
            .iter()
            .find_map(UserAuthIdentity::verification_channel),
    )
}

fn parse_verification_token(raw_token: &str) -> Result<(uuid::Uuid, &str), ApplicationError> {
    let (id, secret) = raw_token
        .split_once('.')
        .ok_or(ApplicationError::Unauthorized)?;
    let id = uuid::Uuid::parse_str(id).map_err(|_| ApplicationError::Unauthorized)?;

    if secret.is_empty() {
        return Err(ApplicationError::Unauthorized);
    }

    Ok((id, secret))
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use std::sync::Mutex;

    use crate::domain::errors::DomainError;

    #[derive(Default)]
    struct EmptyIdentityRepository;

    #[async_trait]
    impl UserAuthIdentityRepository for EmptyIdentityRepository {
        async fn save(&self, _identity: &UserAuthIdentity) -> Result<(), DomainError> {
            Ok(())
        }

        async fn find_by_id(
            &self,
            _id: uuid::Uuid,
        ) -> Result<Option<UserAuthIdentity>, DomainError> {
            Ok(None)
        }

        async fn update_password_hash(
            &self,
            _identity_id: uuid::Uuid,
            _password_hash: String,
        ) -> Result<(), DomainError> {
            Ok(())
        }

        async fn mark_email_verified(&self, _identity_id: uuid::Uuid) -> Result<(), DomainError> {
            Ok(())
        }

        async fn mark_phone_verified(&self, _identity_id: uuid::Uuid) -> Result<(), DomainError> {
            Ok(())
        }

        async fn find_local_by_user_id(
            &self,
            _user_id: uuid::Uuid,
        ) -> Result<Option<UserAuthIdentity>, DomainError> {
            Ok(None)
        }

        async fn find_all_by_user_id(
            &self,
            _user_id: uuid::Uuid,
        ) -> Result<Vec<UserAuthIdentity>, DomainError> {
            Ok(Vec::new())
        }

        async fn find_by_email(
            &self,
            _email: &str,
        ) -> Result<Option<UserAuthIdentity>, DomainError> {
            Ok(None)
        }

        async fn find_by_phone_number(
            &self,
            _phone_number: &str,
        ) -> Result<Option<UserAuthIdentity>, DomainError> {
            Ok(None)
        }

        async fn find_by_provider_subject(
            &self,
            _provider: AuthProvider,
            _provider_subject: &str,
        ) -> Result<Option<UserAuthIdentity>, DomainError> {
            Ok(None)
        }
    }

    struct ProfileUserRepository {
        user: User,
    }

    #[async_trait]
    impl UserRepository for ProfileUserRepository {
        async fn save(&self, _user: &User) -> Result<(), DomainError> {
            Ok(())
        }

        async fn find_by_id(&self, id: uuid::Uuid) -> Result<Option<User>, DomainError> {
            Ok((self.user.id == id).then(|| self.user.clone()))
        }

        async fn update_full_name(
            &self,
            _id: uuid::Uuid,
            _full_name: String,
        ) -> Result<(), DomainError> {
            Ok(())
        }
    }

    struct ProfileIdentityRepository {
        identities: Vec<UserAuthIdentity>,
    }

    #[async_trait]
    impl UserAuthIdentityRepository for ProfileIdentityRepository {
        async fn save(&self, _identity: &UserAuthIdentity) -> Result<(), DomainError> {
            Ok(())
        }

        async fn find_by_id(
            &self,
            id: uuid::Uuid,
        ) -> Result<Option<UserAuthIdentity>, DomainError> {
            Ok(self
                .identities
                .iter()
                .find(|identity| identity.id == id)
                .cloned())
        }

        async fn update_password_hash(
            &self,
            _identity_id: uuid::Uuid,
            _password_hash: String,
        ) -> Result<(), DomainError> {
            Ok(())
        }

        async fn mark_email_verified(&self, _identity_id: uuid::Uuid) -> Result<(), DomainError> {
            Ok(())
        }

        async fn mark_phone_verified(&self, _identity_id: uuid::Uuid) -> Result<(), DomainError> {
            Ok(())
        }

        async fn find_local_by_user_id(
            &self,
            user_id: uuid::Uuid,
        ) -> Result<Option<UserAuthIdentity>, DomainError> {
            Ok(self
                .identities
                .iter()
                .find(|identity| {
                    identity.user_id == user_id
                        && matches!(identity.provider, AuthProvider::Email | AuthProvider::Phone)
                })
                .cloned())
        }

        async fn find_all_by_user_id(
            &self,
            user_id: uuid::Uuid,
        ) -> Result<Vec<UserAuthIdentity>, DomainError> {
            Ok(self
                .identities
                .iter()
                .filter(|identity| identity.user_id == user_id)
                .cloned()
                .collect())
        }

        async fn find_by_email(
            &self,
            email: &str,
        ) -> Result<Option<UserAuthIdentity>, DomainError> {
            Ok(self
                .identities
                .iter()
                .find(|identity| {
                    identity
                        .email
                        .as_ref()
                        .is_some_and(|value| value.as_str() == email)
                })
                .cloned())
        }

        async fn find_by_phone_number(
            &self,
            phone_number: &str,
        ) -> Result<Option<UserAuthIdentity>, DomainError> {
            Ok(self
                .identities
                .iter()
                .find(|identity| {
                    identity
                        .phone_number
                        .as_ref()
                        .is_some_and(|value| value.as_str() == phone_number)
                })
                .cloned())
        }

        async fn find_by_provider_subject(
            &self,
            provider: AuthProvider,
            provider_subject: &str,
        ) -> Result<Option<UserAuthIdentity>, DomainError> {
            Ok(self
                .identities
                .iter()
                .find(|identity| {
                    identity.provider == provider
                        && identity.provider_subject.as_deref() == Some(provider_subject)
                })
                .cloned())
        }
    }

    #[derive(Default)]
    struct EmptyCustomerProfileRepository;

    #[async_trait]
    impl CustomerProfileRepository for EmptyCustomerProfileRepository {
        async fn save(&self, _profile: &CustomerProfile) -> Result<(), DomainError> {
            Ok(())
        }

        async fn find_by_id(
            &self,
            _id: uuid::Uuid,
        ) -> Result<Option<CustomerProfile>, DomainError> {
            Ok(None)
        }

        async fn find_by_user_id(
            &self,
            _user_id: uuid::Uuid,
        ) -> Result<Option<CustomerProfile>, DomainError> {
            Ok(None)
        }
    }

    struct ProfileSellerRepository {
        profile: SellerProfile,
    }

    #[async_trait]
    impl SellerProfileRepository for ProfileSellerRepository {
        async fn save(&self, _profile: &SellerProfile) -> Result<(), DomainError> {
            Ok(())
        }

        async fn find_by_id(&self, id: uuid::Uuid) -> Result<Option<SellerProfile>, DomainError> {
            Ok((self.profile.id == id).then(|| self.profile.clone()))
        }

        async fn find_by_user_id(
            &self,
            user_id: uuid::Uuid,
        ) -> Result<Option<SellerProfile>, DomainError> {
            Ok((self.profile.user_id == user_id).then(|| self.profile.clone()))
        }

        async fn update_shop_name(
            &self,
            _user_id: uuid::Uuid,
            _shop_name: String,
        ) -> Result<(), DomainError> {
            Ok(())
        }
    }

    #[derive(Default)]
    struct RecordingOtpProvider {
        requested_identity: Mutex<Option<String>>,
    }

    #[async_trait]
    impl PhoneOtpProvider for RecordingOtpProvider {
        async fn request_otp(&self, identity: &str) -> Result<String, ApplicationError> {
            *self.requested_identity.lock().expect("provider lock") = Some(identity.to_owned());
            Ok("provider-secret-token".to_owned())
        }

        async fn verify_otp(
            &self,
            _identity: &str,
            _otp: &str,
            _provider_token: &str,
        ) -> Result<bool, ApplicationError> {
            Ok(true)
        }
    }

    #[derive(Default)]
    struct InMemoryChallengeStore {
        challenge: Mutex<Option<PhoneOtpChallenge>>,
    }

    #[async_trait]
    impl PhoneOtpChallengeStore for InMemoryChallengeStore {
        async fn reserve_send(
            &self,
            _phone_number: &str,
            _ttl_seconds: i64,
        ) -> Result<bool, ApplicationError> {
            Ok(true)
        }

        async fn release_send(&self, _phone_number: &str) -> Result<(), ApplicationError> {
            Ok(())
        }

        async fn save(
            &self,
            challenge: &PhoneOtpChallenge,
            _ttl_seconds: i64,
        ) -> Result<(), ApplicationError> {
            *self.challenge.lock().expect("challenge lock") = Some(challenge.clone());
            Ok(())
        }

        async fn find(
            &self,
            _challenge_id: uuid::Uuid,
        ) -> Result<Option<PhoneOtpChallenge>, ApplicationError> {
            Ok(self.challenge.lock().expect("challenge lock").clone())
        }

        async fn record_failed_attempt(
            &self,
            _challenge_id: uuid::Uuid,
            _ttl_seconds: i64,
        ) -> Result<u32, ApplicationError> {
            Ok(1)
        }

        async fn delete(&self, _challenge_id: uuid::Uuid) -> Result<(), ApplicationError> {
            *self.challenge.lock().expect("challenge lock") = None;
            Ok(())
        }
    }

    #[test]
    fn verification_token_exposes_selector_and_secret() {
        let id = uuid::Uuid::now_v7();
        let raw_token = format!("{id}.secret.with.separator");

        let (parsed_id, secret) = parse_verification_token(&raw_token).expect("valid token");

        assert_eq!(parsed_id, id);
        assert_eq!(secret, "secret.with.separator");
    }

    #[test]
    fn malformed_verification_token_is_rejected() {
        assert!(parse_verification_token("not-a-token").is_err());
        assert!(parse_verification_token("not-a-uuid.secret").is_err());
    }

    #[tokio::test]
    async fn phone_registration_keeps_the_provider_token_server_side() {
        let provider = Arc::new(RecordingOtpProvider::default());
        let challenge_store = Arc::new(InMemoryChallengeStore::default());
        let use_case = RequestPhoneOtpUseCase::new(
            Arc::new(EmptyIdentityRepository),
            provider.clone(),
            challenge_store.clone(),
            300,
            60,
        );

        let response = use_case
            .execute(RequestPhoneOtpRequest {
                phone_number: "07 00 00 00 00".to_owned(),
                purpose: PhoneOtpPurpose::Register,
                full_name: None,
                account_type: Some(AccountType::Seller),
                shop_name: None,
            })
            .await
            .expect("OTP request succeeds");

        assert_eq!(response.expires_in_seconds, 300);
        assert_eq!(
            provider
                .requested_identity
                .lock()
                .expect("provider lock")
                .as_deref(),
            Some("2250700000000")
        );
        let challenge = challenge_store
            .challenge
            .lock()
            .expect("challenge lock")
            .clone()
            .expect("stored challenge");
        assert_eq!(challenge.id, response.challenge_id);
        assert_eq!(challenge.phone_number, "+2250700000000");
        assert_eq!(challenge.provider_token, "provider-secret-token");
    }

    #[tokio::test]
    async fn current_profile_exposes_human_facing_account_information() {
        let user = User::new(
            Some("Awa Kouamé".to_owned()),
            Some("https://images.example/avatar.jpg".to_owned()),
        );
        let identity = UserAuthIdentity::google(
            user.id,
            "google-subject".to_owned(),
            Some(EmailAddress::new("awa@example.com").expect("valid email")),
        );
        let seller = SellerProfile::start_trial(user.id, Some("Boutique Awa".to_owned()), None);
        let session_id = uuid::Uuid::now_v7();
        let use_case = GetCurrentUserProfileUseCase::new(
            Arc::new(ProfileUserRepository { user: user.clone() }),
            Arc::new(ProfileIdentityRepository {
                identities: vec![identity],
            }),
            Arc::new(EmptyCustomerProfileRepository),
            Arc::new(ProfileSellerRepository { profile: seller }),
        );

        let profile = use_case
            .execute(user.id, session_id)
            .await
            .expect("profile is loaded");

        assert_eq!(profile.session_id, session_id);
        assert_eq!(profile.full_name.as_deref(), Some("Awa Kouamé"));
        assert_eq!(profile.email.as_deref(), Some("awa@example.com"));
        assert_eq!(profile.shop_name.as_deref(), Some("Boutique Awa"));
        assert_eq!(profile.auth_methods, vec!["google"]);
        assert!(profile.is_seller);
        assert!(profile.account_verified);
        assert!(!profile.can_change_password);
    }
}

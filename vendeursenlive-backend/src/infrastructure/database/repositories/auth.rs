use async_trait::async_trait;
use chrono::{DateTime, FixedOffset, Utc};
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use uuid::Uuid;

use crate::{
    domain::{
        entities::{
            AuthProvider, AuthSession, CustomerProfile, PasswordResetToken, SellerProfile, User,
            UserAuthIdentity, UserStatus,
        },
        errors::DomainError,
        repositories::{
            AuthSessionRepository, CustomerProfileRepository, PasswordResetTokenRepository,
            SellerProfileRepository, UserAuthIdentityRepository, UserRepository,
        },
        value_objects::{EmailAddress, PhoneNumber},
    },
    infrastructure::{
        auth::password::repository_error,
        database::entities::{
            auth_sessions, customer_profiles, password_reset_tokens, seller_profiles,
            user_auth_identities, users,
        },
    },
};

#[derive(Debug, Clone)]
pub struct SeaOrmUserRepository {
    db: DatabaseConnection,
}

impl SeaOrmUserRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

#[async_trait]
impl UserRepository for SeaOrmUserRepository {
    async fn save(&self, user: &User) -> Result<(), DomainError> {
        users::ActiveModel {
            id: Set(user.id),
            full_name: Set(user.full_name.clone()),
            avatar_url: Set(user.avatar_url.clone()),
            status: Set(user_status_to_db(user.status)),
            is_admin: Set(user.is_admin),
            created_at: Set(to_db_datetime(user.created_at)),
            updated_at: Set(to_db_datetime(user.updated_at)),
        }
        .insert(&self.db)
        .await
        .map(|_| ())
        .map_err(repository_error)
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, DomainError> {
        users::Entity::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(repository_error)?
            .map(user_from_model)
            .transpose()
    }
}

#[derive(Debug, Clone)]
pub struct SeaOrmUserAuthIdentityRepository {
    db: DatabaseConnection,
}

impl SeaOrmUserAuthIdentityRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

#[async_trait]
impl UserAuthIdentityRepository for SeaOrmUserAuthIdentityRepository {
    async fn save(&self, identity: &UserAuthIdentity) -> Result<(), DomainError> {
        user_auth_identities::ActiveModel {
            id: Set(identity.id),
            user_id: Set(identity.user_id),
            provider: Set(auth_provider_to_db(identity.provider)),
            provider_subject: Set(identity.provider_subject.clone()),
            email: Set(identity
                .email
                .as_ref()
                .map(|email| email.as_str().to_owned())),
            phone_number: Set(identity
                .phone_number
                .as_ref()
                .map(|phone_number| phone_number.as_str().to_owned())),
            password_hash: Set(identity.password_hash.clone()),
            email_verified: Set(identity.email_verified),
            phone_verified: Set(identity.phone_verified),
            created_at: Set(to_db_datetime(identity.created_at)),
            updated_at: Set(to_db_datetime(identity.updated_at)),
        }
        .insert(&self.db)
        .await
        .map(|_| ())
        .map_err(repository_error)
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<UserAuthIdentity>, DomainError> {
        user_auth_identities::Entity::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(repository_error)?
            .map(identity_from_model)
            .transpose()
    }

    async fn update_password_hash(
        &self,
        identity_id: Uuid,
        password_hash: String,
    ) -> Result<(), DomainError> {
        if let Some(model) = user_auth_identities::Entity::find_by_id(identity_id)
            .one(&self.db)
            .await
            .map_err(repository_error)?
        {
            let mut active_model: user_auth_identities::ActiveModel = model.into();
            active_model.password_hash = Set(Some(password_hash));
            active_model.updated_at = Set(to_db_datetime(Utc::now()));
            active_model
                .update(&self.db)
                .await
                .map(|_| ())
                .map_err(repository_error)?;
        }

        Ok(())
    }

    async fn find_local_by_user_id(
        &self,
        user_id: Uuid,
    ) -> Result<Option<UserAuthIdentity>, DomainError> {
        user_auth_identities::Entity::find()
            .filter(user_auth_identities::Column::UserId.eq(user_id))
            .filter(user_auth_identities::Column::Provider.is_in([
                user_auth_identities::AuthProvider::Email,
                user_auth_identities::AuthProvider::Phone,
            ]))
            .one(&self.db)
            .await
            .map_err(repository_error)?
            .map(identity_from_model)
            .transpose()
    }

    async fn find_by_email(&self, email: &str) -> Result<Option<UserAuthIdentity>, DomainError> {
        user_auth_identities::Entity::find()
            .filter(user_auth_identities::Column::Email.eq(email.trim().to_ascii_lowercase()))
            .one(&self.db)
            .await
            .map_err(repository_error)?
            .map(identity_from_model)
            .transpose()
    }

    async fn find_by_phone_number(
        &self,
        phone_number: &str,
    ) -> Result<Option<UserAuthIdentity>, DomainError> {
        user_auth_identities::Entity::find()
            .filter(user_auth_identities::Column::PhoneNumber.eq(phone_number.trim()))
            .one(&self.db)
            .await
            .map_err(repository_error)?
            .map(identity_from_model)
            .transpose()
    }

    async fn find_by_provider_subject(
        &self,
        provider: AuthProvider,
        provider_subject: &str,
    ) -> Result<Option<UserAuthIdentity>, DomainError> {
        user_auth_identities::Entity::find()
            .filter(user_auth_identities::Column::Provider.eq(auth_provider_to_db(provider)))
            .filter(user_auth_identities::Column::ProviderSubject.eq(provider_subject))
            .one(&self.db)
            .await
            .map_err(repository_error)?
            .map(identity_from_model)
            .transpose()
    }
}

#[derive(Debug, Clone)]
pub struct SeaOrmAuthSessionRepository {
    db: DatabaseConnection,
}

impl SeaOrmAuthSessionRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

#[async_trait]
impl AuthSessionRepository for SeaOrmAuthSessionRepository {
    async fn save(&self, session: &AuthSession) -> Result<(), DomainError> {
        auth_sessions::ActiveModel {
            id: Set(session.id),
            user_id: Set(session.user_id),
            refresh_token_hash: Set(session.refresh_token_hash.clone()),
            user_agent: Set(session.user_agent.clone()),
            ip_address: Set(session.ip_address.clone()),
            revoked_at: Set(session.revoked_at.map(to_db_datetime)),
            expires_at: Set(to_db_datetime(session.expires_at)),
            created_at: Set(to_db_datetime(session.created_at)),
            last_used_at: Set(to_db_datetime(session.last_used_at)),
        }
        .insert(&self.db)
        .await
        .map(|_| ())
        .map_err(repository_error)
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<AuthSession>, DomainError> {
        auth_sessions::Entity::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(repository_error)?
            .map(session_from_model)
            .transpose()
    }

    async fn revoke(&self, id: Uuid) -> Result<(), DomainError> {
        if let Some(model) = auth_sessions::Entity::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(repository_error)?
        {
            let mut active_model: auth_sessions::ActiveModel = model.into();
            active_model.revoked_at = Set(Some(to_db_datetime(Utc::now())));
            active_model
                .update(&self.db)
                .await
                .map(|_| ())
                .map_err(repository_error)?;
        }

        Ok(())
    }

    async fn revoke_all_for_user(&self, user_id: Uuid) -> Result<(), DomainError> {
        let sessions = auth_sessions::Entity::find()
            .filter(auth_sessions::Column::UserId.eq(user_id))
            .filter(auth_sessions::Column::RevokedAt.is_null())
            .all(&self.db)
            .await
            .map_err(repository_error)?;

        for model in sessions {
            let mut active_model: auth_sessions::ActiveModel = model.into();
            active_model.revoked_at = Set(Some(to_db_datetime(Utc::now())));
            active_model
                .update(&self.db)
                .await
                .map_err(repository_error)?;
        }

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct SeaOrmPasswordResetTokenRepository {
    db: DatabaseConnection,
}

impl SeaOrmPasswordResetTokenRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

#[async_trait]
impl PasswordResetTokenRepository for SeaOrmPasswordResetTokenRepository {
    async fn save(&self, token: &PasswordResetToken) -> Result<(), DomainError> {
        password_reset_tokens::ActiveModel {
            id: Set(token.id),
            user_auth_identity_id: Set(token.user_auth_identity_id),
            token_hash: Set(token.token_hash.clone()),
            used_at: Set(token.used_at.map(to_db_datetime)),
            expires_at: Set(to_db_datetime(token.expires_at)),
            created_at: Set(to_db_datetime(token.created_at)),
        }
        .insert(&self.db)
        .await
        .map(|_| ())
        .map_err(repository_error)
    }

    async fn find_active(&self) -> Result<Vec<PasswordResetToken>, DomainError> {
        password_reset_tokens::Entity::find()
            .filter(password_reset_tokens::Column::UsedAt.is_null())
            .filter(password_reset_tokens::Column::ExpiresAt.gt(to_db_datetime(Utc::now())))
            .all(&self.db)
            .await
            .map_err(repository_error)?
            .into_iter()
            .map(password_reset_token_from_model)
            .collect()
    }

    async fn mark_used(&self, id: Uuid) -> Result<(), DomainError> {
        if let Some(model) = password_reset_tokens::Entity::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(repository_error)?
        {
            let mut active_model: password_reset_tokens::ActiveModel = model.into();
            active_model.used_at = Set(Some(to_db_datetime(Utc::now())));
            active_model
                .update(&self.db)
                .await
                .map(|_| ())
                .map_err(repository_error)?;
        }

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct SeaOrmCustomerProfileRepository {
    db: DatabaseConnection,
}

impl SeaOrmCustomerProfileRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

#[async_trait]
impl CustomerProfileRepository for SeaOrmCustomerProfileRepository {
    async fn save(&self, customer_profile: &CustomerProfile) -> Result<(), DomainError> {
        customer_profiles::ActiveModel {
            id: Set(customer_profile.id),
            user_id: Set(customer_profile.user_id),
            default_location: Set(customer_profile.default_location.clone()),
            preferred_payment_method: Set(customer_profile.preferred_payment_method.clone()),
            created_at: Set(to_db_datetime(customer_profile.created_at)),
            updated_at: Set(to_db_datetime(customer_profile.updated_at)),
        }
        .insert(&self.db)
        .await
        .map(|_| ())
        .map_err(repository_error)
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<CustomerProfile>, DomainError> {
        customer_profiles::Entity::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(repository_error)?
            .map(customer_profile_from_model)
            .transpose()
    }

    async fn find_by_user_id(&self, user_id: Uuid) -> Result<Option<CustomerProfile>, DomainError> {
        customer_profiles::Entity::find()
            .filter(customer_profiles::Column::UserId.eq(user_id))
            .one(&self.db)
            .await
            .map_err(repository_error)?
            .map(customer_profile_from_model)
            .transpose()
    }
}

#[derive(Debug, Clone)]
pub struct SeaOrmSellerProfileRepository {
    db: DatabaseConnection,
}

impl SeaOrmSellerProfileRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

#[async_trait]
impl SellerProfileRepository for SeaOrmSellerProfileRepository {
    async fn save(&self, seller_profile: &SellerProfile) -> Result<(), DomainError> {
        seller_profiles::ActiveModel {
            id: Set(seller_profile.id),
            user_id: Set(seller_profile.user_id),
            shop_name: Set(seller_profile.shop_name.clone()),
            payment_link: Set(seller_profile.payment_link.clone()),
            trial_ends_at: Set(to_db_datetime(seller_profile.trial_ends_at)),
            is_active: Set(seller_profile.is_active),
            created_at: Set(to_db_datetime(seller_profile.created_at)),
        }
        .insert(&self.db)
        .await
        .map(|_| ())
        .map_err(repository_error)
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<SellerProfile>, DomainError> {
        seller_profiles::Entity::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(repository_error)?
            .map(seller_profile_from_model)
            .transpose()
    }

    async fn find_by_user_id(&self, user_id: Uuid) -> Result<Option<SellerProfile>, DomainError> {
        seller_profiles::Entity::find()
            .filter(seller_profiles::Column::UserId.eq(user_id))
            .one(&self.db)
            .await
            .map_err(repository_error)?
            .map(seller_profile_from_model)
            .transpose()
    }
}

fn user_from_model(model: users::Model) -> Result<User, DomainError> {
    Ok(User {
        id: model.id,
        full_name: model.full_name,
        avatar_url: model.avatar_url,
        status: user_status_from_db(model.status),
        is_admin: model.is_admin,
        created_at: from_db_datetime(model.created_at),
        updated_at: from_db_datetime(model.updated_at),
    })
}

fn identity_from_model(
    model: user_auth_identities::Model,
) -> Result<UserAuthIdentity, DomainError> {
    Ok(UserAuthIdentity {
        id: model.id,
        user_id: model.user_id,
        provider: auth_provider_from_db(model.provider),
        provider_subject: model.provider_subject,
        email: model
            .email
            .map(EmailAddress::new)
            .transpose()
            .map_err(|error| {
                DomainError::Repository(format!("invalid email stored in database: {error}"))
            })?,
        phone_number: model
            .phone_number
            .map(PhoneNumber::new)
            .transpose()
            .map_err(|error| {
                DomainError::Repository(format!("invalid phone stored in database: {error}"))
            })?,
        password_hash: model.password_hash,
        email_verified: model.email_verified,
        phone_verified: model.phone_verified,
        created_at: from_db_datetime(model.created_at),
        updated_at: from_db_datetime(model.updated_at),
    })
}

fn session_from_model(model: auth_sessions::Model) -> Result<AuthSession, DomainError> {
    Ok(AuthSession {
        id: model.id,
        user_id: model.user_id,
        refresh_token_hash: model.refresh_token_hash,
        user_agent: model.user_agent,
        ip_address: model.ip_address,
        revoked_at: model.revoked_at.map(from_db_datetime),
        expires_at: from_db_datetime(model.expires_at),
        created_at: from_db_datetime(model.created_at),
        last_used_at: from_db_datetime(model.last_used_at),
    })
}

fn password_reset_token_from_model(
    model: password_reset_tokens::Model,
) -> Result<PasswordResetToken, DomainError> {
    Ok(PasswordResetToken {
        id: model.id,
        user_auth_identity_id: model.user_auth_identity_id,
        token_hash: model.token_hash,
        used_at: model.used_at.map(from_db_datetime),
        expires_at: from_db_datetime(model.expires_at),
        created_at: from_db_datetime(model.created_at),
    })
}

fn customer_profile_from_model(
    model: customer_profiles::Model,
) -> Result<CustomerProfile, DomainError> {
    Ok(CustomerProfile {
        id: model.id,
        user_id: model.user_id,
        default_location: model.default_location,
        preferred_payment_method: model.preferred_payment_method,
        created_at: from_db_datetime(model.created_at),
        updated_at: from_db_datetime(model.updated_at),
    })
}

fn seller_profile_from_model(model: seller_profiles::Model) -> Result<SellerProfile, DomainError> {
    Ok(SellerProfile {
        id: model.id,
        user_id: model.user_id,
        shop_name: model.shop_name,
        payment_link: model.payment_link,
        trial_ends_at: from_db_datetime(model.trial_ends_at),
        is_active: model.is_active,
        created_at: from_db_datetime(model.created_at),
    })
}

fn user_status_to_db(status: UserStatus) -> users::UserStatus {
    match status {
        UserStatus::Active => users::UserStatus::Active,
        UserStatus::Disabled => users::UserStatus::Disabled,
    }
}

fn user_status_from_db(status: users::UserStatus) -> UserStatus {
    match status {
        users::UserStatus::Active => UserStatus::Active,
        users::UserStatus::Disabled => UserStatus::Disabled,
    }
}

fn auth_provider_to_db(provider: AuthProvider) -> user_auth_identities::AuthProvider {
    match provider {
        AuthProvider::Email => user_auth_identities::AuthProvider::Email,
        AuthProvider::Phone => user_auth_identities::AuthProvider::Phone,
        AuthProvider::Google => user_auth_identities::AuthProvider::Google,
        AuthProvider::TikTok => user_auth_identities::AuthProvider::TikTok,
    }
}

fn auth_provider_from_db(provider: user_auth_identities::AuthProvider) -> AuthProvider {
    match provider {
        user_auth_identities::AuthProvider::Email => AuthProvider::Email,
        user_auth_identities::AuthProvider::Phone => AuthProvider::Phone,
        user_auth_identities::AuthProvider::Google => AuthProvider::Google,
        user_auth_identities::AuthProvider::TikTok => AuthProvider::TikTok,
    }
}

fn to_db_datetime(value: DateTime<Utc>) -> DateTime<FixedOffset> {
    value.fixed_offset()
}

fn from_db_datetime(value: DateTime<FixedOffset>) -> DateTime<Utc> {
    value.with_timezone(&Utc)
}

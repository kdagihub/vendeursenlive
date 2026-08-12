use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::{
    entities::{
        AuthProvider, AuthSession, CustomerProfile, EmailVerificationToken, EphemeralProduct,
        LiveSession, Order, PasswordResetToken, SellerProfile, User, UserAuthIdentity,
    },
    errors::DomainError,
};

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn save(&self, user: &User) -> Result<(), DomainError>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, DomainError>;
    async fn update_full_name(&self, id: Uuid, full_name: String) -> Result<(), DomainError>;
}

#[async_trait]
pub trait UserAuthIdentityRepository: Send + Sync {
    async fn save(&self, identity: &UserAuthIdentity) -> Result<(), DomainError>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<UserAuthIdentity>, DomainError>;
    async fn update_password_hash(
        &self,
        identity_id: Uuid,
        password_hash: String,
    ) -> Result<(), DomainError>;
    async fn mark_email_verified(&self, identity_id: Uuid) -> Result<(), DomainError>;
    async fn mark_phone_verified(&self, identity_id: Uuid) -> Result<(), DomainError>;
    async fn find_local_by_user_id(
        &self,
        user_id: Uuid,
    ) -> Result<Option<UserAuthIdentity>, DomainError>;
    async fn find_all_by_user_id(
        &self,
        user_id: Uuid,
    ) -> Result<Vec<UserAuthIdentity>, DomainError>;
    async fn find_by_email(&self, email: &str) -> Result<Option<UserAuthIdentity>, DomainError>;
    async fn find_by_phone_number(
        &self,
        phone_number: &str,
    ) -> Result<Option<UserAuthIdentity>, DomainError>;
    async fn find_by_provider_subject(
        &self,
        provider: AuthProvider,
        provider_subject: &str,
    ) -> Result<Option<UserAuthIdentity>, DomainError>;
}

#[async_trait]
pub trait EmailVerificationTokenRepository: Send + Sync {
    async fn save(&self, token: &EmailVerificationToken) -> Result<(), DomainError>;
    async fn find_active_by_id(
        &self,
        id: Uuid,
    ) -> Result<Option<EmailVerificationToken>, DomainError>;
    async fn find_latest_for_identity(
        &self,
        identity_id: Uuid,
    ) -> Result<Option<EmailVerificationToken>, DomainError>;
    async fn invalidate_for_identity(&self, identity_id: Uuid) -> Result<(), DomainError>;
    async fn invalidate_other_for_identity(
        &self,
        identity_id: Uuid,
        retained_token_id: Uuid,
    ) -> Result<(), DomainError>;
}

#[async_trait]
pub trait PasswordResetTokenRepository: Send + Sync {
    async fn save(&self, token: &PasswordResetToken) -> Result<(), DomainError>;
    async fn find_active(&self) -> Result<Vec<PasswordResetToken>, DomainError>;
    async fn mark_used(&self, id: Uuid) -> Result<(), DomainError>;
}

#[async_trait]
pub trait AuthSessionRepository: Send + Sync {
    async fn save(&self, session: &AuthSession) -> Result<(), DomainError>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<AuthSession>, DomainError>;
    async fn revoke(&self, id: Uuid) -> Result<(), DomainError>;
    async fn revoke_all_for_user(&self, user_id: Uuid) -> Result<(), DomainError>;
}

#[async_trait]
pub trait CustomerProfileRepository: Send + Sync {
    async fn save(&self, customer_profile: &CustomerProfile) -> Result<(), DomainError>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<CustomerProfile>, DomainError>;
    async fn find_by_user_id(&self, user_id: Uuid) -> Result<Option<CustomerProfile>, DomainError>;
}

#[async_trait]
pub trait SellerProfileRepository: Send + Sync {
    async fn save(&self, seller_profile: &SellerProfile) -> Result<(), DomainError>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<SellerProfile>, DomainError>;
    async fn find_by_user_id(&self, user_id: Uuid) -> Result<Option<SellerProfile>, DomainError>;
    async fn update_shop_name(&self, user_id: Uuid, shop_name: String) -> Result<(), DomainError>;
}

#[async_trait]
pub trait LiveSessionRepository: Send + Sync {
    async fn save(&self, live_session: &LiveSession) -> Result<(), DomainError>;
    async fn update(&self, live_session: &LiveSession) -> Result<(), DomainError>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<LiveSession>, DomainError>;
    async fn find_ongoing_by_seller(
        &self,
        seller_profile_id: Uuid,
    ) -> Result<Option<LiveSession>, DomainError>;
}

#[async_trait]
pub trait EphemeralProductRepository: Send + Sync {
    async fn save(&self, product: &EphemeralProduct) -> Result<(), DomainError>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<EphemeralProduct>, DomainError>;
    async fn find_by_live_session(
        &self,
        live_session_id: Uuid,
    ) -> Result<Vec<EphemeralProduct>, DomainError>;
}

#[async_trait]
pub trait OrderRepository: Send + Sync {
    async fn save(&self, order: &Order) -> Result<(), DomainError>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Order>, DomainError>;
    async fn find_by_product(&self, product_id: Uuid) -> Result<Vec<Order>, DomainError>;
}

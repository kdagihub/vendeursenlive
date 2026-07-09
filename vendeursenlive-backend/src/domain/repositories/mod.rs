use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::{
    entities::{
        AuthProvider, AuthSession, CustomerProfile, EphemeralProduct, LiveSession, Order,
        SellerProfile, User, UserAuthIdentity,
    },
    errors::DomainError,
};

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn save(&self, user: &User) -> Result<(), DomainError>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, DomainError>;
}

#[async_trait]
pub trait UserAuthIdentityRepository: Send + Sync {
    async fn save(&self, identity: &UserAuthIdentity) -> Result<(), DomainError>;
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
pub trait AuthSessionRepository: Send + Sync {
    async fn save(&self, session: &AuthSession) -> Result<(), DomainError>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<AuthSession>, DomainError>;
    async fn revoke(&self, id: Uuid) -> Result<(), DomainError>;
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
}

#[async_trait]
pub trait LiveSessionRepository: Send + Sync {
    async fn save(&self, live_session: &LiveSession) -> Result<(), DomainError>;
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

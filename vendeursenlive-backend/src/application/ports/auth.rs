use uuid::Uuid;

use crate::application::errors::ApplicationError;

pub trait PasswordHasher: Send + Sync {
    fn hash_password(&self, password: &str) -> Result<String, ApplicationError>;
    fn verify_password(
        &self,
        password: &str,
        password_hash: &str,
    ) -> Result<bool, ApplicationError>;
}

pub trait RefreshTokenService: Send + Sync {
    fn generate(&self) -> String;
    fn hash(&self, refresh_token: &str) -> Result<String, ApplicationError>;
    fn verify(
        &self,
        refresh_token: &str,
        refresh_token_hash: &str,
    ) -> Result<bool, ApplicationError>;
}

pub trait AccessTokenIssuer: Send + Sync {
    fn issue_access_token(
        &self,
        user_id: Uuid,
        session_id: Uuid,
        is_admin: bool,
        is_seller: bool,
    ) -> Result<String, ApplicationError>;
}

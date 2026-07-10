use uuid::Uuid;

use crate::application::{errors::ApplicationError, ports::auth::RefreshTokenService};

use super::password::{hash_secret, verify_secret};

#[derive(Debug, Clone, Default)]
pub struct UuidRefreshTokenService;

impl RefreshTokenService for UuidRefreshTokenService {
    fn generate(&self) -> String {
        format!("{}.{}", Uuid::new_v4(), Uuid::new_v4())
    }

    fn hash(&self, refresh_token: &str) -> Result<String, ApplicationError> {
        hash_secret(refresh_token).map_err(ApplicationError::Infrastructure)
    }

    fn verify(
        &self,
        refresh_token: &str,
        refresh_token_hash: &str,
    ) -> Result<bool, ApplicationError> {
        verify_secret(refresh_token, refresh_token_hash).map_err(ApplicationError::Infrastructure)
    }
}

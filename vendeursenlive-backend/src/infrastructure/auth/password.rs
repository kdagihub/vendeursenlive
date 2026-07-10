use argon2::{
    password_hash::{
        rand_core::OsRng, PasswordHash, PasswordHasher as _, PasswordVerifier, SaltString,
    },
    Argon2,
};

use crate::{
    application::{errors::ApplicationError, ports::auth::PasswordHasher},
    domain::errors::DomainError,
};

#[derive(Debug, Clone, Default)]
pub struct Argon2PasswordHasher;

impl PasswordHasher for Argon2PasswordHasher {
    fn hash_password(&self, password: &str) -> Result<String, ApplicationError> {
        hash_secret(password).map_err(ApplicationError::Infrastructure)
    }

    fn verify_password(
        &self,
        password: &str,
        password_hash: &str,
    ) -> Result<bool, ApplicationError> {
        verify_secret(password, password_hash).map_err(ApplicationError::Infrastructure)
    }
}

pub fn hash_secret(secret: &str) -> Result<String, String> {
    let salt = SaltString::generate(&mut OsRng);

    Argon2::default()
        .hash_password(secret.as_bytes(), &salt)
        .map(|hash| hash.to_string())
        .map_err(|error| error.to_string())
}

pub fn verify_secret(secret: &str, secret_hash: &str) -> Result<bool, String> {
    let parsed_hash = PasswordHash::new(secret_hash).map_err(|error| error.to_string())?;

    Ok(Argon2::default()
        .verify_password(secret.as_bytes(), &parsed_hash)
        .is_ok())
}

pub fn repository_error(error: impl std::fmt::Display) -> DomainError {
    DomainError::Repository(error.to_string())
}

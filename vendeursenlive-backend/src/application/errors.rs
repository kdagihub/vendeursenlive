use thiserror::Error;

use crate::domain::{errors::DomainError, value_objects::ValueObjectError};

#[derive(Debug, Error)]
pub enum ApplicationError {
    #[error("validation error: {0}")]
    Validation(String),
    #[error("credentials are invalid")]
    InvalidCredentials,
    #[error("resource conflict: {0}")]
    Conflict(String),
    #[error("unauthorized")]
    Unauthorized,
    #[error("infrastructure error: {0}")]
    Infrastructure(String),
}

impl From<DomainError> for ApplicationError {
    fn from(error: DomainError) -> Self {
        match error {
            DomainError::BusinessRuleViolation(message) => Self::Validation(message),
            DomainError::Repository(message) => Self::Infrastructure(message),
        }
    }
}

impl From<ValueObjectError> for ApplicationError {
    fn from(error: ValueObjectError) -> Self {
        Self::Validation(error.to_string())
    }
}

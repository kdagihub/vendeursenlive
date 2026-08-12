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
    #[error("resource not found: {0}")]
    NotFound(String),
    #[error("forbidden: {0}")]
    Forbidden(String),
    #[error("unauthorized")]
    Unauthorized,
    #[error("too many requests: {0}")]
    TooManyRequests(String),
    #[error("service unavailable: {0}")]
    ServiceUnavailable(String),
    #[error("infrastructure error: {0}")]
    Infrastructure(String),
}

impl From<DomainError> for ApplicationError {
    fn from(error: DomainError) -> Self {
        match error {
            DomainError::BusinessRuleViolation(message) => Self::Validation(message),
            DomainError::Conflict(message) => Self::Conflict(message),
            DomainError::Repository(message) => Self::Infrastructure(message),
        }
    }
}

impl From<ValueObjectError> for ApplicationError {
    fn from(error: ValueObjectError) -> Self {
        Self::Validation(error.to_string())
    }
}

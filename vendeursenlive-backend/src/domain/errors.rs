use thiserror::Error;

#[derive(Debug, Error)]
#[allow(dead_code)]
pub enum DomainError {
    #[error("business rule violation: {0}")]
    BusinessRuleViolation(String),
    #[error("repository error: {0}")]
    Repository(String),
}

//! Validator interface — declarations for the `core/validator` implementation.

pub mod validator_default;

pub use crate::api::traits::Validator;

/// Result type for validation operations.
pub type ValidateResult = Result<(), String>;

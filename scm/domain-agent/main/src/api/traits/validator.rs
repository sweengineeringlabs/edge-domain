//! Validator trait for domain validation.

use crate::api::AgentError;

/// Validates domain objects for correctness and consistency.
pub trait Validator {
    /// Validate an object and return Result.
    fn validate(&self) -> Result<(), AgentError>;
}

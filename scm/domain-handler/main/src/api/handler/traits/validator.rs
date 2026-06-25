//! [`Validator`] — validates a service-backed handler before registration.

use crate::api::handler::errors::HandlerError;

/// Validates a service-backed handler's configuration before it enters the dispatch pipeline.
pub trait Validator {
    /// Validate this handler.
    ///
    /// Returns `Ok(())` when valid, or [`HandlerError::InvalidRequest`] describing
    /// the constraint that was violated.
    fn validate(&self) -> Result<(), HandlerError>;
}

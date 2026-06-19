//! `Validator` — pre-flight validation of a completion request.

use crate::api::complete::errors::CompleteError;
use crate::api::complete::types::CompletionRequest;

/// Validates a [`CompletionRequest`] before it reaches the network layer.
pub trait Validator: Send + Sync {
    /// Inspect the request and return an error if it is malformed.
    fn validate(&self, request: &CompletionRequest) -> Result<(), CompleteError>;
}

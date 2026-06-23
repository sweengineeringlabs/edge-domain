//! Validator service wrapper — factory and facade for creating validators.
//!
//! This module provides the implementation-facing factory for creating validators.
//! The public API is available through the root crate exports.

use crate::api::{Validator, ValidatorService};

/// Service marker constant for validator factory operations.
pub const VALIDATOR_FACTORY: &str = "validator_factory";

/// Internal factory for creating validator instances.
///
/// This factory conceals the concrete implementation type, returning opaque trait objects.
#[derive(Debug, Clone, Copy)]
pub(crate) struct ValidatorFactory;

impl ValidatorFactory {
    /// Create a config validator strategy.
    ///
    /// This is a convenience wrapper delegating to [`ValidatorService`].
    pub(crate) fn create(enabled: bool) -> Box<dyn Validator> {
        ValidatorService::create_validator(enabled)
    }
}

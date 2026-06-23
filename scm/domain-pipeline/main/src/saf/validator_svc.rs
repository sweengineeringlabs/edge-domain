//! Validator service wrapper — re-exports validator service.
//!
//! This module provides facade for creating validators through the service layer.

use crate::api::{Validator, ValidatorService};

/// Factory for creating validator instances.
///
/// This facade provides convenient factory methods for constructing validators.
#[derive(Debug, Clone, Copy)]
pub struct ValidatorFactory;

impl ValidatorFactory {
    /// Create a config validator strategy.
    ///
    /// This is a convenience wrapper delegating to [`ValidatorService`].
    pub fn create(enabled: bool) -> Box<dyn Validator> {
        ValidatorService::create_validator(enabled)
    }
}

//! Validator service wrapper — re-exports validator service.
//!
//! This module provides wrapper functions for creating validators through the service layer.

use crate::api::{Validator, ValidatorService};

/// Create a config validator strategy.
///
/// This is a convenience wrapper delegating to [`ValidatorService`].
pub fn create(enabled: bool) -> Box<dyn Validator> {
    ValidatorService::create_validator(enabled)
}

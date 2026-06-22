//! Validator service factory — creates validator instances.

use crate::api::{Validator, ValidatorFactory};

/// Create a config validator strategy.
///
/// # Arguments
/// * `enabled` - Whether the validator should enforce validation rules
///
/// # Returns
/// A boxed validator instance
pub fn create_validator(enabled: bool) -> Box<dyn Validator> {
    ValidatorFactory::create(enabled)
}

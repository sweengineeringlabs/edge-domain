//! Validator service factory — creates validator instances.

use crate::api::{Validator, ValidatorFactory};

/// Service for creating validators in the SAF layer.
pub struct ValidatorService;

impl ValidatorService {
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
}

impl ValidatorFactory {
    /// Create a config validator strategy.
    ///
    /// # Arguments
    /// * `enabled` - Whether the validator should enforce validation rules
    ///
    /// # Returns
    /// A boxed validator instance
    pub fn create_saf(enabled: bool) -> Box<dyn Validator> {
        Self::create(enabled)
    }
}

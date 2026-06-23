//! Validator service factory — creates validator instances.

use super::super::Validator;

/// Service for creating validators.
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
        crate::spi::ValidatorFactory::create(enabled)
    }
}

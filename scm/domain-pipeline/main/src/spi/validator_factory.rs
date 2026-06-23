//! Factory for creating validator instances (implementation).

use crate::api::{Validator, ValidatorFactory};
use crate::spi::config_validator::ConfigValidator;

impl ValidatorFactory {
    /// Create a config validator strategy.
    ///
    /// # Arguments
    /// * `enabled` - Whether the validator should enforce validation rules
    ///
    /// # Returns
    /// A boxed validator instance
    pub fn create(enabled: bool) -> Box<dyn Validator> {
        Box::new(ConfigValidator::new(enabled))
    }
}


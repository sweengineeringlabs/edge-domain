//! Factory for creating validator instances (implementation).

use crate::api::Validator;
use crate::spi::{ValidatorFactory, config_validator::ConfigValidator};

impl ValidatorFactory {
    /// Create a config validator strategy.
    ///
    /// # Arguments
    /// * `enabled` - Whether the validator should enforce validation rules
    ///
    /// # Returns
    /// A boxed validator instance
    pub(crate) fn create(enabled: bool) -> Box<dyn Validator> {
        Box::new(ConfigValidator::new(enabled))
    }
}


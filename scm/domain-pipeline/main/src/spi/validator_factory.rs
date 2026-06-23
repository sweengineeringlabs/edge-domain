//! Factory for creating validator instances (implementation).

use crate::api::Validator;
use crate::spi::config_validator::ConfigValidator;

/// Internal factory for creating validator instances.
pub(crate) struct ValidatorFactory;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_happy_enabled() {
        let validator = ValidatorFactory::create(true);
        assert!(validator.is_enabled());
    }

    #[test]
    fn test_create_happy_disabled() {
        let validator = ValidatorFactory::create(false);
        assert!(!validator.is_enabled());
    }
}


//! Validator service factory — creates validator instances.

use crate::api::Validator;
use crate::spi;

/// Create a config validator strategy.
///
/// # Arguments
/// * `enabled` - Whether the validator should enforce validation rules
///
/// # Returns
/// A boxed validator instance
pub fn create_validator(enabled: bool) -> Box<dyn Validator> {
    Box::new(spi::ConfigValidator::new(enabled))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @covers: create_validator
    #[test]
    fn test_create_validator_happy_enabled() {
        let validator = create_validator(true);
        assert!(validator.is_enabled());
    }

    /// @covers: create_validator
    #[test]
    fn test_create_validator_happy_disabled() {
        let validator = create_validator(false);
        assert!(!validator.is_enabled());
    }
}

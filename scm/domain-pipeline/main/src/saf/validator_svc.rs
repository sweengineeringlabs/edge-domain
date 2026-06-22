//! Validator service facade — provides validation interface.

pub use crate::api::Validator;
use crate::spi::config_validator::ConfigValidator;

/// Marker constant for validator service identification.
pub const VALIDATOR_SVC: &str = "validator";

/// Create a config validator strategy.
/// # Arguments
/// * `enabled` - Whether the validator should enforce validation rules
/// # Returns
/// A boxed validator instance
pub fn create_validator(enabled: bool) -> Box<dyn Validator> {
    Box::new(ConfigValidator::new(enabled))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::PipelineConfig;

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

    /// @covers: create_validator
    #[tokio::test]
    async fn test_create_validator_happy_validates() {
        let validator = create_validator(true);
        let config = PipelineConfig::default();
        assert!(validator.validate(&config).await.is_ok());
    }

    /// @covers: VALIDATOR_SVC constant
    #[test]
    fn test_validator_svc_constant() {
        assert_eq!(VALIDATOR_SVC, "validator");
    }
}

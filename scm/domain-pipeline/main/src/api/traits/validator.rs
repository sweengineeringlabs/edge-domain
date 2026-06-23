//! [`Validator`] — validates pipeline configuration and execution constraints.

use crate::api::{PipelineConfig, PipelineError};

/// Validates pipeline configuration and execution constraints.
#[async_trait::async_trait]
pub trait Validator: Send + Sync {
    /// Validate the pipeline configuration.
    async fn validate(&self, config: &PipelineConfig) -> Result<(), PipelineError>;

    /// Check if this validator is enabled.
    fn is_enabled(&self) -> bool;
}

#[cfg(test)]
mod tests {
    use super::*;

    struct AlwaysValidValidator;

    #[async_trait::async_trait]
    impl Validator for AlwaysValidValidator {
        async fn validate(&self, _config: &PipelineConfig) -> Result<(), PipelineError> {
            Ok(())
        }

        fn is_enabled(&self) -> bool {
            true
        }
    }

    struct AlwaysFailValidator;

    #[async_trait::async_trait]
    impl Validator for AlwaysFailValidator {
        async fn validate(&self, _config: &PipelineConfig) -> Result<(), PipelineError> {
            Err(PipelineError::ConfigError("validation failed".to_string()))
        }

        fn is_enabled(&self) -> bool {
            false
        }
    }

    #[tokio::test]
    async fn test_validate_happy_accepts_valid() {
        let validator = AlwaysValidValidator;
        let config = PipelineConfig::default();
        assert!(validator.validate(&config).await.is_ok());
    }

    #[tokio::test]
    async fn test_validate_error_rejects_invalid() {
        let validator = AlwaysFailValidator;
        let config = PipelineConfig::default();
        let result = validator.validate(&config).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_validate_error_message_preserved() {
        let validator = AlwaysFailValidator;
        let config = PipelineConfig::default();
        match validator.validate(&config).await {
            Err(PipelineError::ConfigError(msg)) => assert_eq!(msg, "validation failed"),
            _ => panic!("Expected ConfigError"),
        }
    }

    #[test]
    fn test_is_enabled_happy_true() {
        let validator = AlwaysValidValidator;
        assert!(validator.is_enabled());
    }

    #[test]
    fn test_is_enabled_happy_false() {
        let validator = AlwaysFailValidator;
        assert!(!validator.is_enabled());
    }

    #[test]
    fn test_is_enabled_edge_multiple_calls_consistent() {
        let validator = AlwaysValidValidator;
        assert!(validator.is_enabled());
        assert!(validator.is_enabled());  // Should be idempotent
    }

    #[test]
    fn test_is_enabled_happy_state_toggles() {
        let enabled = AlwaysValidValidator;
        let disabled = AlwaysFailValidator;
        assert!(enabled.is_enabled());
        assert!(!disabled.is_enabled());
    }

    #[test]
    fn test_is_enabled_error_disabled_remains_disabled() {
        let validator = AlwaysFailValidator;
        assert!(!validator.is_enabled());
        assert!(!validator.is_enabled());
    }
}

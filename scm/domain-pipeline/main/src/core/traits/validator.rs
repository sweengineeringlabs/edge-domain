//! [`DefaultValidator`] — primary implementation of the [`Validator`] contract.

use crate::api::{PipelineConfig, PipelineError, Validator};

/// Validates pipeline configuration before execution.
pub(crate) struct DefaultValidator {
    enabled: bool,
}

impl DefaultValidator {
    /// Create a new validator with the given enabled state.
    pub(crate) fn new(enabled: bool) -> Self {
        Self { enabled }
    }
}

impl Default for DefaultValidator {
    fn default() -> Self {
        Self::new(true)
    }
}

#[async_trait::async_trait]
impl Validator for DefaultValidator {
    async fn validate(&self, config: &PipelineConfig) -> Result<(), PipelineError> {
        if !self.enabled {
            return Ok(());
        }
        if config.abort_on_error {
            Ok(())
        } else {
            Err(PipelineError::ConfigError(
                "abort_on_error must be true".to_string(),
            ))
        }
    }

    fn is_enabled(&self) -> bool {
        self.enabled
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @covers: new
    #[test]
    fn test_new_happy_enabled() {
        let validator = DefaultValidator::new(true);
        assert!(validator.is_enabled());
    }

    /// @covers: new
    #[test]
    fn test_new_happy_disabled() {
        let validator = DefaultValidator::new(false);
        assert!(!validator.is_enabled());
    }

    /// @covers: new
    #[test]
    fn test_new_edge_default_is_enabled() {
        let validator = DefaultValidator::default();
        assert!(validator.is_enabled());
    }

    /// @covers: validate
    #[tokio::test]
    async fn test_validate_happy_valid_config() {
        let validator = DefaultValidator::new(true);
        let config = PipelineConfig::default();
        assert!(validator.validate(&config).await.is_ok());
    }

    /// @covers: validate
    #[tokio::test]
    async fn test_validate_error_invalid_config() {
        let validator = DefaultValidator::new(true);
        let config = PipelineConfig {
            timeout_per_step: None,
            emit_lifecycle_events: false,
            abort_on_error: false,
        };
        assert!(validator.validate(&config).await.is_err());
    }

    /// @covers: validate
    #[tokio::test]
    async fn test_validate_edge_disabled_skips_validation() {
        let validator = DefaultValidator::new(false);
        let config = PipelineConfig {
            timeout_per_step: None,
            emit_lifecycle_events: false,
            abort_on_error: false,
        };
        assert!(validator.validate(&config).await.is_ok());
    }

    /// @covers: is_enabled
    #[test]
    fn test_is_enabled_happy_true() {
        let validator = DefaultValidator::new(true);
        assert!(validator.is_enabled());
    }

    /// @covers: is_enabled
    #[test]
    fn test_is_enabled_error_false() {
        let validator = DefaultValidator::new(false);
        assert!(!validator.is_enabled());
    }

    /// @covers: is_enabled
    #[test]
    fn test_is_enabled_edge_default() {
        let validator = DefaultValidator::default();
        assert!(validator.is_enabled());
    }
}

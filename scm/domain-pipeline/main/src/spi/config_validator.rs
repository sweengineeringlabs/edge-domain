//! Configuration validator — strategy implementation.

use crate::api::{PipelineConfig, PipelineError, Validator};

/// Validates pipeline configuration.
pub(crate) struct ConfigValidator {
    enabled: bool,
}

impl ConfigValidator {
    /// Create a new config validator.
    pub(crate) fn new(enabled: bool) -> Self {
        Self { enabled }
    }
}

impl Default for ConfigValidator {
    fn default() -> Self {
        Self::new(true)
    }
}

#[async_trait::async_trait]
impl Validator for ConfigValidator {
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

    #[tokio::test]
    async fn test_validator_enabled_valid_config() {
        let validator = ConfigValidator::new(true);
        let config = PipelineConfig {
            timeout_per_step: None,
            emit_lifecycle_events: false,
            abort_on_error: true,
        };
        assert!(validator.validate(&config).await.is_ok());
    }

    #[tokio::test]
    async fn test_validator_enabled_invalid_config() {
        let validator = ConfigValidator::new(true);
        let config = PipelineConfig {
            timeout_per_step: None,
            emit_lifecycle_events: false,
            abort_on_error: false,
        };
        assert!(validator.validate(&config).await.is_err());
    }

    #[tokio::test]
    async fn test_validator_disabled_skips_validation() {
        let validator = ConfigValidator::new(false);
        let config = PipelineConfig {
            timeout_per_step: None,
            emit_lifecycle_events: false,
            abort_on_error: false,
        };
        assert!(validator.validate(&config).await.is_ok());
    }

    #[test]
    fn test_validator_is_enabled() {
        let enabled = ConfigValidator::new(true);
        let disabled = ConfigValidator::new(false);
        assert!(enabled.is_enabled());
        assert!(!disabled.is_enabled());
    }

    #[test]
    fn test_validator_default_enabled() {
        let validator = ConfigValidator::default();
        assert!(validator.is_enabled());
    }
}

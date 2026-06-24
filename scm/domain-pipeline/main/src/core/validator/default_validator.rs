//! [`DefaultValidator`] — validates pipeline configuration.

use crate::api::{PipelineConfig, PipelineError, Validator};

/// Default validator that always passes.
#[derive(Clone)]
pub(crate) struct DefaultValidator;

#[async_trait::async_trait]
impl Validator for DefaultValidator {
    async fn validate(&self, _config: &PipelineConfig) -> Result<(), PipelineError> {
        Ok(())
    }

    fn is_enabled(&self) -> bool {
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @covers DefaultValidator::validate
    #[tokio::test]
    async fn test_validate_happy_accepts() {
        let validator = DefaultValidator;
        let config = PipelineConfig::default();
        assert!(validator.validate(&config).await.is_ok());
    }

    /// @covers DefaultValidator::is_enabled
    #[test]
    fn test_is_enabled_happy_returns_true() {
        let validator = DefaultValidator;
        assert!(validator.is_enabled());
    }
}

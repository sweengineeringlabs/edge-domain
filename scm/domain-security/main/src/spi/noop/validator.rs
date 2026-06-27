//! Noop [`Validator`] implementation.

use crate::api::ValidationError;
use crate::api::Validator;

/// Noop validator that accepts all values.
#[derive(Debug, Clone, Copy)]
pub(crate) struct NoopValidator;

impl Validator for NoopValidator {
    fn validate(&self) -> Result<(), ValidationError> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_noop_validator_accepts_all() {
        let validator = NoopValidator;
        assert_eq!(validator.validate(), Ok(()));
    }
}

//! `ValidatorDefault` — no-op validator for configurations with no invariants.

use crate::api::validator::traits::Validator;

/// A validator that always returns `Ok(())`.
///
/// Useful for configuration types that have no field-level invariants
/// to enforce at construction time.
#[cfg_attr(
    not(test),
    expect(
        dead_code,
        reason = "SEA core/ structural anchor — constructed only in tests"
    )
)]
pub(crate) struct ValidatorDefault;

impl Validator for ValidatorDefault {
    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validator_default_validate_returns_ok() {
        assert!(ValidatorDefault.validate().is_ok());
    }

    #[test]
    fn test_validator_default_validate_always_succeeds_on_repeated_calls() {
        for _ in 0..3 {
            assert!(ValidatorDefault.validate().is_ok());
        }
    }
}

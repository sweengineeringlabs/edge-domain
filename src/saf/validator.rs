//! SAF wrapper for the `Validator` trait.

use crate::api::traits::Validator;

/// Validate any configuration value using the [`Validator`] contract.
///
/// Returns `Err` with a human-readable description when the configuration
/// contains an invalid combination of fields.
pub fn validate<V: Validator>(v: &V) -> Result<(), String> {
    v.validate()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::validator::validator_default::ValidatorDefault;

    /// @covers: validate
    #[test]
    fn test_validate_delegates_to_validator_trait() {
        assert!(validate(&ValidatorDefault).is_ok());
    }
}

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
    use crate::api::traits::Validator;
    use crate::core::validator::validator_default::ValidatorDefault;

    struct AlwaysErr;
    impl Validator for AlwaysErr {
        fn validate(&self) -> Result<(), String> {
            Err("always fails".into())
        }
    }

    /// @covers: validate
    #[test]
    fn test_validate_ok_delegates_to_validator_trait() {
        assert!(validate(&ValidatorDefault).is_ok());
    }

    /// @covers: validate
    #[test]
    fn test_validate_err_surfaces_reason_from_impl() {
        let err = validate(&AlwaysErr).unwrap_err();
        assert_eq!(err, "always fails");
    }
}

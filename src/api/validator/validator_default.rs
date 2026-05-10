//! `ValidatorDefault` — interface declaration for the no-op validator.

/// Public interface type representing a configuration with no field-level invariants.
///
/// The concrete [`Validator`](super::Validator) implementation lives in
/// `core/validator/validator_default`.
#[allow(dead_code)]
pub struct ValidatorDefault;

#[cfg(test)]
mod tests {
    use super::ValidatorDefault;

    #[test]
    fn test_validator_default_is_constructible() {
        let _ = ValidatorDefault;
    }
}

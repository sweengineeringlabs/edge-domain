//! Integration tests for Validator trait.

use edge_domain_security::{Validator, ValidationError};

struct SuccessValidator;
impl Validator for SuccessValidator {
    fn validate(&self) -> Result<(), ValidationError> {
        Ok(())
    }
}

struct FailValidator;
impl Validator for FailValidator {
    fn validate(&self) -> Result<(), ValidationError> {
        Err(ValidationError("validation failed".to_string()))
    }
}

#[test]
fn test_validate_valid_happy() {
    let validator = SuccessValidator;
    assert_eq!(validator.validate(), Ok(()));
}

#[test]
fn test_validate_invalid_error() {
    let validator = FailValidator;
    let result = validator.validate();
    assert!(result.is_err());
    assert!(matches!(result, Err(ValidationError(_))));
}

#[test]
fn test_validate_always_valid_edge() {
    let validator = SuccessValidator;
    assert_eq!(validator.validate(), Ok(()));
}

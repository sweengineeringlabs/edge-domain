//! Integration tests for [`Validator`] trait.

use edge_domain_security::{Validator, ValidationError};

struct OkValidator;
impl Validator for OkValidator {
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

/// @covers: validate
#[test]
fn test_validator_validate_happy() {
    let validator = OkValidator;
    let result = validator.validate();
    assert!(result.is_ok(), "validator must accept valid input");
    assert_eq!(result.unwrap(), (), "validate must return Ok(())");
}

/// @covers: validate
#[test]
fn test_validator_validate_error() {
    let validator = FailValidator;
    assert!(validator.validate().is_err());
}

/// @covers: validate
#[test]
fn test_validator_validate_edge() {
    let validator = OkValidator;
    let r1 = validator.validate();
    let r2 = validator.validate();
    assert_eq!(r1.is_ok(), r2.is_ok());
}

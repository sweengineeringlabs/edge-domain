//! Integration tests for [`Validator`] trait.

use edge_domain_security::Validator;

struct OkValidator;
impl Validator for OkValidator {
    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}

struct FailValidator;
impl Validator for FailValidator {
    fn validate(&self) -> Result<(), String> {
        Err("validation failed".to_string())
    }
}

/// @covers: validate
#[test]
fn test_validator_validate_happy() {
    let validator = OkValidator;
    assert!(validator.validate().is_ok());
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

//! SAF facade tests — `Validator` trait exported from the crate root.

use edge_domain_validator::{Validator, ValidatorError};

struct NonEmpty(String);
impl Validator for NonEmpty {
    fn validate(&self) -> Result<(), ValidatorError> {
        if self.0.is_empty() {
            Err(ValidatorError::Invalid("must not be empty".into()))
        } else {
            Ok(())
        }
    }
}

/// @covers: Validator::validate — valid input
#[test]
fn test_validate_valid_input_returns_ok_happy() {
    assert!(NonEmpty("hello".into()).validate().is_ok());
}

/// @covers: Validator::validate — returns Err for invalid input
#[test]
fn test_validate_empty_input_returns_err_error() {
    let result = NonEmpty("".into()).validate();
    assert!(result.is_err());
}

/// @covers: Validator::validate — works via dyn dispatch
#[test]
fn test_validate_via_dyn_trait_object_returns_ok_edge() {
    let v: &dyn Validator = &NonEmpty("x".into());
    assert!(v.validate().is_ok());
}

//! SAF facade tests — `ValidatorFactory` constructors.

use edge_domain_validator::{Validator, ValidatorFactory};

struct TestValidators;
impl ValidatorFactory for TestValidators {}

/// @covers: ValidatorFactory::always_valid — returns a passing validator
#[test]
fn test_always_valid_returns_passing_validator_happy() {
    let v = TestValidators::always_valid();
    assert!(v.validate().is_ok());
}

/// @covers: ValidatorFactory::always_valid — never rejects
#[test]
fn test_always_valid_never_rejects_error() {
    let v = TestValidators::always_valid();
    assert!(v.validate().is_ok());
}

/// @covers: ValidatorFactory::always_valid — zero-sized marker
#[test]
fn test_always_valid_is_zero_size_edge() {
    let v = TestValidators::always_valid();
    assert_eq!(std::mem::size_of_val(&v), 0);
}

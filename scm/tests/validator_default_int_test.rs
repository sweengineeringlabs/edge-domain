//! Integration tests for `ValidatorDefault`.

use edge_domain::{Validator, ValidatorDefault};

/// @covers: ValidatorDefault (Validator::validate)
#[test]
fn test_validate_returns_ok_happy() {
    assert!(ValidatorDefault.validate().is_ok());
}

/// @covers: ValidatorDefault (Validator::validate repeated)
#[test]
fn test_validate_always_returns_ok_on_repeated_calls_edge() {
    for _ in 0..3 {
        assert!(ValidatorDefault.validate().is_ok());
    }
}

/// @covers: ValidatorDefault (dyn Validator dispatch)
#[test]
fn test_validate_via_trait_object_returns_ok_error() {
    let v: &dyn Validator = &ValidatorDefault;
    assert!(v.validate().is_ok());
}

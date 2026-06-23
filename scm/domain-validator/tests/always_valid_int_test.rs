//! Integration tests for the `AlwaysValid` reference validator.

use edge_domain_validator::{AlwaysValid, Validator};

/// @covers: AlwaysValid (Validator::validate) — accepts by default
#[test]
fn test_validate_accepts_by_default_happy() {
    let result = AlwaysValid.validate();
    assert_eq!(result, Ok(()), "AlwaysValid should always succeed");
}

/// @covers: AlwaysValid — never rejects
#[test]
fn test_validate_never_returns_err_error() {
    let result = AlwaysValid.validate();
    assert_eq!(result, Ok(()), "should never return error");
}

/// @covers: AlwaysValid — usable via dyn dispatch
#[test]
fn test_validate_via_dyn_dispatch_edge() {
    let v: &dyn Validator = &AlwaysValid;
    let result = v.validate();
    assert_eq!(result, Ok(()), "dyn Validator should succeed");
}

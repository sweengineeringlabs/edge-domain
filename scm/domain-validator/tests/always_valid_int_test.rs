//! Integration tests for the `AlwaysValid` reference validator.

use edge_domain_validator::{AlwaysValid, Validator};

/// @covers: AlwaysValid (Validator::validate) — accepts by default
#[test]
fn test_validate_accepts_by_default_happy() {
    assert!(AlwaysValid.validate().is_ok());
}

/// @covers: AlwaysValid — never rejects
#[test]
fn test_validate_never_returns_err_error() {
    assert!(AlwaysValid.validate().is_ok());
}

/// @covers: AlwaysValid — usable via dyn dispatch
#[test]
fn test_validate_via_dyn_dispatch_edge() {
    let v: &dyn Validator = &AlwaysValid;
    assert!(v.validate().is_ok());
}

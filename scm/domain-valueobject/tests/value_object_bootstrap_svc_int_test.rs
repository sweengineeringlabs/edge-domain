//! Rule-222 coverage for [`ValueObjectBootstrap`] trait fn `bootstrap_name`.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_domain_valueobject::{NonEmptyString, ValueObjectBootstrap};

/// @covers: ValueObjectBootstrap::bootstrap_name
#[test]
fn test_bootstrap_name_returns_nonempty_string_happy() {
    let f = NonEmptyString::new("x").expect("valid non-empty string");
    assert!(!f.bootstrap_name().is_empty(), "bootstrap_name must return a non-empty identifier");
}

/// @covers: ValueObjectBootstrap::bootstrap_name
#[test]
fn test_bootstrap_name_is_idempotent_error() {
    let f = NonEmptyString::new("x").expect("valid non-empty string");
    assert_eq!(
        f.bootstrap_name(),
        f.bootstrap_name(),
        "bootstrap_name must return the same value on repeated calls"
    );
}

/// @covers: ValueObjectBootstrap::bootstrap_name
#[test]
fn test_bootstrap_name_is_callable_via_trait_object_edge() {
    let f = NonEmptyString::new("x").expect("valid non-empty string");
    let t: &dyn ValueObjectBootstrap = &f;
    let _ = t.bootstrap_name();
}

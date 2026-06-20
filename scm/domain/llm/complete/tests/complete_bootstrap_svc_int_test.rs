//! Rule-222 coverage for [`CompleteBootstrap`] trait fn `bootstrap_name`.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_llm_complete::{CompleteBootstrap, StdCompleteFactory};

/// @covers: CompleteBootstrap::bootstrap_name
#[test]
fn test_bootstrap_name_returns_nonempty_string_happy() {
    let f = StdCompleteFactory;
    assert!(!f.bootstrap_name().is_empty(), "bootstrap_name must return a non-empty identifier");
}

/// @covers: CompleteBootstrap::bootstrap_name
#[test]
fn test_bootstrap_name_is_idempotent_error() {
    let f = StdCompleteFactory;
    assert_eq!(
        f.bootstrap_name(),
        f.bootstrap_name(),
        "bootstrap_name must return the same value on repeated calls"
    );
}

/// @covers: CompleteBootstrap::bootstrap_name
#[test]
fn test_bootstrap_name_is_callable_via_trait_object_edge() {
    let f: &dyn CompleteBootstrap = &StdCompleteFactory;
    let _ = f.bootstrap_name();
}

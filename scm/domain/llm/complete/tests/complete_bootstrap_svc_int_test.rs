//! Rule-222 coverage for [`CompleteBootstrap`] trait fn `bootstrap_name`.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_llm_complete::{CompleteBootstrap, CompleteBootstrapNameRequest, StdCompleteFactory};

/// @covers: CompleteBootstrap::bootstrap_name
#[test]
fn test_bootstrap_name_returns_nonempty_string_happy() {
    let f = StdCompleteFactory;
    assert!(
        !f.bootstrap_name(CompleteBootstrapNameRequest)
            .unwrap()
            .name
            .is_empty(),
        "bootstrap_name must return a non-empty identifier"
    );
}

/// @covers: CompleteBootstrap::bootstrap_name
#[test]
fn test_bootstrap_name_returns_known_identifier_error() {
    let f = StdCompleteFactory;
    let name = f.bootstrap_name(CompleteBootstrapNameRequest).unwrap().name;
    assert_eq!(
        name, "complete",
        "StdCompleteFactory should return 'complete' as bootstrap_name"
    );
}

/// @covers: CompleteBootstrap::bootstrap_name
#[test]
fn test_bootstrap_name_is_callable_via_trait_object_edge() {
    let f: &dyn CompleteBootstrap = &StdCompleteFactory;
    let _ = f.bootstrap_name(CompleteBootstrapNameRequest).unwrap();
}

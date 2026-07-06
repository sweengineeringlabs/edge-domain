//! Rule-222 coverage for [`ReasoningBootstrap`] trait fn `bootstrap_name`.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_llm_reasoning::{ReasoningBootstrap, ReasoningBootstrapNameRequest, StdReasoningFactory};

/// @covers: ReasoningBootstrap::bootstrap_name
#[test]
fn test_bootstrap_name_returns_nonempty_string_happy() {
    let f = StdReasoningFactory;
    let name = f
        .bootstrap_name(ReasoningBootstrapNameRequest)
        .expect("bootstrap_name must succeed")
        .name;
    assert!(
        !name.is_empty(),
        "bootstrap_name must return a non-empty identifier"
    );
}

/// @covers: ReasoningBootstrap::bootstrap_name
#[test]
fn test_bootstrap_name_is_idempotent_error() {
    let f = StdReasoningFactory;
    let name1 = f
        .bootstrap_name(ReasoningBootstrapNameRequest)
        .expect("bootstrap_name must succeed")
        .name;
    let name2 = f
        .bootstrap_name(ReasoningBootstrapNameRequest)
        .expect("bootstrap_name must succeed")
        .name;
    assert_eq!(
        name1, name2,
        "bootstrap_name must return the same value on repeated calls"
    );
    assert_eq!(
        name1, "reasoning",
        "bootstrap_name must return expected identifier"
    );
}

/// @covers: ReasoningBootstrap::bootstrap_name
#[test]
fn test_bootstrap_name_is_callable_via_trait_object_edge() {
    let f: &dyn ReasoningBootstrap = &StdReasoningFactory;
    let _ = f
        .bootstrap_name(ReasoningBootstrapNameRequest)
        .expect("bootstrap_name must succeed");
}

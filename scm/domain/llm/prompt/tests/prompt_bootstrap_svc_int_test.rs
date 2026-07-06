//! Rule-222 coverage for [`PromptBootstrap`] trait fn `bootstrap_name`.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_llm_prompt::{PromptBootstrap, PromptBootstrapNameRequest, StdPromptFactory};

/// @covers: PromptBootstrap::bootstrap_name
#[test]
fn test_bootstrap_name_returns_nonempty_string_happy() {
    let f = StdPromptFactory;
    assert!(
        !f.bootstrap_name(PromptBootstrapNameRequest)
            .expect("bootstrap_name ok")
            .name
            .is_empty(),
        "bootstrap_name must return a non-empty identifier"
    );
}

/// @covers: PromptBootstrap::bootstrap_name
#[test]
fn test_bootstrap_name_is_idempotent_error() {
    let f = StdPromptFactory;
    let name1 = f
        .bootstrap_name(PromptBootstrapNameRequest)
        .expect("ok")
        .name;
    let name2 = f
        .bootstrap_name(PromptBootstrapNameRequest)
        .expect("ok")
        .name;
    assert_eq!(
        name1, name2,
        "bootstrap_name must return the same value on repeated calls"
    );
    assert_eq!(
        name1, "prompt",
        "bootstrap_name must return expected identifier"
    );
}

/// @covers: PromptBootstrap::bootstrap_name
#[test]
fn test_bootstrap_name_is_callable_via_trait_object_edge() {
    let f: &dyn PromptBootstrap = &StdPromptFactory;
    let _ = f.bootstrap_name(PromptBootstrapNameRequest);
}

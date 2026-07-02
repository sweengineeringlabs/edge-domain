//! Scenario coverage for the `completer_svc` SAF surface.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_llm_complete::{Completer, EchoCompleter, SupportedModelsRequest, COMPLETER_SVC};

#[test]
fn test_completer_svc_constant_is_expected_value_happy() {
    assert_eq!(COMPLETER_SVC, "completer");
}

#[test]
fn test_completer_svc_constant_is_nonempty_error() {
    assert!(!COMPLETER_SVC.is_empty());
}

#[test]
fn test_completer_trait_accessible_via_svc_surface_edge() {
    let c: &dyn Completer = &EchoCompleter;
    assert!(!c
        .supported_models(SupportedModelsRequest)
        .unwrap()
        .models
        .is_empty());
}

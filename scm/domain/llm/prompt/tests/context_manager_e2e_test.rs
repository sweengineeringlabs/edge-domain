//! End-to-end contract tests for the `ContextManager` trait, exercised through
//! the crate's reference implementation via the public API.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_llm_prompt::{
    ClearVariablesRequest, CompletenessRequest, ContextBuildRequest, ContextManager,
    MapContextManager, RegisterVariableRequest, Variable, VariableKind, VariableLookupRequest,
};

/// @covers: ContextManager::register_variable
#[test]
fn test_register_variable_stores_named_happy() {
    let mut m = MapContextManager::new();
    let var = Variable::new("a".to_string(), VariableKind::String);
    m.register_variable(RegisterVariableRequest {
        name: "a".to_string(),
        var: &var,
    })
    .expect("register ok");
    assert!(m
        .get_variable(VariableLookupRequest { name: "a" })
        .expect("get ok")
        .variable
        .is_some());
}

/// @covers: ContextManager::get_variable
#[test]
fn test_get_variable_unknown_returns_none_edge() {
    let m = MapContextManager::new();
    assert!(m
        .get_variable(VariableLookupRequest { name: "missing" })
        .expect("get ok")
        .variable
        .is_none());
}

/// @covers: ContextManager::build_context
#[test]
fn test_build_context_errors_when_required_missing() {
    let mut m = MapContextManager::new();
    let var = Variable::new("a".to_string(), VariableKind::String);
    m.register_variable(RegisterVariableRequest {
        name: "a".to_string(),
        var: &var,
    })
    .expect("register ok");
    assert!(m.build_context(ContextBuildRequest).is_err());
}

/// @covers: ContextManager::clear
#[test]
fn test_clear_removes_registered_variables() {
    let mut m = MapContextManager::new();
    let var = Variable::new("a".to_string(), VariableKind::String);
    m.register_variable(RegisterVariableRequest {
        name: "a".to_string(),
        var: &var,
    })
    .expect("register ok");
    m.clear(ClearVariablesRequest).expect("clear ok");
    assert!(m
        .get_variable(VariableLookupRequest { name: "a" })
        .expect("get ok")
        .variable
        .is_none());
}

/// @covers: ContextManager::is_complete
#[test]
fn test_is_complete_true_for_empty_manager() {
    let m = MapContextManager::new();
    assert!(m.is_complete(CompletenessRequest).expect("ok").complete);
}

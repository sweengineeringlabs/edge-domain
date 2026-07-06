//! Tests for the `MapContextManager` concrete implementation.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_llm_prompt::{
    CompletenessRequest, ContextBuildRequest, ContextManager, MapContextManager,
    RegisterVariableRequest, Variable, VariableKind,
};

fn satisfied(name: &str) -> Variable {
    Variable::with_default(
        name.to_string(),
        VariableKind::String,
        serde_json::json!("v"),
    )
}

/// @covers: MapContextManager::new — starts empty and complete
#[test]
fn test_map_context_manager_starts_empty() {
    let m = MapContextManager::new();
    assert!(m.is_empty());
    assert!(
        m.is_complete(CompletenessRequest)
            .expect("is_complete")
            .complete
    );
}

/// @covers: MapContextManager — builds a context from satisfied variables
#[test]
fn test_map_context_manager_builds_context() {
    let mut m = MapContextManager::new();
    m.register_variable(RegisterVariableRequest {
        name: "a".to_string(),
        var: &satisfied("a"),
    })
    .expect("register");
    assert_eq!(
        m.build_context(ContextBuildRequest)
            .expect("build")
            .variables
            .len(),
        1
    );
}

/// @covers: MapContextManager — empty name registration is rejected
#[test]
fn test_map_context_manager_rejects_empty_name() {
    let mut m = MapContextManager::new();
    assert!(m
        .register_variable(RegisterVariableRequest {
            name: String::new(),
            var: &satisfied("a"),
        })
        .is_err());
}

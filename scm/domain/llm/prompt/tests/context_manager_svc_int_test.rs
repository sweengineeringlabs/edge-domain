//! SAF facade tests — `ContextManager` trait via `MapContextManager`.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_llm_prompt::{ContextManager, PromptBootstrap, StdPromptFactory, Variable, VariableType};

fn manager() -> impl ContextManager {
    StdPromptFactory::context_manager()
}

fn required(name: &str) -> Variable {
    Variable::new(name.to_string(), VariableType::String)
}

fn with_value(name: &str, value: serde_json::Value) -> Variable {
    let mut v = Variable::new(name.to_string(), VariableType::String);
    v.set_value(value);
    v
}

// --- register_variable ---

/// @covers: ContextManager::register_variable — stores a named variable
#[test]
fn test_register_variable_stores_named_happy() {
    let mut m = manager();
    m.register_variable("a".to_string(), required("a"))
        .expect("register");
    assert!(m.get_variable("a").is_some());
}

/// @covers: ContextManager::register_variable — empty name is rejected
#[test]
fn test_register_variable_empty_name_rejected_error() {
    let mut m = manager();
    assert!(m.register_variable(String::new(), required("a")).is_err());
}

/// @covers: ContextManager::register_variable — re-registering overwrites
#[test]
fn test_register_variable_overwrites_edge() {
    let mut m = manager();
    m.register_variable("a".to_string(), required("a"))
        .expect("register");
    m.register_variable("a".to_string(), with_value("a", serde_json::json!("v")))
        .expect("register again");
    assert!(m.get_variable("a").expect("present").value.is_some());
}

// --- get_variable ---

/// @covers: ContextManager::get_variable — returns a registered variable
#[test]
fn test_get_variable_returns_registered_happy() {
    let mut m = manager();
    m.register_variable("a".to_string(), required("a"))
        .expect("register");
    assert_eq!(m.get_variable("a").expect("present").name, "a");
}

/// @covers: ContextManager::get_variable — unknown name returns None
#[test]
fn test_get_variable_unknown_returns_none_error() {
    let m = manager();
    assert!(m.get_variable("missing").is_none());
}

/// @covers: ContextManager::get_variable — empty name returns None
#[test]
fn test_get_variable_empty_name_none_edge() {
    let m = manager();
    assert!(m.get_variable("").is_none());
}

// --- build_context ---

/// @covers: ContextManager::build_context — builds from satisfied variables
#[test]
fn test_build_context_includes_values_happy() {
    let mut m = manager();
    m.register_variable("a".to_string(), with_value("a", serde_json::json!("v")))
        .expect("register");
    let ctx = m.build_context().expect("build");
    assert_eq!(ctx.get_variable("a"), Some(&serde_json::json!("v")));
}

/// @covers: ContextManager::build_context — errors when required is unsatisfied
#[test]
fn test_build_context_unsatisfied_required_error() {
    let mut m = manager();
    m.register_variable("a".to_string(), required("a"))
        .expect("register");
    assert!(m.build_context().is_err());
}

/// @covers: ContextManager::build_context — empty manager builds empty context
#[test]
fn test_build_context_empty_is_ok_edge() {
    let ctx = manager().build_context().expect("build");
    assert_eq!(ctx.variable_count(), 0);
}

// --- clear ---

/// @covers: ContextManager::clear — removes registered variables
#[test]
fn test_clear_removes_variables_happy() {
    let mut m = manager();
    m.register_variable("a".to_string(), required("a"))
        .expect("register");
    m.clear();
    assert!(m.get_variable("a").is_none());
}

/// @covers: ContextManager::clear — clearing twice is harmless
#[test]
fn test_clear_idempotent_error() {
    let mut m = manager();
    m.clear();
    m.clear();
    assert!(m.is_complete());
}

/// @covers: ContextManager::clear — clear on empty manager is a no-op
#[test]
fn test_clear_empty_no_op_edge() {
    let mut m = manager();
    m.clear();
    assert!(m.build_context().expect("build").variables.is_empty());
}

// --- is_complete ---

/// @covers: ContextManager::is_complete — true when all required are satisfied
#[test]
fn test_is_complete_all_satisfied_happy() {
    let mut m = manager();
    m.register_variable("a".to_string(), with_value("a", serde_json::json!("v")))
        .expect("register");
    assert!(m.is_complete());
}

/// @covers: ContextManager::is_complete — false with unsatisfied required
#[test]
fn test_is_complete_unsatisfied_required_error() {
    let mut m = manager();
    m.register_variable("a".to_string(), required("a"))
        .expect("register");
    assert!(!m.is_complete());
}

/// @covers: ContextManager::is_complete — empty manager is complete
#[test]
fn test_is_complete_empty_is_true_edge() {
    assert!(manager().is_complete());
}

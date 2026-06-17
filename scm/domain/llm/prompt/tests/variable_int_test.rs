//! Tests for the `Variable` value type.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_llm_prompt::{Variable, VariableType};

/// @covers: Variable::new — creates a required variable with no default
#[test]
fn test_variable_new_is_required() {
    let v = Variable::new("a".to_string(), VariableType::String);
    assert!(v.required);
    assert!(v.default.is_none());
}

/// @covers: Variable::with_default — default makes it optional and satisfied
#[test]
fn test_variable_with_default_is_satisfied() {
    let v = Variable::with_default(
        "a".to_string(),
        VariableType::String,
        serde_json::json!("x"),
    );
    assert!(!v.required);
    assert!(v.is_satisfied());
}

/// @covers: Variable::get_value — current value takes precedence over default
#[test]
fn test_variable_get_value_prefers_current() {
    let mut v = Variable::with_default(
        "a".to_string(),
        VariableType::String,
        serde_json::json!("d"),
    );
    v.set_value(serde_json::json!("c"));
    assert_eq!(v.get_value(), Some(&serde_json::json!("c")));
}

/// @covers: Variable::is_satisfied — unset required variable is unsatisfied
#[test]
fn test_variable_required_unset_unsatisfied() {
    let v = Variable::new("a".to_string(), VariableType::String);
    assert!(!v.is_satisfied());
}

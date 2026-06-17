//! Tests for the `VariableType` value type.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_llm_prompt::VariableType;

/// @covers: VariableType::is_scalar — string is scalar
#[test]
fn test_variable_type_string_is_scalar() {
    assert!(VariableType::String.is_scalar());
}

/// @covers: VariableType::is_scalar — list is not scalar
#[test]
fn test_variable_type_list_not_scalar() {
    assert!(!VariableType::List.is_scalar());
}

/// @covers: VariableType::as_str — reports the lowercase name
#[test]
fn test_variable_type_as_str() {
    assert_eq!(VariableType::Object.as_str(), "object");
}

/// @covers: VariableType — serde round-trip
#[test]
fn test_variable_type_serde_roundtrip() {
    let json = serde_json::to_string(&VariableType::Boolean).expect("serialize");
    let back: VariableType = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(back, VariableType::Boolean);
}

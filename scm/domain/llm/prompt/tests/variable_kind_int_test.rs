//! Tests for the `VariableKind` value type.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_llm_prompt::VariableKind;

/// @covers: VariableKind::is_scalar — string is scalar
#[test]
fn test_variable_kind_string_is_scalar() {
    assert!(VariableKind::String.is_scalar());
}

/// @covers: VariableKind::is_scalar — list is not scalar
#[test]
fn test_variable_kind_list_not_scalar() {
    assert!(!VariableKind::List.is_scalar());
}

/// @covers: VariableKind::as_str — reports the lowercase name
#[test]
fn test_variable_kind_as_str() {
    assert_eq!(VariableKind::Object.as_str(), "object");
}

/// @covers: VariableKind — serde round-trip
#[test]
fn test_variable_kind_serde_roundtrip() {
    let json = serde_json::to_string(&VariableKind::Boolean).expect("serialize");
    let back: VariableKind = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(back, VariableKind::Boolean);
}

#![allow(clippy::unwrap_used, clippy::expect_used)]
//! SAF tests for the `SchemaValidator` trait and `SCHEMA_VALIDATOR_SVC` constant.

use edge_llm_agent::{
    CacheControl, NoopSchemaValidator, SchemaValidator, ToolCall, ValidationError,
    SCHEMA_VALIDATOR_SVC,
};
use serde_json::json;

/// A validator that only accepts JSON objects carrying a `name` string field.
struct NamedSchemaValidator;

impl SchemaValidator for NamedSchemaValidator {
    fn validate(&self, input: &serde_json::Value) -> Result<(), ValidationError> {
        match input.get("name").and_then(|v| v.as_str()) {
            Some(_) => Ok(()),
            None => Err(ValidationError::new(
                "name".to_string(),
                "missing required string field 'name'".to_string(),
            )),
        }
    }
}

// --- SCHEMA_VALIDATOR_SVC ---

#[test]
fn test_schema_validator_svc_constant_value() {
    assert_eq!(SCHEMA_VALIDATOR_SVC, "schema_validator");
}

// --- validate ---

/// @covers: validate
#[test]
fn test_validate_accepts_valid_object_happy() {
    assert!(NamedSchemaValidator
        .validate(&json!({"name": "ok"}))
        .is_ok());
}

/// @covers: validate
#[test]
fn test_validate_rejects_missing_field_error() {
    let result = NamedSchemaValidator.validate(&json!({"other": 1}));
    assert!(result.is_err());
}

/// @covers: validate
#[test]
fn test_validate_rejects_non_object_edge() {
    assert!(NoopSchemaValidator.validate(&json!(42)).is_err());
}

// --- validate_tool_call ---

/// @covers: validate_tool_call
#[test]
fn test_validate_tool_call_valid_arguments_happy() {
    let call = ToolCall {
        id: "1".to_string(),
        name: "search".to_string(),
        arguments: r#"{"name":"rust"}"#.to_string(),
    };
    assert_eq!(NamedSchemaValidator.validate_tool_call(&call), Ok(()));
}

/// @covers: validate_tool_call
#[test]
fn test_validate_tool_call_malformed_json_error() {
    let call = ToolCall {
        id: "1".to_string(),
        name: "search".to_string(),
        arguments: "not json".to_string(),
    };
    assert!(NamedSchemaValidator.validate_tool_call(&call).is_err());
}

/// @covers: validate_tool_call
#[test]
fn test_validate_tool_call_schema_violation_edge() {
    let call = ToolCall {
        id: "1".to_string(),
        name: "search".to_string(),
        arguments: r#"{"missing":"name"}"#.to_string(),
    };
    assert!(NamedSchemaValidator.validate_tool_call(&call).is_err());
}

// --- cache_control ---

/// @covers: cache_control
#[test]
fn test_cache_control_defaults_ephemeral_happy() {
    let cc: CacheControl = NamedSchemaValidator.cache_control();
    assert!(cc.is_ephemeral());
}

/// @covers: cache_control
#[test]
fn test_cache_control_noop_default_error() {
    // The no-op validator inherits the same default cache-control hint.
    assert!(NoopSchemaValidator.cache_control().is_ephemeral());
}

/// @covers: cache_control
#[test]
fn test_cache_control_type_is_ephemeral_edge() {
    assert_eq!(NamedSchemaValidator.cache_control().cache_type, "ephemeral");
}

#![allow(clippy::unwrap_used, clippy::expect_used)]
//! SAF tests for the `SchemaValidator` trait and `SCHEMA_VALIDATOR_SVC` constant.

use edge_llm_agent::{
    CacheControl, NoopSchemaValidator, SchemaCacheControlRequest, SchemaValidationRequest,
    SchemaValidator, ToolCall, ToolCallValidationRequest, ValidationError, SCHEMA_VALIDATOR_SVC,
};
use serde_json::json;

/// A validator that only accepts JSON objects carrying a `name` string field.
struct NamedSchemaValidator;

impl SchemaValidator for NamedSchemaValidator {
    fn validate(&self, req: SchemaValidationRequest<'_>) -> Result<(), ValidationError> {
        match req.input.get("name").and_then(|v| v.as_str()) {
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
    let input = json!({"name": "ok"});
    assert!(matches!(
        NamedSchemaValidator.validate(SchemaValidationRequest { input: &input }),
        Ok(())
    ));
}

/// @covers: validate
#[test]
fn test_validate_rejects_missing_field_error() {
    let input = json!({"other": 1});
    let result = NamedSchemaValidator.validate(SchemaValidationRequest { input: &input });
    assert!(result.is_err());
}

/// @covers: validate
#[test]
fn test_validate_rejects_non_object_edge() {
    let input = json!(42);
    assert!(NoopSchemaValidator
        .validate(SchemaValidationRequest { input: &input })
        .is_err());
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
    assert!(matches!(
        NamedSchemaValidator.validate_tool_call(ToolCallValidationRequest { call: &call }),
        Ok(())
    ));
}

/// @covers: validate_tool_call
#[test]
fn test_validate_tool_call_malformed_json_error() {
    let call = ToolCall {
        id: "1".to_string(),
        name: "search".to_string(),
        arguments: "not json".to_string(),
    };
    assert!(NamedSchemaValidator
        .validate_tool_call(ToolCallValidationRequest { call: &call })
        .is_err());
}

/// @covers: validate_tool_call
#[test]
fn test_validate_tool_call_schema_violation_edge() {
    let call = ToolCall {
        id: "1".to_string(),
        name: "search".to_string(),
        arguments: r#"{"missing":"name"}"#.to_string(),
    };
    assert!(NamedSchemaValidator
        .validate_tool_call(ToolCallValidationRequest { call: &call })
        .is_err());
}

// --- cache_control ---

/// @covers: cache_control
#[test]
fn test_cache_control_defaults_ephemeral_happy() {
    let cc: CacheControl = *NamedSchemaValidator
        .cache_control(SchemaCacheControlRequest)
        .unwrap()
        .cache;
    assert!(cc.is_ephemeral());
}

/// @covers: cache_control
#[test]
fn test_cache_control_noop_default_error() {
    // The no-op validator inherits the same default cache-control hint.
    assert!(NoopSchemaValidator
        .cache_control(SchemaCacheControlRequest)
        .unwrap()
        .cache
        .is_ephemeral());
}

/// @covers: cache_control
#[test]
fn test_cache_control_type_is_ephemeral_edge() {
    assert_eq!(
        NamedSchemaValidator
            .cache_control(SchemaCacheControlRequest)
            .unwrap()
            .cache
            .cache_type,
        "ephemeral"
    );
}

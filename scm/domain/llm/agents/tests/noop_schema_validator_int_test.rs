#![allow(clippy::unwrap_used, clippy::expect_used)]
//! Coverage tests for the no-op `SchemaValidator` implementation.

use edge_llm_agent::{
    NoopSchemaValidator, SchemaCacheControlRequest, SchemaValidationRequest, SchemaValidator,
};
use serde_json::json;

#[test]
fn test_noop_schema_validator_accepts_object() {
    let input = json!({"k": "v"});
    assert!(matches!(
        NoopSchemaValidator.validate(SchemaValidationRequest { input: &input }),
        Ok(())
    ));
}

#[test]
fn test_noop_schema_validator_rejects_non_object() {
    let input = json!("string");
    assert!(NoopSchemaValidator
        .validate(SchemaValidationRequest { input: &input })
        .is_err());
}

#[test]
fn test_noop_schema_validator_default_cache_control_ephemeral() {
    assert!(NoopSchemaValidator
        .cache_control(SchemaCacheControlRequest)
        .unwrap()
        .cache
        .is_ephemeral());
}

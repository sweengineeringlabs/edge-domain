#![allow(clippy::unwrap_used, clippy::expect_used)]
//! Coverage tests for the no-op `SchemaValidator` implementation.

use edge_llm_agent::{NoopSchemaValidator, SchemaValidator};
use serde_json::json;

#[test]
fn test_noop_schema_validator_accepts_object() {
    assert!(NoopSchemaValidator.validate(&json!({"k": "v"})).is_ok());
}

#[test]
fn test_noop_schema_validator_rejects_non_object() {
    assert!(NoopSchemaValidator.validate(&json!("string")).is_err());
}

#[test]
fn test_noop_schema_validator_default_cache_control_ephemeral() {
    assert!(NoopSchemaValidator.cache_control().is_ephemeral());
}

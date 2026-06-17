//! No-op [`SchemaValidator`] implementation for testing the contract.

use crate::api::noop::NoopSchemaValidator;
use crate::api::{SchemaValidator, ValidationError};
use serde_json::Value;

impl SchemaValidator for NoopSchemaValidator {
    fn validate(&self, input: &Value) -> Result<(), ValidationError> {
        // Accepts any JSON object; rejects non-object payloads so the contract
        // has an observable failure mode.
        if input.is_object() {
            Ok(())
        } else {
            Err(ValidationError::new(
                "$".to_string(),
                "expected a JSON object".to_string(),
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_noop_schema_validator_accepts_object() {
        assert!(NoopSchemaValidator.validate(&json!({"k": "v"})).is_ok());
    }

    #[test]
    fn test_noop_schema_validator_rejects_non_object() {
        assert!(NoopSchemaValidator.validate(&json!(42)).is_err());
    }

    #[test]
    fn test_noop_schema_validator_validate_tool_call_parses_arguments() {
        use crate::api::ToolCall;
        let call = ToolCall {
            id: "1".to_string(),
            name: "search".to_string(),
            arguments: r#"{"q":"rust"}"#.to_string(),
        };
        assert!(NoopSchemaValidator.validate_tool_call(&call).is_ok());
    }
}

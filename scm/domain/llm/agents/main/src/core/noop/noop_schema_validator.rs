//! No-op [`SchemaValidator`] implementation for testing the contract.

use crate::api::NoopSchemaValidator;
use crate::api::SchemaValidationRequest;
use crate::api::{SchemaValidator, ValidationError};

impl SchemaValidator for NoopSchemaValidator {
    fn validate(&self, req: SchemaValidationRequest<'_>) -> Result<(), ValidationError> {
        // Accepts any JSON object; rejects non-object payloads so the contract
        // has an observable failure mode.
        if req.input.is_object() {
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
        let input = json!({"k": "v"});
        assert!(matches!(
            NoopSchemaValidator.validate(SchemaValidationRequest { input: &input }),
            Ok(())
        ));
    }

    #[test]
    fn test_noop_schema_validator_rejects_non_object() {
        let input = json!(42);
        assert!(NoopSchemaValidator
            .validate(SchemaValidationRequest { input: &input })
            .is_err());
    }

    #[test]
    fn test_noop_schema_validator_validate_tool_call_parses_arguments() {
        use crate::api::ToolCall;
        use crate::api::ToolCallValidationRequest;
        let call = ToolCall {
            id: "1".to_string(),
            name: "search".to_string(),
            arguments: r#"{"q":"rust"}"#.to_string(),
        };
        assert!(matches!(
            NoopSchemaValidator.validate_tool_call(ToolCallValidationRequest { call: &call }),
            Ok(())
        ));
    }
}

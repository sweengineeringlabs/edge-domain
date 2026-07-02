use serde_json::Value;

/// Request for [`SchemaValidator::validate`](crate::api::traits::SchemaValidator::validate).
#[derive(Debug, Clone, Copy)]
pub struct SchemaValidationRequest<'a> {
    /// The input payload to validate against the schema.
    pub input: &'a Value,
}

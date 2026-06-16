use serde_json::Value;
use crate::api::types::ValidationError;

/// Validate skill parameters
pub trait SchemaValidator: Send + Sync {
    fn validate(&self, input: &Value) -> Result<(), ValidationError>;
}

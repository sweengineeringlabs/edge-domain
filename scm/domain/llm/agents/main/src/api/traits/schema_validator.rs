use crate::api::types::ValidationError;
use crate::api::types::{CacheControl, ToolCall};
use serde_json::Value;

/// Validate skill parameters against a declared schema.
pub trait SchemaValidator: Send + Sync {
    /// Validate `input`, returning a [`ValidationError`] describing the first
    /// offending field when the payload does not satisfy the schema.
    fn validate(&self, input: &Value) -> Result<(), ValidationError>;

    /// Validate the arguments carried by a tool call.
    ///
    /// The default parses [`ToolCall::arguments`] as JSON and delegates to
    /// [`SchemaValidator::validate`], rejecting malformed JSON.
    fn validate_tool_call(&self, call: &ToolCall) -> Result<(), ValidationError> {
        let parsed: Value = serde_json::from_str(&call.arguments)
            .map_err(|e| ValidationError::new("arguments".to_string(), e.to_string()))?;
        self.validate(&parsed)
    }

    /// The cache-control hint to attach to messages this validator approves.
    ///
    /// Defaults to an ephemeral hint; validators may override to disable or
    /// customise prompt caching.
    fn cache_control(&self) -> CacheControl {
        CacheControl::ephemeral()
    }
}

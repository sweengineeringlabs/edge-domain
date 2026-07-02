use serde_json::Value;

use crate::api::types::ValidationError;
use crate::api::types::{
    SchemaCacheControlRequest, SchemaCacheControlResponse, SchemaValidationRequest,
    ToolCallValidationRequest,
};

/// Validate skill parameters against a declared schema.
pub trait SchemaValidator: Send + Sync {
    /// Validate `input`, returning a [`ValidationError`] describing the first
    /// offending field when the payload does not satisfy the schema.
    fn validate(&self, req: SchemaValidationRequest<'_>) -> Result<(), ValidationError>;

    /// Validate the arguments carried by a tool call.
    ///
    /// The default parses the call's arguments as JSON and delegates to
    /// [`SchemaValidator::validate`], rejecting malformed JSON.
    fn validate_tool_call(
        &self,
        req: ToolCallValidationRequest<'_>,
    ) -> Result<(), ValidationError> {
        let parsed: Value = serde_json::from_str(&req.call.arguments)
            .map_err(|e| ValidationError::new("arguments".to_string(), e.to_string()))?;
        self.validate(SchemaValidationRequest { input: &parsed })
    }

    /// The cache-control hint to attach to messages this validator approves.
    ///
    /// Defaults to an ephemeral hint; validators may override to disable or
    /// customise prompt caching.
    fn cache_control(
        &self,
        _req: SchemaCacheControlRequest,
    ) -> Result<SchemaCacheControlResponse, ValidationError> {
        Ok(SchemaCacheControlResponse {
            cache: Box::new(crate::api::types::CacheControl::ephemeral()),
        })
    }
}

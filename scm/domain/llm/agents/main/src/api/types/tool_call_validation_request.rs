use crate::api::types::ToolCall;

/// Request for [`SchemaValidator::validate_tool_call`](crate::api::traits::SchemaValidator::validate_tool_call).
#[derive(Debug, Clone, Copy)]
pub struct ToolCallValidationRequest<'a> {
    /// The tool call whose arguments should be validated.
    pub call: &'a ToolCall,
}

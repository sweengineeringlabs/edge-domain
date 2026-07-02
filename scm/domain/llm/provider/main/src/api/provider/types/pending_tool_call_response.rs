use crate::api::provider::types::ToolCallDelta;

/// Response for [`StreamHandler::pending_tool_call`](crate::api::provider::traits::StreamHandler::pending_tool_call).
#[derive(Debug, Clone)]
pub struct PendingToolCallResponse {
    /// The tool call currently being assembled, if any.
    pub tool_call: Option<Box<ToolCallDelta>>,
}

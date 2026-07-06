use crate::api::complete::types::ToolCall;

/// Request for [`ToolOps::execute`](crate::api::complete::traits::ToolOps::execute).
#[derive(Debug, Clone, Copy)]
pub struct ToolExecutionRequest<'a> {
    /// Tool invocation to execute.
    pub call: &'a ToolCall,
}

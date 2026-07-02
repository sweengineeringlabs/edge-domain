use crate::api::complete::types::ToolDefinition;

/// Response for [`ToolOps::available_tools`](crate::api::complete::traits::ToolOps::available_tools).
#[derive(Debug, Clone, PartialEq)]
pub struct AvailableToolsResponse {
    /// Tool definitions exposed to the model.
    pub tools: Vec<ToolDefinition>,
}

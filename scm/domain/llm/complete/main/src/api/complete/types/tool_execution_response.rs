/// Response for [`ToolOps::execute`](crate::api::complete::traits::ToolOps::execute).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ToolExecutionResponse {
    /// Output produced by the tool.
    pub output: String,
}

//! `ToolOps` — execution contract for tool-augmented completions.

use crate::api::complete::errors::CompleteError;
use crate::api::complete::types::{
    AvailableToolsRequest, AvailableToolsResponse, DeltaMergeRequest, ToolChoicePreferenceRequest,
    ToolChoicePreferenceResponse, ToolExecutionRequest, ToolExecutionResponse,
};

/// Execution contract for completers that support tool calling.
pub trait ToolOps: Send + Sync {
    /// Execute a tool invocation emitted by the model.
    fn execute(
        &self,
        req: ToolExecutionRequest<'_>,
    ) -> Result<ToolExecutionResponse, CompleteError>;

    /// Tool definitions exposed to the model.
    fn available_tools(
        &self,
        req: AvailableToolsRequest,
    ) -> Result<AvailableToolsResponse, CompleteError>;

    /// Tool calling preference for this completer.
    fn tool_choice(
        &self,
        req: ToolChoicePreferenceRequest,
    ) -> Result<ToolChoicePreferenceResponse, CompleteError>;

    /// Merge an incremental streaming fragment into an existing delta.
    fn merge_delta(&self, req: DeltaMergeRequest<'_>) -> Result<(), CompleteError>;
}

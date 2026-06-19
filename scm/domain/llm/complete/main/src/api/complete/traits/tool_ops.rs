//! `ToolOps` — execution contract for tool-augmented completions.

use crate::api::complete::errors::CompleteError;
use crate::api::complete::types::{ToolCall, ToolCallDelta, ToolChoice, ToolDefinition};

/// Execution contract for completers that support tool calling.
pub trait ToolOps: Send + Sync {
    /// Execute a tool invocation emitted by the model.
    fn execute(&self, call: &ToolCall) -> Result<String, CompleteError>;

    /// Tool definitions exposed to the model.
    fn available_tools(&self) -> Vec<ToolDefinition>;

    /// Tool calling preference for this completer.
    fn tool_choice(&self) -> ToolChoice;

    /// Merge an incremental streaming fragment into an existing delta.
    fn merge_delta(&self, existing: &mut ToolCallDelta, incoming: ToolCallDelta);
}

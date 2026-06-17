//! `StreamHandler` — incremental streamed-response accumulation contract.

use crate::api::provider::types::{StreamChunk, StreamDelta, ToolCallDelta};

/// Accumulates streamed [`StreamDelta`]s into completed [`StreamChunk`]s.
pub trait StreamHandler: Send + Sync {
    /// Pop the next fully-formed chunk, if one is ready.
    fn next_chunk(&mut self) -> Option<StreamChunk>;

    /// Fold an incremental delta into the in-progress response.
    fn accumulate(&mut self, delta: StreamDelta);

    /// The tool call currently being assembled, if any.
    fn pending_tool_call(&self) -> Option<ToolCallDelta>;
}

//! `StreamHandler` — incremental streamed-response accumulation contract.

use crate::api::provider::errors::ExecutionError;
use crate::api::provider::types::{
    AccumulateRequest, NextChunkRequest, NextChunkResponse, PendingToolCallRequest,
    PendingToolCallResponse,
};

/// Accumulates streamed deltas into completed chunks.
pub trait StreamHandler: Send + Sync {
    /// Pop the next fully-formed chunk, if one is ready.
    fn next_chunk(&mut self, req: NextChunkRequest) -> Result<NextChunkResponse, ExecutionError>;

    /// Fold an incremental delta into the in-progress response.
    fn accumulate(&mut self, req: AccumulateRequest) -> Result<(), ExecutionError>;

    /// The tool call currently being assembled, if any.
    fn pending_tool_call(
        &self,
        req: PendingToolCallRequest,
    ) -> Result<PendingToolCallResponse, ExecutionError>;
}

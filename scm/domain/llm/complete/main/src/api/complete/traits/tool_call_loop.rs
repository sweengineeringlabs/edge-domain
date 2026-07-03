//! `ToolCallLoop` — multi-turn tool-augmented conversation contract.

use async_trait::async_trait;

use crate::api::complete::errors::CompleteError;
use crate::api::complete::types::{ToolCallLoopRequest, ToolCallLoopResponse};

/// Orchestrates a tool-augmented conversation: call the model, execute any
/// requested tool calls, feed results back, and repeat until the model stops
/// requesting tools or `max_turns` is reached.
///
/// Composes [`Completer`](crate::api::complete::traits::Completer) (one model call) and
/// [`ToolOps`](crate::api::complete::traits::ToolOps) (one tool invocation) — neither trait
/// owns multi-turn orchestration itself.
#[async_trait]
pub trait ToolCallLoop: Send + Sync {
    /// Run the loop to completion.
    ///
    /// # Errors
    ///
    /// Returns [`CompleteError::TurnLimitExceeded`] if `max_turns` is reached without a
    /// terminal finish reason, or propagates the first [`CompleteError`] from either the
    /// model call or a tool execution.
    async fn run(
        &self,
        req: ToolCallLoopRequest<'_>,
    ) -> Result<ToolCallLoopResponse, CompleteError>;
}

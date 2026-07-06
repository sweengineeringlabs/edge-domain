use crate::api::complete::types::{CompletionResponse, Message};

/// Response for [`ToolCallLoop::run`](crate::api::complete::traits::ToolCallLoop::run).
#[derive(Debug, Clone)]
pub struct ToolCallLoopResponse {
    /// The final (terminal) completion — the turn where `finish_reason` was no longer
    /// [`FinishReason::ToolCalls`](crate::api::complete::types::FinishReason::ToolCalls).
    pub response: CompletionResponse,
    /// The full transcript, including assistant tool-call turns and tool-result
    /// round-trips, ready to seed a follow-up call.
    pub messages: Vec<Message>,
    /// Number of model turns actually made.
    pub turns: u32,
}

use crate::api::complete::types::CompletionRequest;

/// Request for [`ToolCallLoop::run`](crate::api::complete::traits::ToolCallLoop::run).
#[derive(Debug, Clone, Copy)]
pub struct ToolCallLoopRequest<'a> {
    /// The initial completion request. The caller sets `model`/`tools`/`tool_choice`;
    /// `messages` seeds the conversation.
    pub request: &'a CompletionRequest,
    /// Maximum number of model turns before the loop gives up with
    /// [`CompleteError::TurnLimitExceeded`](crate::api::complete::errors::CompleteError::TurnLimitExceeded).
    /// Must be greater than zero.
    pub max_turns: u32,
}

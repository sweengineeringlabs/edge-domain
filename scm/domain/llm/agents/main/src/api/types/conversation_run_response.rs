use crate::api::types::Message;

/// Response for [`ConversationLoop::run`](crate::api::traits::ConversationLoop::run).
#[derive(Debug, Clone)]
pub struct ConversationRunResponse {
    /// Full conversation history, including every turn taken.
    pub messages: Vec<Message>,
    /// Number of turns actually executed.
    pub turns: u32,
}

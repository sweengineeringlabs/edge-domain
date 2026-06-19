use serde::{Deserialize, Serialize};

/// Role of a participant in a completion conversation.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum MessageRole {
    /// End-user turn.
    User,
    /// Model response turn.
    Assistant,
    /// Tool result injected back into the conversation.
    Tool,
}

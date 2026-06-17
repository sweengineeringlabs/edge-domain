use serde::{Deserialize, Serialize};

/// The role of a message in a conversation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Role {
    /// System role — instructions that frame the conversation.
    System,
    /// User role — input from the end user.
    User,
    /// Assistant role — responses from the model.
    Assistant,
    /// Tool role — results returned from a tool call.
    Tool,
}

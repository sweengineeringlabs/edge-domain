use serde::{Deserialize, Serialize};

/// Message author role in a conversation turn.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    /// System-level instruction.
    System,
    /// Human turn.
    #[default]
    User,
    /// Model turn.
    Assistant,
    /// Tool result injected back into the conversation.
    Tool,
}

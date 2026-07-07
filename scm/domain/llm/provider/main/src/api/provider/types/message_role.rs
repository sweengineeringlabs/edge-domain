use serde::{Deserialize, Serialize};

/// Role of a participant in a completion conversation.
///
/// Orphan-type note: only ever appears nested inside
/// [`CompletionMessage`](super::CompletionMessage), never directly in a trait method signature.
/// Same rationale as `CompletionInput`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum MessageRole {
    /// End-user turn.
    User,
    /// Model response turn.
    Assistant,
    /// Tool result injected back into the conversation.
    Tool,
}

use serde::{Deserialize, Serialize};

/// Role of a participant in a completion conversation.
///
/// Orphan-type note: only ever appears nested inside
/// [`CompletionMessage`](super::CompletionMessage), which is itself only ever nested inside
/// [`CompletionInput`](super::CompletionInput) — never directly in a trait method signature.
/// Same rationale as `CompletionMessage`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum MessageRole {
    /// End-user turn.
    User,
    /// Model response turn.
    Assistant,
    /// Tool result injected back into the conversation.
    Tool,
}

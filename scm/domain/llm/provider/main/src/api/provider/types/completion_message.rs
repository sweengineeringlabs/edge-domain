use serde::{Deserialize, Serialize};

use crate::api::provider::types::MessageRole;

/// A single message in a multi-turn completion conversation.
///
/// Orphan-type note: only ever appears nested inside [`CompletionInput`](super::CompletionInput),
/// never directly in a trait method signature. Same rationale as `CompletionInput` itself.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CompletionMessage {
    /// Who produced this message.
    pub role: MessageRole,
    /// Text content of the message.
    pub content: String,
}

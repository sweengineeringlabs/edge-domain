use serde::{Deserialize, Serialize};

use crate::api::provider::types::MessageRole;

/// A single message in a multi-turn completion conversation.
///
/// Orphan-type note: only ever appears nested inside [`CompletionInput`](super::CompletionInput),
/// which is carried by [`ProviderCompleteRequest`](super::ProviderCompleteRequest) — the request
/// type of [`Provider::complete`](crate::api::provider::traits::Provider::complete) — never
/// directly in a trait method signature. `no_orphan_types` only checks direct references, not
/// nested ones, so this still flags as an orphan even though the type is live-wired.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CompletionMessage {
    /// Who produced this message.
    pub role: MessageRole,
    /// Text content of the message.
    pub content: String,
}

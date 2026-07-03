use std::sync::Arc;

/// Reference [`ConversationLoop`](crate::api::traits::ConversationLoop) implementation:
/// a bounded multi-turn conversation loop over `agent`. Construction and the trait impl
/// live in `core::conversation` — api/ is a declaration layer only.
pub struct BoundedConversationLoop {
    pub(crate) agent: Arc<dyn crate::api::traits::Agent>,
}

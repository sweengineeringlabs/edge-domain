use std::sync::Arc;

/// Request for [`AgentManager::conversation_loop`](crate::api::traits::AgentManager::conversation_loop).
pub struct ConversationLoopRequest {
    /// The agent the built loop will drive.
    pub agent: Arc<dyn crate::api::traits::Agent>,
}

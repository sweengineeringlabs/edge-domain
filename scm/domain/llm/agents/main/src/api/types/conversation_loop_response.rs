/// Response for [`AgentManager::conversation_loop`](crate::api::traits::AgentManager::conversation_loop).
pub struct ConversationLoopResponse {
    /// The constructed conversation loop.
    pub conversation_loop: Box<dyn crate::api::traits::ConversationLoop>,
}

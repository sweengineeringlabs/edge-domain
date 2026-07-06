use crate::api::types::{ConversationRunRequest, ConversationRunResponse};
use crate::api::AgentError;

/// Drives an [`Agent`](crate::api::traits::Agent) through a bounded multi-turn conversation:
/// each turn requests a completion from the agent's provider and, if the completion requests
/// a skill call, executes it and feeds the result back before continuing.
#[async_trait::async_trait]
pub trait ConversationLoop: Send + Sync {
    /// Run the loop to completion or until `req.max_turns` is reached.
    async fn run(&self, req: ConversationRunRequest)
        -> Result<ConversationRunResponse, AgentError>;
}

use std::sync::Arc;

use edge_llm_provider::Provider;

/// Response for [`Agent::provider`](crate::api::traits::Agent::provider).
pub struct AgentProviderResponse {
    /// The LLM provider this agent delegates completions to.
    pub provider: Arc<dyn Provider>,
}

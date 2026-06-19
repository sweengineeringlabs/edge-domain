//! AgentEndpoint trait — factory for dispatchable agent skill handlers and default agents.

use std::sync::Arc;

use edge_domain_handler::Handler;
use edge_llm_provider::Provider;

use super::agent::Agent;
use super::skill::Skill;

/// Factory for creating `Handler` instances routed to a named agent skill,
/// and for instantiating concrete [`Agent`] implementations.
pub trait AgentEndpoint {
    /// Construct a dispatchable handler that routes requests to `skill`.
    fn agent_handler(
        skill: impl Into<String>,
    ) -> impl Handler<Request = String, Response = String>;

    /// Construct a concrete [`Agent`] backed by the given provider and skill registry.
    ///
    /// The returned agent delegates `execute_skill` calls to the matching skill
    /// in `skills` and exposes the provider via [`Agent::provider`].
    fn default_agent(
        id: impl Into<String>,
        name: impl Into<String>,
        description: impl Into<String>,
        provider: Arc<dyn Provider>,
        skills: Vec<Arc<dyn Skill<Request = String, Response = String>>>,
    ) -> Arc<dyn Agent>;
}

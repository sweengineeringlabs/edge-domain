//! AgentEndpoint trait — factory for dispatchable agent skill handlers.

use edge_domain_handler::Handler;

/// Factory for creating `Handler` instances routed to a named agent skill.
///
/// Implementations return a handler that can be wired into the dispatch table
/// and invoked via `Handler::execute`.
pub trait AgentEndpoint {
    /// Construct a dispatchable handler that routes requests to `skill`.
    fn agent_handler(
        skill: impl Into<String>,
    ) -> impl Handler<Request = String, Response = String>;
}

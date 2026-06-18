//! SAF factory for agent dispatch handlers.

use edge_domain_handler::Handler;

use crate::core::types::DefaultAgentHandler;

/// Service identifier for agent handler discovery.
pub const AGENT_ENDPOINT_SVC: &str = "agent_endpoint";

/// Construct a dispatchable agent handler routed to the named skill.
pub fn agent_handler(
    skill: impl Into<String>,
) -> impl Handler<Request = String, Response = String> {
    DefaultAgentHandler { skill: skill.into() }
}

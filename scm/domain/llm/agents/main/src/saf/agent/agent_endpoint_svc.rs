//! Service export for the `AgentEndpoint` type (ADR-037 connection).

pub use crate::api::AgentEndpoint;

/// Service identifier for AgentEndpoint discovery.
pub const AGENT_ENDPOINT_SVC: &str = "agent_endpoint";

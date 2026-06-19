//! SAF service constant and trait re-export for agent dispatch.

/// Service identifier for agent handler discovery.
pub const AGENT_ENDPOINT_SVC: &str = "agent_endpoint";

pub use crate::api::AgentEndpoint;

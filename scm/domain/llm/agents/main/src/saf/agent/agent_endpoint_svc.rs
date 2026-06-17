//! Service export for the `DefaultAgent` type (ADR-037 connection).

pub use crate::api::DefaultAgent;

/// Service identifier for DefaultAgent discovery.
pub const AGENT_ENDPOINT_SVC: &str = "agent_endpoint";

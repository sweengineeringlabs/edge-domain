//! Skill trait — named capability that extends Handler.

use edge_domain_handler::Handler;

/// A Skill is a named capability an Agent can execute.
///
/// Semantically, a Skill IS-A Handler — it processes requests to produce responses.
/// This allows skills to:
/// - Be invoked directly over ingress/HTTP, gRPC, async queue
/// - Benefit from middleware (auth, rate-limit, cache, trace)
/// - Compose naturally with other domain contracts
pub trait Skill: Handler + Send + Sync {
    /// Skill name (e.g., "code_review", "planning", "memory_retrieve").
    fn name(&self) -> &str;

    /// Human-readable description of what this skill does.
    fn description(&self) -> &str;

    /// Optional: list of input parameters this skill accepts.
    fn parameters(&self) -> Vec<Parameter> {
        vec![]
    }
}

/// Describes a skill parameter for discovery and documentation.
#[derive(Debug, Clone)]
pub struct Parameter {
    pub name: String,
    pub description: String,
    pub param_type: String, // e.g., "string", "number", "object"
    pub required: bool,
}

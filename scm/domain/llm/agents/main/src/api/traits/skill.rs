//! Skill trait — named capability that extends Handler.

use crate::api::types::SkillMetadata;

use super::parameter::Parameter;
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

    /// Get skill metadata including documentation and schemas.
    fn metadata(&self) -> SkillMetadata {
        SkillMetadata {
            name: self.name().to_string(),
            description: self.description().to_string(),
            input_schema: None,
            output_schema: None,
            async_execution: true,
            long_running: false,
        }
    }
}

//! Validator trait — validates agent and skill configurations.

use crate::api::types::{
    AgentIdValidationRequest, SkillInputValidationRequest, SkillNameValidationRequest,
    ValidationError,
};

/// A validator checks agent and skill configurations for correctness and compliance.
///
/// Implementations verify that configurations meet requirements before agents are deployed.
pub trait Validator: Send + Sync {
    /// Validate an agent identifier.
    fn validate_agent_id(&self, req: AgentIdValidationRequest<'_>) -> Result<(), ValidationError>;

    /// Validate a skill name.
    fn validate_skill_name(
        &self,
        req: SkillNameValidationRequest<'_>,
    ) -> Result<(), ValidationError>;

    /// Validate skill input payload.
    fn validate_skill_input(
        &self,
        req: SkillInputValidationRequest<'_>,
    ) -> Result<(), ValidationError>;
}

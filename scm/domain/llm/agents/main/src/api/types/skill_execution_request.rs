use edge_domain_handler::HandlerContext;

/// Request for [`Agent::execute_skill`](crate::api::traits::Agent::execute_skill).
pub struct SkillExecutionRequest<'a> {
    /// Name of the skill to execute (e.g., "code_review").
    pub skill_name: &'a str,
    /// Serialized input (typically JSON) to the skill.
    pub input: String,
    /// Handler context with security principal and observer seam.
    pub ctx: HandlerContext<'a>,
}

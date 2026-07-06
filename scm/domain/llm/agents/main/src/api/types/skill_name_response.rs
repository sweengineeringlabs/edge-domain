/// Response for [`Skill::name`](crate::api::traits::Skill::name).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SkillNameResponse {
    /// Skill name (e.g., "code_review", "planning", "memory_retrieve").
    pub name: String,
}

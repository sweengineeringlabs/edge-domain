/// Response for [`Skill::description`](crate::api::traits::Skill::description).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SkillDescriptionResponse {
    /// Human-readable description of what this skill does.
    pub description: String,
}

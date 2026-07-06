use crate::api::Parameter;

/// Response for [`Skill::parameters`](crate::api::traits::Skill::parameters).
#[derive(Debug, Clone)]
pub struct SkillParametersResponse {
    /// The input parameters this skill accepts.
    pub parameters: Vec<Parameter>,
}

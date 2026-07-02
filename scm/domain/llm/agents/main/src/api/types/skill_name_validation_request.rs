/// Request for [`Validator::validate_skill_name`](crate::api::traits::Validator::validate_skill_name).
#[derive(Debug, Clone, Copy)]
pub struct SkillNameValidationRequest<'a> {
    /// The skill name to validate (e.g., "code_review").
    pub skill_name: &'a str,
}

/// Request for [`Validator::validate_skill_input`](crate::api::traits::Validator::validate_skill_input).
#[derive(Debug, Clone, Copy)]
pub struct SkillInputValidationRequest<'a> {
    /// The input payload (typically JSON string) to validate.
    pub input: &'a str,
}

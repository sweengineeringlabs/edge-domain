/// Request for [`Reasoning::validate_problem`](crate::api::reasoning::traits::Reasoning::validate_problem).
#[derive(Debug, Clone, Copy)]
pub struct ProblemValidationRequest<'a> {
    /// Problem statement to validate.
    pub problem: &'a str,
}

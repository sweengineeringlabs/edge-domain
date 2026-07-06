use crate::api::types::ParameterDocumentation;

/// Response for [`Skill::parameter_documentation`](crate::api::traits::Skill::parameter_documentation).
#[derive(Debug, Clone)]
pub struct ParameterDocumentationListResponse {
    /// Structured documentation for each input parameter.
    pub documentation: Vec<ParameterDocumentation>,
}

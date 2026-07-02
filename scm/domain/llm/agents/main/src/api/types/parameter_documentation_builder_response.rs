use crate::api::types::ParameterDocumentationBuilder;

/// Response for [`Skill::parameter_documentation_builder`](crate::api::traits::Skill::parameter_documentation_builder).
pub struct ParameterDocumentationBuilderResponse {
    /// A fluent builder for the named parameter's documentation.
    pub builder: Box<ParameterDocumentationBuilder>,
}

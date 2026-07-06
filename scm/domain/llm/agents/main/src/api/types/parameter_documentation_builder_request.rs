/// Request for [`Skill::parameter_documentation_builder`](crate::api::traits::Skill::parameter_documentation_builder).
#[derive(Debug, Clone)]
pub struct ParameterDocumentationBuilderRequest {
    /// The parameter name.
    pub name: String,
    /// Human-readable description of the parameter.
    pub description: String,
    /// The parameter's type (e.g. "string", "number", "object").
    pub param_type: String,
    /// Whether the parameter is required.
    pub required: bool,
}

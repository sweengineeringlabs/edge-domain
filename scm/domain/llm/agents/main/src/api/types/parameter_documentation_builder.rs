//! Builder for [`ParameterDocumentation`] with a fluent API.

use serde_json::Value;

/// Builder for [`ParameterDocumentation`] with fluent setters.
///
/// The four required fields (`name`, `description`, `param_type`, `required`)
/// are supplied up front via [`ParameterDocumentationBuilder::new`]; the
/// optional fields default to empty/`None` until overridden.
#[derive(Debug, Clone)]
pub struct ParameterDocumentationBuilder {
    pub(crate) name: String,
    pub(crate) description: String,
    pub(crate) param_type: String,
    pub(crate) required: bool,
    pub(crate) default: Option<Value>,
    pub(crate) examples: Vec<Value>,
    pub(crate) validation_rules: Option<String>,
}

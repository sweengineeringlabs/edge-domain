//! `PromptTemplateBuilder` — fluent builder for [`PromptTemplate`].

use crate::api::prompt::types::Variable;

/// Fluent builder for [`PromptTemplate`](crate::api::prompt::types::PromptTemplate).
#[derive(Clone, Debug, Default)]
pub struct PromptTemplateBuilder {
    pub(crate) id: String,
    pub(crate) name: String,
    pub(crate) category: String,
    pub(crate) system_prompt: String,
    pub(crate) user_template: String,
    pub(crate) description: Option<String>,
    pub(crate) variables: Vec<Variable>,
}

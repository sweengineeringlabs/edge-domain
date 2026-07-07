//! `PromptTemplateBuilder` — fluent builder for [`PromptTemplate`].

use crate::api::prompt::types::Variable;

/// Fluent builder for [`PromptTemplate`](crate::api::prompt::types::PromptTemplate).
///
/// Orphan-type note: `TemplateProvider`'s methods key/return templates by id (`TemplateLookupRequest`/
/// `Response`), never this builder directly — plain builder, no interface behind it, same
/// rationale as `PromptCacheBuilder`.
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

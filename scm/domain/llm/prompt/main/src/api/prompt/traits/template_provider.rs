//! `TemplateProvider` — prompt template registry/catalog contract.

use crate::api::prompt::types::PromptTemplate;

/// A registry of prompt templates: lookup by id, list all, or filter by category.
pub trait TemplateProvider: Send + Sync {
    /// Fetch a template by `id`, or `None` if it is not registered.
    fn get_template(&self, id: &str) -> Option<&PromptTemplate>;

    /// List every registered template.
    fn list_templates(&self) -> Vec<&PromptTemplate>;

    /// List the templates whose `category` matches `category`.
    fn list_by_category(&self, category: &str) -> Vec<&PromptTemplate>;
}

//! `TemplateProvider` — prompt template registry/catalog contract.

use crate::api::prompt::errors::PromptError;
use crate::api::prompt::types::{
    ListByCategoryRequest, ListByCategoryResponse, ListTemplatesRequest, ListTemplatesResponse,
    TemplateLookupRequest, TemplateLookupResponse,
};

/// A registry of prompt templates: lookup by id, list all, or filter by category.
pub trait TemplateProvider: Send + Sync {
    /// Fetch a template by `id`, or `None` if it is not registered.
    fn get_template(
        &self,
        req: TemplateLookupRequest<'_>,
    ) -> Result<TemplateLookupResponse<'_>, PromptError>;

    /// List every registered template.
    fn list_templates(
        &self,
        req: ListTemplatesRequest,
    ) -> Result<ListTemplatesResponse<'_>, PromptError>;

    /// List the templates whose `category` matches `category`.
    fn list_by_category(
        &self,
        req: ListByCategoryRequest<'_>,
    ) -> Result<ListByCategoryResponse<'_>, PromptError>;
}

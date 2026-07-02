//! Constructors and `TemplateProvider` impl for `CatalogTemplateProvider`.

use std::collections::BTreeMap;

use crate::api::CatalogTemplateProvider;
use crate::api::PromptError;
use crate::api::PromptTemplate;
use crate::api::TemplateProvider;
use crate::api::{
    ListByCategoryRequest, ListByCategoryResponse, ListTemplatesRequest, ListTemplatesResponse,
    TemplateLookupRequest, TemplateLookupResponse,
};

impl CatalogTemplateProvider {
    /// Construct an empty registry.
    pub fn new() -> Self {
        Self::default()
    }

    /// Construct a registry seeded with `templates` (keyed by each template's `id`).
    pub fn with_templates(templates: Vec<PromptTemplate>) -> Self {
        let mut map = BTreeMap::new();
        for template in templates {
            map.insert(template.id.clone(), template);
        }
        Self { templates: map }
    }

    /// Insert or replace a template (keyed by its `id`).
    pub fn insert(&mut self, template: PromptTemplate) {
        self.templates.insert(template.id.clone(), template);
    }

    /// Number of registered templates.
    pub fn len(&self) -> usize {
        self.templates.len()
    }

    /// Whether no templates are registered.
    pub fn is_empty(&self) -> bool {
        self.templates.is_empty()
    }
}

impl TemplateProvider for CatalogTemplateProvider {
    fn get_template(
        &self,
        req: TemplateLookupRequest<'_>,
    ) -> Result<TemplateLookupResponse<'_>, PromptError> {
        Ok(TemplateLookupResponse {
            template: self.templates.get(req.id),
        })
    }

    fn list_templates(
        &self,
        _req: ListTemplatesRequest,
    ) -> Result<ListTemplatesResponse<'_>, PromptError> {
        Ok(ListTemplatesResponse {
            templates: self.templates.values().collect(),
        })
    }

    fn list_by_category(
        &self,
        req: ListByCategoryRequest<'_>,
    ) -> Result<ListByCategoryResponse<'_>, PromptError> {
        Ok(ListByCategoryResponse {
            templates: self
                .templates
                .values()
                .filter(|template| template.category == req.category)
                .collect(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn template(id: &str, category: &str) -> PromptTemplate {
        PromptTemplate::new(id.to_string(), id.to_string(), category.to_string())
    }

    #[test]
    fn test_get_template_known_id_returns_template() {
        let provider =
            CatalogTemplateProvider::with_templates(vec![template("code-review", "code")]);
        let found = provider
            .get_template(TemplateLookupRequest { id: "code-review" })
            .expect("get ok")
            .template;
        assert!(found.is_some());
        assert_eq!(found.expect("present").id, "code-review");
    }

    #[test]
    fn test_get_template_unknown_id_returns_none() {
        let provider = CatalogTemplateProvider::new();
        let found = provider
            .get_template(TemplateLookupRequest { id: "missing" })
            .expect("get ok")
            .template;
        assert!(found.is_none());
    }

    #[test]
    fn test_list_templates_returns_all_registered() {
        let provider = CatalogTemplateProvider::with_templates(vec![
            template("a", "code"),
            template("b", "general"),
        ]);
        let result = provider
            .list_templates(ListTemplatesRequest)
            .expect("list ok");
        assert_eq!(result.templates.len(), 2);
    }

    #[test]
    fn test_list_by_category_returns_only_matching_category() {
        let provider = CatalogTemplateProvider::with_templates(vec![
            template("a", "code"),
            template("b", "general"),
            template("c", "code"),
        ]);
        let code = provider
            .list_by_category(ListByCategoryRequest { category: "code" })
            .expect("list ok")
            .templates;
        assert_eq!(code.len(), 2);
        assert!(code.iter().all(|t| t.category == "code"));
    }

    #[test]
    fn test_list_by_category_unknown_category_is_empty() {
        let provider = CatalogTemplateProvider::with_templates(vec![template("a", "code")]);
        let result = provider
            .list_by_category(ListByCategoryRequest { category: "nope" })
            .expect("list ok")
            .templates;
        assert!(result.is_empty());
    }

    /// @covers: with_templates
    #[test]
    fn test_with_templates_seeds_registry_keyed_by_id() {
        let provider = CatalogTemplateProvider::with_templates(vec![template("a", "code")]);
        assert_eq!(provider.len(), 1);
    }

    /// @covers: insert
    #[test]
    fn test_insert_replaces_existing_id() {
        let mut provider = CatalogTemplateProvider::new();
        provider.insert(template("a", "code"));
        provider.insert(template("a", "general"));
        assert_eq!(provider.len(), 1);
    }

    /// @covers: len
    #[test]
    fn test_len_reflects_registered_count() {
        let provider = CatalogTemplateProvider::with_templates(vec![template("a", "code")]);
        assert_eq!(provider.len(), 1);
    }

    /// @covers: is_empty
    #[test]
    fn test_is_empty_true_for_new_registry() {
        assert!(CatalogTemplateProvider::new().is_empty());
    }
}

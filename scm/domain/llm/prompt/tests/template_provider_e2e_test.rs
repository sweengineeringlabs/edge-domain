//! End-to-end contract tests for the `TemplateProvider` trait, exercised
//! through the crate's reference implementation via the public API.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_llm_prompt::{
    CatalogTemplateProvider, ListByCategoryRequest, ListTemplatesRequest, PromptTemplate,
    TemplateLookupRequest, TemplateProvider,
};

fn template(id: &str, category: &str) -> PromptTemplate {
    PromptTemplate::new(id.to_string(), id.to_string(), category.to_string())
}

/// @covers: TemplateProvider::get_template
#[test]
fn test_get_template_known_id_returns_template() {
    let provider = CatalogTemplateProvider::with_templates(vec![template("a", "code")]);
    let found = provider
        .get_template(TemplateLookupRequest { id: "a" })
        .expect("get ok")
        .template;
    assert!(found.is_some());
}

/// @covers: TemplateProvider::list_templates
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

/// @covers: TemplateProvider::list_by_category
#[test]
fn test_list_by_category_filters_matching_category() {
    let provider = CatalogTemplateProvider::with_templates(vec![
        template("a", "code"),
        template("b", "general"),
    ]);
    let result = provider
        .list_by_category(ListByCategoryRequest { category: "code" })
        .expect("list ok");
    assert_eq!(result.templates.len(), 1);
}

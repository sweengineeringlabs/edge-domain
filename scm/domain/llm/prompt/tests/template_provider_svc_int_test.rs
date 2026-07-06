//! SAF facade tests — `TemplateProvider` trait via `CatalogTemplateProvider`.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_llm_prompt::{
    ListByCategoryRequest, ListTemplatesRequest, PromptBootstrap, PromptTemplate, StdPromptFactory,
    TemplateLookupRequest, TemplateProvider, TEMPLATE_PROVIDER_SVC,
};

fn template(id: &str, category: &str) -> PromptTemplate {
    PromptTemplate::new(id.to_string(), id.to_string(), category.to_string())
}

fn provider() -> impl TemplateProvider {
    let mut p = StdPromptFactory::template_provider();
    p.insert(template("code-review", "code"));
    p.insert(template("refactor", "code"));
    p.insert(template("explain", "general"));
    p
}

/// @covers: TEMPLATE_PROVIDER_SVC — the service identifier is stable
#[test]
fn test_template_provider_svc_identifier_is_stable() {
    assert_eq!(TEMPLATE_PROVIDER_SVC, "template_provider");
}

// --- get_template ---

/// @covers: TemplateProvider::get_template — returns a registered template
#[test]
fn test_get_template_registered_id_happy() {
    let p = provider();
    assert_eq!(
        p.get_template(TemplateLookupRequest { id: "code-review" })
            .expect("get ok")
            .template
            .expect("present")
            .id,
        "code-review"
    );
}

/// @covers: TemplateProvider::get_template — unknown id returns None
#[test]
fn test_get_template_unknown_id_error() {
    assert!(provider()
        .get_template(TemplateLookupRequest { id: "missing" })
        .expect("get ok")
        .template
        .is_none());
}

/// @covers: TemplateProvider::get_template — empty registry returns None
#[test]
fn test_get_template_empty_registry_edge() {
    let empty = StdPromptFactory::template_provider();
    assert!(empty
        .get_template(TemplateLookupRequest { id: "code-review" })
        .expect("get ok")
        .template
        .is_none());
}

// --- list_templates ---

/// @covers: TemplateProvider::list_templates — returns every registered template
#[test]
fn test_list_templates_returns_all_happy() {
    assert_eq!(
        provider()
            .list_templates(ListTemplatesRequest)
            .expect("list ok")
            .templates
            .len(),
        3
    );
}

/// @covers: TemplateProvider::list_templates — empty registry yields an empty list
#[test]
fn test_list_templates_empty_registry_error() {
    let empty = StdPromptFactory::template_provider();
    assert!(empty
        .list_templates(ListTemplatesRequest)
        .expect("list ok")
        .templates
        .is_empty());
}

/// @covers: TemplateProvider::list_templates — a duplicate id collapses to one entry
#[test]
fn test_list_templates_duplicate_id_collapses_edge() {
    let mut p = StdPromptFactory::template_provider();
    p.insert(template("dup", "code"));
    p.insert(template("dup", "general"));
    assert_eq!(
        p.list_templates(ListTemplatesRequest)
            .expect("list ok")
            .templates
            .len(),
        1
    );
}

// --- list_by_category ---

/// @covers: TemplateProvider::list_by_category — returns only the matching category
#[test]
fn test_list_by_category_matching_happy() {
    let p = provider();
    let code = p
        .list_by_category(ListByCategoryRequest { category: "code" })
        .expect("list ok")
        .templates;
    assert_eq!(code.len(), 2);
    assert!(code.iter().all(|t| t.category == "code"));
}

/// @covers: TemplateProvider::list_by_category — an unknown category is empty
#[test]
fn test_list_by_category_unknown_category_error() {
    assert!(provider()
        .list_by_category(ListByCategoryRequest { category: "nope" })
        .expect("list ok")
        .templates
        .is_empty());
}

/// @covers: TemplateProvider::list_by_category — empty registry is empty
#[test]
fn test_list_by_category_empty_registry_edge() {
    let empty = StdPromptFactory::template_provider();
    assert!(empty
        .list_by_category(ListByCategoryRequest { category: "code" })
        .expect("list ok")
        .templates
        .is_empty());
}

//! Tests for the `CatalogTemplateProvider` reference registry.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_llm_prompt::{CatalogTemplateProvider, PromptTemplate, TemplateProvider};

fn template(id: &str, category: &str) -> PromptTemplate {
    PromptTemplate::new(id.to_string(), id.to_string(), category.to_string())
}

/// @covers: CatalogTemplateProvider::new — starts empty
#[test]
fn test_catalog_template_provider_new_is_empty() {
    let p = CatalogTemplateProvider::new();
    assert!(p.is_empty());
    assert_eq!(p.len(), 0);
}

/// @covers: CatalogTemplateProvider::with_templates — seeds keyed by id
#[test]
fn test_catalog_template_provider_with_templates_seeds_registry() {
    let p = CatalogTemplateProvider::with_templates(vec![
        template("a", "code"),
        template("b", "general"),
    ]);
    assert_eq!(p.len(), 2);
    assert!(p.get_template("a").unwrap());
}

/// @covers: CatalogTemplateProvider::insert — adds and replaces by id
#[test]
fn test_catalog_template_provider_insert_replaces_same_id() {
    let mut p = CatalogTemplateProvider::new();
    p.insert(template("a", "code"));
    p.insert(template("a", "general"));
    assert_eq!(p.len(), 1);
    assert_eq!(p.get_template("a").expect("present").category, "general");
}

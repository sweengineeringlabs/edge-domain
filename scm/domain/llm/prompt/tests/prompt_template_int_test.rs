//! Tests for the `PromptTemplate` value type.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_llm_prompt::PromptTemplate;

/// @covers: PromptTemplate::new — sets identity fields and empty bodies
#[test]
fn test_prompt_template_new_sets_identity_fields() {
    let t = PromptTemplate::new("id".to_string(), "Name".to_string(), "code".to_string());
    assert_eq!(t.id, "id");
    assert_eq!(t.name, "Name");
    assert_eq!(t.category, "code");
    assert!(t.system_prompt.is_empty());
    assert!(t.variables.is_empty());
}

/// @covers: PromptTemplate::metadata — derives id/name and tags the category
#[test]
fn test_prompt_template_metadata_derives_id_and_category_tag() {
    let mut t = PromptTemplate::new("id".to_string(), "Name".to_string(), "code".to_string());
    t.description = Some("docs".to_string());
    let meta = t.metadata();
    assert_eq!(meta.id, "id");
    assert_eq!(meta.name, "Name");
    assert_eq!(meta.tags, vec!["code".to_string()]);
    assert_eq!(meta.description.as_deref(), Some("docs"));
}

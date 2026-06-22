//! Tests for the `PromptTemplateBuilder` type.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_llm_prompt::PromptTemplateBuilder;

/// @covers: PromptTemplateBuilder — builds identity fields
#[test]
fn test_prompt_template_builder_identity_fields() {
    let t = PromptTemplateBuilder::new()
        .id("code-review".to_string())
        .name("Code Review".to_string())
        .category("code".to_string())
        .build();
    assert_eq!(t.id, "code-review");
    assert_eq!(t.name, "Code Review");
    assert_eq!(t.category, "code");
}

/// @covers: PromptTemplateBuilder — carries system/user bodies through
#[test]
fn test_prompt_template_builder_carries_bodies() {
    let t = PromptTemplateBuilder::new()
        .id("t".to_string())
        .system_prompt("you are a reviewer".to_string())
        .user_template("review {{code}}".to_string())
        .build();
    assert_eq!(t.system_prompt, "you are a reviewer");
    assert_eq!(t.user_template, "review {{code}}");
}

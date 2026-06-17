//! Tests for the `PromptCacheBuilder` type.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_llm_prompt::{PromptFactory, StdPromptFactory};

/// @covers: PromptCacheBuilder — builds with key and rendered text
#[test]
fn test_prompt_cache_builder_core_fields() {
    let c = StdPromptFactory::prompt_cache_builder()
        .key("k".to_string())
        .rendered("body".to_string())
        .token_count(4)
        .build();
    assert_eq!(c.key, "k");
    assert_eq!(c.rendered, "body");
}

/// @covers: PromptCacheBuilder — custom TTL is applied
#[test]
fn test_prompt_cache_builder_custom_ttl() {
    let c = StdPromptFactory::prompt_cache_builder()
        .ttl_seconds(120)
        .build();
    assert_eq!(c.ttl_seconds, 120);
}

/// @covers: PromptCacheBuilder — default TTL falls back to one hour
#[test]
fn test_prompt_cache_builder_default_ttl() {
    let c = StdPromptFactory::prompt_cache_builder().build();
    assert_eq!(c.ttl_seconds, 3600);
}

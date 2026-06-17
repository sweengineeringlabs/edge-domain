//! Tests for the `PromptCache` value type.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_llm_prompt::PromptCache;

/// @covers: PromptCache::new — sets key, rendered text, and token count
#[test]
fn test_prompt_cache_new_sets_fields() {
    let c = PromptCache::new("k".to_string(), "body".to_string(), 4);
    assert_eq!(c.key, "k");
    assert_eq!(c.token_count, 4);
}

/// @covers: PromptCache::is_expired — fresh entry is not expired
#[test]
fn test_prompt_cache_fresh_not_expired() {
    let c = PromptCache::new("k".to_string(), "body".to_string(), 4);
    assert!(!c.is_expired());
}

/// @covers: PromptCache::is_expired — zero TTL expires immediately
#[test]
fn test_prompt_cache_zero_ttl_expired() {
    let c = PromptCache::new("k".to_string(), "body".to_string(), 4).with_ttl(0);
    assert!(c.is_expired());
}

/// @covers: PromptCache::record_hit — increments the hit count
#[test]
fn test_prompt_cache_record_hit() {
    let mut c = PromptCache::new("k".to_string(), "body".to_string(), 4);
    c.record_hit();
    assert_eq!(c.hit_count, 1);
}

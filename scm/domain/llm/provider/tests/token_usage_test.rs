use edge_llm_provider::TokenUsage;

#[test]
fn test_token_usage_new_happy() {
    let usage = TokenUsage::new(100, 50, 0, 0);
    assert_eq!(usage.prompt_tokens, 100);
    assert_eq!(usage.completion_tokens, 50);
    assert_eq!(usage.total_tokens, 150);
}

#[test]
fn test_token_usage_new_calculates_total() {
    let usage = TokenUsage::new(200, 100, 0, 0);
    assert_eq!(usage.total_tokens, 300);
}

#[test]
fn test_token_usage_with_cache_read() {
    let usage = TokenUsage::new(100, 50, 80, 0);
    assert_eq!(usage.cache_read_input_tokens, 80);
    assert_eq!(usage.cache_hit(), true);
}

#[test]
fn test_token_usage_with_cache_creation() {
    let usage = TokenUsage::new(100, 50, 0, 120);
    assert_eq!(usage.cache_creation_input_tokens, 120);
    assert_eq!(usage.cache_hit(), false);
}

#[test]
fn test_token_usage_total_with_cache_read() {
    let usage = TokenUsage::new(100, 50, 80, 0);
    assert_eq!(usage.total_with_cache(), 230); // 150 + 80
}

#[test]
fn test_token_usage_total_with_cache_creation() {
    let usage = TokenUsage::new(100, 50, 0, 120);
    assert_eq!(usage.total_with_cache(), 270); // 150 + 120
}

#[test]
fn test_token_usage_total_with_both_cache_ops() {
    let usage = TokenUsage::new(100, 50, 80, 120);
    assert_eq!(usage.total_with_cache(), 350); // 150 + 80 + 120
}

#[test]
fn test_token_usage_cache_hit_happy() {
    let usage = TokenUsage::new(100, 50, 10, 0);
    assert!(usage.cache_hit());
}

#[test]
fn test_token_usage_cache_hit_edge_no_reads() {
    let usage = TokenUsage::new(100, 50, 0, 100);
    assert!(!usage.cache_hit());
}

#[test]
fn test_token_usage_zero_tokens() {
    let usage = TokenUsage::new(0, 0, 0, 0);
    assert_eq!(usage.total_tokens, 0);
    assert!(!usage.cache_hit());
}

#[test]
fn test_token_usage_equality() {
    let usage1 = TokenUsage::new(100, 50, 10, 20);
    let usage2 = TokenUsage::new(100, 50, 10, 20);
    assert_eq!(usage1, usage2);
}

#[test]
fn test_token_usage_inequality() {
    let usage1 = TokenUsage::new(100, 50, 10, 20);
    let usage2 = TokenUsage::new(100, 50, 15, 20);
    assert_ne!(usage1, usage2);
}

#[test]
fn test_token_usage_clone() {
    let usage1 = TokenUsage::new(100, 50, 10, 20);
    let usage2 = usage1.clone();
    assert_eq!(usage1, usage2);
}

#[test]
fn test_token_usage_copy() {
    let usage1 = TokenUsage::new(100, 50, 10, 20);
    let usage2 = usage1;
    assert_eq!(usage1, usage2);
}

#[test]
fn test_token_usage_debug_format() {
    let usage = TokenUsage::new(100, 50, 10, 20);
    let debug_str = format!("{:?}", usage);
    assert!(debug_str.contains("prompt_tokens"));
    assert!(debug_str.contains("completion_tokens"));
}

#[test]
fn test_token_usage_serialization() {
    let usage = TokenUsage::new(100, 50, 10, 20);
    let json = serde_json::to_string(&usage).expect("serialize");
    let deserialized: TokenUsage = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(usage, deserialized);
}

#[test]
fn test_token_usage_hash_consistency() {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let usage1 = TokenUsage::new(100, 50, 10, 20);
    let usage2 = TokenUsage::new(100, 50, 10, 20);

    let mut hasher1 = DefaultHasher::new();
    usage1.hash(&mut hasher1);
    let hash1 = hasher1.finish();

    let mut hasher2 = DefaultHasher::new();
    usage2.hash(&mut hasher2);
    let hash2 = hasher2.finish();

    assert_eq!(hash1, hash2);
}

use edge_llm_complete::TokenUsage;

#[test]
fn test_new_sets_all_fields() {
    let u = TokenUsage::new(100, 50, 150, 20, 10);
    assert_eq!(u.prompt_tokens, 100);
    assert_eq!(u.completion_tokens, 50);
    assert_eq!(u.total_tokens, 150);
    assert_eq!(u.cache_read_input_tokens, 20);
    assert_eq!(u.cache_creation_input_tokens, 10);
}

#[test]
fn test_default_is_all_zeros() {
    let u = TokenUsage::default();
    assert_eq!(u.total_tokens, 0);
}

#[test]
fn test_roundtrip_serialization() {
    let u = TokenUsage::new(1, 2, 3, 4, 5);
    let json = serde_json::to_string(&u).unwrap();
    let back: TokenUsage = serde_json::from_str(&json).unwrap();
    assert_eq!(u, back);
}

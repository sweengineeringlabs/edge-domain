//! Tests for the `TokenCounter` SAF factory marker.
use edge_llm_prompt::TOKEN_COUNTER_SVC_FACTORY;

#[test]
fn test_token_counter_svc_factory_value_happy() {
    assert_eq!(TOKEN_COUNTER_SVC_FACTORY, "token_counter_factory");
}

#[test]
fn test_token_counter_svc_factory_not_empty_error() {
    assert!(!TOKEN_COUNTER_SVC_FACTORY.is_empty());
}

#[test]
fn test_token_counter_svc_factory_no_whitespace_edge() {
    assert!(!TOKEN_COUNTER_SVC_FACTORY.contains(' '));
}

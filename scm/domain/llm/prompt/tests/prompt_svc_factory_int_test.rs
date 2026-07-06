//! Tests for the `Prompt` SAF factory marker.
use edge_llm_prompt::PROMPT_SVC_FACTORY;

#[test]
fn test_prompt_svc_factory_value_happy() {
    assert_eq!(PROMPT_SVC_FACTORY, "prompt_svc_factory");
}

#[test]
fn test_prompt_svc_factory_not_empty_error() {
    assert!(!PROMPT_SVC_FACTORY.is_empty());
}

#[test]
fn test_prompt_svc_factory_no_whitespace_edge() {
    assert!(!PROMPT_SVC_FACTORY.contains(' '));
}

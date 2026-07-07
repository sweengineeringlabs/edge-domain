//! Tests for the `PromptBootstrap` SAF factory marker.
use edge_llm_prompt::PROMPT_FACTORY_SVC_FACTORY;

#[test]
fn test_prompt_bootstrap_svc_factory_value_happy() {
    assert_eq!(PROMPT_FACTORY_SVC_FACTORY, "prompt_factory_factory");
}

#[test]
fn test_prompt_bootstrap_svc_factory_not_empty_error() {
    assert!(!PROMPT_FACTORY_SVC_FACTORY.is_empty());
}

#[test]
fn test_prompt_bootstrap_svc_factory_no_whitespace_edge() {
    assert!(!PROMPT_FACTORY_SVC_FACTORY.contains(' '));
}

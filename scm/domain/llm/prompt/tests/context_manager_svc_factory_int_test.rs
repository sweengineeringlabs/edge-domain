//! Tests for the `ContextManager` SAF factory marker.
use edge_llm_prompt::CONTEXT_MANAGER_SVC_FACTORY;

#[test]
fn test_context_manager_svc_factory_value_happy() {
    assert_eq!(CONTEXT_MANAGER_SVC_FACTORY, "context_manager_factory");
}

#[test]
fn test_context_manager_svc_factory_not_empty_error() {
    assert!(!CONTEXT_MANAGER_SVC_FACTORY.is_empty());
}

#[test]
fn test_context_manager_svc_factory_no_whitespace_edge() {
    assert!(!CONTEXT_MANAGER_SVC_FACTORY.contains(' '));
}

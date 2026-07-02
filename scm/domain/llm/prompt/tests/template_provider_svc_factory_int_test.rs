//! Tests for the `TemplateProvider` SAF factory marker.
use edge_llm_prompt::TEMPLATE_PROVIDER_SVC_FACTORY;

#[test]
fn test_template_provider_svc_factory_value_happy() {
    assert_eq!(TEMPLATE_PROVIDER_SVC_FACTORY, "template_provider_factory");
}

#[test]
fn test_template_provider_svc_factory_not_empty_error() {
    assert!(!TEMPLATE_PROVIDER_SVC_FACTORY.is_empty());
}

#[test]
fn test_template_provider_svc_factory_no_whitespace_edge() {
    assert!(!TEMPLATE_PROVIDER_SVC_FACTORY.contains(' '));
}

//! Scenario coverage for the `complete_factory_svc` SAF surface.

use edge_llm_complete::{CompleteFactory, StdCompleteFactory, COMPLETE_FACTORY_SVC};

#[test]
fn test_complete_factory_svc_constant_is_expected_value_happy() {
    assert_eq!(COMPLETE_FACTORY_SVC, "complete_factory");
}

#[test]
fn test_complete_factory_svc_constant_is_nonempty_error() {
    assert!(!COMPLETE_FACTORY_SVC.is_empty());
}

#[test]
fn test_std_complete_factory_is_accessible_via_svc_surface_edge() {
    let factory = StdCompleteFactory::std_complete_factory();
    let _ = StdCompleteFactory::user_message("test".to_string());
    drop(factory);
}

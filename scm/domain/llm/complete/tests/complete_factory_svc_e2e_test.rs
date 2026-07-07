//! Scenario coverage for the `complete_factory_svc` SAF surface.

use edge_llm_complete::{Message, COMPLETE_FACTORY_SVC};

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
    let msg = Message::user("test".to_string());
    assert_eq!(
        msg.content,
        edge_llm_complete::MessageContent::Text("test".to_string())
    );
}

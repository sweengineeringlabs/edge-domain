//! Scenario coverage for the `cacheable_message_svc` SAF surface.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_llm_complete::{CacheableMessage, Message, CACHEABLE_MESSAGE_SVC};

#[test]
fn test_cacheable_message_svc_constant_is_expected_value_happy() {
    assert_eq!(CACHEABLE_MESSAGE_SVC, "cacheable_message");
}

#[test]
fn test_cacheable_message_svc_constant_is_nonempty_error() {
    assert!(!CACHEABLE_MESSAGE_SVC.is_empty());
}

#[test]
fn test_cacheable_message_trait_accessible_via_svc_surface_edge() {
    use edge_llm_complete::{CacheControl, CacheControlRequest};
    let msg = Message::user("hi");
    let cached = msg
        .with_cache_control(CacheControlRequest {
            cache: Box::new(CacheControl::ephemeral()),
        })
        .unwrap()
        .message;
    assert!(cached.cache_control.is_some());
}

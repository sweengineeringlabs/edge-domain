//! Scenario coverage for the `CacheableMessage` trait (impl for `Message`).
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_llm_complete::{
    CacheControl, CacheControlRequest, CacheableMessage, MarkEphemeralRequest, Message,
    MessageContent, Role,
};

// ── with_cache_control ────────────────────────────────────────────────────────

#[test]
fn test_with_cache_control_attaches_hint_happy() {
    let msg = Message::user("hello")
        .with_cache_control(CacheControlRequest {
            cache: Box::new(CacheControl::ephemeral()),
        })
        .unwrap()
        .message;
    assert_eq!(
        msg.cache_control.as_ref().map(|c| c.cache_type.as_str()),
        Some("ephemeral")
    );
}

#[test]
fn test_with_cache_control_overrides_existing_hint_error() {
    let second = CacheControl::ephemeral();
    let msg = Message::user("x")
        .with_cache_control(CacheControlRequest {
            cache: Box::new(CacheControl::new("persistent")),
        })
        .unwrap()
        .message
        .with_cache_control(CacheControlRequest {
            cache: Box::new(second.clone()),
        })
        .unwrap()
        .message;
    assert_eq!(msg.cache_control, Some(second));
}

#[test]
fn test_with_cache_control_preserves_role_and_content_edge() {
    let msg = Message {
        role: Role::System,
        content: MessageContent::Text("sys".to_string()),
        ..Default::default()
    }
    .with_cache_control(CacheControlRequest {
        cache: Box::new(CacheControl::ephemeral()),
    })
    .unwrap()
    .message;
    assert_eq!(msg.role, Role::System);
    assert_eq!(msg.content, MessageContent::Text("sys".to_string()));
}

// ── mark_ephemeral ────────────────────────────────────────────────────────────

#[test]
fn test_mark_ephemeral_sets_ephemeral_hint_happy() {
    let msg = Message::user("hi")
        .mark_ephemeral(MarkEphemeralRequest)
        .unwrap()
        .message;
    assert_eq!(
        msg.cache_control.as_ref().map(|c| c.cache_type.as_str()),
        Some("ephemeral")
    );
}

#[test]
fn test_mark_ephemeral_on_assistant_message_error() {
    let msg = Message::assistant("ok")
        .mark_ephemeral(MarkEphemeralRequest)
        .unwrap()
        .message;
    let cache_control = msg.cache_control;
    assert!(
        cache_control.is_some(),
        "mark_ephemeral should set cache_control"
    );
    assert_eq!(
        cache_control.unwrap().cache_type,
        "ephemeral",
        "should be ephemeral type"
    );
}

#[test]
fn test_mark_ephemeral_cache_type_matches_ephemeral_constant_edge() {
    let msg = Message::system("s")
        .mark_ephemeral(MarkEphemeralRequest)
        .unwrap()
        .message;
    assert_eq!(msg.cache_control.unwrap().cache_type, "ephemeral");
}

//! Scenario coverage for the `CacheableMessage` trait (impl for `Message`).

use edge_llm_complete::{CacheControl, CacheableMessage, Message, MessageContent, Role};

// ── with_cache_control ────────────────────────────────────────────────────────

#[test]
fn test_with_cache_control_attaches_hint_happy() {
    let msg = Message::user("hello").with_cache_control(CacheControl::ephemeral());
    assert_eq!(
        msg.cache_control.as_ref().map(|c| c.cache_type.as_str()),
        Some("ephemeral")
    );
}

#[test]
fn test_with_cache_control_overrides_existing_hint_error() {
    let second = CacheControl::ephemeral();
    let msg = Message::user("x")
        .with_cache_control(CacheControl::new("persistent"))
        .with_cache_control(second.clone());
    assert_eq!(msg.cache_control, Some(second));
}

#[test]
fn test_with_cache_control_preserves_role_and_content_edge() {
    let msg = Message {
        role: Role::System,
        content: MessageContent::Text("sys".to_string()),
        ..Default::default()
    }
    .with_cache_control(CacheControl::ephemeral());
    assert_eq!(msg.role, Role::System);
    assert_eq!(msg.content, MessageContent::Text("sys".to_string()));
}

// ── mark_ephemeral ────────────────────────────────────────────────────────────

#[test]
fn test_mark_ephemeral_sets_ephemeral_hint_happy() {
    let msg = Message::user("hi").mark_ephemeral();
    assert_eq!(
        msg.cache_control.as_ref().map(|c| c.cache_type.as_str()),
        Some("ephemeral")
    );
}

#[test]
fn test_mark_ephemeral_on_assistant_message_error() {
    let msg = Message::assistant("ok").mark_ephemeral();
    assert!(msg.cache_control.is_some());
}

#[test]
fn test_mark_ephemeral_cache_type_matches_ephemeral_constant_edge() {
    let msg = Message::system("s").mark_ephemeral();
    assert_eq!(msg.cache_control.unwrap().cache_type, "ephemeral");
}

#![allow(clippy::unwrap_used, clippy::expect_used)]
//! Coverage tests for the `Role` value type.

use edge_llm_agent::Role;

#[test]
fn test_role_variants_are_distinct() {
    assert_ne!(Role::System, Role::User);
    assert_ne!(Role::User, Role::Assistant);
    assert_ne!(Role::Assistant, Role::Tool);
}

#[test]
fn test_role_is_copy() {
    let r = Role::User;
    let copied = r;
    assert_eq!(r, copied);
}

#[test]
fn test_role_serde_roundtrip() {
    let json = serde_json::to_string(&Role::Assistant).expect("serialize");
    let back: Role = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(back, Role::Assistant);
}

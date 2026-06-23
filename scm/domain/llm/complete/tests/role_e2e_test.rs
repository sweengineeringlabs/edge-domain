//! Scenario coverage for `Role`.

use edge_llm_complete::Role;

#[test]
fn test_role_user_is_distinct_from_assistant_happy() {
    assert_ne!(Role::User, Role::Assistant);
}

#[test]
fn test_role_system_is_distinct_from_tool_error() {
    assert_ne!(Role::System, Role::Tool);
}

#[test]
fn test_role_all_variants_are_clone_edge() {
    for role in [Role::User, Role::Assistant, Role::System, Role::Tool] {
        let cloned = role.clone();
        assert_eq!(cloned, role);
    }
}

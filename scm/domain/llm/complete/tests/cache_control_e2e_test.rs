//! Scenario coverage for `CacheControl`.

use edge_llm_complete::CacheControl;

#[test]
fn test_cache_control_ephemeral_sets_type_happy() {
    let cc = CacheControl::ephemeral();
    assert_eq!(cc.cache_type, "ephemeral");
}

#[test]
fn test_cache_control_cache_type_is_nonempty_error() {
    let cc = CacheControl::ephemeral();
    assert!(!cc.cache_type.is_empty());
}

#[test]
fn test_cache_control_clone_equals_original_edge() {
    let cc = CacheControl::ephemeral();
    assert_eq!(cc.clone(), cc);
}

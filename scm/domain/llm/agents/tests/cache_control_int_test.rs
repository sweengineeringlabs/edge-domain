#![allow(clippy::unwrap_used, clippy::expect_used)]
//! Coverage tests for the `CacheControl` value type.

use edge_llm_agent::CacheControl;

#[test]
fn test_cache_control_ephemeral_is_ephemeral() {
    assert!(CacheControl::ephemeral().is_ephemeral());
}

#[test]
fn test_cache_control_custom_is_not_ephemeral() {
    assert!(!CacheControl::custom("persistent").is_ephemeral());
}

#[test]
fn test_cache_control_serde_roundtrip() {
    let cc = CacheControl::ephemeral();
    let json = serde_json::to_string(&cc).expect("serialize");
    let back: CacheControl = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(back, cc);
}

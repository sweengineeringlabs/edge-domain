//! SAF facade tests — `Registry` trait via `InMemoryRegistry`.
// @allow: no_mocks_in_integration — InMemoryRegistry is the production in-process reference impl, not a test double
#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::sync::Arc;

use edge_domain_registry::{InMemoryRegistry, Registry, RegistryError};

fn reg() -> InMemoryRegistry<str> {
    InMemoryRegistry::new()
}

// ── register ──────────────────────────────────────────────────────────────────
/// @covers: Registry::register
#[test]
fn test_register_then_get_returns_entry_happy() {
    let r = reg();
    r.register("a", Arc::from("alpha"));
    assert_eq!(r.get("a").as_deref(), Some("alpha"));
}

/// @covers: Registry::register
#[test]
fn test_register_does_not_create_unrelated_entries_error() {
    let r = reg();
    r.register("a", Arc::from("alpha"));
    assert!(r.get("b").is_none());
}

/// @covers: Registry::register
#[test]
fn test_register_same_id_twice_replaces_edge() {
    let r = reg();
    r.register("a", Arc::from("alpha"));
    r.register("a", Arc::from("beta"));
    assert_eq!(r.get("a").as_deref(), Some("beta"));
    assert_eq!(r.len(), 1);
}

// ── try_register ──────────────────────────────────────────────────────────────
/// @covers: Registry::try_register
#[test]
fn test_try_register_new_id_succeeds_happy() {
    let r = reg();
    assert_eq!(r.try_register("a", Arc::from("alpha")), Ok(()));
    assert_eq!(r.get("a").as_deref(), Some("alpha"));
}

/// @covers: Registry::try_register
#[test]
fn test_try_register_duplicate_id_returns_err_error() {
    let r = reg();
    r.register("a", Arc::from("alpha"));
    assert_eq!(
        r.try_register("a", Arc::from("beta")),
        Err(RegistryError::DuplicateId("a".to_string()))
    );
    assert_eq!(r.get("a").as_deref(), Some("alpha"));
}

/// @covers: Registry::try_register
#[test]
fn test_try_register_after_deregister_succeeds_edge() {
    let r = reg();
    r.register("a", Arc::from("alpha"));
    r.deregister("a");
    assert_eq!(r.try_register("a", Arc::from("beta")), Ok(()));
}

// ── deregister ────────────────────────────────────────────────────────────────
/// @covers: Registry::deregister
#[test]
fn test_deregister_existing_removes_and_returns_true_happy() {
    let r = reg();
    r.register("a", Arc::from("alpha"));
    assert!(r.deregister("a"));
    assert!(r.get("a").is_none());
}

/// @covers: Registry::deregister
#[test]
fn test_deregister_missing_returns_false_error() {
    let r = reg();
    assert!(!r.deregister("absent"));
}

/// @covers: Registry::deregister
#[test]
fn test_deregister_twice_second_returns_false_edge() {
    let r = reg();
    r.register("a", Arc::from("alpha"));
    assert!(r.deregister("a"));
    assert!(!r.deregister("a"));
}

// ── get ───────────────────────────────────────────────────────────────────────
/// @covers: Registry::get
#[test]
fn test_get_registered_id_returns_entry_happy() {
    let r = reg();
    r.register("a", Arc::from("alpha"));
    assert_eq!(r.get("a").as_deref(), Some("alpha"));
}

/// @covers: Registry::get
#[test]
fn test_get_unregistered_id_returns_none_error() {
    let r = reg();
    assert!(r.get("absent").is_none());
}

/// @covers: Registry::get
#[test]
fn test_get_after_replace_returns_latest_edge() {
    let r = reg();
    r.register("a", Arc::from("alpha"));
    r.register("a", Arc::from("beta"));
    assert_eq!(r.get("a").as_deref(), Some("beta"));
}

// ── list_ids ──────────────────────────────────────────────────────────────────
/// @covers: Registry::list_ids
#[test]
fn test_list_ids_returns_all_registered_happy() {
    let r = reg();
    r.register("a", Arc::from("alpha"));
    r.register("b", Arc::from("beta"));
    let mut ids = r.list_ids();
    ids.sort();
    assert_eq!(ids, vec!["a".to_string(), "b".to_string()]);
}

/// @covers: Registry::list_ids
#[test]
fn test_list_ids_empty_registry_returns_empty_error() {
    let r = reg();
    assert!(r.list_ids().is_empty());
}

/// @covers: Registry::list_ids
#[test]
fn test_list_ids_excludes_deregistered_edge() {
    let r = reg();
    r.register("a", Arc::from("alpha"));
    r.register("b", Arc::from("beta"));
    r.deregister("a");
    assert_eq!(r.list_ids(), vec!["b".to_string()]);
}

// ── len ───────────────────────────────────────────────────────────────────────
/// @covers: Registry::len
#[test]
fn test_len_counts_registered_entries_happy() {
    let r = reg();
    r.register("a", Arc::from("alpha"));
    r.register("b", Arc::from("beta"));
    assert_eq!(r.len(), 2);
}

/// @covers: Registry::len
#[test]
fn test_len_empty_registry_is_zero_error() {
    let r = reg();
    assert_eq!(r.len(), 0);
}

/// @covers: Registry::len
#[test]
fn test_len_unchanged_after_replace_edge() {
    let r = reg();
    r.register("a", Arc::from("alpha"));
    r.register("a", Arc::from("beta"));
    assert_eq!(r.len(), 1);
}

// ── is_empty ──────────────────────────────────────────────────────────────────
/// @covers: Registry::is_empty
#[test]
fn test_is_empty_new_registry_is_true_happy() {
    let r = reg();
    assert!(r.is_empty());
}

/// @covers: Registry::is_empty
#[test]
fn test_is_empty_after_register_is_false_error() {
    let r = reg();
    r.register("a", Arc::from("alpha"));
    assert!(!r.is_empty());
}

/// @covers: Registry::is_empty
#[test]
fn test_is_empty_true_again_after_clearing_edge() {
    let r = reg();
    r.register("a", Arc::from("alpha"));
    r.deregister("a");
    assert!(r.is_empty());
}

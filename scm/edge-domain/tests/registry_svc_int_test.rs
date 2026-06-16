//! Umbrella-level integration tests that exercise `edge-domain-registry` as a
//! dependency — verifying the sub-crate contract is accessible end-to-end.
// @allow: no_mocks_in_integration — InMemoryRegistry is a reference implementation in the public API, not a mock.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_domain_registry::{InMemoryRegistry, Registry, RegistryFactory, StdRegistryFactory};

/// @covers: Registry::register, Registry::get
#[test]
fn test_in_memory_registry_register_and_get_happy() {
    let registry: InMemoryRegistry<String, i32> = InMemoryRegistry::new();
    registry.register("key1".to_string(), 42).expect("register should succeed");
    assert_eq!(registry.get("key1").expect("get should succeed"), Some(42));
}

/// @covers: Registry::get (nonexistent key)
#[test]
fn test_in_memory_registry_get_nonexistent_key_returns_none_happy() {
    let registry: InMemoryRegistry<String, i32> = InMemoryRegistry::new();
    assert_eq!(registry.get("missing").expect("get should succeed"), None);
}

/// @covers: Registry::register (duplicate key)
#[test]
fn test_in_memory_registry_register_duplicate_key_overwrites_happy() {
    let registry: InMemoryRegistry<String, i32> = InMemoryRegistry::new();
    registry.register("key1".to_string(), 42).expect("first register should succeed");
    registry.register("key1".to_string(), 100).expect("second register should succeed");
    assert_eq!(registry.get("key1").expect("get should succeed"), Some(100));
}

/// @covers: Registry::unregister
#[test]
fn test_in_memory_registry_unregister_removes_entry_happy() {
    let registry: InMemoryRegistry<String, i32> = InMemoryRegistry::new();
    registry.register("key1".to_string(), 42).expect("register should succeed");
    registry.unregister("key1").expect("unregister should succeed");
    assert_eq!(registry.get("key1").expect("get should succeed"), None);
}

/// @covers: Registry::list
#[test]
fn test_in_memory_registry_list_returns_all_registered_keys_happy() {
    let registry: InMemoryRegistry<String, i32> = InMemoryRegistry::new();
    registry.register("key1".to_string(), 42).expect("register should succeed");
    registry.register("key2".to_string(), 100).expect("register should succeed");

    let keys = registry.list().expect("list should succeed");
    assert_eq!(keys.len(), 2);
    assert!(keys.contains(&"key1".to_string()));
    assert!(keys.contains(&"key2".to_string()));
}

/// @covers: InMemoryRegistry construction
#[test]
fn test_in_memory_registry_new_creates_empty_registry_happy() {
    let registry: InMemoryRegistry<String, String> = InMemoryRegistry::new();
    let keys = registry.list().expect("list should succeed");
    assert!(keys.is_empty());
}

/// @covers: Registry::register (multiple items)
#[test]
fn test_in_memory_registry_register_multiple_items_happy() {
    let registry: InMemoryRegistry<i32, String> = InMemoryRegistry::new();
    for i in 0..10 {
        registry.register(i, format!("value_{}", i)).expect("register should succeed");
    }

    let keys = registry.list().expect("list should succeed");
    assert_eq!(keys.len(), 10);
}

/// @covers: RegistryFactory trait usage
#[test]
fn test_std_registry_factory_creates_registry_happy() {
    let factory = StdRegistryFactory;
    let registry = factory.create().expect("create should succeed");

    registry.register("key1".to_string(), 42).expect("register should succeed");
    assert_eq!(registry.get("key1").expect("get should succeed"), Some(42));
}

/// @covers: Registry::unregister (nonexistent key)
#[test]
fn test_in_memory_registry_unregister_nonexistent_key_edge() {
    let registry: InMemoryRegistry<String, i32> = InMemoryRegistry::new();
    registry.unregister("missing").expect("unregister should succeed");
    assert_eq!(registry.get("missing").expect("get should succeed"), None);
}

/// @covers: Registry::list (empty state)
#[test]
fn test_in_memory_registry_list_empty_returns_no_items_edge() {
    let registry: InMemoryRegistry<String, String> = InMemoryRegistry::new();
    let keys = registry.list().expect("list should succeed");
    assert!(keys.is_empty());
}

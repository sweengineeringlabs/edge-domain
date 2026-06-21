//! Umbrella-level integration tests that exercise `edge-domain-registry` as a
//! dependency — verifying the sub-crate contract is accessible end-to-end.
// @allow: no_mocks_in_integration — InMemoryRegistry is a reference implementation in the public API, not a mock.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_domain_registry::{InMemoryRegistry, Registry, RegistryBootstrap};
use std::sync::Arc;

struct TestFactory;
impl RegistryBootstrap for TestFactory {}

/// @covers: Registry::register, Registry::get
#[test]
fn test_in_memory_registry_register_and_get_happy() {
    let registry: InMemoryRegistry<i32> = TestFactory::in_memory();
    registry.register("key1", Arc::new(42));
    assert_eq!(registry.get("key1").map(|v| *v), Some(42));
}

/// @covers: Registry::get (nonexistent key)
#[test]
fn test_in_memory_registry_get_nonexistent_key_returns_none_happy() {
    let registry: InMemoryRegistry<i32> = TestFactory::in_memory();
    assert_eq!(registry.get("missing"), None);
}

/// @covers: Registry::register (duplicate key)
#[test]
fn test_in_memory_registry_register_duplicate_key_overwrites_happy() {
    let registry: InMemoryRegistry<i32> = TestFactory::in_memory();
    registry.register("key1", Arc::new(42));
    registry.register("key1", Arc::new(100));
    assert_eq!(registry.get("key1").map(|v| *v), Some(100));
}

/// @covers: Registry::deregister
#[test]
fn test_in_memory_registry_deregister_removes_entry_happy() {
    let registry: InMemoryRegistry<i32> = TestFactory::in_memory();
    registry.register("key1", Arc::new(42));
    let removed = registry.deregister("key1");
    assert!(removed);
    assert_eq!(registry.get("key1"), None);
}

/// @covers: Registry::list_ids
#[test]
fn test_in_memory_registry_list_ids_returns_all_registered_keys_happy() {
    let registry: InMemoryRegistry<i32> = TestFactory::in_memory();
    registry.register("key1", Arc::new(42));
    registry.register("key2", Arc::new(100));

    let keys = registry.list_ids();
    assert_eq!(keys.len(), 2);
    assert!(keys.contains(&"key1".to_string()));
    assert!(keys.contains(&"key2".to_string()));
}

/// @covers: InMemoryRegistry construction
#[test]
fn test_in_memory_registry_new_creates_empty_registry_happy() {
    let registry: InMemoryRegistry<String> = TestFactory::in_memory();
    let keys = registry.list_ids();
    assert!(keys.is_empty());
}

/// @covers: Registry::register (multiple items)
#[test]
fn test_in_memory_registry_register_multiple_items_happy() {
    let registry: InMemoryRegistry<String> = TestFactory::in_memory();
    for i in 0..10 {
        registry.register(&format!("key{}", i), Arc::new(format!("value_{}", i)));
    }

    let keys = registry.list_ids();
    assert_eq!(keys.len(), 10);
}

/// @covers: RegistryBootstrap construction
#[test]
fn test_registry_factory_creates_registry_happy() {
    let registry: InMemoryRegistry<i32> = TestFactory::in_memory();
    registry.register("key1", Arc::new(42));
    assert_eq!(registry.get("key1").map(|v| *v), Some(42));
}

/// @covers: Registry::deregister (nonexistent key)
#[test]
fn test_in_memory_registry_deregister_nonexistent_key_returns_false_edge() {
    let registry: InMemoryRegistry<i32> = TestFactory::in_memory();
    let removed = registry.deregister("missing");
    assert!(!removed);
}

/// @covers: Registry::list_ids (empty state)
#[test]
fn test_in_memory_registry_list_ids_empty_returns_no_items_edge() {
    let registry: InMemoryRegistry<String> = TestFactory::in_memory();
    let keys = registry.list_ids();
    assert!(keys.is_empty());
}

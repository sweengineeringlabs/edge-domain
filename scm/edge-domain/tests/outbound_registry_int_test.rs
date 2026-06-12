//! Integration tests for `OutboundRegistry`.

use edge_domain::OutboundRegistry;

/// @covers: OutboundRegistry::new
#[test]
fn test_outbound_registry_struct_new_creates_empty_registry() {
    let reg: OutboundRegistry<String> = OutboundRegistry::new();
    assert!(reg.is_empty());
    assert_eq!(reg.len(), 0);
}

/// @covers: OutboundRegistry::register
#[test]
fn test_outbound_registry_struct_register_stores_handle() {
    let reg: OutboundRegistry<String> = OutboundRegistry::new();
    reg.register("svc", "url".to_string());
    assert_eq!(reg.get("svc").as_deref(), Some("url"));
}

/// @covers: OutboundRegistry::deregister
#[test]
fn test_outbound_registry_struct_deregister_removes_handle() {
    let reg: OutboundRegistry<String> = OutboundRegistry::new();
    reg.register("svc", "url".to_string());
    assert!(reg.deregister("svc"));
    assert!(reg.get("svc").is_none());
}

/// @covers: OutboundRegistry::names
#[test]
fn test_outbound_registry_struct_names_returns_all_registered() {
    let reg: OutboundRegistry<u32> = OutboundRegistry::new();
    reg.register("a", 1);
    reg.register("b", 2);
    let mut names = reg.names();
    names.sort();
    assert_eq!(names, vec!["a", "b"]);
}

/// @covers: OutboundRegistry::len
#[test]
fn test_outbound_registry_struct_len_reflects_count() {
    let reg: OutboundRegistry<u32> = OutboundRegistry::new();
    assert_eq!(reg.len(), 0);
    reg.register("x", 1);
    assert_eq!(reg.len(), 1);
}

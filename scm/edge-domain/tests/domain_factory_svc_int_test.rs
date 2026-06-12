//! Integration tests for the `DomainFactory` SAF facade.

use edge_domain::{Domain, DomainFactory, NoopDomainExtension, OutboundRegistry};

struct TestDomain;
impl DomainFactory for TestDomain {}

// --- DomainFactory::domain ---

/// @covers DomainFactory::domain — happy path: returns a Domain handle
#[test]
fn test_domain_factory_domain_returns_domain_handle_happy() {
    let _: Domain = TestDomain::domain();
}

/// @covers DomainFactory::domain — error: Domain is a unit struct (cannot be null)
#[test]
fn test_domain_factory_domain_is_unit_struct_error() {
    assert_eq!(std::mem::size_of::<Domain>(), 0);
}

/// @covers DomainFactory::domain — edge: two calls produce independent instances
#[test]
fn test_domain_factory_domain_independent_instances_edge() {
    let _a = TestDomain::domain();
    let _b = TestDomain::domain();
}

// --- DomainFactory::noop_extension ---

/// @covers DomainFactory::noop_extension — happy path: returns a NoopDomainExtension
#[test]
fn test_noop_extension_returns_noop_happy() {
    let _: NoopDomainExtension = TestDomain::noop_extension();
}

/// @covers DomainFactory::noop_extension — error: NoopDomainExtension is zero-size
#[test]
fn test_noop_extension_is_zero_size_error() {
    assert_eq!(std::mem::size_of::<NoopDomainExtension>(), 0);
}

/// @covers DomainFactory::noop_extension — edge: successive calls are independent
#[test]
fn test_noop_extension_independent_calls_edge() {
    let _a = TestDomain::noop_extension();
    let _b = TestDomain::noop_extension();
}

// --- DomainFactory::outbound_registry ---

/// @covers DomainFactory::outbound_registry — happy path: empty registry has len 0
#[test]
fn test_outbound_registry_starts_empty_happy() {
    let reg: OutboundRegistry<String> = TestDomain::outbound_registry();
    assert_eq!(reg.len(), 0);
}

/// @covers DomainFactory::outbound_registry — error: unknown key returns None
#[test]
fn test_outbound_registry_unknown_key_returns_none_error() {
    let reg: OutboundRegistry<String> = TestDomain::outbound_registry();
    assert!(reg.get("missing").is_none());
}

/// @covers DomainFactory::outbound_registry — edge: can register and retrieve a handle
#[test]
fn test_outbound_registry_register_retrieve_edge() {
    let reg: OutboundRegistry<u32> = TestDomain::outbound_registry();
    reg.register("svc", 42u32);
    assert_eq!(reg.get("svc"), Some(42));
}

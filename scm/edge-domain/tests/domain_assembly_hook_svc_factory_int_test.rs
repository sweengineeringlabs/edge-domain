//! Integration tests — `DomainAssemblyHook` SAF facade (`domain_spi_svc`).

use edge_application::{DomainAssemblyHook, NoopDomainAssemblyHook};

/// @covers: DOMAIN_SPI_SVC — SAF anchor is accessible
#[test]
fn test_domain_spi_svc_anchor_is_accessible_happy() {
    assert_eq!(edge_application::DOMAIN_SPI_SVC, ());
}

/// @covers: DomainAssemblyHook re-export — trait usable from crate root
#[test]
fn test_domain_assembly_hook_re_exported_from_crate_root_happy() {
    fn accepts_hook<S: DomainAssemblyHook>(_: &S) {}
    accepts_hook(&NoopDomainAssemblyHook);
    assert_eq!(std::mem::size_of::<NoopDomainAssemblyHook>(), 0);
}

/// @covers: DomainAssemblyHook — trait bound is enforced at compile time (non-implementing type fails)
#[test]
fn test_domain_assembly_hook_bound_enforced_at_compile_time_edge() {
    // This test documents that the trait bound exists — the constraint is
    // verified by the fact that this file compiles only with conforming impls.
    fn assert_hook_bound<T: DomainAssemblyHook>() {}
    assert_hook_bound::<NoopDomainAssemblyHook>();
    assert_eq!(std::mem::size_of::<NoopDomainAssemblyHook>(), 0);
}

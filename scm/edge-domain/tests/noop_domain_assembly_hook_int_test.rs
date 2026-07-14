//! Integration tests for `NoopDomainAssemblyHook`.

use edge_application::{DomainAssemblyHook, NoopDomainAssemblyHook};

/// @covers: NoopDomainAssemblyHook
#[test]
fn test_noop_domain_assembly_hook_is_zero_sized_happy() {
    assert_eq!(std::mem::size_of::<NoopDomainAssemblyHook>(), 0);
}

/// @covers: NoopDomainAssemblyHook
#[test]
fn test_noop_domain_assembly_hook_satisfies_trait_bound_error() {
    fn assert_hook_bound<T: DomainAssemblyHook>() {}
    assert_hook_bound::<NoopDomainAssemblyHook>();
    assert_eq!(std::mem::size_of::<NoopDomainAssemblyHook>(), 0);
}

/// @covers: NoopDomainAssemblyHook
#[test]
fn test_noop_domain_assembly_hook_constructible_repeatedly_edge() {
    let a = NoopDomainAssemblyHook;
    let b = NoopDomainAssemblyHook;
    assert_eq!(std::mem::size_of_val(&a), 0);
    assert_eq!(std::mem::size_of_val(&b), 0);
}

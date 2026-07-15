//! Integration tests — `DomainAssemblyHook` trait.

use edge_application::{DomainAssemblyHook, NoopDomainAssemblyHook};

/// @covers: DomainAssemblyHook — NoopDomainAssemblyHook satisfies the DomainAssemblyHook contract
#[test]
fn test_noop_assembly_hook_satisfies_domain_assembly_hook_happy() {
    fn accepts_hook<S: DomainAssemblyHook>(_: S) {}
    accepts_hook(NoopDomainAssemblyHook);
    assert_eq!(std::mem::size_of::<NoopDomainAssemblyHook>(), 0);
}

/// @covers: DomainAssemblyHook — trait is object-safe when wrapped in Arc
#[test]
fn test_domain_assembly_hook_is_object_safe_error() {
    use std::sync::Arc;
    let _: Arc<dyn DomainAssemblyHook> = Arc::new(NoopDomainAssemblyHook);
}

/// @covers: DomainAssemblyHook — Send + Sync bounds hold for NoopDomainAssemblyHook
#[test]
fn test_domain_assembly_hook_is_send_sync_edge() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<NoopDomainAssemblyHook>();
    assert_eq!(std::mem::size_of::<NoopDomainAssemblyHook>(), 0);
}

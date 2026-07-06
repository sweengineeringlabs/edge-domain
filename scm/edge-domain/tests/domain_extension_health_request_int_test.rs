//! Integration tests for `DomainExtensionHealthRequest`.

use edge_domain::{
    DomainError, DomainExtension, DomainExtensionHealthRequest, NoopDomainExtension,
};

/// @covers: DomainExtensionHealthRequest
#[test]
fn test_domain_extension_health_request_is_zero_sized_happy() {
    assert_eq!(std::mem::size_of::<DomainExtensionHealthRequest>(), 0);
}

struct FailingExtension;
impl DomainExtension for FailingExtension {
    fn health(&self, _req: DomainExtensionHealthRequest) -> Result<(), DomainError> {
        Err(DomainError::ExtensionRejected("down".into()))
    }
}

/// @covers: DomainExtensionHealthRequest
#[test]
fn test_domain_extension_health_request_rejected_by_failing_extension_error() {
    let result = FailingExtension.health(DomainExtensionHealthRequest);
    assert!(matches!(result, Err(DomainError::ExtensionRejected(msg)) if msg == "down"));
}

/// @covers: DomainExtensionHealthRequest
#[test]
fn test_domain_extension_health_request_constructible_repeatedly_edge() {
    let first = NoopDomainExtension
        .health(DomainExtensionHealthRequest)
        .is_ok();
    let second = NoopDomainExtension
        .health(DomainExtensionHealthRequest)
        .is_ok();
    assert!(first, "first call must succeed");
    assert!(second, "second call must succeed");
}

//! Contract tests for the DomainExtension marker trait.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use edge_application::{DomainExtension, DomainExtensionHealthRequest, NoopDomainExtension};

/// @covers: DomainExtension
#[test]
fn test_noop_domain_extension_satisfies_domain_extension_contract() {
    fn accepts_extension<E: DomainExtension>(_: E) {}
    accepts_extension(NoopDomainExtension);
    assert_eq!(std::mem::size_of::<NoopDomainExtension>(), 0);
}

/// @covers: DomainExtension::health — noop implementation always returns Ok
#[test]
fn test_health_noop_extension_returns_ok_happy() {
    NoopDomainExtension
        .health(DomainExtensionHealthRequest)
        .unwrap();
    assert_eq!(std::mem::size_of::<NoopDomainExtension>(), 0);
}

/// @covers: DomainExtension::health — custom implementation can return Err
#[test]
fn test_health_failing_extension_returns_err_error() {
    use edge_application::DomainError;

    struct FailingExtension;
    impl DomainExtension for FailingExtension {
        fn health(&self, _req: DomainExtensionHealthRequest) -> Result<(), DomainError> {
            Err(DomainError::ExtensionRejected("unavailable".into()))
        }
    }
    let result = FailingExtension.health(DomainExtensionHealthRequest);
    assert!(matches!(result, Err(DomainError::ExtensionRejected(msg)) if msg == "unavailable"));
}

/// @covers: DomainExtension::health — calling health twice is idempotent
#[test]
fn test_health_called_twice_is_idempotent_edge() {
    let first_result = NoopDomainExtension.health(DomainExtensionHealthRequest);
    let second_result = NoopDomainExtension.health(DomainExtensionHealthRequest);
    assert_eq!(format!("{first_result:?}"), "Ok(())");
    assert_eq!(format!("{second_result:?}"), "Ok(())");
}

//! Contract tests for the DomainExtension marker trait.

use edge_domain::{DomainExtension, NoopDomainExtension};

/// @covers: DomainExtension
#[test]
fn test_noop_domain_extension_satisfies_domain_extension_contract() {
    fn accepts_extension<E: DomainExtension>(_: E) {}
    accepts_extension(NoopDomainExtension);
}

/// @covers: DomainExtension::health — noop implementation always returns Ok
#[test]
fn test_health_noop_extension_returns_ok_happy() {
    let result = NoopDomainExtension.health();
    assert_eq!(result, Ok(()));
}

/// @covers: DomainExtension::health — custom implementation can return Err
#[test]
fn test_health_failing_extension_returns_err_error() {
    use edge_domain::DomainError;

    struct FailingExtension;
    impl DomainExtension for FailingExtension {
        fn health(&self) -> Result<(), DomainError> {
            Err(DomainError::ExtensionRejected("unavailable".into()))
        }
    }
    let result = FailingExtension.health();
    assert!(matches!(result, Err(DomainError::ExtensionRejected(msg)) if msg == "unavailable"));
}

/// @covers: DomainExtension::health — calling health twice is idempotent
#[test]
fn test_health_called_twice_is_idempotent_edge() {
    let first_result = NoopDomainExtension.health();
    let second_result = NoopDomainExtension.health();
    assert_eq!(first_result, Ok(()));
    assert_eq!(second_result, Ok(()));
}

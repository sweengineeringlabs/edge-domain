//! Integration tests — `NoopAppSvcFactory` type.

use edge_domain_app::{AppServiceProvider, NoopAppSvcFactory};

/// @covers: NoopAppSvcFactory — build returns a working bootstrap
#[test]
fn test_noop_app_svc_factory_build_returns_bootstrap_happy() {
    let factory = NoopAppSvcFactory;
    let app = factory.build().build().expect("NoopAppSvcFactory must produce a buildable bootstrap");
    assert_eq!(app.name(), "application");
}

/// @covers: NoopAppSvcFactory — name is stable
#[test]
fn test_noop_app_svc_factory_name_is_stable_error() {
    let factory = NoopAppSvcFactory;
    assert_eq!(factory.name(), "app_service_provider");
}

/// @covers: NoopAppSvcFactory — is Copy
#[test]
fn test_noop_app_svc_factory_is_copy_edge() {
    let a = NoopAppSvcFactory;
    let b = a;
    assert_eq!(a, b);
}

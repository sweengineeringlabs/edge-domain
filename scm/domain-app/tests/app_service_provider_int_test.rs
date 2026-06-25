//! Integration tests — `AppServiceProvider` trait.

use edge_domain_app::{AppServiceProvider, Bootstrap, NoopAppBootstrap, NoopAppSvcFactory};

struct PassthroughProvider(NoopAppBootstrap);

impl AppServiceProvider for PassthroughProvider {
    fn build(&self) -> Box<dyn Bootstrap> {
        Box::new(self.0)
    }
}

/// @covers: AppServiceProvider::build — built bootstrap resolves application successfully
#[test]
fn test_build_returns_bootstrap_that_builds_application_happy() {
    let provider = PassthroughProvider(NoopAppBootstrap);
    let bootstrap = provider.build();
    let app = bootstrap.build().expect("bootstrap must succeed");
    assert_eq!(app.name(), "application");
}

/// @covers: AppServiceProvider::build — build always succeeds (no error path for noop)
#[test]
fn test_build_bootstrap_is_infallible_for_noop_error() {
    let provider = NoopAppSvcFactory;
    let bootstrap = provider.build();
    let app = bootstrap.build().expect("noop provider bootstrap must build");
    assert_eq!(app.name(), "application");
}

/// @covers: AppServiceProvider::noop — factory method returns NoopAppSvcFactory
#[test]
fn test_noop_returns_noop_app_svc_factory_edge() {
    let f: NoopAppSvcFactory = NoopAppSvcFactory::noop();
    assert_eq!(format!("{f:?}"), "NoopAppSvcFactory");
}

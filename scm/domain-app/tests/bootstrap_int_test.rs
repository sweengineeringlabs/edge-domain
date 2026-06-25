//! Integration tests — `Bootstrap` trait via SAF facade.

use edge_domain_app::{AppError, Application, Bootstrap};
use futures::executor::block_on;
use futures::future::BoxFuture;

struct EchoApp;
impl Application for EchoApp {
    fn name(&self) -> &str { "echo" }
    fn run(&self) -> BoxFuture<'_, Result<(), AppError>> {
        Box::pin(async { Ok(()) })
    }
}

struct AlwaysBuilds;
impl Bootstrap for AlwaysBuilds {
    fn build(&self) -> Result<Box<dyn Application>, AppError> {
        Ok(Box::new(EchoApp))
    }
}

struct NeverBuilds;
impl Bootstrap for NeverBuilds {
    fn build(&self) -> Result<Box<dyn Application>, AppError> {
        Err(AppError::CreationFailed("not wired".into()))
    }
}

/// @covers: Bootstrap::build — success path
#[test]
fn test_build_returns_application_happy() {
    let result = AlwaysBuilds.build();
    assert!(result.is_ok());
    let app = result.unwrap();
    assert_eq!(app.name(), "echo");
}

/// @covers: Bootstrap::build — failure propagates error
#[test]
fn test_build_fails_with_creation_error_error() {
    let result = NeverBuilds.build();
    assert!(result.is_err());
    let msg = match result {
        Err(e) => e.to_string(),
        Ok(_) => panic!("expected error"),
    };
    assert_eq!(msg, "service creation failed: not wired");
}

/// @covers: Bootstrap::build — produced application is runnable
#[test]
fn test_build_application_runs_successfully_edge() {
    let app = AlwaysBuilds.build().unwrap();
    assert_eq!(block_on(app.run()), Ok(()));
}

/// @covers: Bootstrap::noop — returns NoopAppBootstrap that builds successfully
#[test]
fn test_noop_returns_noop_bootstrap_happy() {
    use edge_domain_app::NoopAppBootstrap;
    let noop: NoopAppBootstrap = AlwaysBuilds::noop();
    let app = noop.build().expect("noop bootstrap should build");
    assert_eq!(app.name(), "application");
}

/// @covers: Bootstrap::noop — built application name differs from caller's custom app
#[test]
fn test_noop_bootstrap_build_returns_default_name_error() {
    use edge_domain_app::NoopAppBootstrap;
    let noop: NoopAppBootstrap = AlwaysBuilds::noop();
    let app = noop.build().expect("build succeeds");
    assert_ne!(app.name(), "echo");
    assert_eq!(app.name(), "application");
}

/// @covers: Bootstrap::noop — NoopAppBootstrap is Copy
#[test]
fn test_noop_bootstrap_is_copy_edge() {
    use edge_domain_app::NoopAppBootstrap;
    let noop: NoopAppBootstrap = AlwaysBuilds::noop();
    let copy = noop;
    let app = copy.build().expect("copy builds ok");
    assert_eq!(app.name(), "application");
}

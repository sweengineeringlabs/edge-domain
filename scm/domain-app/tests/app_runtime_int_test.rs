//! Integration tests — `AppRuntime` trait.

use edge_domain_app::{AppError, AppRuntime, Application, Bootstrap, NoopAppBootstrap, NoopAppRuntime};
use futures::executor::block_on;
use futures::future::BoxFuture;

struct FailBootstrap;

impl Bootstrap for FailBootstrap {
    fn build(&self) -> Result<Box<dyn Application>, AppError> {
        Err(AppError::BootFailed("forced failure".into()))
    }
}

/// @covers: AppRuntime::boot — successful boot via noop bootstrap
#[test]
fn test_boot_noop_bootstrap_completes_ok_happy() {
    let runtime = NoopAppRuntime;
    let result = block_on(runtime.boot(&NoopAppBootstrap));
    assert_eq!(result, Ok(()));
}

/// @covers: AppRuntime::boot — boot propagates bootstrap build failure
#[test]
fn test_boot_propagates_bootstrap_failure_error() {
    struct DirectRuntime;
    impl AppRuntime for DirectRuntime {
        fn boot<'a>(
            &'a self,
            bootstrap: &'a dyn Bootstrap,
        ) -> BoxFuture<'a, Result<(), AppError>> {
            Box::pin(async move {
                let app = bootstrap.build()?;
                app.run().await
            })
        }
    }
    let runtime = DirectRuntime;
    let result = block_on(runtime.boot(&FailBootstrap));
    assert_eq!(result, Err(AppError::BootFailed("forced failure".into())));
}

/// @covers: AppRuntime::boot — booting a second time after a first successful boot is idempotent
#[test]
fn test_boot_idempotent_second_call_succeeds_edge() {
    let runtime = NoopAppRuntime;
    let r1 = block_on(runtime.boot(&NoopAppBootstrap));
    let r2 = block_on(runtime.boot(&NoopAppBootstrap));
    assert_eq!(r1, Ok(()));
    assert_eq!(r2, Ok(()));
}

/// @covers: AppRuntime::noop — factory method returns NoopAppRuntime
#[test]
fn test_noop_returns_noop_app_runtime_edge() {
    let r: NoopAppRuntime = NoopAppRuntime::noop();
    assert_eq!(format!("{r:?}"), "NoopAppRuntime");
}

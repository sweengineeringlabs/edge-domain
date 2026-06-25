use futures::future::BoxFuture;

use crate::api::AppError;
use crate::api::AppRuntime;
use crate::api::Bootstrap;
use crate::api::NoopAppRuntime;

impl AppRuntime for NoopAppRuntime {
    fn boot<'a>(
        &'a self,
        _bootstrap: &'a dyn Bootstrap,
    ) -> BoxFuture<'a, Result<(), AppError>> {
        Box::pin(async { Ok(()) })
    }
}

#[cfg(test)]
mod tests {
    use futures::executor::block_on;

    use super::*;
    use crate::api::Application;

    struct NoopAppRuntimeStubBootstrap;
    impl Bootstrap for NoopAppRuntimeStubBootstrap {
        fn build(&self) -> Result<Box<dyn Application>, AppError> {
            Err(AppError::BootFailed("stub".into()))
        }
    }

    #[test]
    fn test_boot_noop_always_ok_happy() {
        let r = NoopAppRuntime;
        let b = NoopAppRuntimeStubBootstrap;
        assert_eq!(block_on(r.boot(&b)), Ok(()));
    }

    #[test]
    fn test_boot_noop_ignores_bootstrap_error_error() {
        let r = NoopAppRuntime;
        let b = NoopAppRuntimeStubBootstrap;
        assert_eq!(block_on(r.boot(&b)), Ok(()));
    }

    #[test]
    fn test_name_returns_app_runtime_edge() {
        assert_eq!(NoopAppRuntime.name(), "app_runtime");
    }
}

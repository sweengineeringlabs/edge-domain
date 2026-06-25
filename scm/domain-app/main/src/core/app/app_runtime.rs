//! `AppRuntime` impl for [`DefaultAppRuntime`] — executes Bootstrap → Application → run.

use futures::future::BoxFuture;

use crate::api::AppError;
use crate::api::AppRuntime;
use crate::api::Bootstrap;

pub(crate) struct DefaultAppRuntime;

impl AppRuntime for DefaultAppRuntime {
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

#[cfg(test)]
mod tests {
    use futures::executor::block_on;

    use super::*;
    use crate::api::{AppRuntime, NoopAppBootstrap};

    #[test]
    fn test_boot_ok_bootstrap_completes_happy() {
        assert_eq!(block_on(DefaultAppRuntime.boot(&NoopAppBootstrap)), Ok(()));
    }

    #[test]
    fn test_boot_called_twice_both_complete_error() {
        let r = DefaultAppRuntime;
        assert_eq!(block_on(r.boot(&NoopAppBootstrap)), Ok(()));
        assert_eq!(block_on(r.boot(&NoopAppBootstrap)), Ok(()));
    }

    #[test]
    fn test_name_returns_app_runtime_edge() {
        assert_eq!(DefaultAppRuntime.name(), "app_runtime");
    }
}

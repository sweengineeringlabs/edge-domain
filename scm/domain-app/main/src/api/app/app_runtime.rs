//! [`AppRuntime`] — the mandatory boot gate for an edge application.

use futures::future::BoxFuture;

use crate::api::AppError;
use crate::api::Bootstrap;
use crate::api::NoopAppRuntime;

/// Enforces the canonical boot sequence: `Bootstrap::build` → `Application::run`.
///
/// Every edge application MUST enter through this gate.  Callers may not
/// construct an [`Application`](crate::api::Application) and call `run` directly;
/// they must go through an `AppRuntime` implementation so the boot sequence is
/// auditable and replaceable without changing call sites.
pub trait AppRuntime: Send + Sync {
    /// Stable identifier for this runtime.
    fn name(&self) -> &str {
        "app_runtime"
    }

    /// Execute the full boot gate: build an application via `bootstrap`, then run it.
    ///
    /// Propagates any error from `Bootstrap::build` or `Application::run`.
    fn boot<'a>(
        &'a self,
        bootstrap: &'a dyn Bootstrap,
    ) -> BoxFuture<'a, Result<(), AppError>>;

    /// Return the no-operation runtime for tests and default wiring.
    fn noop() -> NoopAppRuntime
    where
        Self: Sized,
    {
        NoopAppRuntime
    }
}

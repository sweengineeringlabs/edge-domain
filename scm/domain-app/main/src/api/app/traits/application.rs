//! `Application` trait — the boot gate for an edge application.

use futures::future::BoxFuture;

use crate::api::AppError;
use crate::api::NoopApplication;

/// The top-level lifecycle contract for an edge application.
///
/// Implementors bring up all subsystems and hold the application open
/// until shutdown is signalled.
pub trait Application: Send + Sync {
    /// Stable identifier for this application.
    fn name(&self) -> &str {
        "application"
    }

    /// Boot the application.
    ///
    /// Resolves when the application terminates or encounters a fatal error.
    fn run(&self) -> BoxFuture<'_, Result<(), AppError>>;

    /// Return a no-operation application for testing or default wiring.
    fn noop() -> NoopApplication
    where
        Self: Sized,
    {
        NoopApplication
    }
}

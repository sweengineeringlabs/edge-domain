//! `Bootstrap` trait — produces [`Application`] instances from a wired service graph.

use crate::api::AppError;
use crate::api::Application;
use crate::api::NoopAppBootstrap;

/// Constructs an [`Application`] from a resolved service graph.
///
/// Implementors wire all subsystems and return a ready-to-run application.
pub trait Bootstrap: Send + Sync {
    /// Build and return a ready-to-run [`Application`].
    fn build(&self) -> Result<Box<dyn Application>, AppError>;

    /// Return a no-operation bootstrap for testing or default wiring.
    fn noop() -> NoopAppBootstrap
    where
        Self: Sized,
    {
        NoopAppBootstrap
    }
}

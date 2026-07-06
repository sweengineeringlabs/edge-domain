//! [`BootstrapNameResponse`] — wrapper for a bootstrap implementation's identifier.

/// Result of [`LifecycleBootstrap::bootstrap_name`](crate::api::lifecycle::traits::LifecycleBootstrap::bootstrap_name).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BootstrapNameResponse {
    /// Stable identifier for this implementation.
    pub name: &'static str,
}

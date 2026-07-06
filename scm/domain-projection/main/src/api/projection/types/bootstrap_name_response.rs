//! [`BootstrapNameResponse`] — wrapper for a bootstrap implementation's identifier.

/// Result of [`ProjectionBootstrap::bootstrap_name`](crate::api::projection::traits::ProjectionBootstrap::bootstrap_name).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BootstrapNameResponse {
    /// Stable identifier for this implementation.
    pub name: &'static str,
}

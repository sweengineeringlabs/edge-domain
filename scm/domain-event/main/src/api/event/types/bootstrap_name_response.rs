//! [`BootstrapNameResponse`] — wrapper for a bootstrap implementation's identifier.

/// Result of [`EventBootstrap::bootstrap_name`](crate::api::EventBootstrap::bootstrap_name).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BootstrapNameResponse {
    /// Stable identifier for this implementation.
    pub name: &'static str,
}

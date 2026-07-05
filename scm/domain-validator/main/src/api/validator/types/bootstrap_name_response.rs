//! [`BootstrapNameResponse`] — wrapper for a bootstrap implementation's identifier.

/// Result of [`ValidatorBootstrap::bootstrap_name`](crate::api::ValidatorBootstrap::bootstrap_name).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BootstrapNameResponse {
    /// Stable identifier for this bootstrap implementation.
    pub name: &'static str,
}

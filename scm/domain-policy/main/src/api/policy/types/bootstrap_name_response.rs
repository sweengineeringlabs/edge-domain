//! [`BootstrapNameResponse`] — wrapper for a bootstrap implementation's identifier.

/// Result of [`PolicyBootstrap::bootstrap_name`](crate::api::policy::traits::PolicyBootstrap::bootstrap_name).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BootstrapNameResponse {
    /// Stable identifier for this implementation.
    pub name: &'static str,
}

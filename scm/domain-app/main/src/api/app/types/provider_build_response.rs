//! [`ProviderBuildResponse`] — wrapper for a wired `Bootstrap`.

use crate::api::Bootstrap;

/// Result of [`AppServiceProvider::build`](crate::api::AppServiceProvider::build).
pub struct ProviderBuildResponse {
    /// The configured bootstrap produced from the wired service graph.
    pub bootstrap: Box<dyn Bootstrap>,
}

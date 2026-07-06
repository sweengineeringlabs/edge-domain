//! [`ApplicationBuildResponse`] — wrapper for a ready-to-run `Application`.

use crate::api::Application;

/// Result of [`Bootstrap::build`](crate::api::Bootstrap::build).
pub struct ApplicationBuildResponse {
    /// The ready-to-run application.
    pub application: Box<dyn Application>,
}

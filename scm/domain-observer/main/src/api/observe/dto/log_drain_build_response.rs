//! [`LogDrainBuildResponse`] — wrapper for a constructed `LogDrain`.

use crate::api::LogDrain;

/// Result of [`ObserveBootstrap::build_log_drain`](crate::api::ObserveBootstrap::build_log_drain).
pub struct LogDrainBuildResponse {
    /// The constructed log drain.
    pub log_drain: Box<dyn LogDrain>,
}

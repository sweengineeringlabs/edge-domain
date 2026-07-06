//! [`TryDrainResponse`] — wrapper for the number of events drained.

/// Result of [`ProjectionBootstrap::try_drain`](crate::api::projection::traits::ProjectionBootstrap::try_drain).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TryDrainResponse {
    /// The number of events folded into the projection.
    pub count: usize,
}

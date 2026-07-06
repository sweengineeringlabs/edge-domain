//! [`EmptinessResponse`] — wrapper for the empty-check result.

/// Result of [`Registry::is_empty`](crate::api::Registry::is_empty).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EmptinessResponse {
    /// `true` if no entries are registered.
    pub empty: bool,
}

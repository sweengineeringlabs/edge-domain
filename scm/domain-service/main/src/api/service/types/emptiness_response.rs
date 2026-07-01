//! [`EmptinessResponse`] — wrapper for empty-check result.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EmptinessResponse {
    pub empty: bool,
}

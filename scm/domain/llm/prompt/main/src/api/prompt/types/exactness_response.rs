//! [`ExactnessResponse`] — response for [`TokenCounter::is_exact`](crate::api::prompt::traits::TokenCounter::is_exact).

/// Whether token counting is exact rather than an estimate.
#[derive(Debug, PartialEq)]
pub struct ExactnessResponse {
    /// `true` if counting is exact.
    pub exact: bool,
}

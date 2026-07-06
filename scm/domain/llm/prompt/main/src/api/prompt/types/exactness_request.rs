//! [`ExactnessRequest`] — request for [`TokenCounter::is_exact`](crate::api::prompt::traits::TokenCounter::is_exact).

/// Request to check whether counting is exact. Carries no data.
#[derive(Debug, PartialEq)]
pub struct ExactnessRequest;

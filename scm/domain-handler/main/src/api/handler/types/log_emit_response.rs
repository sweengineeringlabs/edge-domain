//! [`LogEmitResponse`] — wrapper for a successful log emission.

/// Result of [`LogDrain::emit`](crate::api::handler::traits::LogDrain::emit).
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct LogEmitResponse;

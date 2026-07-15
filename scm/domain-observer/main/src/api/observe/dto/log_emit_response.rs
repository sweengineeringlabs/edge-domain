//! [`LogEmitResponse`] — wrapper for a successful log emission.

/// Result of [`LogDrain::emit`](crate::api::LogDrain::emit).
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct LogEmitResponse;

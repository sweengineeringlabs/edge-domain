//! [`IncrementRequest`] — request to increment a counter.

/// Request to increment a [`Counter`](crate::api::Counter) by `delta`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct IncrementRequest {
    /// The amount to increment by.
    pub delta: u64,
}

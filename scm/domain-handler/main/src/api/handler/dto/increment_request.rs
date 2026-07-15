//! [`IncrementRequest`] — request to increment a counter.

/// Request to increment a [`Counter`](crate::api::handler::traits::Counter) by `delta`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IncrementRequest {
    /// The amount to increment by.
    pub delta: u64,
}

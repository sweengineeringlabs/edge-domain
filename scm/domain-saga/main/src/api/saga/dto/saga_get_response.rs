//! [`SagaGetResponse`] — wrapper borrowing the saga instance found by id.

/// Result of [`SagaStore::get`](crate::api::saga::traits::SagaStore::get).
#[derive(Debug)]
pub struct SagaGetResponse<'a, S> {
    /// The saga registered under the requested id.
    pub saga: &'a S,
}

//! [`SagaGetRequest`] — request identifying a saga instance by id.

/// Request to borrow the saga registered under `id`.
pub struct SagaGetRequest<'a, Id> {
    /// The identifier to look up.
    pub id: &'a Id,
}

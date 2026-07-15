//! [`ProjectionReadModelResponse`] — wrapper for the current read model.

/// Result of [`Projection::read_model`](crate::api::projection::traits::Projection::read_model).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProjectionReadModelResponse<'a, R> {
    /// The current read model.
    pub read_model: &'a R,
}

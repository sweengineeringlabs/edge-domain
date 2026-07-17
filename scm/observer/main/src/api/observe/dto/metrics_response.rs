//! [`MetricsResponse`] — wrapper for the active `MetricRegistry`.
// @allow: dto_types_must_serialize — holds a `&dyn MetricRegistry` reference, not
// wire-format data; a trait object reference cannot derive Serialize/Deserialize.

use crate::api::MetricRegistry;

/// Result of [`ObserverContext::metrics`](crate::api::ObserverContext::metrics).
pub struct MetricsResponse<'a> {
    /// The active metric registry.
    pub metrics: &'a dyn MetricRegistry,
}

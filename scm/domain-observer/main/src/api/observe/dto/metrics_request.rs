//! [`MetricsRequest`] — zero-sized marker for querying the active `MetricRegistry`.

/// Request for the active [`MetricRegistry`](crate::api::MetricRegistry).
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct MetricsRequest;

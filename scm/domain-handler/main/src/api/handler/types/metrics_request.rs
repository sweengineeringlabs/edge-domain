//! [`MetricsRequest`] — input for [`ObserverContext::metrics`](crate::api::handler::traits::ObserverContext::metrics).

/// Marker request; `metrics` takes no data beyond `&self`.
#[derive(Debug, Clone, Copy, Default)]
pub struct MetricsRequest;
